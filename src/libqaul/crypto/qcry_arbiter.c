/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <memory.h>
#include <stdlib.h>

#include <qaullib/qcry_arbiter.h>
#include <mbedtls/platform.h>
#include "qcry_helper.h"
#include "qcry_context.h"
#include "qcry_helper.h"
#include "qcry_keys.h"

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
#define SANE_ARBIT(to_return) if(arbiter == NULL || arbiter->keygen == NULL || arbiter->max < 0) return to_return;

/************* FORWARD DECLARED PRIVATE UTILITY FUNCTINS BELOW **************/
int init_key_write(mbedtls_pk_context *key, const char *path, const char *username, const char *passphrase);

int load_keypair(mbedtls_pk_context **pub, mbedtls_pk_context **pri,
                  const char *path, const char *username, const char *passphrase);

qcry_usr_ctx *get_ctx_with_token(struct qcry_arbit_token *token);

unsigned char *create_token();

/*****************************************************************************/

int qcry_arbit_init(unsigned int max_concurrent, const char *path)
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
    size_t path_len = strlen(path) + sizeof(char); // Include space for \0
    arbiter->path = (char*) malloc(path_len * sizeof(char));
    strcpy(arbiter->path, path);

    /** Set session counter to 0 */
    session_ctr = 0x0;

    /** Then return all OK */
    return QCRY_STATUS_OK;
}

int qcry_arbit_free()
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)
    int retval;

    /** Free key generator */
    retval = qcry_keys_free(arbiter->keygen);
    if(!retval)
        return QCRY_STATUS_ERROR;

    int i;
    for(i = 0; i <= arbiter->users; i++)
    {
        // qcry_context_free(arbiter->usr_list[i]->ctx);
    }

    return QCRY_STATUS_OK;
}

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(const char *username, const char *passphrase, unsigned int key_type)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    int ret;
    char *fingerprint;

    /** First allocate space for our new user in the arbiter context */
    arbiter_user *item = (arbiter_user*) calloc(sizeof(arbiter_user), 1);
    item->ctx = (qcry_usr_ctx*) malloc(sizeof(qcry_usr_ctx));

    /** Initialise the user context for use with RSA keys */
    ret = qcry_context_init(item->ctx, username, PK_RSA);
    if(ret != 0) {
        printf("Context init failed with code %d", ret);
        return QCRY_STATUS_INVALID_CTX;
    }

    /** Create a token and a session ID for this user */
//    item->token = (struct qcry_arbit_token*) calloc(sizeof(struct qcry_arbit_token), 1);
//    item->token->sess_id = (int) session_ctr++;
//
//    /** Copy token and release old pointer */
//    unsigned char *token = create_token();
//    memcpy(item->token->token, token, 256);
//    free(token);

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
    free(comb);

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
        return QCRY_STATUS_INVALID_CTX;
    }

    // If we made it this far we can accept :)
    return QCRY_STATUS_OK;
}

int qcry_arbit_getusrinfo(const char *(*fingerprint), const char *username)
{

}

int qcry_arbit_usrdestroy(const char *fingerprint)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

/**
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint, struct qcry_arbit_token *token)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

/**
 * This function takes a user identifier (username) and their private passphrase to restore
 * the keys that are stored on disk for them.
 *
 * @param username: Username space to unlock
 * @param passphrase: A passphrase used to encrypt the keys
 */
int qcry_arbit_restore(const char *username, const char *passphrase)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

/**
 * Starts a "session" between a local user (as a fingerprint) and a remote user (as a fingerprint).
 * Fingerprints are used in the crypto engine to identify keys and outside the crypto module to map
 * users to routing data
 */
int qcry_arbit_start(const char *fp_self, const char *fp_trgt, struct qcry_arbit_token *(*token))
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

/**
 * Stops a session with a token.
 */
int qcry_arbit_stop(struct qcry_arbit_token *token)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

int qcry_arbit_sendmsg(struct qcry_arbit_token *token, char *(*encrypted), const char *plain)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

int qcry_arbit_parsemsg(struct qcry_arbit_token *token, char *(*parsed), const char *encrypted)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

int qcry_arbit_signmsg(struct qcry_arbit_token *token, char *(*sgn_buffer), const char *message)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}

/**
 * Verify the validity of a signature on a message cryptographically. Provide the session token and an active context
 * as well as a message and the elegid signature to verify the pair.
 *
 * Returns 0 if signature could be verified.
 * Returns -1 if signature was faulty
 * Returns 1...255 for runtime errors
 */
int qcry_arbit_verify(struct qcry_arbit_token *token, const char *message, const char *signature)
{
    SANE_ARBIT(QCRY_STATUS_INVALID_CTX)

    return QCRY_STATUS_OK;
}


/*************************** PRIVATE UTILITY FUNCTIONS BELOW **************************/

qcry_usr_ctx *get_ctx_with_token(struct qcry_arbit_token *token)
{
    SANE_ARBIT(NULL)

    int i;
    for(i = 0; i <= arbiter->users; i++)
    {
        /** Check if the token is exactly the same TODO: Turn this into MACRO */
        if(arbiter->usr_list[i]->token->sess_id == token->sess_id
           && arbiter->usr_list[i]->token->token == token->token)
        {
            return arbiter->usr_list[i]->ctx;
        }
    }

    /** If we couldn't find anything the token wasn't valid **/
    return NULL;
}

unsigned char *create_token()
{
    unsigned char *buffer;
    qcry_keys_gen_r(arbiter->keygen, 256, &buffer);

    /** Don't forget to free the pointer again! */
    return buffer;
}

// FIXME: Encrypt the private key !!!
int init_key_write(mbedtls_pk_context *key, const char *path, const char *username, const char *passphrase)
{

    printf("Keypair stashing...");
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

    if(fwrite(c, 1, len, f) != len)
    {
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
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s/00_%s.key", path, username);
    else                                    mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s00_%s.key", path, username);

    /* Build public key path */
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/00_%s.pub", path, username);
    else                                    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s00_%s.pub", path, username);

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