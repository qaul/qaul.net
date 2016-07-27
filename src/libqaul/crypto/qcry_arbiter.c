/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <memory.h>
#include <stdlib.h>

#include <qaullib/qcry_arbiter.h>
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
} arbit_bind_item;

typedef struct {

    /** Multi-User contexts */
    arbit_bind_item     **bind_lst;
    size_t              users, max;

    /** Key generator context */
    qcry_keys_context   *keys;
} qcry_arbit_ctx ;

/** Static reference to our main arbiter context **/
static qcry_arbit_ctx *arbiter;
static unsigned int session_ctr;

/** Inline macro that's used to verify that the arbiter context we're operating on is valid **/
#define SANE_ARBIT(to_return) if(arbiter == NULL || arbiter->keys == NULL || arbiter->max < 0) return to_return;

// Private utility function
qcry_usr_ctx *get_ctx_with_token(struct qcry_arbit_token *token)
{
    SANE_ARBIT(NULL)

    int i;
    for(i = 0; i <= arbiter->users; i++)
    {
        /** Check if the token is exactly the same TODO: Turn this into MACRO */
        if(arbiter->bind_lst[i]->token->sess_id == token->sess_id
                && arbiter->bind_lst[i]->token->token == token->token)
        {
            return arbiter->bind_lst[i]->ctx;
        }
    }

    /** If we couldn't find anything the token wasn't valid **/
    return NULL;
}

unsigned char *create_token()
{
    unsigned char *buffer;
    qcry_keys_gen_r(arbiter->keys, 256, &buffer);

    /** Don't forget to free the pointer again! */
    return buffer;
}


int qcry_arbit_init(unsigned int max_concurrent)
{
    /** Cleanly allocate memory */
    arbiter = (qcry_arbit_ctx*) calloc(sizeof(qcry_arbit_ctx), 1);

    /** Initialise our keygenerator context */
    arbiter->keys = (qcry_keys_context*) malloc(sizeof(qcry_keys_context));
    if(arbiter->keys == NULL)
        return QCRY_STATUS_MALLOC_FAIL;
    qcry_keys_init(arbiter->keys);

    /** Initialise our context and token lists */
    arbiter->bind_lst = (arbit_bind_item**) calloc(sizeof(arbit_bind_item*), MIN_BFR_S);
    if(arbiter->bind_lst == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    arbiter->max = MIN_BFR_S;
    arbiter->users = 0;

    /** Set session counter to 0 */
    session_ctr = 0x0;

    /** Then return all OK */
    return QCRY_STATUS_OK;
}

int qcry_arbit_free()
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)
    int retval;

    /** Free key generator */
    retval = qcry_keys_free(arbiter->keys);
    if(!retval)
        return QCRY_STATUS_ERROR;

    int i;
    for(i = 0; i <= arbiter->users; i++)
    {
        // qcry_context_free(arbiter->bind_lst[i]->ctx);
    }

    return QCRY_STATUS_OK;
}

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(const char *username, const char *passphrase, unsigned int key_type)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    int ret;
    char *fingerprint;

    /** First allocate space for our new user in the arbiter context */
    arbit_bind_item *item = (arbit_bind_item*) calloc(sizeof(arbit_bind_item), 1);
    item->ctx = (qcry_usr_ctx*) malloc(sizeof(qcry_usr_ctx));
    ret = qcry_context_init(item->ctx, username, PK_RSA);
    if(ret != 0) {
        printf("Context init failed with code %d", ret);
        return QCRY_STATUS_CTX_INVALID;
    }

    item->token = (struct qcry_arbit_token*) calloc(sizeof(struct qcry_arbit_token), 1);
    item->token->sess_id = (int) session_ctr++;

    /** Copy token and release old pointer */
    unsigned char *token = create_token();
    memcpy(item->token->token, token, 256);
    free(token);

    /** Generate a primary user key */
    unsigned char *pri_k;
    ret = qcry_keys_gen_m(arbiter->keys, QCRY_KEYS_KL_RSA, &pri_k);
    if(ret != 0)
        return QCRY_STATUS_KEYGEN_FAILED;

    /** Then attach our private key to the user context */
    qcry_context_prk_attach(item->ctx, pri_k);



    return QCRY_STATUS_OK;
}

int qcry_arbit_getusrinfo(const char *(*fingerprint), const char *username)
{

}

int qcry_arbit_usrdestroy(const char *fingerprint)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}

/**
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint, struct qcry_arbit_token *token)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

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
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}

/**
 * Starts a "session" between a local user (as a fingerprint) and a remote user (as a fingerprint).
 * Fingerprints are used in the crypto engine to identify keys and outside the crypto module to map
 * users to routing data
 */
int qcry_arbit_start(const char *fp_self, const char *fp_trgt, struct qcry_arbit_token *(*token))
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}

/**
 * Stops a session with a token.
 */
int qcry_arbit_stop(struct qcry_arbit_token *token)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}


int qcry_arbit_sendmsg(struct qcry_arbit_token *token, char *(*encrypted), const char *plain)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}

int qcry_arbit_parsemsg(struct qcry_arbit_token *token, char *(*parsed), const char *encrypted)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}

int qcry_arbit_signmsg(struct qcry_arbit_token *token, char *(*sgn_buffer), const char *message)
{
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

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
    SANE_ARBIT(QCRY_STATUS_CTX_INVALID)

    return QCRY_STATUS_OK;
}
