/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLUSER_H
#define _QAUL_QLUSER_H

#include <qaul/mod/structures.h>
#include <qaul/error.h>
#include <stdlib.h>


/**
 * Initialise the user store and fill it with already known users in
 * a network from a database and public keystore path. All other functions
 * will fail if this function was not called first.
 *
 * @param db_path Provide the path to the persistent user db
 * @param key_path Provide the path to the public keystore folder
 * @param flags Provide some configuration flags. See docs for details
 * @return Status return codesu
 */
ql_error_t qluser_store_initialise(const char *db_path, const char *key_path, unsigned int flags);


/**
 * This function adds the bare-bone data required into two of the lookup tables
 * which allows future data to be added as well. To retrieve the newly created
 * used after the insertion please use @link{qluser_store_getby_fp}
 *
 * @param fp    The fingerprint of this user
 * @param name  The username of this user
 * @return      Status return code
 */
ql_error_t qluser_store_adduser(const char *fp, const char *name);


/**
 * Remove a user from all storage tables, cleaning up all pre-allocated
 * memory from the tables and qluser_t structs. It also removes the reference
 * left in qluser_node_t and cleans it up if it was the only user on this
 * node.
 *
 * @param fp    The fingerprint of the user to remove
 * @return      Status return code
 */
ql_error_t qluser_store_rmuser(const char *fp);


/**
 * Add an IP address to a user which represents a node on
 * the network. If a user has changed nodes, the old one
 * will be removed before
 *
 * @param fp    The fingerprint of the user
 * @param ip    IP data of the node involved
 * @return      Status return code
 */
ql_error_t qluser_store_add_ip(const char *fp, union olsr_ip_addr *ip);


/** Functions to fill up user data */
ql_error_t qluser_store_set_keytrust(const char *fp, enum qluser_trust_t trust);
ql_error_t qluser_store_set_msgtrust(const char *fp, int32_t trust);
ql_error_t qluser_store_set_filetrust(const char *fp, int32_t trust);
ql_error_t qluser_store_set_ltrust(const char *fp, int32_t trust);


/** Functions to search users with */
ql_error_t qluser_store_getby_fp(struct ql_user_external *user, const char *fp);
ql_error_t qluser_store_getby_name(struct ql_user_external *user, const char *name);
ql_error_t qluser_store_getby_ip(struct ql_user_external *user, union olsr_ip_addr *ip);


/**
 * Remove the user from the current user storage as well as scrub
 * all information about this user from the persistent database as
 * well
 *
 * @param user The user to delete
 * @return
 */
ql_error_t qluser_store_rmuser_all(const char *fp);


/**
 * Free all resources from the user store and lock the database.
 *
 * @return Status return code
 */
ql_error_t qluser_store_free();


/**
 * Create a new user for a certain type
 *
 * The user that is created is still slightly raw and doesn't
 * contain all information that a user might want/ could have
 * @return
 */
ql_error_t qluser_create(enum qluser_t t, const char *username, const char *fp, union ql_user **user);


/**
 * Syncs the changes for a specific user to disk forcibly
 *
 * @param user
 * @return
 */
ql_error_t qluser_sync(union ql_user *user);


/**
 * Load all data from a user from disk/ and db, initialising a new user
 *
 * @param fp
 * @param user
 * @return
 */
ql_error_t qluser_load(const char *fp, union ql_user **user);


/**
 * Destroy all data from an associated user. This deletes known
 * keys and database entries for this user.
 *
 * @param user
 * @return
 */
ql_error_t qluser_destroy(union ql_user *user);


/**
 * Free all memory associated with a user
 * @param user
 * @return
 */
ql_error_t qluser_free(union ql_user *user);


#endif //_QAUL_QLUSER_H
