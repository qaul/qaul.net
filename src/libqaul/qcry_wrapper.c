/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <qaullib/qcry_wrapper.h>
#include <stdio.h>
#include <string.h>

#include "crypto/qcry_arbiter.h"
#include "crypto/qcry_keys.h"
#include "crypto/qcry_helper.h"

int qcry_devel_init() {
    int ret = 0, i, k;
    unsigned char key[QCRY_KEYS_KL_AES];
//
//    qcry_keys_context context;
//    qcry_keys_init(&context);
//
//    printf("Context init...done\n");
//
////    ret = mbedtls_ctr_drbg_random(&context.rand, key, QCRY_KEYS_KL_AES);
//
//    printf("Our key buffer: %s of size %d", key, sizeof(key));

    qcry_keys_context ctx;
    qcry_keys_init(&ctx);

    unsigned char *buffer;
    qcry_keys_gen_m(&ctx, QCRY_KEYS_RSA, &buffer);

    printf("Our key buffer is: %s", buffer);

//    qcry_keys_context context;
//    ret = qcry_keys_init(&context);
//    if(ret != 0)
//    {
//        printf("Init return is: %d", ret);
//        goto exit;
//    }
//
//    ret = qcry_keys_gen(&context, QCRY_KEYS_AES, key);
//
//    printf("Our key is: %s with the length: %d", key, strlen(key));
//    exit:
//    return ret;
}