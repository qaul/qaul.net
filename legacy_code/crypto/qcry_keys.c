/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdlib.h>
#include <stdio.h>
#include <memory.h>
#include <mbedtls/platform.h>

#include "qcry_keys.h"
#include "qcry_helper.h"

/** Returns the length of a required key buffer as an unsigned integer.
 *  Returns insanely large number if buffer type not known
 */
unsigned int key_length_by_type(short type)
{
    switch (type){
        case QCRY_KEYS_AES: return QCRY_KEYS_KL_AES;
        case QCRY_KEYS_ECC: return QCRY_KEYS_KL_ECC;
        case QCRY_KEYS_RSA: return QCRY_KEYS_KL_RSA;
        default:            return (unsigned int) -1;
    }
}

// TODO: Refactor this module
// TODO: Turn this into a few issues

int qcry_keys_init(qcry_keys_context *context)
{
    // if (context) return QCRY_STATUS_INVALID_PARAMS;
    qcry_keys_init_all(context, QCRY_KEYS_PREST_ON, 0, QCRY_KEYS_PERM_OFF, QCRY_KEYS_QUIET_OFF);
}

int qcry_keys_init_all(qcry_keys_context *context, short pr, short mseed, short perm, short quiet)
{
    // if (context) return QCRY_STATUS_INVALID_PARAMS;
    int ret;

    /** Go and initialise the contexts */
    mbedtls_ctr_drbg_init(&context->rand);
    mbedtls_entropy_init(&context->entropy);

    /*********************************************************
     * Actually check parameters for accuracy and then fill them in
     *
     * Past this point the context needs to be a well defined state!
     *********************************************************/
    if (perm == QCRY_KEYS_PERM_OFF || perm == QCRY_KEYS_PERM_ON) {
        context->perm = perm;
    } else {
        return QCRY_STATUS_INVALID_PARAMS;
    }

    if (quiet == QCRY_KEYS_QUIET_OFF || quiet == QCRY_KEYS_QUIET_ON) {
        context->quiet = quiet;
    } else {
        return QCRY_STATUS_INVALID_PARAMS;
    }

    if (pr == QCRY_KEYS_PREST_OFF || pr == QCRY_KEYS_PREST_ON) {
        context->pr = pr;
    } else {
        return QCRY_STATUS_INVALID_PARAMS;
    }

    /** This doesn't matter for now! */
    context->mseed = 0;

    /** Setup seed with sane defaults or user params */
    if (context->mseed == 0) {
        ret = mbedtls_ctr_drbg_seed(&context->rand, mbedtls_entropy_func, &context->entropy, (const unsigned char *) "RANDOM_GEN", 10 );
        if (ret != 0) return QCRY_STATUS_SEED_FAILED;
    } else {
        printf("The function (mseed) has not been implemented yet...\n");
    }

    /** Set prediction resistance according to context */
    if (context->pr) mbedtls_ctr_drbg_set_prediction_resistance(&context->rand, MBEDTLS_CTR_DRBG_PR_ON);
    else mbedtls_ctr_drbg_set_prediction_resistance(&context->rand, MBEDTLS_CTR_DRBG_PR_OFF);

    /** We're done... */
    return QCRY_STATUS_OK;
}

int qcry_keys_gen(qcry_keys_context *context, short type, unsigned char *buf)
{
    int ret = 0;
    int buf_size = key_length_by_type(type);

    //if(context == NULL) return QCRY_STATUS_INVALID_PARAMS;
    //if(context->rand) return QCRY_STATUS_INVALID;
    if(sizeof(buf) != buf_size) return QCRY_STATUS_BFR_TOO_SMALL;

    /** At this point we should be ready for some randomness :) */
    ret = mbedtls_ctr_drbg_random(&context->rand, buf, buf_size);
    if(ret != 0) return QCRY_STATUS_KEYGEN_FAILED;

    /** If everything went well return positive */
    return QCRY_STATUS_OK;
}

int qcry_keys_gen_m(qcry_keys_context *context, short type, unsigned char *(*buf))
{
    int ret = 0;
    int buf_size = key_length_by_type(type);

    //if(context) return QCRY_STATUS_INVALID_PARAMS;
    //if(context->rand) return QCRY_STATUS_INVALID;

    *buf = (unsigned char*) malloc(sizeof(unsigned char) * buf_size);
    if(buf == NULL) return QCRY_STATUS_BFR_TOO_SMALL;

    unsigned char tmp[buf_size];

    /** At this point we should be ready for some randomness :) */
    ret = mbedtls_ctr_drbg_random(&context->rand, tmp, (size_t) buf_size);
    if(ret != 0) return QCRY_STATUS_KEYGEN_FAILED;

    memcpy(buf, tmp, buf_size);

    /** If everything went well return positive */
    return QCRY_STATUS_OK;
}

int qcry_keys_gen_r(qcry_keys_context *context, unsigned int length, unsigned char *(*buf))
{
    int ret = 0;

    *buf = (unsigned char*) malloc(sizeof(unsigned char) * length);
    if(buf == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    unsigned char tmp[length];
    ret = mbedtls_ctr_drbg_random(&context->rand, tmp, (size_t) length);
    if(ret != 0)
        return QCRY_STATUS_KEYGEN_FAILED;

    memcpy(buf, tmp, length);
    return QCRY_STATUS_OK;
}

int qcry_keys_rsagen(qcry_keys_context *ctx, mbedtls_pk_context *(*mul), const char *pers)
{
    int ret = 0;

    /* Temp buffers */
    mbedtls_pk_context tmp_pri;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    /** Setup core state for key generation */
    mbedtls_pk_init(&tmp_pri);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    /* Set some state variables */
    int type = MBEDTLS_PK_RSA;
    int rsa_keysize = QCRY_KEYS_KL_RSA;

    /*********************************************/

    /** Prepare key generation procedure */
    printf("Seeding random number generators...");
    mbedtls_entropy_init(&entropy);
    ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy, (const unsigned char *) pers, strlen(pers));

    if (ret != 0) {
        printf("FAILED!\n\tmbedtls_ctr_drbg_seed returned %d\n", ret);
        goto exit;
    }
    printf("OK\n");

    /*********************************************/

    printf("Generating private key...");
    fflush(stdout);
    ret = mbedtls_pk_setup(&tmp_pri, mbedtls_pk_info_from_type(type));
    if (ret != 0) {
        printf("FAILED!\nmbedtls_pk_setup returned %d\n", ret);
        goto exit;
    }

    ret = mbedtls_rsa_gen_key(mbedtls_pk_rsa(tmp_pri), mbedtls_ctr_drbg_random, &ctr_drbg, rsa_keysize, 65537);

    if (ret != 0) {
        printf("FAILED!\nmbedtls_ctr_drbg_seed returned %d\n", ret);
        goto exit;
    }
    printf("OK\n");

    /** Malloc the apropriate space we need and memcpy */
    (*mul) = (mbedtls_pk_context *) malloc(sizeof(mbedtls_pk_context));
    memcpy(*mul, &tmp_pri, sizeof(mbedtls_pk_context));

    /*********************************************/

    exit:
    mbedtls_ctr_drbg_free(&ctr_drbg);
    return ret;
}

int qcry_keys_free(qcry_keys_context *context)
{
    if(context == NULL) return QCRY_STATUS_INVALID_PARAMS;
    mbedtls_ctr_drbg_free(&context->rand);
    mbedtls_entropy_free(&context->entropy);
    memset((void*) context, 0, sizeof(qcry_keys_context));

    return QCRY_STATUS_OK;
}