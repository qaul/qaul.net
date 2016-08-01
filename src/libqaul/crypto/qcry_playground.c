/*
 *  Example RSA key generation program
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

#include "qcry_playground.h"

#define KEY_SIZE 4096
#define EXPONENT 65537


/**********************************************************************************************/
/***************** GET ENTROPY SOURCE ON UNIX SYSTEMS THAT SUPPORT IT *************************/
/**********************************************************************************************/

#if !defined(_WIN32)

#include <unistd.h>

#define DEV_RANDOM_THRESHOLD        32
#define DFL_TYPE                    MBEDTLS_PK_RSA
#define DFL_EC_CURVE                mbedtls_ecp_curve_list()->grp_id
#define DFL_RSA_KEYSIZE             4096
#define DFL_FILENAME                "keyfile.key"
#define DFL_USE_DEV_RANDOM          0

/**
 * A function that provides better entropy for key generation if it's available for a specific platform
 * i.e. Linux
 *
 * @param data
 * @param output
 * @param len
 * @param olen
 * @return
 */
int dev_random_entropy_poll(void *data, unsigned char *output, size_t len, size_t *olen)
{
    FILE *file;
    size_t ret, left = len;
    unsigned char *p = output;
    ((void) data);

    *olen = 0;

    file = fopen("/dev/random", "rb");
    if (file == NULL)
        return MBEDTLS_ERR_ENTROPY_SOURCE_FAILED;

    while (left > 0) {
        /** Because /dev/random doesn't block, we need to retry if not enough */
        ret = fread(p, 1, left, file);
        if (ret == 0 && ferror(file)) {
            fclose(file);
            return MBEDTLS_ERR_ENTROPY_SOURCE_FAILED;
        }

        p += ret;
        left -= ret;
        sleep(1);
    }
    fclose(file);
    *olen = len;

    return 0;
}

#endif /* !_WIN32 */


/**********************************************************************************************/
/**********************************************************************************************/
/**********************************************************************************************/

/**
 * Writing private key to specific path
 */
int write_keys(mbedtls_pk_context *key, const char *output_file)
{
    /** A block to write the publc key! */
    {
        int ret;
        FILE *f;
        unsigned char output_buf[16000];
        unsigned char *c = output_buf;
        size_t len = 0;

        memset(output_buf, 0, 16000);
        ret = mbedtls_pk_write_pubkey_pem(key, output_buf, 16000);
        if(ret != 0) return(ret);

        /** Get the exact length of what we're supposed to write */
        len = strlen((char *) output_buf);

        char fiiile[strlen(output_file) + strlen(".pub")];
        strcpy(fiiile, output_file);
        strcat(fiiile, ".pub");

        if((f = fopen(fiiile, "w")) == NULL)
            return(-1);

        if(fwrite(c, 1, len, f) != len)
        {
            fclose(f);
            return(-1);
        }

        fclose(f);
    }

    int ret;
    FILE *f;
    size_t buf_s = 16000;
    unsigned char output_buf[buf_s];
    unsigned char *c = output_buf;
    size_t len = 0;

    /** Manually clearing buffer  */
    memset(output_buf, 0, buf_s);

    if ((ret = mbedtls_pk_write_key_pem(key, output_buf, buf_s)) != 0)
        return (ret);

    len = strlen((char *) output_buf);

    char fiiile[strlen(output_file) + strlen(".key")];
    strcpy(fiiile, output_file);
    strcat(fiiile, ".key");

    if ((f = fopen(fiiile, "wb")) == NULL)
        return (-1);

    if (fwrite(c, 1, len, f) != len) {
        fclose(f);
        return (-1);
    }

    fclose(f);
    return 0;
}

