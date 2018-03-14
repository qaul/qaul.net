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
