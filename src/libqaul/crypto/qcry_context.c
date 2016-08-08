/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <stdlib.h>
#include <memory.h>
#include <zconf.h>

#include "qcry_context.h"
#include "qcry_keys.h"
#include <qaullib/qcry_hashing.h>
#include <mbedtls/pk.h>
#include <mbedtls/pk_internal.h>

//////////////////////////// SOME HELPFUL MACROS ////////////////////////////

#define FIND_TRGT(username) \
    int i;  \
    qcry_trgt_t     *target = NULL; \
    for(i = 0; i < ctx->usd_trgt; i++) {    \
        if(ctx->trgts[i]->username == username) {   \
            target = ctx->trgts[i];\
            break; \
        }\
    } \
    if(target == NULL) return QCRY_STATUS_INVALID_TARGET;

int qcry_context_init(qcry_usr_ctx *ctx, const char *usr_name, qcry_ciph_t ciph_t)
{
    /* Zero out so we are clean */
    memset(ctx, 0, sizeof(qcry_usr_ctx));

    /* Set some basic identity metadata */
    ctx->username = usr_name;
    ctx->birthdate = NULL;
    ctx->ciph_t = ciph_t;

    switch(ciph_t) {
        case PK_RSA: ctx->ciph_len = QCRY_KEYS_KL_RSA; break;
        case ECDSA: ctx->ciph_len = QCRY_KEYS_KL_ECC; break;
        case AES256: ctx->ciph_len = QCRY_KEYS_KL_AES; break;
        default: return QCRY_STATUS_INVALID_PARAMS;
    }

    /* Make sure we have some space for targets */
    ctx->trgts = (qcry_trgt_t**) calloc(sizeof(qcry_trgt_t*), MIN_BFR_S);
    ctx->max_trgt = MIN_BFR_S;
    ctx->usd_trgt = 0;

    /* Set our magic number to indicate that we are awesome */
    ctx->mgno = MAGICK_NO;

    /* Return success :) */
    return QCRY_STATUS_OK;
}

int qcry_context_attach(qcry_usr_ctx *ctx, mbedtls_pk_context *pub, mbedtls_pk_context *pri)
{
    CHECK_SANE

    // TODO: Check the key type is correct!

    /* Assign the keys */
    ctx->public = pub;
    ctx->private = pri;

    /* Write private key into text buffer */
    size_t buffer_s = 16000;
    unsigned char pri_buf[buffer_s];
    ret = mbedtls_pk_write_key_pem(pri, pri_buf, buffer_s);
    if(ret != 0) return QCRY_STATUS_INVALID_KEYS;

    /* Prepare some data */
    char *tmp_buf = (char*) malloc(buffer_s);

    if(tmp_buf == NULL) return QCRY_STATUS_MALLOC_FAIL;
    memcpy(tmp_buf, pri_buf, buffer_s);

    /* Compute your own fingerprint */
    struct qcry_hash_ctx hash;

    ret = qcry_hashing_init(&hash, QCRY_HASH_SHA256, ctx->username);
    if(ret) goto exit;

    ret = qcry_hashing_append(&hash, tmp_buf);
    if(ret) goto exit;

    ret = qcry_hashing_build(&hash, QCRY_HASH_BASE64, &ctx->fingerprint);
    if(ret) goto exit;

    /* Allow to skip steps to free resources again */
    exit:

    /* Free our resources */
    memset(pri_buf, 0, buffer_s);
    qcry_hashing_free(&hash);
    free(tmp_buf);

    /* Return success :) */
    return ret;
}

int qcry_context_get_finterprint(qcry_usr_ctx *ctx, unsigned char *(*fingerprint))
{
    CHECK_SANE

    if(ctx->fingerprint) {
        (*fingerprint) = (unsigned char*) ctx->fingerprint;
        return QCRY_STATUS_OK;
    } else {
        return QCRY_STATUS_INVALID_CTX;
    }
}

