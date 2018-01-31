#ifndef QAUL_QLAUTH_H
#define QAUL_QLAUTH_H

#include <qaul/qlformat.h>

int qlauth_initialise();

int qlauth_stop();

int qlauth_create_user();

int qlauth_delete_user();

int qlauth_get_user_info(enum ql_userdata_t, void*);

#endif //QAUL_QLAUTH_H
