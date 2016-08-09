/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QCRY_ARBITER_H
#define QAUL_QCRY_ARBITER_H

#include "qcry_keystore.h"

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

// Don't allow for more than one access at atime
#define QAUL_CONC_LOCK      1

#define QAUL_FINGERPRINT    1
#define QAUL_PUBKEY         2

struct qcry_arbit_token {
    unsigned int        sess_id;
    unsigned char       token[128];
};

/**
 * Called during initisation. Sets up crypto stack, allocates memory, etc.
 * Should be given a valid pointer to a context that can be used to write into and
 * the number of maximum concurrent jobs that it's allowed to execute.
 *
 * @param ctx: Context to populate with this procedure
 * @param max_concurrent: Highest number of concurrent jobs allowed
 */
int qcry_arbit_init(unsigned int max_concurrent, const char *path, struct qcry_usr_id **known_keys);
int qcry_arbit_free();

/**
 * Creates a local user context with a username, passphrase and keytype.
 */
int qcry_arbit_usrcreate(int *usrno, const char *username, const char *passphrase, unsigned int key_type);
int qcry_arbit_usrdestroy(int usrno);

/** Get data about user */
int qcry_arbit_getusrinfo(char *(*buffer), int usrno, int type);

/**
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint,  int usrno);

/**
 * This function adds the key context of the provided finterprint
 * to the target list of the provided user giving quick access to all
 * relevant data when doing encrypted or signed messaging.
 *
 * In addition to an added speed benefit, this function also provides
 * memory localisation between users to ensure that two identities can't
 * accidentally bleed data into each other.
 *
 * @param userno
 * @param fingerprint
 * @return
 */
int qcry_arbit_addtarget(int userno, const char *fingerprint);

/**
 * This function should be used to add a new key fresh off the TCP socket
 * to the crypto keystore.
 *
 * As such it just takes a plain text encoded buffer that should contain the key
 * body with all the markers and headers. Do not submit a modified keyfile body!
 *
 * This keyfile will then be mapped against a fingerprint and username for
 * convenience
 *
 * @param keybody
 * @param key_len
 * @param fingerprint
 * @param username
 * @return
 */
int qcry_arbit_addkey(const char *keybody, size_t key_len, const char *fingerprint, const char *username);

/**
 * This function takes a user identifier (username) and their private passphrase to restore
 * the keys that are stored on disk for them.
 *
 * @param username: Username space to unlock
 * @param passphrase: A passphrase used to encrypt the keys
 */
int qcry_arbit_restore(int *usrno, const char *username, const char *passphrase);

int qcry_arbit_signmsg(int usrno, char *(*sgn_buffer), const char *message);

int qcry_arbit_verify(int usrno, int trgtno, const char *message, const char *signature);

//int qcry_arbit_sendmsg(int usrno, char *(*encrypted), const char *plain);
//int qcry_arbit_parsemsg(int usrno, char *(*parsed), const char *encrypted);

#endif //QAUL_QCRY_ARBITER_H
