/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLDB_H_
#define _QAUL_QLDB_H_

#include <qaul/mod/structures.h>
#include <qaul/error.h>


/**
 * Take a qaul internal path and turn it into a os-specific path
 *
 * @param path
 * @param transformed
 * @return
 */
ql_error_t ql_fs_transform_path(struct ql_path *path, char **transformed);

#endif //QAUL_QLDB_H
