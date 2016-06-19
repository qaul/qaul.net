#include "qcry_context.h"

#include <memory.h>
#include <malloc.h>
#include <zconf.h>


int qcry_context_init(qcry_usr_ctx *ctx, unsigned char *usr_name, qcry_ciph_t ciph_t)
{
    memset(ctx, 0, sizeof(qcry_usr_ctx));
    ctx->usr_name = usr_name;
    ctx->ciph_t = ciph_t;

    /* Prepare space for future crypto targets */
    ctx->trgts = (qcry_trgt_t*) calloc(sizeof(qcry_trgt_t), TRGT_C);
    ctx->usd_trgt = 0;
    ctx->max_trgt = TRGT_C;

    /* Signal we're done */
    ctx->mgno = MAGICK_NO;
    return QCRY_STATUS_OK;
}

int qcry_context_prik_attach(qcry_usr_ctx *ctx, const unsigned char *usr_key_pri)
{
    CHECK_SANE

    /* Make sure our data is valid */
    size_t pri_len = strlen(usr_key_pri);
    if(pri_len != QCRY_KEY_LEN[ctx->ciph_t]) return QCRY_STATUS_INVALID_KEYS;

    /* Allocate memory for our key */
    ctx->usr_key_pri = (unsigned char *) calloc(sizeof(unsigned char), QCRY_KEY_LEN[ctx->ciph_t]);
    if(!ctx->usr_key_pri) return QCRY_STATUS_MALLOC_FAIL;

    /* Copy key from const buffer and exit */
    memcpy(ctx->usr_key_pri, usr_key_pri, pri_len);
    ctx->use_ctr = 0;
    return QCRY_STATUS_OK;
}

int qcry_context_prik_detach(qcry_usr_ctx *ctx)
{
    CHECK_SANE

    int ctr_t;
    while(ctx->use_ctr != 0) {
        ctr_t += TIME_SLEEP;
        sleep(TIME_SLEEP);

        if(ctr_t >= MAX_TIMEOUT) {
            return QCRY_STATUS_KEY_BUSY;
        }
    }

    /* Then simply free the key */
    free(ctx->usr_key_pri);
    return QCRY_STATUS_OK;
}

int qcry_context_add_trgt(qcry_usr_ctx *ctx, const qcry_trgt_t *trgt, qcry_ciph_t ciph_t, unsigned int *trgt_no)
{
    CHECK_SANE

    /* Check the key length for our target */
    size_t pri_len;
    if(ciph_t == AES256) pri_len = strlen(trgt->data.sk->sh_key_pri);
    else                 pri_len = strlen(trgt->data.pk->usr_key_pub);
    if(QCRY_KEY_LEN[pri_len] != ciph_t) return QCRY_STATUS_INVALID_TARGET;

    /* Increases the target buffer if needs be */
    CHECK_BUFFER(qcry_trgt_t* , ctx->trgts, ctx->max_trgt, ctx->usd_trgt)

    /* Copy in target data from const source */
    ctx->usd_trgt++;
    memcpy(ctx->trgts[ctx->usd_trgt], trgt, sizeof(qcry_trgt_t));
    *trgt_no = ctx->usd_trgt;

    if(ciph_t == AES256) {
        ctx->trgts[ctx->usd_trgt]->mbed_ctx.sk_ctx = (mbedtls_aes_context*) malloc(sizeof(mbedtls_aes_context));
        mbedtls_aes_init(ctx->trgts[ctx->usd_trgt]->mbed_ctx.sk_ctx);
    } else {
        ctx->trgts[ctx->usd_trgt]->mbed_ctx.pk_ctx = (mbedtls_pk_context*) malloc(sizeof(mbedtls_pk_context));
        mbedtls_pk_init(ctx->trgts[ctx->usd_trgt]->mbed_ctx.pk_ctx);
    }

    /* Signal back all clear */
    return QCRY_STATUS_OK;
}

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int *trgt_no)
{
    CHECK_SANE

    if(!ctx->trgts[*trgt_no]) return QCRY_STATUS_INVALID_TARGET;

    /* Clear target memory and resize buffer if needed */
    CLEAR_TARGET(ctx->ciph_t, ctx->trgts[*trgt_no])
    CHECK_BUFFER(qcry_trgt_t* , ctx->trgts, ctx->max_trgt, ctx->usd_trgt)

    return QCRY_STATUS_OK;
}