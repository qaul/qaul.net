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

int load_key(mbedtls_pk_context *pub, char *keyfile)
{
    int ret;
    mbedtls_printf( "\n  . Reading public key from '%s'", keyfile);
    fflush( stdout );

    if( ( ret = mbedtls_pk_parse_public_keyfile(pub, keyfile) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret );
        return 55;
    }

    return 0;
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
    mbedtls_pk_context pub;


    char *keyfile = "/home/spacekookie/Downloads/mbedtls-2.3.0/build/programs/pkey/00_keyfile.pub";
    char *msgfile = "/home/spacekookie/message.txt";

    context_init(&pub);

    load_key(&pub, keyfile);

    verify(&pub, msgfile);

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

/*
 *  Public key-based signature verification program
 *
 *  Copyright (C) 2006-2015, ARM Limited, All Rights Reserved
 *  SPDX-License-Identifier: Apache-2.0
 *
 *  Licensed under the Apache License, Version 2.0 (the "License"); you may
 *  not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *
 *  http://www.apache.org/licenses/LICENSE-2.0
 *
 *  Unless required by applicable law or agreed to in writing, software
 *  distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
 *  WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 *  See the License for the specific language governing permissions and
 *  limitations under the License.
 *
 *  This file is part of mbed TLS (https://tls.mbed.org)
 */

#if !defined(MBEDTLS_CONFIG_FILE)
#include "mbedtls/config.h"
#else
#include MBEDTLS_CONFIG_FILE
#endif

#if defined(MBEDTLS_PLATFORM_C)
#include "mbedtls/platform.h"
#else
#include <stdio.h>
#define mbedtls_snprintf   snprintf
#define mbedtls_printf     printf
#endif

#if !defined(MBEDTLS_BIGNUM_C) || !defined(MBEDTLS_MD_C) || \
    !defined(MBEDTLS_SHA256_C) || !defined(MBEDTLS_PK_PARSE_C) ||   \
    !defined(MBEDTLS_FS_IO)
int main( void )
{
    mbedtls_printf("MBEDTLS_BIGNUM_C and/or MBEDTLS_MD_C and/or "
           "MBEDTLS_SHA256_C and/or MBEDTLS_PK_PARSE_C and/or "
           "MBEDTLS_FS_IO not defined.\n");
    return( 0 );
}
#else

#include "mbedtls/error.h"
#include "mbedtls/md.h"
#include "mbedtls/pk.h"

#include <stdio.h>
#include <string.h>

int fooooo( int argc, char *argv[] )
{
    FILE *f;
    int ret = 1;
    size_t i;
    mbedtls_pk_context pk;
    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
    char filename[512];

    mbedtls_pk_init(&pk);

    mbedtls_printf( "\n  . Reading public key from '%s'", argv[1] );
    fflush( stdout );

    if( ( ret = mbedtls_pk_parse_public_keyfile( &pk, argv[1] ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret );
        goto exit;
    }

    /*
     * Extract the signature from the file
     */
    ret = 1;
    mbedtls_snprintf( filename, sizeof(filename), "%s.sig", argv[2] );

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
            argv[2], hash ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! Could not open or read %s\n\n", argv[2] );
        goto exit;
    }

    if( ( ret = mbedtls_pk_verify( &pk, MBEDTLS_MD_SHA256, hash, 0,
                                   buf, i ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_verify returned -0x%04x\n", -ret );
        goto exit;
    }

    mbedtls_printf( "\n  . OK (the signature is valid)\n\n" );

    ret = 0;

    exit:
    mbedtls_pk_free( &pk );

#if defined(MBEDTLS_ERROR_C)
    if( ret != 0 )
    {
        mbedtls_strerror( ret, (char *) buf, sizeof(buf) );
        mbedtls_printf( "  !  Last error was: %s\n", buf );
    }
#endif

#if defined(_WIN32)
    mbedtls_printf( "  + Press Enter to exit this program.\n" );
    fflush( stdout ); getchar();
#endif

    return( ret );
}
#endif /* MBEDTLS_BIGNUM_C && MBEDTLS_SHA256_C &&
          MBEDTLS_PK_PARSE_C && MBEDTLS_FS_IO */