int qcry_key_generate(mbedtls_pk_context **k)
{
    int ret = 0;
    const char *pers = "qaul.net_generated_keys"; // Should be username or MAC Address

    /* Temp buffers */
    mbedtls_pk_context tmp_ctx;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    /** Setup core state for key generation */
    mbedtls_pk_init(&tmp_ctx);
    mbedtls_ctr_drbg_init(&ctr_drbg);
//    memset(buf, 0, sizeof(buf));

    int type = DFL_TYPE;
    int rsa_keysize = DFL_RSA_KEYSIZE;

    /*********************************************/

    /** Prepare key generation procedure */
    printf("Seeding random number generators...\n");
    mbedtls_entropy_init(&entropy);
    ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy, (const unsigned char *) pers, strlen(pers));


    if (ret != 0) {
        mbedtls_printf(" failed\n  ! mbedtls_ctr_drbg_seed returned -0x%04x\n", -ret);
        goto exit;
    }

    /*********************************************/

    printf("Generating private key...");
    ret = mbedtls_pk_setup(&tmp_ctx, mbedtls_pk_info_from_type(type));
    ret = mbedtls_rsa_gen_key(mbedtls_pk_rsa(tmp_ctx), mbedtls_ctr_drbg_random, &ctr_drbg, rsa_keysize, 65537);

    if (ret != 0) {
        printf(" failed\nmbedtls_pk_setup returned -0x%04x", -ret);
        goto exit;
    }

    mbedtls_printf(" ok\n==> Key information:\n");
    /*********************************************/

    /** Malloc the apropriate space we need and memcpy */
    (*k) = (mbedtls_pk_context *) malloc(sizeof(mbedtls_pk_context));
    memcpy(*k, &tmp_ctx, sizeof(mbedtls_pk_context));

    ret = write_keys(&tmp_ctx, "/home/spacekookie/.qaul/01_spacekookie");
    if (ret != 0) {
        mbedtls_printf(" failed\n");
        goto exit;
    }

    /*********************************************/

    exit:
    mbedtls_ctr_drbg_free(&ctr_drbg);
    return ret;
}

int read_private_key(mbedtls_pk_context **key, const char *input_file)
{

    return 0;
}

int sign_msg(mbedtls_pk_context *key, const char *msgfile)
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

    if( ( ret = mbedtls_ctr_drbg_seed( &ctr_drbg, mbedtls_entropy_func, &entropy,
                                       (const unsigned char *) pers,
                                       strlen( pers ) ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_ctr_drbg_seed returned -0x%04x\n", -ret );
        goto exit;
    }

    /*
     * Compute the SHA-256 hash of the input file,
     * then calculate the signature of the hash.
     */
    mbedtls_printf( "\n  . Generating the SHA-256 signature" );
    fflush( stdout );

    if( ( ret = mbedtls_md_file(
            mbedtls_md_info_from_type( MBEDTLS_MD_SHA256 ),
            msgfile, hash ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! Could not open or read %s\n\n", msgfile);
        goto exit;
    }

    if( ( ret = mbedtls_pk_sign(key, MBEDTLS_MD_SHA256, hash, 0, buf, &olen,
                                 mbedtls_ctr_drbg_random, &ctr_drbg ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_sign returned -0x%04x\n", -ret );
        goto exit;
    }

    /*
     * Write the signature into <filename>.sig
     */
    mbedtls_snprintf( filename, sizeof(filename), "%s.sig", msgfile);

    if( ( f = fopen( filename, "wb+" ) ) == NULL )
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
    mbedtls_ctr_drbg_free( &ctr_drbg );
    mbedtls_entropy_free( &entropy );
    return(ret);
}

int verify_msg(mbedtls_pk_context *k, const char *signfile)
{
    int ret = 1;
    FILE *f;
    size_t i;
    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
    char filename[512];
    mbedtls_pk_context pk;

    char *publickeyname = "/home/spacekookie/.qaul/00_spacekookie.pub";

    fflush( stdout );

    if( ( ret = mbedtls_pk_parse_public_keyfile( &pk, publickeyname ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret );
        goto exit;
    }

    ret = 1;
    mbedtls_snprintf( filename, sizeof(filename), "%s", signfile );

    if((f = fopen(filename, "rb")) == NULL )
    {
        mbedtls_printf( "\n  ! Could not open %s\n\n", filename );
        goto exit;
    }

    /** Validate size and then close stream */
    i = fread( buf, 1, sizeof(buf), f );
    fclose(f);


    /*
     * Compute the SHA-256 hash of the input file and
     * verify the signature
     */
    mbedtls_printf( "\n  . Verifying the SHA-256 signature" );
    fflush( stdout );

    if( ( ret = mbedtls_md_file(mbedtls_md_info_from_type( MBEDTLS_MD_SHA256 ), signfile, hash ) ) != 0 )
    {
        mbedtls_printf( " failed\n  ! Could not open or read %s\n\n", signfile );
        goto exit;
    }

    if((ret = mbedtls_pk_verify(&pk, MBEDTLS_MD_SHA256, hash, 0, buf, i ) ) != 0)
    {
        mbedtls_printf( " failed\n  ! mbedtls_pk_verify returned -0x%04x\n", -ret );
        goto exit;
    }

    mbedtls_printf( "\n  . OK (the signature is valid)\n\n" );

    exit:
    return ret;
}


int qcry_sign_with_key() {

}
