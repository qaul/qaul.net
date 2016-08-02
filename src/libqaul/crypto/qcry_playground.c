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
 * A function that provides better entropy for key generation if it's available
 * for a specific platform i.e. Linux
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

int qcry_key_destroy(mbedtls_pk_context *key)
{
    mbedtls_pk_free(key);
    free(key);
    return 0;
}
/**********************************************************************************************/
/**********************************************************************************************/
/**********************************************************************************************/


int qcry_key_write(mbedtls_pk_context *key, const char *path, const char *username)
{
    size_t p_s = strlen(path);
    size_t u_s = strlen(username);

    /* Create an array and make sure it's REALLY empty */
    char write_path[p_s + u_s + 4];
    memset(write_path, 0, p_s + u_s + 4);

    /* Copy the path into it and add a slash between folder and user if required */
    strcpy(write_path, path);
    if(strcmp(&path[p_s - 1], "/") != 0) strcat(write_path, "/");
    strcat(write_path, "00_"); // TODO: Get number of keys from somewhere?
    strcat(write_path, username);

    /***************** PREPARE KEY WRITE *****************/

    int ret;
    FILE *f;
    size_t buf_s = 16000;
    unsigned char output_buf[buf_s];
    unsigned char *c = output_buf;
    char new_filename[strlen(write_path) + strlen(".ext")];

    size_t len = 0;

    /***************** WRITE PUBLIC KEY *****************/

    /** Clear Buffer and write into it  */
    memset(output_buf, 0, buf_s);
    ret = mbedtls_pk_write_pubkey_pem(key, output_buf, 16000);
    if(ret != 0) return(ret);

    /** Get the exact length of what we're supposed to write */
    len = strlen((char *) output_buf);

    memset(new_filename, 0, sizeof(new_filename));
    strcpy(new_filename, write_path);
    strcat(new_filename, ".pub");

    if((f = fopen(new_filename, "w")) == NULL)
        return -1;

    if(fwrite(c, 1, len, f) != len)
    {
        fclose(f);
        return -1;
    }

    fclose(f);

    /***************** WRITE PRIVATE KEY *****************/

    /** Clear Buffer and write into it  */
    memset(output_buf, 0, buf_s);
    ret = mbedtls_pk_write_key_pem(key, output_buf, buf_s);
    if(ret != 0) return(ret);

    len = strlen((char *) output_buf);

    memset(new_filename, 0, sizeof(new_filename));
    strcpy(new_filename, write_path);
    strcat(new_filename, ".key");

    if ((f = fopen(new_filename, "wb")) == NULL)
        return (-1);

    if (fwrite(c, 1, len, f) != len) {
        fclose(f);
        return (-1);
    }

    fclose(f);

    /***************** RETURN SUCCESS :) *****************/

    return 0;
}

int qcry_key_generate(mbedtls_pk_context **pri, const char *pers)
{
    int ret = 0;

    /* Temp buffers */
    mbedtls_pk_context tmp_pri;
    mbedtls_entropy_context entropy;
    mbedtls_ctr_drbg_context ctr_drbg;

    /** Setup core state for key generation */
    mbedtls_pk_init(&tmp_pri);
    mbedtls_ctr_drbg_init(&ctr_drbg);

    /* Set some state variables */
    int type = DFL_TYPE;
    int rsa_keysize = DFL_RSA_KEYSIZE;

    /*********************************************/

    /** Prepare key generation procedure */
    printf("Seeding random number generators...\n");
    mbedtls_entropy_init(&entropy);
    ret = mbedtls_ctr_drbg_seed(&ctr_drbg, mbedtls_entropy_func, &entropy, (const unsigned char *) pers, strlen(pers));

    if (ret != 0) {
        printf(" failed!\n\tmbedtls_ctr_drbg_seed returned %d\n", ret);
        goto exit;
    }

    /*********************************************/

    printf("Generating private key...");
    ret = mbedtls_pk_setup(&tmp_pri, mbedtls_pk_info_from_type(type));
    ret = mbedtls_rsa_gen_key(mbedtls_pk_rsa(tmp_pri), mbedtls_ctr_drbg_random, &ctr_drbg, rsa_keysize, 65537);

    if (ret != 0) {
        printf(" failed!\nmbedtls_ctr_drbg_seed returned %d\n", ret);
        goto exit;
    }

    /** Malloc the apropriate space we need and memcpy */
    (*pri) = (mbedtls_pk_context *) malloc(sizeof(mbedtls_pk_context));
    memcpy(*pri, &tmp_pri, sizeof(mbedtls_pk_context));

    /*********************************************/

    exit:
    mbedtls_ctr_drbg_free(&ctr_drbg);
    return ret;
}

