/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <memory.h>
#include <stdlib.h>

#include "qcry_arbiter.h"
#include <mbedtls/platform.h>
#include "qcry_helper.h"
#include "qcry_context.h"
#include "qcry_helper.h"
#include "qcry_keys.h"
#include "qcry_keystore.h"

/***********
 *
 * Static data required for arbiter to do it's work. Instead of passing a context around
 * the libary a static context is used to store data about multi users. Tokens are used
 * to identity user sessions and different threads to prevent race conditions.
 *
 * This API is completely tread safe. Therefore calling lower level functions (for example
 * key gens or context functions) can and will lead to race conditions. DO NOT DO THAT!
 *
 */

/** Maps session tokens to contexts. One context can be referenced multiple times */
typedef struct {
    int                     *user_no;
    struct qcry_arbit_token *token;
    qcry_usr_ctx            *ctx;
} arbiter_user;

typedef struct {

    /** Multi-User contexts */
    arbiter_user        **usr_list;
    size_t              users, max;

    char                *path;

    /** Key generator context */
    qcry_keys_context   *keygen;
} qcry_arbit_ctx ;

/** Static reference to our main arbiter context **/
static qcry_arbit_ctx *arbiter;
static unsigned int session_ctr;

/** Inline macro that's used to verify that the arbiter context we're operating on is valid **/
#define SANE_ARBIT int ret; if(arbiter == NULL || arbiter->keygen == NULL || arbiter->max < 0) return QCRY_STATUS_INVALID_CTX;
#define USER_OK if(usrno > arbiter->users) return QCRY_STATUS_INVALID_USERNO;
#define TARGET_OK if(target_no > arbiter->usr_list[usrno]->ctx->usd_trgt || target_no < 0) return QCRY_STATUS_INVALID_TARGET;

/************* FORWARD DECLARED PRIVATE UTILITY FUNCTINS BELOW **************/
int init_key_write(mbedtls_pk_context *key, const char *path, const char *username, const char *passphrase);

int load_keypair(mbedtls_pk_context **pub, mbedtls_pk_context **pri,
                 const char *path, const char *username, const char *passphrase);

/*****************************************************************************/

