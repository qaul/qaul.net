/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <qaullib/qcry_wrapper.h>
#include <stdio.h>
#include <string.h>
#include <mbedtls/md5.h>
#include <qaullib/qcry_hashing.h>
#include <mbedtls/base64.h>

#include "crypto/qcry_arbiter.h"
#include "crypto/qcry_keys.h"
#include "crypto/qcry_helper.h"
#include "crypto/qcry_playground.h"

int load_only_public_file(mbedtls_pk_context **pub, const char *path, const char *username)
{
    printf("Unicorns are attacking this library blob!\n");
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    mbedtls_pk_context *tmp = *pub;
    tmp = (mbedtls_pk_context*) malloc(sizeof(mbedtls_pk_context));
    if(tmp == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    /*** Initialise the key contexts properly ***/
    mbedtls_pk_init(tmp);

    /*** Construct the required file names ***/
    char pri_pth[512];
    char pub_pth[512];

    size_t p_s = strlen(path);
    size_t u_s = strlen(username);

    /* Build public key path */
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/00_%s.pub", path, username);
    else                                    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s00_%s.pub", path, username);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("Parsing public key...");
    fflush(stdout);

    ret = mbedtls_pk_parse_public_keyfile(tmp, pub_pth);
    if(ret != 0) {
        mbedtls_printf("FAILED! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret);
        goto cleanup;
    }
    mbedtls_printf("OK\n");


    printf("== Keys loaded successfully ==\n\n");
    *pub = tmp;
    return 0;

    cleanup:
    mbedtls_pk_free(tmp);
    return ret;
}

void l_public(char **pubkey)
{
    mbedtls_pk_context *pub_target;
    load_only_public_file(&pub_target, "/home/spacekookie/.qaul/keystore/", "spacekookie");

    size_t buf_s = 16000;
    unsigned char output_buf[buf_s];

    int ret = mbedtls_pk_write_pubkey_pem(pub_target, output_buf, 16000);

    /* Allocate some memory for our buffer and copy the key */
    (*pubkey) = (char*) calloc(sizeof(char), strlen((char *) output_buf) + 1); // Consider \0 !
    strcpy((char *) *pubkey, (char *)output_buf);

    printf("Public key:\n\n%s", *pubkey);
}

#define ASSERT \
    printf("Return: %d\n", ret); \
    if(ret != 0) goto end;

#define TEST(msg) \
    printf("Return %s: %d\n", #msg, ret); if(ret != 0) goto end;

int qcry_devel_init(int argc, char *argv[])
{
    char *key_path = "/home/spacekookie/.qaul/";
    char *msgfile = "/home/spacekookie/message.txt";
    char *name_kookie = "spacekookie";
    char *name_jane = "janethemaine";

    char *signature;
    char *message = "This is my message that is really cool and will definately fit in all of my buffers!";

    qcry_arbit_init(1, key_path);

    int ret;
    int kookie;
    ret = qcry_arbit_usrcreate(&kookie, name_kookie, "mypassphrase", QCRY_KEYS_RSA);
    TEST("CREATE")

    int jane;
    ret = qcry_arbit_usrcreate(&jane, name_jane, "mypassphrase", QCRY_KEYS_RSA);
    TEST("CREATE")

    char *kookie_fp;
    qcry_arbit_getusrinfo(&kookie_fp, kookie, QAUL_FINGERPRINT);

    char *jane_fp;
    qcry_arbit_getusrinfo(&jane_fp, jane, QAUL_FINGERPRINT);


    { // Manually add keys

        char *kookiekey;
        qcry_arbit_getusrinfo(&kookiekey, kookie, QAUL_PUBKEY);

        char *janekey;
        qcry_arbit_getusrinfo(&janekey, jane, QAUL_PUBKEY);

        ret = qcry_arbit_addkey(kookiekey, strlen(kookiekey) + 1, kookie_fp, name_kookie);
        TEST("ADD KEY")

        ret = qcry_arbit_addkey(janekey, strlen(janekey) + 1, jane_fp, name_jane);
        TEST("ADD KEY")
    };

    ret = qcry_arbit_signmsg(jane, &signature, message);
    TEST("SIGN")

    printf("Signature: %s\n", signature);

    ret = qcry_arbit_addtarget(kookie, jane_fp);
    TEST("ADD TARGET")

    ret = qcry_arbit_verify(kookie, 0, message, signature);
    printf("Signature: %s\n", (ret == 0) ? "GOOD" : "BOGUS! DO NOT TRUST!");

//    char *signature;
//    ret = qcry_arbit_signmsg(usrno, &signature, message);
//
//    ret = qcry_arbit_verify(target, usrno, message, signature);
//    if(ret == 0) {
//        printf("Message was signed properly!\n");
//    } else {
//        printf("Signature is BOGUS! Do not trust!\n");
//    }
    end:
    return ret;
}
