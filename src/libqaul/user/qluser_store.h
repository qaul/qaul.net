/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QLUSER_STORE_H_H
#define QAUL_QLUSER_STORE_H_H

#define QLUSER_STATUS_SUCCESS           0
#define QLUSER_STATUS_DB_INVALID        (1 << 0)
#define QLUSER_STATUS_DB_LOCKED         (1 << 1)
#define QLUSER_STATUS_INVALID_KEYSTORE  (1 << 2)


/** Forward declare structs */
typedef struct qluser_t qluser_t;
typedef struct qluser_node_t qluser_node_t;


/**
 * Initialise the user store and fill it with already known users in
 * a network from a database and public keystore path. All other functions
 * will fail if this function was not called first.
 *
 * @param db_path Provide the path to the persistent user db
 * @param key_path Provide the path to the public keystore folder
 * @param flags Provide some configuration flags. See docs for details
 * @return Status return code
 */
int qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags);


/**
 * Add a new user with fingerprint username and ip into the user store. This is
 * the bare minimum of information that is required to store a user in the store
 *
 * @param user Will be filled with a pointer representing the user for future operations
 * @param fp The fingerprint of this user
 * @return
 */
int qluser_store_adduser(struct qluser_t *user, const char *fp, const char *name);


/**
 * Free all resources from the user store and lock the database.
 *
 * @return Status return code
 */
int qluser_store_free();

#endif //QAUL_QLUSER_STORE_H_H
