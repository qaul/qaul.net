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


ql_error_t qldb_insert_user(struct qldb_session_ctx *ctx, union ql_user *user);
ql_error_t qldb_insert_file(struct qldb_session_ctx *ctx, struct ql_file *file);
ql_error_t qldb_insert_message(struct qldb_session_ctx *ctx, struct ql_message *msg);


ql_error_t qldb_delete_user(struct qldb_session_ctx *ctx, union ql_user *user);
ql_error_t qldb_delete_file(struct qldb_session_ctx *ctx, struct ql_file *file);
ql_error_t qldb_delete_message(struct qldb_session_ctx *ctx, struct ql_message *msg);


ql_error_t qldb_find_user(struct qldb_session_ctx *ctx, union ql_user **users);
ql_error_t qldb_find_file(struct qldb_session_ctx *ctx, struct ql_file **files);
ql_error_t qldb_find_message(struct qldb_session_ctx *ctx, struct ql_message *msgs);


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
