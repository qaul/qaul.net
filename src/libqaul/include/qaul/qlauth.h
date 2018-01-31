#ifndef QAUL_QLAUTH_H
#define QAUL_QLAUTH_H

#include <qaul/qlformat.h>
#include <qaul/error.h>

ql_error_t qlauth_initialise();

ql_error_t qlauth_stop();

ql_error_t qlauth_create_user();

ql_error_t qlauth_delete_user();

ql_error_t qlauth_get_user_info(enum ql_userdata_t, void*);

#endif //QAUL_QLAUTH_H