/**
 * NEW AND IMPROVED! Now actually works! :)
 *
 * @param pub Pointer to reference future public key
 * @param pri Pointer to reference future private key
 * @param path Path to the keystore (usually ~/.qaul/keys/)
 * @param username Username the keys belong to
 *
 * @return Status code for errors
 */
int qcry_key_load(mbedtls_pk_context **pub, mbedtls_pk_context **pri, const char *path, const char *username)
{
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    (*pub) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context),  1);
    if(*pub == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    (*pri) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context), 1);
    if(*pri == NULL) {
        ret = EXIT_FAILURE;
        goto cleanup;
    }

    /*** Initialise the key contexts properly ***/
    mbedtls_pk_init(*pub);
    mbedtls_pk_init(*pri);

    /*** Construct the required file names ***/
    char pri_pth[512];
    char pub_pth[512];

    size_t p_s = strlen(path);
    size_t u_s = strlen(username);

    /** Build private key path **/
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s/00_%s.key", path, username);
    else                                    mbedtls_snprintf(pri_pth, sizeof(pri_pth), "%s00_%s.key", path, username);

    /* Build public key path */
    if(strcmp(&path[p_s - 1], "/") != 0)    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s/00_%s.pub", path, username);
    else                                    mbedtls_snprintf(pub_pth, sizeof(pub_pth), "%s00_%s.pub", path, username);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("Parsing public key...");
    fflush(stdout);

    ret = mbedtls_pk_parse_public_keyfile(*pub, pub_pth);
    if(ret != 0) {
        mbedtls_printf("FAILED! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret);
        goto cleanup;
    }
    mbedtls_printf("OK\n");

    mbedtls_printf("Parsing private key...");
    fflush(stdout);

    ret = mbedtls_pk_parse_keyfile(*pri, pri_pth, "" );
    if(ret != 0) {
        mbedtls_printf("\"FAILED! mbedtls_pk_parse_keyfile returned -0x%04x\n", -ret);
        goto cleanup;
    }
    mbedtls_printf("OK\n");

    printf("== Keys loaded successfully ==\n\n");
    return 0;

    cleanup:
    mbedtls_pk_free(*pub);
    mbedtls_pk_free(*pri);
    return ret;
}

int sign_msg(mbedtls_pk_context *pri, const char *msgfile)
{
    FILE *f;
    int ret = 1;

    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];

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
    mbedtls_snprintf(sig_name, sizeof(sig_name), "%s.sig", msgfile);

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

int verify_msg(mbedtls_pk_context *pub, const char *msgfile)
{
    /*** Prapre are few variables we might need ***/

    int ret = 0;
    FILE *f;
    size_t i;

    unsigned char hash[32];
    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];

    char filename[512];
    mbedtls_snprintf(filename, sizeof(filename), "%s.sig", msgfile);

    /*** Read the signature file in from disk ***/
    mbedtls_printf("Reading signature file...");
    fflush(stdout);

    if((f = fopen( filename, "rb")) == NULL)
    {
        ret = 1;
        mbedtls_printf("FAILED!\n\tCould not open %s\n\n", filename);
        goto exit;
    }
    mbedtls_printf("OK\n");

    /* Cleanup first buffer */
    i = fread(buf, 1, sizeof(buf), f);
    fclose(f);

    /*** Compute SHA-256 Digest of our message ***/
    mbedtls_printf("Computing SHA-256 Digest...");
    fflush(stdout);

    ret = mbedtls_md_file(mbedtls_md_info_from_type(MBEDTLS_MD_SHA256), msgfile, hash);
    if(ret != 0) {
        mbedtls_printf("FAILED!\n\tCould not open or read %s\n", msgfile);
        goto exit;
    }
    mbedtls_printf("OK\n");

    /*** Verify signature and JIT hash ***/
    mbedtls_printf("Verifying message integrety...");
    fflush(stdout);

    ret = mbedtls_pk_verify(pub, MBEDTLS_MD_SHA256, hash, 0, buf, i);
    if(ret != 0) {
        mbedtls_printf("FAILED!\n\tTHIS MESSAGE HAS BEEN TEMPERED WITH. DO NOT TRUST!\n", -ret);
        goto exit;
    }
    mbedtls_printf("OK\n");

    ret = 0;
    printf("== The message signature is valid ==\n\n");

    exit:
    return ret;
}


int qcry_sign_with_key() {

}
