/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef _QAUL_QLDB_UTILS_H_
#define _QAUL_QLDB_UTILS_H_

/**
 * Build SQL query form query objects
 */
ql_error_t qldb_build_query();

/**
 * SLQ query snippets
 */
ql_error_t qldb_qs_select();
ql_error_t qldb_qs_delete();
ql_error_t qldb_qs_insert();

ql_error_t qldb_qs_where();
ql_error_t qldb_qs_and();
ql_error_t qldb_qs_or();
ql_error_t qldb_qs_ascening();
ql_error_t qldb_qs_descending();




#endif //QAUL_QLDB_UTILS_H
