/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QCRY_ARBITER_H
#define QAUL_QCRY_ARBITER_H

/************************************************************************************************
 *** Comprehensive API to interact with the qaul.net crypto code. Every request needs to be done
 *** under a certain context with a certain token passed in. Requests will then be arbited for their
 *** validity and then handled.
 ***
 *** See the function headers for more in depth documentation on how to use.
 ***
 ************************************************************************************************/

/** Define flags for our key sizes */
#define QAUL_KEYS_AES256    (1 << 1)
#define QAUL_KEYS_RSA4096   (1 << 2)
#define QAUL_KEYS_ECDSA     (1 << 3)

struct qcry_arbit_ctx {
    void            *dispatcher;

    unsigned int    max_conc;
    short           magno;
} qcry_arbit_ctx;

typedef struct {
    unsigned int        *sess_id;
    unsigned char       token[128];
} qcry_arbit_token;

/**
 * Called during initisation. Sets up crypto stack, allocates memory, etc.
 * Should be given a valid pointer to a context that can be used to write into and
 * the number of maximum concurrent jobs that it's allowed to execute.
 *
 * @param ctx: Context to populate with this procedure
 * @param max_concurrent: Highest number of concurrent jobs allowed
 */
int qcry_arbit_init(struct qcry_arbit_ctx *ctx, unsigned int max_concurrent);
int qcry_arbit_free(struct qcry_arbit_ctx *ctx);

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(char *(*fingerprint), const char *username, const char *passphrase, unsigned int key_type);
int qcry_arbit_usrdestroy(const char *fingerprint);

/**
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint, qcry_arbit_token *token);

/**
 * This function takes a user identifier (username) and their private passphrase to restore
 * the keys that are stored on disk for them.
 *
 * @param username: Username space to unlock
 * @param passphrase: A passphrase used to encrypt the keys
 */
int qcry_arbit_restore(const char *username, const char *passphrase);

/**
 * Starts a "session" between a local user (as a fingerprint) and a remote user (as a fingerprint).
 * Fingerprints are used in the crypto engine to identify keys and outside the crypto module to map
 * users to routing data
 */
int qcry_arbit_start(struct qcry_arbit_ctx *ctx, const char *fp_self, const char *fp_trgt, qcry_arbit_token *(*token));

/**
 * Stops a session with a token.
 */
int qcry_arbit_stop(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token);


int qcry_arbit_sendmsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*encrypted), const char *plain);

int qcry_arbit_parsemsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*parsed), const char *encrypted);

int qcry_arbit_signmsg(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, char *(*sgn_buffer), const char *message);

/**
 * Verify the validity of a signature on a message cryptographically. Provide the session token and an active context
 * as well as a message and the elegid signature to verify the pair.
 *
 * Returns 0 if signature could be verified.
 * Returns -1 if signature was faulty
 * Returns 1...255 for runtime errors
 */
int qcry_arbit_verify(struct qcry_arbit_ctx *ctx, qcry_arbit_token *token, const char *message, const char *signature);

#endif //QAUL_QCRY_ARBITER_H
