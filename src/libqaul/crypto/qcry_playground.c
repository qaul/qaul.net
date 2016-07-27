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

struct options {
    int type;
    int rsa_keysize;
    int ec_curve;
    const char *filename;
    int use_dev_random;
} opt;

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
int write_private_key(mbedtls_pk_context *key, const char *output_file)
{
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

    if ((f = fopen(output_file, "wb")) == NULL)
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
    char buf[1024];
    int i;
    char *p, *q;
    const char *pers = "gen_key";

    /* Temp buffers */
    mbedtls_pk_context tmp_ctx;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    /** Setup core state for key generation */
    mbedtls_pk_init(&tmp_ctx);
    mbedtls_ctr_drbg_init(&ctr_drbg);
    memset(buf, 0, sizeof(buf));

    /** Set sane defaults for key gen */
    opt.type = DFL_TYPE;
    opt.rsa_keysize = DFL_RSA_KEYSIZE;
    opt.use_dev_random = DFL_USE_DEV_RANDOM;

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
    ret = mbedtls_pk_setup(&tmp_ctx, mbedtls_pk_info_from_type(opt.type));
    ret = mbedtls_rsa_gen_key(mbedtls_pk_rsa(tmp_ctx), mbedtls_ctr_drbg_random, &ctr_drbg, opt.rsa_keysize, 65537);

    if (ret != 0) {
        printf(" failed\nmbedtls_pk_setup returned -0x%04x", -ret);
        goto exit;
    }

    mbedtls_printf(" ok\n==> Key information:\n");
    /*********************************************/

    /** Malloc the apropriate space we need and memcpy */
    (*k) = (mbedtls_pk_context *) malloc(sizeof(mbedtls_pk_context));
    memcpy(*k, &tmp_ctx, sizeof(mbedtls_pk_context));

//    mbedtls_rsa_context *rsa = mbedtls_pk_rsa(tmp_ctx);
//    mbedtls_mpi_write_file("N:  ", &rsa->N, 16, NULL);
//    mbedtls_mpi_write_file("E:  ", &rsa->E, 16, NULL);
//    mbedtls_mpi_write_file("D:  ", &rsa->D, 16, NULL);
//    mbedtls_mpi_write_file("P:  ", &rsa->P, 16, NULL);
//    mbedtls_mpi_write_file("Q:  ", &rsa->Q, 16, NULL);
//    mbedtls_mpi_write_file("DP: ", &rsa->DP, 16, NULL);
//    mbedtls_mpi_write_file("DQ:  ", &rsa->DQ, 16, NULL);
//    mbedtls_mpi_write_file("QP:  ", &rsa->QP, 16, NULL);
//
//    ret = write_private_key(&tmp_ctx, opt.filename);
//    if (ret != 0) {
//        mbedtls_printf(" failed\n");
//        goto exit;
//    }

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


int qcry_generate_key() {
    int ret = 0;
    mbedtls_pk_context key;
    char buf[1024];
    int i;
    char *p, *q;
    const char *pers = "gen_key";
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    /** Setup core state for key generation */
    mbedtls_pk_init(&key);
    mbedtls_ctr_drbg_init(&ctr_drbg);
    memset(buf, 0, sizeof(buf));

    /** Set key generation values */
    opt.type = DFL_TYPE;
    opt.rsa_keysize = DFL_RSA_KEYSIZE;
    opt.ec_curve = DFL_EC_CURVE;
    opt.filename = "/home/spacekookie/.qaul/00_spacekookie.key";
    opt.use_dev_random = DFL_USE_DEV_RANDOM;

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
    ret = mbedtls_pk_setup(&key, mbedtls_pk_info_from_type(opt.type));
    ret = mbedtls_rsa_gen_key(mbedtls_pk_rsa(key), mbedtls_ctr_drbg_random, &ctr_drbg, opt.rsa_keysize, 65537);

    if (ret != 0) {
        printf(" failed\nmbedtls_pk_setup returned -0x%04x", -ret);
        goto exit;
    }

    mbedtls_printf(" ok\n==> Key information:\n");
    /*********************************************/

    /** Encode the key and save it somewhere */
    mbedtls_rsa_context *rsa = mbedtls_pk_rsa(key);
    mbedtls_mpi_write_file("N:  ", &rsa->N, 16, NULL);
    mbedtls_mpi_write_file("E:  ", &rsa->E, 16, NULL);
    mbedtls_mpi_write_file("D:  ", &rsa->D, 16, NULL);
    mbedtls_mpi_write_file("P:  ", &rsa->P, 16, NULL);
    mbedtls_mpi_write_file("Q:  ", &rsa->Q, 16, NULL);
    mbedtls_mpi_write_file("DP: ", &rsa->DP, 16, NULL);
    mbedtls_mpi_write_file("DQ:  ", &rsa->DQ, 16, NULL);
    mbedtls_mpi_write_file("QP:  ", &rsa->QP, 16, NULL);

    ret = write_private_key(&key, opt.filename);
    if (ret != 0) {
        mbedtls_printf(" failed\n");
        goto exit;
    }

    /*********************************************/

    return 0;

    exit:
    printf("ERROR!\n");
    mbedtls_ctr_drbg_free(&ctr_drbg);
}

int qcry_sign_with_key() {

}
