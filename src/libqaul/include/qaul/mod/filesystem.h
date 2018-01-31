/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLFS_H_
#define _QAUL_QLFS_H_

#include <qaul/mod/structures.h>
#include <qaul/error.h>
#include <qaul/qaul.h>


/**
 * Create a new path with a unix-y path and an OS
 *
 * @param path
 * @param os
 * @param inital
 * @return
 */
ql_error_t ql_path_new(struct ql_path **path, enum qaul_os os, const char *initial);


/**
 * Free the memory associated with a path
 *
 * @param path
 * @return
 */
ql_error_t ql_path_free(struct ql_path *path);


/**
 * Create a new path, cloning it from a previous one
 *
 * @param path
 * @param to_clone
 * @return
 */
ql_error_t ql_path_clone(struct ql_path **path, struct ql_path *to_clone);


/**
 * Append a new element to the path
 *
 * @param path
 * @param element
 * @return
 */
ql_error_t ql_path_append(struct ql_path *path, const char *element);


/**
 * Remove a number of elements from the path
 *
 * @param path
 * @param elements
 * @return
 */
ql_error_t ql_path_pop(struct ql_path *path, size_t elements);


/**
 * Turn the path object into an OS specific path
 *
 * @param path
 * @param string
 * @return
 */
ql_error_t ql_path_tostring(struct ql_path *path, char **string);


/**
 * Create the folders outlined in the path, similar to what
 * mkdir -p would do
 *
 * e.g. "~/.local/qaul/keystore" would create "qaul/keystore" under ".local"
 *
 * @param path
 * @return
 */
ql_error_t ql_path_mkdirs(struct ql_path *path);


/**
 * Utility function which returns the file extention of the
 * leaf-child (e.g. `.txt`)
 *
 * @param path
 * @return
 */
char *ql_path_child_ext(struct ql_path *path);


/**
 * Get the most leaf-child name, without any extentions
 *
 * e.g. for "~/directory/child/" this would yield "child"
 *
 * @param path
 * @return
 */
char *ql_path_child_name(struct ql_path *path);


/**
 * List all files in a directory
 *
 * @param path
 * @param files
 * @return
 */
ql_error_t ql_path_list(struct ql_path *path, struct ql_file_list **files);


/**
 * Write contents to a file
 *
 * @param path
 * @param contents
 * @param size
 * @return
 */
ql_error_t ql_fs_write(struct ql_path *path, const char *contents, size_t size);


/**
 * Read the contents of a file into a buffer
 *
 * @param path
 * @param contents
 * @param size
 * @return
 */
ql_error_t ql_fs_read(struct ql_path *path, char **contents, size_t *size);


#endif //QAUL_QLFS_H
