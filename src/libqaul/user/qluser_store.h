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
 * @return Status return code
 */
int qluser_store_adduser(struct qluser_t *user, const char *fp);


/** Functions to fill up user data */
int qluser_store_add_ip(struct qluser_t *user, const char *ip);
int qluser_store_add_username(struct qluser_t *user, const char *username);
int qluser_store_add_pubkey(struct qluser_t *user, const char *pubkey);


/** Functions to search users with */
int qluser_store_getwith_ip(struct qluser_t *user, const char *ip);
int qluser_store_getwith_fp(struct qluser_t *user, const char *fp);
int qluser_store_getwith_username(struct qluser_t *user, const char *username);


/** Functions to get specific data fields from a specified user */
int qluser_store_get_ip(struct qluser_t *user, char **ip);
int qluser_store_get_fp(struct qluser_t *user, char **fp);
int qluser_store_get_username(struct qluser_t *user, char **username);
int qluser_store_get_pubkey(struct qluser_t *user, char **pubkey);


/**
 * Remove a user from the current non-persistent user storage
 *
 * @param user The user to delete
 * @return
 */
int qluser_store_remove(struct qluser_t *user);


/**
 * Remove the user from the current user storage as well as scrube
 * all information about this user from the persistent database as
 * well
 *
 * @param user The user to delete
 * @return
 */
int qluser_store_removeall(struct qluser_t *user);


/**
 * Free all resources from the user store and lock the database.
 *
 * @return Status return code
 */
int qluser_store_free();

#endif //QAUL_QLUSER_STORE_H_H
