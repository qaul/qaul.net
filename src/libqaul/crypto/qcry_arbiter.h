/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_QCRY_ARBITER_H
#define QAUL_QCRY_ARBITER_H

/************************************************************************************************
 ***
 ***
 ***
 ***
 ***
 ***
 ************************************************************************************************/

typedef struct {
    void            *dispatcher;


    unsigned int    max_conc;
    short           magno;
} qcry_arbit_ctx;

typedef struct {
    unsigned int        *sess_id;
    unsigned char       token[128];
} qcry_arbit_token;

int qcry_arbit_init(qcry_arbit_ctx *ctx, unsigned int max_concurrent);

int qcry_arbit_start(qcry_arbit_ctx *ctx, char *self, char *trgt, qcry_arbit_token *(*token));

int qcry_arbit_stop(qcry_arbit_ctx *ctx, qcry_arbit_token *token);


#endif //QAUL_QCRY_ARBITER_H
