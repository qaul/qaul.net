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

int sign(mbedtls_pk_context *pri, char *msgfile)
{
    FILE *f;
    int ret = 1;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;
    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
    char filename[512];
    const char *pers = "mbedtls_pk_sign";
    size_t olen = 0;

    mbedtls_entropy_init( &entropy );
    mbedtls_ctr_drbg_init( &ctr_drbg );

    mbedtls_printf( "\n  . Seeding the random number generator..." );
    fflush( stdout );

    ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy, (const unsigned char *) pers, strlen(pers));
    if(ret != 0) {
        mbedtls_printf( " failed\n  ! mbedtls_ctr_drbg_seed returned -0x%04x\n", -ret );
        goto exit;
    }

    /*
 * Compute the SHA-256 hash of the input file,
 * then calculate the signature of the hash.
 */
    mbedtls_printf( "\n  . Generating the SHA-256 signature" );
    fflush( stdout );

    if( ( ret = mbedtls_md_file(mbedtls_md_info_from_type( MBEDTLS_MD_SHA256 ), msgfile, hash ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! Could not open or read %s\n\n", msgfile);
        goto exit;
    }

    if( ( ret = mbedtls_pk_sign( pri, MBEDTLS_MD_SHA256, hash, 0, buf, &olen,
                                 mbedtls_ctr_drbg_random, &ctr_drbg ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_sign returned -0x%04x\n", -ret );
        goto exit;
    }

    /*
     * Write the signature into <filename>.sig
     */
    mbedtls_snprintf(filename, sizeof(filename), "%s.sig", msgfile);

    if((f = fopen(filename, "wb+" ) ) == NULL )
    {
        ret = 1;
        mbedtls_printf( " failed\n  ! Could not create %s\n\n", filename );
        goto exit;
    }

    if( fwrite( buf, 1, olen, f ) != olen )
    {
        mbedtls_printf( "failed\n  ! fwrite failed\n\n" );
        fclose( f );
        goto exit;
    }

    fclose( f );

    mbedtls_printf( "\n  . Done (created \"%s\")\n\n", filename );

    exit:
    return ret;
}

int verify(mbedtls_pk_context *pub, char *msgfile)
{
    int ret = 0;

    FILE *f;
    size_t i;
    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
    char filename[512];

    /*
     * Extract the signature from the file
     */
    ret = 1;
    mbedtls_snprintf( filename, sizeof(filename), "%s.sig", msgfile);

    if( ( f = fopen( filename, "rb" ) ) == NULL )
    {
        mbedtls_printf( "\n  ! Could not open %s\n\n", filename );
        goto exit;
    }


    i = fread( buf, 1, sizeof(buf), f );

    fclose( f );

    /*
     * Compute the SHA-256 hash of the input file and
     * verify the signature
     */
    mbedtls_printf( "\n  . Verifying the SHA-256 signature" );
    fflush( stdout );

    if( ( ret = mbedtls_md_file(
            mbedtls_md_info_from_type( MBEDTLS_MD_SHA256 ),
            msgfile, hash ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! Could not open or read %s\n\n", msgfile);
        goto exit;
    }

    if( ( ret = mbedtls_pk_verify(pub, MBEDTLS_MD_SHA256, hash, 0,
                                   buf, i ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_verify returned -0x%04x\n", -ret );
        goto exit;
    }

    mbedtls_printf( "\n  . OK (the signature is valid)\n\n" );

    ret = 0;

    exit:
    return 0;
}

int qcry_devel_init(int argc, char *argv[])
{
    int ret = 1;
    mbedtls_pk_context *pub, *pri;


    char *key_path = "/home/spacekookie/Downloads/mbedtls-2.3.0/build/programs/pkey/";
    char *msgfile = "/home/spacekookie/message.txt";
    char *username = "keyfile";

    /* Load the keys */
    ret = qcry_key_load(&pub, &pri, key_path, username);

    /* Sign our message */
//    sign_msg(pri, msgfile);

//    load_key(&pub, &pri, keyfile);

//    sign(pri, msgfile);

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
