Crypto module API
=================

This is as raw API documentation, as there is no auto-generated API 
documentation at the moment.


Arbiter
-------

```C
/**
 * Called during initisation. Sets up crypto stack, allocates memory, etc.
 * Should be given a valid pointer to a context that can be used to write into and
 * the number of maximum concurrent jobs that it's allowed to execute.
 *
 * @param ctx: Context to populate with this procedure
 * @param max_concurrent: Highest number of concurrent jobs allowed
 */
int qcry_arbit_init(unsigned int max_concurrent, const char *path, struct qcry_usr_id **known_keys);


/** Call this function to cleanly shut down the arbiter and all of its submodules */
int qcry_arbit_free();


/**
 * Create a new user with a username, passphrase and the desired key-type (currently 
 * only RSA is supported). The first parameter (usrno) is used to assign the user an
 * ID that can be used in later exchanges.
 * 
 * @param usrno: A pointer to an int that the user ID is stored in
 * @param username: The desired username for this user
 * @param passphrase: The desired passphrase. Can not be @{NULL}
 * @param key_type: The desired key type for this username. Currently only @{QCRY_KEYS_RSA} supported.
 */
int qcry_arbit_usrcreate(int *usrno, const char *username, const char *passphrase, unsigned int key_type);


/** Destroy a user with their user ID */
int qcry_arbit_usrdestroy(int usrno);


/**
 * Get data about a user from the arbiter database. Provide a pointer to a buffer
 * as the first parameter to return data in. Also needs the type of data that is
 * requested as well as a user ID to match a user.
 * 
 * @param buffer: Reference pointer to write into. Must not be called @{free(...)} on!
 * @param usrno: The user ID to lookup
 * @param type: Choose either @{QAUL_FINGERPRINT} or @{QAUL_PUBKEY} for lookup
 */
 int qcry_arbit_getusrinfo(char *(*buffer), int usrno, int type);


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
int qcry_arbit_start(int usrno, const char *fingerprint);


/**
 * This function is the inverse of #{qcry_arbit_start}. It stops communication
 * with a "target" to free up memory and potentially speed up operation for
 * other nodes.
 *
 * @param userno
 * @param fingerprint
 * @return
 */
int qcry_arbit_stop(int usrno, const char *fingerprint);


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
 * Opposite of #{qcry_arbit_restore}. This function will take an identity and save it's context
 * including keys and sensitive data in an encrypted blob on the disk.
 * Passphrase needs to have been created in before.
 */
int qcry_arbit_save(const char *finterprint,  int usrno);


/**
 * This function takes a user identifier (username) and their private passphrase to restore
 * the keys that are stored on disk for them.
 *
 * @param username: Username space to unlock
 * @param passphrase: A passphrase used to encrypt the keys
 */
int qcry_arbit_restore(int *usrno, const char *username, const char *passphrase);


/**
 * This function is used to sign message bodies to any recipient. The usrno
 * provided is the currently active user! The signature buffer is a standard 
 * reference pointer to some field.
 *
 * @param usrno: Current active user ID (requires their private key)
 * @param sgn_buffer: Reference pointer that will be allocated memory for
 * @param message: Provide a const char copy of the message to sign
 */
int qcry_arbit_signmsg(int usrno, unsigned char *(*sgn_buffer), const char *message);


/**
 * This function is used to verify signed messages from a specific user. Here we require
 * two user context fields. One is the currently active user number. The other is the
 * communication "target" (the message sender). Additionally provide both the unmodified
 * message and the signature.
 *
 * This function will return values according to the signature success!
 *
 * @param usrno: Current active user ID
 * @param trgtno: The message sender user ID
 * @param message: Provide a const char copy of the message
 * @param signature: Provide a const unsigned char copy of the signature
 */
int qcry_arbit_verify(int usrno, int trgtno, const char *message, const unsigned char *signature);

```
