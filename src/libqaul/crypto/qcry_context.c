/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <stdlib.h>
#include <memory.h>

#include "qcry_context.h"
#include "qcry_keys.h"
#include "qcry_keystore.h"
#include <qaullib/qcry_hashing.h>
#include <mbedtls/pk.h>
#include <mbedtls/pk_internal.h>
#include <mbedtls/platform.h>
#include <mbedtls/base64.h>
#include <mbedtls/md_internal.h>
#include <mbedtls/md.h>

//////////////////////////// SOME HELPFUL MACROS ////////////////////////////

#define FIND_TRGT(finterprint) \
    int i;  \
    qcry_trgt_t     *target = NULL; \
    for(i = 0; i < ctx->usd_trgt; i++) {    \
        if(strcmp(ctx->trgts[i]->finterprint, finterprint) == 0) {   \
            target = ctx->trgts[i];\
            break; \
        }\
    } \
    if(target == NULL) return QCRY_STATUS_INVALID_TARGET;

int qcry_context_init(qcry_usr_ctx *ctx, const char *usr_name, qcry_ciph_t ciph_t)
{
    /* Zero out so we are clean */
    memset(ctx, 0, sizeof(qcry_usr_ctx));

    int ret;

    /* Set some basic identity metadata */
    ctx->username = usr_name;
    ctx->birthdate = NULL;
    ctx->ciph_t = ciph_t;

    switch(ciph_t) {
        case PK_RSA:
            ctx->ciph_len = QCRY_KEYS_KL_RSA;
            break;
        case ECDSA:
            ctx->ciph_len = QCRY_KEYS_KL_ECC;
            break;
        case AES256:
            ctx->ciph_len = QCRY_KEYS_KL_AES;
            break;
        default:
            return QCRY_STATUS_INVALID_PARAMS;
    }

    /* Setup entropy and random number generators */
    ctx->ctr_drbg = (mbedtls_ctr_drbg_context*) malloc(sizeof(mbedtls_ctr_drbg_context));
    if(ctx->ctr_drbg == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    ctx->entropy = (mbedtls_entropy_context*) malloc(sizeof(mbedtls_entropy_context));
    if(ctx->entropy == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    mbedtls_ctr_drbg_init(ctx->ctr_drbg);
    mbedtls_entropy_init(ctx->entropy);

    /* Seed the random number generator for a user-by-user basis */
    ret = mbedtls_ctr_drbg_seed(ctx->ctr_drbg, mbedtls_entropy_func,
                                ctx->entropy, (unsigned char *) ctx->username,
                                strlen(ctx->username));
    if(ret != 0)
        return QCRY_STATUS_SEED_FAILED;

    /* Make sure we have some space for targets */
    ctx->trgts = (qcry_trgt_t**) calloc(sizeof(qcry_trgt_t*), MIN_BFR_S);
    if(ctx->trgts == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    ctx->max_trgt = MIN_BFR_S;
    ctx->usd_trgt = 0;

    /* Set our magic number to indicate that we are awesome */
    ctx->mgno = MAGICK_NO;

    /* Return success :) */
    return QCRY_STATUS_OK;
}

int qcry_context_free(qcry_usr_ctx *ctx)
{
    return QCRY_STATUS_OK;
}

int qcry_context_mktarget(qcry_trgt_t *(*t), const char *fingerprint)
{
    /* Allocate some memory for us to use */
    *t = (qcry_trgt_t*) malloc(sizeof(qcry_trgt_t));
    if(*t == NULL)
        return QCRY_STATUS_MALLOC_FAIL;
    memset(*t, 0, sizeof(qcry_trgt_t));

    /* Get the known public key for this target */
    mbedtls_pk_context *pub;
    qcry_ks_getkey(&pub, fingerprint);
    if(pub == NULL)
        return QCRY_STATUS_KS_NOTFOUND;

    /* Get username information from keystore */
    char *username;
    qcry_ks_getusername(&username, fingerprint);
    if(username == NULL)
        return QCRY_STATUS_KS_NOTFOUND;

    /* Copy the fingerprint into memory WE can control */
    size_t fplen = strlen(fingerprint) + 1;
    char *fp = (char*) malloc(sizeof(char) * fplen);
    strcpy(fp, fingerprint);

    /* Copy the username into memory WE can control */
    size_t usrlen = strlen(fingerprint) + 1;
    char *user = (char*) malloc(sizeof(char) * usrlen);
    strcpy(user, username);

    /* Populate! */
    (*t)->fingerprint = fp;
    (*t)->username = user;
    (*t)->public = pub;
    (*t)->types = PK_RSA;

    /* Such magic! */
    (*t)->mgno = MAGICK_NO;

    return QCRY_STATUS_OK;
}

int qcry_context_attach(qcry_usr_ctx *ctx, mbedtls_pk_context *pub, mbedtls_pk_context *pri)
{
    CHECK_SANE

    // TODO: Check the key type is correct!

    /* Assign the keys */
    ctx->public = pub;
    ctx->private = pri;

    /******* Calculating Fingerprint for this context *******/

    size_t buffer_s = 16000;
    unsigned char pub_buf[buffer_s];
    ret = mbedtls_pk_write_pubkey_pem(pub, pub_buf, buffer_s);
    if(ret != 0) return QCRY_STATUS_INVALID_KEYS;

    /* Copy the required keylength into a heap buffer */
    size_t keylen = strlen((char*) pub_buf) + 1; // Consider phlebas (\0) !
    char *tmp_buf = (char*) malloc(sizeof(char) * keylen);

    /* Check our memory space is valid and copy key into it */
    if(tmp_buf == NULL) return QCRY_STATUS_MALLOC_FAIL;
    strcpy(tmp_buf, (char*) pub_buf);

    /*** Fingerprint is SHA256 digest of private key ***/

    struct qcry_hash_ctx hash; // Keep on stack for speed

    ret = qcry_hashing_init(&hash, QCRY_HASH_SHA256, ctx->username);
    if(ret) goto exit;

    ret = qcry_hashing_append(&hash, tmp_buf);
    if(ret) goto exit;

    ret = qcry_hashing_build(&hash, QCRY_HASH_BASE64, &ctx->fingerprint);
    if(ret) goto exit;

    /* Allow to skip steps to free resources again */
    exit:

    /* Free our resources */
    memset(pub_buf, 0, buffer_s);
    qcry_hashing_free(&hash);

    free(tmp_buf);

    /* Return success :) */
    return ret;
}


int qcry_context_signmsg(qcry_usr_ctx *ctx, const char *msg, unsigned char *(*sign))
{
    CHECK_SANE

    /* Creating a few variables to use */
    unsigned char hash_buf[QAUL_SIGN_HASH_LEN];
    unsigned char sign_buf[QAUL_SIGNATURE_LEN];
    size_t olen = 0;

    /* Hash the message to sign it */
    mbedtls_md_context_t md_ctx;
    const mbedtls_md_info_t *md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA256);

    mbedtls_md_init( &md_ctx );
    ret = mbedtls_md_setup( &md_ctx, md_info, 0 );
    if(ret != 0) {
        printf("An error occured setting up digest environment: %d!\n", ret);
        goto exit;
    }

    /** Compute SHA-256 digest of message for signature */
    md_info->starts_func(md_ctx.md_ctx);
    md_info->update_func(md_ctx.md_ctx, (const unsigned char*) msg, strlen(msg) + 1);
    md_info->finish_func(md_ctx.md_ctx, hash_buf);

    /** Compute signature with message digest and private key */
    ret = mbedtls_pk_sign(ctx->private, MBEDTLS_MD_SHA256,
                          hash_buf, 0, sign_buf, &olen,
                          mbedtls_ctr_drbg_random, ctx->ctr_drbg);

    if(olen != QAUL_SIGNATURE_LEN)
        printf("[WARNING] Signature length doesn't match for message '%s'...Misalignment probable!", msg);

    if(ret != 0)
        return QCRY_STATUS_ERROR;

    /** Finally allocate some memory for the signature */
    (*sign) = (unsigned char*) calloc(sizeof(unsigned char), olen);
    if(*sign == NULL)
        return QCRY_STATUS_MALLOC_FAIL;

    /* Copy signature to public and return */
    memcpy(*sign, sign_buf, olen);

    exit:
    mbedtls_md_free(&md_ctx);
    return ret;
}

int qcry_context_verifymsg(qcry_usr_ctx *ctx, const unsigned int trgt_no, const char *msg, const unsigned char *sign, bool *ok)
{
    CHECK_SANE

    /* Check the target is valid */
    if(ctx->usd_trgt < trgt_no)
        return QCRY_STATUS_INVALID_TARGET;

    /* Check message length - Consider \0 term */
    if(strlen(msg) >= QAUL_MAX_MSG_LENGTH) {
        printf("[FATAL] Message to sign too long!\n");
        return QCRY_STATUS_FATAL;
    }

    /* Always asume the worst */
    *ok = false;

    /*** Prapre are few variables we might need ***/
    qcry_trgt_t *t = ctx->trgts[trgt_no];
    mbedtls_pk_context *pub = t->public;

    /* Buffer for plain, decoded signature and hashed msg */
    unsigned char msg_hash[QAUL_SIGN_HASH_LEN];

    /*******************************************************************/
    mbedtls_md_context_t md_ctx;
    const mbedtls_md_info_t *md_info = mbedtls_md_info_from_type(MBEDTLS_MD_SHA256);

    mbedtls_md_init( &md_ctx );
    ret = mbedtls_md_setup( &md_ctx, md_info, 0 );
    if(ret != 0) {
        printf("An error occured setting up digest environment: %d!\n", ret);
        goto exit;
    }

    /** Compute SHA-256 digest of message for verification */
    md_info->starts_func(md_ctx.md_ctx);
    md_info->update_func(md_ctx.md_ctx, (const unsigned char*) msg, strlen(msg) + 1);
    md_info->finish_func(md_ctx.md_ctx, msg_hash);

    /* Verify message signature from public key and self-computed SHA-256 digest */
    ret = mbedtls_pk_verify(pub, MBEDTLS_MD_SHA256, msg_hash, QAUL_SIGN_HASH_LEN, sign, QAUL_SIGNATURE_LEN);

    /* Set our OK flag and return */
    *ok = (ret == 0) ? true : false;

    exit:
    mbedtls_md_free(&md_ctx);
    return ret;
}

int qcry_context_add_trgt(qcry_usr_ctx *ctx, qcry_trgt_t *trgt, qcry_ciph_t ciph_t)
{
    CHECK_SANE

    /* Make sure we have enough space allocated for another target */
    if(ctx->usd_trgt >= ctx->max_trgt) {
        ctx->max_trgt += 10;
        ctx->trgts = (qcry_trgt_t**) realloc(ctx->trgts, sizeof(qcry_trgt_t*) * ctx->max_trgt);
        if(ctx->trgts == NULL)
            return QCRY_STATUS_MALLOC_FAIL;
    }

    /* Safely add our target */
    ctx->trgts[ctx->usd_trgt++] = trgt;

    return QCRY_STATUS_OK;
}

int qcry_context_remove_trgt(qcry_usr_ctx *ctx, unsigned int trgt_no)
{
    CHECK_SANE

    if(trgt_no < 0)
        return QCRY_STATUS_INVALID_TARGET;

    if(!ctx->trgts[trgt_no])
        return QCRY_STATUS_INVALID_TARGET;

    /** Removing the target is easy ... */
    qcry_trgt_t *target = ctx->trgts[trgt_no];
    free((char*) target->fingerprint);
    free((char*) target->username);
    free(target);

    /** Sanitising the datastructure is hard ... */
    ctx->trgts[trgt_no] = NULL;
    ctx->usd_trgt--;

    /** Iterate over all targets to find empty (NULL) spaces **/
    int index;
    for(index = 0; index < ctx->usd_trgt; index++) {

        /* Check if the current space is NULL (as we marked it before) */
        if(ctx->trgts[index] == NULL) {

            /* If it is, check if the next space is NULL */
            if(ctx->trgts[index + 1] == NULL) {

                /*
                 * If it is as well, we have reached the end of the array and are
                 * done because: [A, B, C, NULL, NULL, ...]
                 */
                goto exit;

            } else {

                /* Otherwise, we move the item up by one */
                ctx->trgts[index] = ctx->trgts[index + 1];
                ctx->trgts[index + 1] = NULL;
            }
        }
    }

    exit:
    return QCRY_STATUS_OK;
}
