#include <qaullib/qcry_wrapper.h>
#include "crypto/qaullib_cryptography.h"
#include "crypto/qcry_keys.h"

#include <stdio.h>
#include <string.h>
#include <mbedtls/platform.h>

int qcry_devel_init() {
    int ret = 0, i, k;
    unsigned char key[QCRY_KEYS_KL_AES];

    qcry_keys_context context;
    qcry_keys_init(&context);

    printf("Context init...done\n");

//    ret = mbedtls_ctr_drbg_random(&context.rand, key, QCRY_KEYS_KL_AES);

    printf("Our key buffer: %s of size %d", key, sizeof(key));

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