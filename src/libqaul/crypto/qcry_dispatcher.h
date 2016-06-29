#ifndef QAUL_QCRY_DISPATCHER_H
#define QAUL_QCRY_DISPATCHER_H

#include "qcry_context.h"


struct qcry_disp_ctx{

    /** **/
    static struct qcry_usr_ctx  **usr_ctx;
    size_t                      users, max;

    /** Concurrency data **/
} qcry_disp_ctx;

#endif //QAUL_QCRY_DISPATCHER_H