int qcry_context_prk_detach(qcry_usr_ctx *ctx)
{
//    CHECK_SANE
//
//    int ctr_t = 0;
//    while(ctx->use_ctr != 0) {
//        ctr_t += TIME_SLEEP;
//        sleep(TIME_SLEEP);
//
//        if(ctr_t >= MAX_TIMEOUT) {
//            return QCRY_STATUS_KEY_BUSY;
//        }
//    }
//
//    /* Then simply free the key */
//    free(ctx->usr_key_pri);
//    return QCRY_STATUS_OK;
}

int qcry_context_add_trgt(qcry_usr_ctx *ctx, const qcry_trgt_t *trgt, qcry_ciph_t ciph_t, unsigned int *trgt_no)
{
//    CHECK_SANE
//
//    /* Check the key length for our target */
//    size_t pri_len;
//    if(ciph_t == AES256) {
//        pri_len = strlen(trgt->d.sk->sh_key_pri);
//    } else {
//        pri_len = strlen(trgt->d.pk->usr_key_pub);
//        if(trgt->d.pk->key_len == 0) pri_len = -1;
//    }
//
//    if(pri_len != QCRY_KEY_LEN[ciph_t]) return QCRY_STATUS_INVALID_TARGET;
//
//    /* Increases the target buffer if needs be */
//    CHECK_BUFFER(qcry_trgt_t* , ctx->trgts, ctx->max_trgt, ctx->usd_trgt)
//
//    /* Copy in target data from const source */
//    ctx->usd_trgt++;
//    memcpy(ctx->trgts[ctx->usd_trgt], trgt, sizeof(qcry_trgt_t));
//    *trgt_no = ctx->usd_trgt;
//
//    if(ciph_t == AES256) {
//        ctx->trgts[ctx->usd_trgt]->ctx.sk = (mbedtls_aes_context*) malloc(sizeof(mbedtls_aes_context));
//        mbedtls_aes_init(ctx->trgts[ctx->usd_trgt]->ctx.sk);
//    } else {
//        ctx->trgts[ctx->usd_trgt]->ctx.pk = (mbedtls_pk_context*) malloc(sizeof(mbedtls_pk_context));
//        mbedtls_pk_init(ctx->trgts[ctx->usd_trgt]->ctx.pk);
//    }
//
//    /* Set "magic" number and return all clear */
//    ctx->trgts[ctx->usd_trgt]->mgno = MAGICK_NO;
//    return QCRY_STATUS_OK;
}

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int *trgt_no)
{
//    CHECK_SANE
//
//    if(!ctx->trgts[*trgt_no]) return QCRY_STATUS_INVALID_TARGET;
//
//    /* Clear target memory and resize buffer if needed */
//    CLEAR_TARGET(ctx->ciph_t, ctx->trgts[*trgt_no])
//    CHECK_BUFFER(qcry_trgt_t* , ctx->trgts, ctx->max_trgt, ctx->usd_trgt)
//
//    return QCRY_STATUS_OK;
}

int qcry_encrypt_trgt(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, size_t msg_len, unsigned char *(*ciph))
{
//    /* Check our context is sane */
//    CHECK_SANE
//
//    /* Check our target is sane */
//    CHECK_TARGET(ctx, trgt_no);
//
//    /* Store pointer to target we're working with directly */
//    qcry_trgt_t *trgt = ctx->trgts[trgt_no];
//
//    if(ctx->ciph_t == AES256) {
//
//    } else if(ctx->ciph_t == PK_RSA) {
//        mbedtls_pk_parse_public_key(trgt->ctx.pk, trgt->d.pk->usr_key_pub, trgt->d.pk->key_len);
//        size_t out_len = mbedtls_pk_get_len(trgt->ctx.pk);
//
//        mbedtls_pk_encrypt( trgt->ctx.pk, msg, msg_len, *ciph, &out_len, sizeof(*ciph), mbedtls_ctr_drbg_random, ctx->ctr_drbg);
//
//    } else {
//        return QCRY_STATUS_NOT_IMPLEMENTED;
//    }
}