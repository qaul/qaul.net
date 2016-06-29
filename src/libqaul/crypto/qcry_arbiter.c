/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <memory.h>
#include <stdlib.h>

#include <qaullib/qcry_arbiter.h>
#include "qcry_dispatcher.h"
#include "qcry_helper.h"

/**
 ** Static reference to a single dispatcher which is thread safe and allows
 **     for multi threaded access to all crypto functions
 **/
static struct qcry_disp_ctx *dispatcher;

int qcry_arbit_init(struct qcry_arbit_ctx *ctx, unsigned int max_concurrent)
{
    /** Make sure our context is clean */
    memset(ctx, 0, sizeof(qcry_arbit_ctx));
    if(!dispatcher) {
        dispatcher = (struct qcry_disp_ctx*) malloc(sizeof(struct qcry_disp_ctx));
    }

    /** Check it again if it worked **/
    if(!dispatcher) {
        return QCRY_STATUS_MALLOC_FAIL;
    }

    ctx->max_conc = max_concurrent;
    ctx->magno = MAGICK_NO;
    return QCRY_STATUS_OK;
}

int qcry_arbit_free(struct qcry_arbit_ctx *ctx)
{
  return QCRY_STATUS_OK;
}

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(char *(*fingerprint), const char *username, const char *passphrase, unsigned int key_type)
{
  return QCRY_STATUS_OK;
}
int qcry_arbit_usrdestroy(const char *fingerprint)
{
  return QCRY_STATUS_OK;
}

/**
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint, qcry_arbit_token *token)
{
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
  return QCRY_STATUS_OK;
}

/**
 * Starts a "session" between a local user (as a fingerprint) and a remote user (as a fingerprint).
 * Fingerprints are used in the crypto engine to identify keys and outside the crypto module to map
 * users to routing data
 */
int qcry_arbit_start(struct qcry_arbit_ctx *ctx, const char *fp_self, const char *fp_trgt, qcry_arbit_token *(*token))
{
  return QCRY_STATUS_OK;
}

/**
 * Stops a session with a token.
 */
int qcry_arbit_stop(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token)
{
  return QCRY_STATUS_OK;
}


int qcry_arbit_sendmsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*encrypted), const char *plain)
{
  return QCRY_STATUS_OK;
}

int qcry_arbit_parsemsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*parsed), const char *encrypted)
{
  return QCRY_STATUS_OK;
}

int qcry_arbit_signmsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*sgn_buffer), const char *message)
{
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
int qcry_arbit_verify(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, const char *message, const char *signature)
{
  return QCRY_STATUS_OK;
}


