#ifndef QAUL_QUERY_H
#define QAUL_QUERY_H

#include <glob.h>
#include <qaul/error.h>

typedef enum qaul_query_limit {
  
  STARTS_WITH, ENDS_WITH, EQUALS,
  
  NEWER, OLDER,
};

typedef struct qaul_query {
  char **name;
  enum qaul_query_limit *name_limits;
  short names;

  unsigned int *time;
  enum qaul_query_limit *time_limits;
  short times;

  size_t max_count;
};


ql_error_t ql_query_create(struct qaul_query *init);

#endif // QAUL_QUERY_H