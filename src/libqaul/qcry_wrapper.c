/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#include <qaullib/qcry_wrapper.h>
#include <stdio.h>
#include <string.h>

#include "qaullib/qcry_arbiter.h"
#include "crypto/qcry_keys.h"
#include "crypto/qcry_helper.h"
#include "crypto/qcry_playground.h"

// Forward declaration to make it easier to work with this monster function
int fooooooo(int argc, char *argv[]);

int context_init(mbedtls_pk_context *key)
{
    mbedtls_pk_init(key);
    return 0;
}

int load_key(mbedtls_pk_context *pub, mbedtls_pk_context *pri, char *keyfile)
{
    int ret;
    char private[512];
    char public[512];

    mbedtls_snprintf( private, sizeof(private), "%s.key", keyfile);
    mbedtls_snprintf( public, sizeof(public), "%s.pub", keyfile);

    mbedtls_printf( "\n  . Reading public key from '%s'", public);
    fflush( stdout );

    if( ( ret = mbedtls_pk_parse_public_keyfile(pub, public) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret );
        return 55;
    }

    mbedtls_printf( "\n  . Reading private key from '%s'",private);
    fflush( stdout );

    if( ( ret = mbedtls_pk_parse_keyfile(pri, private, "" ) ) != 0 )
    {
        ret = 1;
        mbedtls_printf( " failed\n  ! mbedtls_pk_parse_keyfile returned -0x%04x\n", -ret );
    }

    return 0;
}

int sign(mbedtls_pk_context *pri, char *message, size_t msg_len, char *msgfile)
{
    FILE *f;
    int ret = 1;

    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
//    unsigned char buf2[MBEDTLS_MPI_MAX_SIZE];

    char sig_name[512];
    const char *pers = "mbedtls_pk_sign";
    size_t olen = 0;

    /*** Setup entropy and random number generators for hashing ***/
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    mbedtls_entropy_init(&entropy);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    mbedtls_printf("Seeding the random number generator...");
    fflush(stdout);

    ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy, (const unsigned char *) pers, strlen(pers));
    if(ret != 0) {
        mbedtls_printf("FAILED\n\tmbedtls_ctr_drbg_seed returned -0x%04x\n", -ret);
        goto exit;
    }
    mbedtls_printf("OK\n");

    /*** (For now) Read input file from disk ***/
    mbedtls_printf("Reading messagefile...");
    fflush(stdout);

    ret = mbedtls_md_file(mbedtls_md_info_from_type(MBEDTLS_MD_SHA256), msgfile, hash);
    if(ret != 0) {
        mbedtls_printf("FAILED\n\tCould not open or read %s\n", msgfile);
        goto exit;
    }
    mbedtls_printf("OK\n");

    /** Copy our message into the buffer */
//    strcpy(buf2, message);

    /*** Compute SHA-256 Digest of our message ***/
    mbedtls_printf("Computing SHA-256 Digest...");
    fflush(stdout);

    ret = mbedtls_pk_sign(pri, MBEDTLS_MD_SHA256, hash, 0, buf, &olen, mbedtls_ctr_drbg_random, &ctr_drbg);
    if(ret != 0) {
        mbedtls_printf("FAILED\n\tmbedtls_pk_sign returned -0x%04x\n", -ret);
        goto exit;
    }
    mbedtls_printf("OK\n");



    /*** Go ahead and write the signature to disk ***/
    mbedtls_snprintf(sig_name, sizeof(sig_name), "%s.sig", "");

    mbedtls_printf("Writing signature file...");
    if((f = fopen(sig_name, "wb+" ) ) == NULL ) {
        ret = 1;
        mbedtls_printf("FAILED\n\tCould not create %s\n", sig_name);
        goto exit;
    }

    if(fwrite(buf, 1, olen, f) != olen) {
        mbedtls_printf("FAILED\n\tfwrite returned bad dump length!\n");
        fclose(f);
        goto exit;
    }

    mbedtls_printf("OK\n");

    /*** Cleanup ***/
    fclose(f);
    ret = 0;
    printf("== Signature written successfully ==\n\n");

    exit:
    return ret;
}


int qcry_devel_init(int argc, char *argv[])
{
    int ret = 1;
    mbedtls_pk_context *pub, *pri;


    char *key_path = "/home/spacekookie/Downloads/mbedtls-2.3.0/build/programs/pkey/";
    char *msgfile = "/home/spacekookie/message.txt";
    char *username = "keyfile";

    char *message = "This is my message!";
    char *signature;
    size_t sign_len;

    /* Load the keys */
    ret = qcry_key_load(&pub, &pri, key_path, username);

    /* Sign our message */
//    sign_msg(pri, msgfile);

//    load_key(&pub, &pri, keyfile);

    sign(pri, message, sizeof(message) + 1, msgfile);

    ret = verify_msg(pub, msgfile);
    return ret;

//    mbedtls_pk_context *tmp;

//    const char *path = "/home/spacekookie/Downloads/mbedtls-2.3.0/build/programs/pkey";
//    const char *user = "keyfile";
//
//    const char *pub_usr = "/home/spacekookie/Downloads/mbedtls-2.3.0/build/programs/pkey/00_keyfile.pub";
////
////    ret = qcry_key_generate(&tmp, "Qaul.net is the secret!");
////    printf((ret == 0) ? "[KEYGEN]: ALL CLEAR\n" : "[KEYGEN]: AN ERROR OCCURED WITH CODE %d\n", ret);
////
////    ret = qcry_key_write(tmp, path, user);
////    printf((ret == 0) ? "[WRITE]: ALL CLEAR\n" : "[KEYGEN]: AN ERROR OCCURED WITH CODE %d\n", ret);
////
////    ret = qcry_key_destroy(tmp);
////    printf((ret == 0) ? "[DESTROY]: ALL CLEAR\n" : "[KEYGEN]: AN ERROR OCCURED WITH CODE %d\n", ret);
////
////    /*********************************************************************/
//    /*** Reading in the keys to get a split context ***/
//    mbedtls_pk_context pub, *pri;
//    mbedtls_pk_init(&pub);
//
////    ret = qcry_key_load(&pub, &pri, path, user);
////    printf((ret == 0) ? "[LOAD]: ALL CLEAR\n" : "[KEYGEN]: AN ERROR OCCURED WITH CODE %d\n", ret);
//
//    printf("\n=======================\n\n");
//
//    /** Sign a message file */
//
//
//    ret = sign_msg(pri, "/home/spacekookie/message.txt");
//    printf((ret == 0) ? "[SIGN]: ALL CLEAR\n" : "[SIGN]: AN ERROR OCCURED WITH CODE %d", ret);
//
//    printf("Reading public key file: %s\n", pub_usr);
//
//    ret = mbedtls_pk_parse_public_keyfile(&pub, pub_usr);
//    if(ret != 0) {
//        printf("An error occured while parsing public key file: %d!", ret);
//    }
//
//    /** Then sign an arbitrary msg with them */
//    ret = verify_msg(&pub, "/home/spacekookie/message.txt");
//    printf((ret == 0) ? "[VERIFY]: ALL CLEAR\n" : "[VERIFY]: AN ERROR OCCURED WITH CODE %d", ret);
}
