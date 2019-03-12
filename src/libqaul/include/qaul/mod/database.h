/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLDB_H_
#define _QAUL_QLDB_H_

#include <qaul/mod/structures.h>
#include <qaul/error.h>


/**
 * Initialise a new (sqlite3) database connection to a file on the filesystem.
 *
 * Returns database specific errors to debug broken or previously
 * dropped connections. See the DATABASE_* section in type ql_err_t.
 *
 * @param ctx
 * @param path
 * @return
 */
ql_error_t qldb_initialise(struct qldb_session_ctx **ctx, const char *path);

/**
 * Free (and drop) the database context.
 *
 * It is very important that this function is called before terminating
 * libqaul to allow the next instance to lock the database again!
 *
 * @param ctx
 * @return
 */
ql_error_t qldb_free(struct qldb_session_ctx *ctx);

/**
 * Insert a new user into the database.
 *
 * The type of user (internal, external) is irrelevant in that case. Will return
 * errors if the user already exists or the user object provided is malformed
 * (i.e. doesn't contain enough information to complete insertion).
 *
 * @param ctx
 * @param user
 * @return
 */
ql_error_t qldb_insert_user(struct qldb_session_ctx *ctx, union ql_user *user);

/**
 * Insert a new file into the database.
 *
 * Can return errors if the file already exists or type is malformed, i.e.
 * not enough information is available to complete the insertion.
 *
 * @param ctx
 * @param file
 * @return
 */
ql_error_t qldb_insert_file(struct qldb_session_ctx *ctx, struct ql_file *file);

/**
 * Insert a new message into the database.
 *
 * Can return errors if the file already exists or type is malformed, i.e.
 * not enough information is available to complete the insertion.
 *
 * @param ctx
 * @param msg
 * @return
 */
ql_error_t qldb_insert_message(struct qldb_session_ctx *ctx, struct ql_message *msg);

/**
 * Delete a user from the database. Returns error if not exists.
 *
 * @param ctx
 * @param user
 * @return
 */
ql_error_t qldb_delete_user(struct qldb_session_ctx *ctx, union ql_user *user);

/**
 * Delete a file from the database. Returns error if not exists.
 *
 * @param ctx
 * @param file
 * @return
 */
ql_error_t qldb_delete_file(struct qldb_session_ctx *ctx, struct ql_file *file);

/**
 * Delete a message from the database. Returns error if not exists.
 *
 * @param ctx
 * @param msg
 * @return
 */
ql_error_t qldb_delete_message(struct qldb_session_ctx *ctx, struct ql_message *msg);


/////////// USER QUERIES ///////////


/**
 * Return the number of users in the database.
 *
 * @param ctx
 * @param count
 * @return
 */
ql_error_t qldb_find_user_count(struct qldb_session_ctx *ctx, size_t *count);

/**
 * Return a number of users from the database.
 *
 * Depending on the `order` flag this returns the `count` first or last
 * items from the database.
 *
 * @param ctx
 * @param count
 * @param order
 * @param users
 * @return
 */
ql_error_t qldb_find_user_index(struct qldb_session_ctx *ctx, size_t count, enum qldb_query_order order, union ql_user **users);

/**
 * Find a set of users in the database according to a query.
 *
 * @param ctx
 * @param query
 * @param qul
 * @param order
 * @param users
 * @param count
 * @return
 */
ql_error_t qldb_find_user(struct qldb_session_ctx *ctx, union qldb_query **query, size_t qul, enum qldb_query_order order, union ql_user **users, size_t *count);


/////////// FILE QUERIES ///////////


/**
 * Return the number of files in the database.
 *
 * @param ctx
 * @param count
 * @return
 */
ql_error_t qldb_find_files_count(struct qldb_session_ctx *ctx, size_t *count);

/**
 * Return a number of files from the database.
 *
 * Depending on the `order` flag this returns the `count` first or last
 * items from the database.
 *
 * @param ctx
 * @param count
 * @param order
 * @param files
 * @return
 */
ql_error_t qldb_find_files_index(struct qldb_session_ctx *ctx, size_t count, enum qldb_query_order order, struct ql_file **files);

/**
 * Find a set of files in the database according to a query.
 *
 * @param ctx
 * @param query
 * @param qul
 * @param order
 * @param files
 * @param count
 * @return
 */
ql_error_t qldb_find_file(struct qldb_session_ctx *ctx, union qldb_query **query, size_t qul, enum qldb_query_order order, struct ql_file **files, size_t *count);


/////////// MESSAGE QUERIES ///////////


/**
 * Return the number of messages in the database.
 *
 * @param ctx
 * @param count
 * @return
 */
ql_error_t qldb_find_message_count(struct qldb_session_ctx *ctx, size_t *count);

/**
 * Return a number of messages from the database.
 *
 * Depending on the `order` flag this returns the `count` first or last
 * items from the database.
 *
 * @param ctx
 * @param count
 * @param order
 * @param messages
 * @return
 */
ql_error_t qldb_find_message_index(struct qldb_session_ctx *ctx, size_t count, enum qldb_query_order order, struct ql_message **messages);

/**
 * Find a set of messages in the database according to a query.
 *
 * @param ctx
 * @param query
 * @param qul
 * @param order
 * @param messages
 * @param count
 * @return
 */
ql_error_t qldb_find_message(struct qldb_session_ctx *ctx, union qldb_query **query, size_t qul, enum qldb_query_order order, struct ql_message **messages, size_t *count);


/////////// QUERY CREATION UTILITIES ///////////


/**
 * A utility function which allocates memory for a query.
 *
 * Provide a specific type to allocate the correct union-size. No
 * further type-checking is done on a query!
 *
 * @param type      The field-type that is queried
 * @param query     Returns a pointer to allocated memory
 * @return
 */
ql_error_t qldb_query_alloc(enum qldb_query_t type, union qldb_query **query);

/**
 * Free all memory used up by a query
 *
 * @param type
 * @param query
 * @return
 */
ql_error_t qldb_query_free(enum qldb_query_t type, union qldb_query *query);


#endif //QAUL_QLDB_H
