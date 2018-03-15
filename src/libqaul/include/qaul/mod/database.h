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
 * @param ctx
 * @param path
 * @return
 */
ql_error_t qldb_initialise(struct qldb_session_ctx **ctx, const char *path);

/**
 *
 * @param ctx
 * @param user
 * @return
 */
ql_error_t qldb_insert_user(struct qldb_session_ctx *ctx, union ql_user *user);

/**
 *
 * @param ctx
 * @param file
 * @return
 */
ql_error_t qldb_insert_file(struct qldb_session_ctx *ctx, struct ql_file *file);

/**
 *
 * @param ctx
 * @param msg
 * @return
 */
ql_error_t qldb_insert_message(struct qldb_session_ctx *ctx, struct ql_message *msg);

/**
 *
 * @param ctx
 * @param user
 * @return
 */
ql_error_t qldb_delete_user(struct qldb_session_ctx *ctx, union ql_user *user);

/**
 *
 * @param ctx
 * @param file
 * @return
 */
ql_error_t qldb_delete_file(struct qldb_session_ctx *ctx, struct ql_file *file);

/**
 *
 * @param ctx
 * @param msg
 * @return
 */
ql_error_t qldb_delete_message(struct qldb_session_ctx *ctx, struct ql_message *msg);


/////////// USER QUERY FUNCTIONS ///////////


/**
 * Only count the number of users on this system
 *
 * @param ctx
 * @param len
 * @return
 */
ql_error_t qldb_find_user_count(struct qldb_session_ctx *ctx, size_t *len);

/**
 * Returns the first or last number of users
 *
 * @param ctx
 * @param number
 * @param order
 * @param messages
 * @return
 */
ql_error_t qldb_find_user_index(struct qldb_session_ctx *ctx, size_t number, enum qldb_query_order order, union ql_user **users);

/**
 * Get all users ordered in a certain way
 *
 * @param ctx
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_all(struct qldb_session_ctx *ctx, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_with_name(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_with_name_starts(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_with_name_ends(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_seen_before(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_seen_after(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_seen_between(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_added_before(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_added_after(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_user_added_between(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, union ql_user **users, size_t *len);


/////////// FILE QUERY FUNCTIONS ///////////


/**
 * Only count the number of files on this system
 *
 * @param ctx
 * @param len
 * @return
 */
ql_error_t qldb_find_file_count(struct qldb_session_ctx *ctx, size_t *len);

/**
 * Returns the first or last number of files
 *
 * @param ctx
 * @param number
 * @param order
 * @param messages
 * @return
 */
ql_error_t qldb_find_file_index(struct qldb_session_ctx *ctx, size_t number, enum qldb_query_order order, struct ql_file **files);

/**
 * Get all files ordered in a certain way
 *
 * @param ctx
 * @param order
 * @param files
 * @param len
 * @return
 */
ql_error_t qldb_find_file_all(struct qldb_session_ctx *ctx, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_with_name(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_with_name_starts(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param name
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_with_name_ends(struct qldb_session_ctx *ctx, const char *name, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_added_before(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_added_after(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param users
 * @param len
 * @return
 */
ql_error_t qldb_find_file_added_between(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param user
 * @param order
 * @param files
 * @param len
 * @return
 */
ql_error_t qldb_find_file_by_user(struct qldb_session_ctx *ctx, union ql_user *user, enum qldb_query_order order, struct ql_file **files, size_t *len);

/**
 *
 * @param ctx
 * @param hashtag
 * @param order
 * @param files
 * @param len
 * @return
 */
ql_error_t qldb_find_file_with_hashtag(struct qldb_session_ctx *ctx, const char *hashtag, enum qldb_query_order order, struct ql_file **files, size_t *len);



/////////// MESSAGE QUERY FUNCTIONS ///////////


/**
 * Only count the number of messages on this system
 *
 * @param ctx
 * @param len
 * @return
 */
ql_error_t qldb_find_message_count(struct qldb_session_ctx *ctx, size_t *len);

/**
 * Returns the first or last number of messages
 *
 * @param ctx
 * @param number
 * @param order
 * @param messages
 * @return
 */
ql_error_t qldb_find_message_index(struct qldb_session_ctx *ctx, size_t number, enum qldb_query_order order, struct ql_message **messages);

/**
 * Returns all messages known to this system
 *
 * @param ctx
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_all(struct qldb_session_ctx *ctx, enum qldb_query_order order, struct ql_message **messages, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_added_before(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_message **messages, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_added_after(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_message **messages, size_t *len);

/**
 *
 * @param ctx
 * @param time
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_added_between(struct qldb_session_ctx *ctx, struct ql_timestamp *time, enum qldb_query_order order, struct ql_message **messages, size_t *len);

/**
 *
 * @param ctx
 * @param user
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_by_user(struct qldb_session_ctx *ctx, union ql_user *user, enum qldb_query_order order, struct ql_message **messages, size_t *len);

/**
 *
 * @param ctx
 * @param hashtag
 * @param order
 * @param messages
 * @param len
 * @return
 */
ql_error_t qldb_find_message_with_hashtag(struct qldb_session_ctx *ctx, const char *hashtag, enum qldb_query_order order, struct ql_message **messages, size_t *len);


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


#endif //QAUL_QLDB_H
