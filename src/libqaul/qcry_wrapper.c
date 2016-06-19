#include <qaullib/qcry_wrapper.h>
#include "crypto/qcry_context.h"
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
    printf("AES256 key length is.....%d",);

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

int foobar(int argc, char *argv[]) {
    FILE *f;
    int ret;
    size_t i;
    mbedtls_rsa_context rsa;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    unsigned char input[1024];
    unsigned char buf[512];
    const char *pers = "rsa_encrypt";

    mbedtls_ctr_drbg_init(&ctr_drbg);
    ret = 1;

    mbedtls_entropy_init(&entropy);
    if ((ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy,
                                     (const unsigned char *) pers,
                                     strlen(pers))) != 0) {
        mbedtls_printf(" failed\n  ! mbedtls_ctr_drbg_seed returned %d\n", ret);
        goto exit;
    }

    mbedtls_printf("\n  . Reading public key from rsa_pub.txt");
    fflush(stdout);

    if ((f = fopen("rsa_pub.txt", "rb")) == NULL) {
        ret = 1;
        mbedtls_printf(" failed\n  ! Could not open rsa_pub.txt\n" \
                "  ! Please run rsa_genkey first\n\n");
        goto exit;
    }

    MBEDTLS_RSA_PKCS_V15

    mbedtls_rsa_init(&rsa, MBEDTLS_RSA_PKCS_V15, 0);

    if ((ret = mbedtls_mpi_read_file(&rsa.N, 16, f)) != 0 ||
        (ret = mbedtls_mpi_read_file(&rsa.E, 16, f)) != 0) {
        mbedtls_printf(" failed\n  ! mbedtls_mpi_read_file returned %d\n\n", ret);
        fclose(f);
        goto exit;
    }

    rsa.len = (mbedtls_mpi_bitlen(&rsa.N) + 7) >> 3;

    fclose(f);

    if (strlen(argv[1]) > 100) {
        mbedtls_printf(" Input data larger than 100 characters.\n\n");
        goto exit;
    }

    memcpy(input, argv[1], strlen(argv[1]));

/*
 * Calculate the RSA encryption of the hash.
 */
    mbedtls_printf("\n  . Generating the RSA encrypted value");
    fflush(stdout);

    if ((ret = mbedtls_rsa_pkcs1_encrypt(&rsa, mbedtls_ctr_drbg_random, &ctr_drbg,
                                         MBEDTLS_RSA_PUBLIC, strlen(argv[1]),
                                         input, buf)) != 0) {
        mbedtls_printf(" failed\n  ! mbedtls_rsa_pkcs1_encrypt returned %d\n\n", ret);
        goto exit;
    }

    /*
     * Write the signature into result-enc.txt
     */
    if ((f = fopen("result-enc.txt", "wb+")) == NULL) {
        ret = 1;
        mbedtls_printf(" failed\n  ! Could not create %s\n\n", "result-enc.txt");
        goto exit;
    }

    for (i = 0; i < rsa.len; i++)
        mbedtls_fprintf(f, "%02X%s", buf[i],
                        (i + 1) % 16 == 0 ? "\r\n" : " ");

    fclose(f);

    mbedtls_printf("\n  . Done (created \"%s\")\n\n", "result-enc.txt");

    exit:
    mbedtls_ctr_drbg_free(&ctr_drbg);
    mbedtls_entropy_free(&entropy);

    return (ret);
}