int qcry_arbit_init(unsigned int max_concurrent, const char *path, struct qcry_usr_id **known_keys)
{
    /** Cleanly allocate memory */
    arbiter = (qcry_arbit_ctx*) calloc(sizeof(qcry_arbit_ctx), 1);

    /** Initialise our keygenerator context */
    arbiter->keygen = (qcry_keys_context*) malloc(sizeof(qcry_keys_context));
    if(arbiter->keygen == NULL)
        return QCRY_STATUS_MALLOC_FAIL;
    qcry_keys_init(arbiter->keygen);

    /** Initialise our context and token lists */
    arbiter->usr_list = (arbiter_user**) calloc(sizeof(arbiter_user*), MIN_BFR_S);
    if(arbiter->usr_list == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    arbiter->max = MIN_BFR_S;
    arbiter->users = 0;

    /* Save our config path ourselves */
    size_t path_len = strlen(path) + 1; // Include space for \0
    arbiter->path = (char*) malloc(path_len * sizeof(char));
    strcpy(arbiter->path, path);

    /** Set session counter to 0 */
    session_ctr = 0;

    /** Load keys from disk into keystore **/
    char keystore_path[512];
    size_t p_s = strlen(path);

    if(strcmp(&path[p_s - 1], "/") != 0)
        mbedtls_snprintf(keystore_path, sizeof(keystore_path), "%s/%s", path, "keystore");
    else
        mbedtls_snprintf(keystore_path, sizeof(keystore_path), "%s%s", path, "keystore");

     qcry_ks_init(keystore_path, (struct qcry_usr_id**) known_keys, 0);

    /** Then return all OK */
    return QCRY_STATUS_OK;
}

int qcry_arbit_free()
{
    SANE_ARBIT
    int retval;

    /* Make sure the key store is freed */
    retval = qcry_ks_free();
    if(!retval) return QCRY_STATUS_ERROR;

    /** Free key generator */
    retval = qcry_keys_free(arbiter->keygen);
    if(!retval) return QCRY_STATUS_ERROR;

    int i;
    for(i = 0; i <= arbiter->users; i++) {
        qcry_context_free(arbiter->usr_list[i]->ctx);
    }

    return QCRY_STATUS_OK;
}

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(int *user_number, const char *username, const char *passphrase, unsigned int key_type)
{
    SANE_ARBIT

    char *fingerprint;

    /** First allocate space for our new user in the arbiter context */
    arbiter_user *item = (arbiter_user*) malloc(sizeof(arbiter_user));
    if(item == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    item->ctx = (qcry_usr_ctx*) malloc(sizeof(qcry_usr_ctx));
    if(item->ctx == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    /** Initialise the user context for use with RSA keys */
    ret = qcry_context_init(item->ctx, username, PK_RSA);
    if(ret != 0) {
        printf("Context init failed with code %d\n", ret);
        return QCRY_STATUS_INVALID_CTX;
    }

    if(arbiter->users >= arbiter->max) {
        /* This means we need to increase the user buffer size */

        arbiter->max += 2;
        arbiter->usr_list = (arbiter_user**) realloc(arbiter->usr_list, sizeof(arbiter_user) * arbiter->max);
    }

    /* Now we can add the user safely */
    *user_number = (int) arbiter->users++;
    arbiter->usr_list[*user_number] = item;

    mbedtls_pk_context *comb;
    ret = qcry_keys_rsagen(arbiter->keygen, &comb, username);
    if(ret != 0) {
        printf("A critical error occured while creating RSA keypair: %d\n", ret);
        return QCRY_STATUS_KEYGEN_FAILED;
    }

    /** Save the combined part to disk */
    ret = init_key_write(comb, arbiter->path, username, passphrase);
    if(ret != 0) {
        printf("Writing keypair to disk for buffer failed due to: %d\n", ret);
        return QCRY_STATUS_FATAL;
    }

    /* Free combined key - we will never need it again */
    mbedtls_pk_free(comb);

    /** Load split key contexts */
    mbedtls_pk_context *pri, *pub;
    ret = load_keypair(&pub, &pri, arbiter->path, username, passphrase);
    if(ret != 0) {
        printf("Loading keypair from buffer failed: %d\n", ret);
        return QCRY_STATUS_FATAL;
    }

    /** Then attach our private key to the user context */
    ret = qcry_context_attach(item->ctx, pub, pri);
    if(ret != 0) {
        printf("Initialising user context FAILED! > %d <\n", ret);
        return ret;
    }

    // If we made it this far we can accept :)
    return QCRY_STATUS_OK;
}

int qcry_arbit_getusrinfo(char *(*buffer), int usrno, int type)
{
    SANE_ARBIT

    USER_OK

    switch(type) {
        case QAUL_FINGERPRINT:
            (*buffer) = arbiter->usr_list[usrno]->ctx->fingerprint;
            return QCRY_STATUS_OK;

        case QAUL_PUBKEY:
            {
                size_t buf_s = 16000;
                unsigned char output_buf[buf_s];
                memset(output_buf, 0, buf_s);

                /* Write public key part to buffer */
                mbedtls_pk_context *pub = arbiter->usr_list[usrno]->ctx->public;
                ret = mbedtls_pk_write_pubkey_pem(pub, output_buf, 16000);
                if(ret != 0)
                    return QCRY_STATUS_INVALID_KEYS;

                /* Allocate some memory for our buffer and copy the key */
                (*buffer) = (char*) calloc(sizeof(char), strlen((char *) output_buf) + 1); // Consider \0 !
                strcpy((char *) *buffer, (char *)output_buf);
            }
            return QCRY_STATUS_OK;

        default:
            (*buffer) = NULL;
            return QCRY_STATUS_INVALID_PARAMS;
    }

    return QCRY_STATUS_OK;
}

int qcry_arbit_usrdestroy(int usrno)
{
    SANE_ARBIT

    return QCRY_STATUS_NOT_IMPLEMENTED;
}

/**
 *
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint,  int usrno)
{
    SANE_ARBIT

    return QCRY_STATUS_NOT_IMPLEMENTED;
}

/**
 * This function takes a user identifier (username) and their private passphrase to restore
 * the keys that are stored on disk for them.
 *
 * @param username: Username space to unlock
 * @param passphrase: A passphrase used to encrypt the keys
 */
int qcry_arbit_restore(int *usrno, const char *username, const char *passphrase)
{
    SANE_ARBIT

    /** First allocate space for our new user in the arbiter context */
    arbiter_user *item = (arbiter_user*) calloc(sizeof(arbiter_user), 1);
    item->ctx = (qcry_usr_ctx*) malloc(sizeof(qcry_usr_ctx));

    /** Initialise the user context for use with RSA keys */
    ret = qcry_context_init(item->ctx, username, PK_RSA);
    if(ret != 0) {
        printf("Context init failed with code %d", ret);
        return QCRY_STATUS_INVALID_CTX;
    }

    /* Check if we have to increase the user buffer size */
    if(arbiter->users >= arbiter->max) {
        arbiter->max += 2;
        arbiter->usr_list = (arbiter_user**) realloc(arbiter->usr_list, sizeof(arbiter_user) * arbiter->max);
    }

    /* Now we can add the user safely */
    *usrno = (int) arbiter->users++;
    arbiter->usr_list[*usrno] = item;

    /* Attempt to load keypair off disk */
    mbedtls_pk_context *pri, *pub;
    ret = load_keypair(&pub, &pri, arbiter->path, username, passphrase);
    if(ret) {
        printf("Failed to load keypair for user %s\n", username);
        return ret;
    }

    ret = qcry_context_attach(item->ctx, pub, pri);
    if(ret) {
        printf("Failed to attach keypair to new user context!\n");
        return ret;
    }

    return QCRY_STATUS_OK;
}

int qcry_arbit_signmsg(int usrno, unsigned char *(*sgn_buffer), const char *message)
{
    /* Make sure our environment is sane */
    SANE_ARBIT USER_OK

    /* Store usr locally for easy handling */
    arbiter_user *usr = arbiter->usr_list[usrno];

    /* Call the context handle for sign message */
    ret = qcry_context_signmsg(usr->ctx, message, sgn_buffer);
    if(ret != 0) {
        printf("An error occured while signing the message: %d\n", ret);
        return ret;
    }

    return QCRY_STATUS_OK;
}

int qcry_arbit_verify(int usrno, int target_no, const char *message, const unsigned char *signature)
{
    /* Make sure our environment is sane */
    SANE_ARBIT

    USER_OK

    TARGET_OK

    if(strlen(message) >= QAUL_MAX_MSG_LENGTH) {
        printf("Message provided exceeds maximum signable message!\n");
        return QCRY_STATUS_INVALID_PARAMS;
    }

    /* Store usr locally for easy handling */
    arbiter_user *usr = arbiter->usr_list[usrno];
    bool ok = false;

    ret = qcry_context_verifymsg(usr->ctx, (unsigned int) target_no, message, signature, &ok);
    if(ret != 0) {
        printf("An error occured while checking a signature: %d!\n", ret);
        return ret;
    }

    /** Check if our signature was OK and return an apropriate code back to the developer **/
    return ok ? QCRY_STATUS_OK : QCRY_STATUS_SIGN_BOGUS;
}

int qcry_arbit_start(int usrno, const char *fingerprint)
{
    /* Make sure our environment is sane */
    SANE_ARBIT USER_OK

    /* Store usr locally for easy handling */
    arbiter_user *usr = arbiter->usr_list[usrno];

    /* We need to check if the target already exists */
    int tused = usr->ctx->usd_trgt;
    qcry_trgt_t **targets = usr->ctx->trgts;

    int i;
    for(i = 0; i < tused; i++) {
        qcry_trgt_t *t = targets[i];

        /* No need to add again if exists */
        if(strcmp(t->fingerprint, fingerprint) == 0) {
            return QCRY_STATUS_TARGET_EXISTS;
        }
    }

    /* If we reached this point it means the target didn't yet exist. Safely add it */
    qcry_trgt_t *t;
    ret = qcry_context_mktarget(&t, fingerprint);
    if(ret != 0) return ret;

    /* Attach our new target to the user context */
    ret = qcry_context_add_trgt(usr->ctx, t, PK_RSA);
    if(ret != 0) return ret;

    /* Signal ok */
    return QCRY_STATUS_OK;
}

int qcry_arbit_stop(int usrno, const char *fingerprint)
{
    /* Make sure our environment is sane */
    SANE_ARBIT USER_OK

    arbiter_user *user = arbiter->usr_list[usrno];
    int target_no = -1;

    /* Try to find fingerprint */
    int i;
    for(i = 0; i < user->ctx->usd_trgt; i++) {

        /* If the fingerprints match ... */
        if(strcmp((char*) user->ctx->trgts[i]->fingerprint, fingerprint) == 0) {
            target_no = i;

            /* If target is sane --> Cleanup */
            TARGET_OK goto rem_target;
        }
    }

    rem_target:
    ret = qcry_context_remove_trgt(user->ctx, (unsigned int) target_no);

    /* Return with confidence :) */
    return ret;
}

int qcry_arbit_addkey(const char *keybody, size_t key_len, const char *fingerprint, const char *username)
{
    SANE_ARBIT

    /* Allocate memory for a new key */
    mbedtls_pk_context *ctx = (mbedtls_pk_context*) malloc(sizeof(mbedtls_pk_context));
    mbedtls_pk_init(ctx);

    /* Parse the key from the buffer provided */
    ret = mbedtls_pk_parse_public_key(ctx, (unsigned char*) keybody, key_len);
    if(ret != 0)
        return QCRY_STATUS_INVALID_KEYS;

    /* Then simply add the key to our collection */
    ret = qcry_ks_save(ctx, fingerprint, username);
    return ret;
}


/*************************** PRIVATE UTILITY FUNCTIONS BELOW **************************/


// FIXME: Encrypt the private key !!!
int init_key_write(mbedtls_pk_context *key, const char *path, const char *username, const char *passphrase)
{
    if(key == NULL)
        return QCRY_STATUS_ERROR;

    printf("Keypair stashing...");
    fflush(stdout);
    size_t p_s = strlen(path);
    size_t u_s = strlen(username);

    /* Create an array and make sure it's REALLY empty */
    char write_path[p_s + u_s + 4];
    memset(write_path, 0, p_s + u_s + 4);

    /* Copy the path into it and add a slash between folder and user if required */
    strcpy(write_path, path);
    if(strcmp(&path[p_s - 1], "/") != 0) strcat(write_path, "/");
    strcat(write_path, "00_"); // TODO: Get number of keys from somewhere?
    strcat(write_path, username);

    /***************** PREPARE KEY WRITE *****************/

    int ret;
    FILE *f;
    size_t buf_s = 16000;
    unsigned char output_buf[buf_s];
    unsigned char *c = output_buf;
    char new_filename[strlen(write_path) + strlen(".ext")];

    size_t len = 0;

    /***************** WRITE PUBLIC KEY *****************/

    /** Clear Buffer and write into it  */
    memset(output_buf, 0, buf_s);
    ret = mbedtls_pk_write_pubkey_pem(key, output_buf, 16000);
    if(ret != 0) return(ret);

    /** Get the exact length of what we're supposed to write */
    len = strlen((char *) output_buf);

    memset(new_filename, 0, sizeof(new_filename));
    strcpy(new_filename, write_path);
    strcat(new_filename, ".pub");

    if((f = fopen(new_filename, "w")) == NULL) {
        printf("FAILED\n");
        return -1;
    }

    if(fwrite(c, 1, len, f) != len) {
        printf("FAILED\n");
        fclose(f);
        return -1;
    }

    fclose(f);

    /***************** WRITE PRIVATE KEY *****************/

    /** Clear Buffer and write into it  */
    memset(output_buf, 0, buf_s);
    ret = mbedtls_pk_write_key_pem(key, output_buf, buf_s);
    if(ret != 0) return(ret);

    len = strlen((char *) output_buf);

    memset(new_filename, 0, sizeof(new_filename));
    strcpy(new_filename, write_path);
    strcat(new_filename, ".key");

    if ((f = fopen(new_filename, "wb")) == NULL) {
        printf("FAILED\n");
        return (-1);
    }

    if (fwrite(c, 1, len, f) != len) {
        printf("FAILED\n");
        fclose(f);
        return (-1);
    }

    fclose(f);
    printf("OK\n");

    /***************** RETURN SUCCESS :) *****************/

    return 0;
}

/**
 *
 * @param pub Pointer to reference future public key
 * @param pri Pointer to reference future private key
 * @param path Path to the keystore (usually ~/.qaul/keys/)
 * @param username Username the keys belong to
 *
 * @return Status code for errors
 */
// FIXME: CAN'T LOAD ENCRYPTED PRIVATE KEYS YET!
int load_keypair(mbedtls_pk_context **pub, mbedtls_pk_context **pri,
                 const char *path, const char *username, const char *passphrase)
{
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    (*pub) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context),  1);
    if(*pub == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    (*pri) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context), 1);
    if(*pri == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    /*** Initialise the key contexts properly ***/
    mbedtls_pk_init(*pub);
    mbedtls_pk_init(*pri);

    /*** Construct the required file names ***/
    char pri_pth[512];
    char pub_pth[512];

    size_t p_s = strlen(path);
    size_t u_s = strlen(username);

    /** Build private key path **/
    if(strcmp(&path[p_s - 1], "/") != 0)
        mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s/00_%s.key", path, username);
    else
        mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s00_%s.key", path, username);

    /* Build public key path */
    if(strcmp(&path[p_s - 1], "/") != 0)
        mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/00_%s.pub", path, username);
    else
        mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s00_%s.pub", path, username);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("Parsing public key...");
    fflush(stdout);

    ret = mbedtls_pk_parse_public_keyfile(*pub, pub_pth);
    if(ret != 0) {
        mbedtls_printf("FAILED! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret);
        goto cleanup;
    }
    mbedtls_printf("OK\n");

    mbedtls_printf("Parsing private key...");
    fflush(stdout);

    ret = mbedtls_pk_parse_keyfile(*pri, pri_pth, "" );
    if(ret != 0) {
        mbedtls_printf("\"FAILED! mbedtls_pk_parse_keyfile returned -0x%04x\n", -ret);
        goto cleanup;
    }
    mbedtls_printf("OK\n");

    printf("== Keys loaded successfully ==\n\n");
    return 0;

    cleanup:
    mbedtls_pk_free(*pub);
    mbedtls_pk_free(*pri);
    return ret;
}