#include "qcry_helper.h"

#include <stdlib.h>
#include <memory.h>
#include <stdbool.h>
#include <mbedtls/platform.h>


/** Static const ASCII character lookup table */
static const unsigned char pr2six[256] =
        {
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 62, 64, 64, 64, 63,
                52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 64, 64, 64, 64, 64, 64,
                64,  0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11, 12, 13, 14,
                15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 64, 64, 64, 64, 64,
                64, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40,
                41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64,
                64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64, 64
        };

/* Static const Base64 encoding lookup table */
static const char base64[] =
        "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

int qcry_base64_enclength(int str_len)
{
    return ((str_len + 2) / 3 * 4) + 1;
}

int qcry_base64_encode(char *buffer, const char *src, int src_len)
{
    long nbytesdecoded;
    register const unsigned char *bufin;
    register unsigned char *bufout;
    long nprbytes;

    bufin = (const unsigned char *) src;
    while (pr2six[*(bufin++)] <= 63);
    nprbytes = (bufin - (const unsigned char *) src) - 1;
    nbytesdecoded = ((nprbytes + 3) / 4) * 3;

    bufout = (unsigned char *) buffer;
    bufin = (const unsigned char *) src;

    while (nprbytes > 4) {
        *(bufout++) =
                (unsigned char) (pr2six[*bufin] << 2 | pr2six[bufin[1]] >> 4);
        *(bufout++) =
                (unsigned char) (pr2six[bufin[1]] << 4 | pr2six[bufin[2]] >> 2);
        *(bufout++) =
                (unsigned char) (pr2six[bufin[2]] << 6 | pr2six[bufin[3]]);
        bufin += 4;
        nprbytes -= 4;
    }

    /* Note: (nprbytes == 1) would be an error, so just ingore that case */
    if (nprbytes > 1) {
        *(bufout++) =
                (unsigned char) (pr2six[*bufin] << 2 | pr2six[bufin[1]] >> 4);
    }
    if (nprbytes > 2) {
        *(bufout++) =
                (unsigned char) (pr2six[bufin[1]] << 4 | pr2six[bufin[2]] >> 2);
    }
    if (nprbytes > 3) {
        *(bufout++) =
                (unsigned char) (pr2six[bufin[2]] << 6 | pr2six[bufin[3]]);
    }

    *(bufout++) = '\0';
    nbytesdecoded -= (4 - nprbytes) & 3;
    return nbytesdecoded;
}

int qcry_base64_declen(const char *encoded)
{
    int nbytesdecoded;
    register const unsigned char *bufin;
    register int nprbytes;

    bufin = (const unsigned char *) encoded;
    while (pr2six[*(bufin++)] <= 63);

    nprbytes = (bufin - (const unsigned char *) encoded) - 1;
    nbytesdecoded = ((nprbytes + 3) / 4) * 3;

    return nbytesdecoded + 1;
}

int qcry_base64_decode(char *encoded, const char *string, int enc_len)
{
    int i;
    char *p;

    p = encoded;
    for (i = 0; i < enc_len - 2; i += 3) {
        *p++ = base64[(string[i] >> 2) & 0x3F];
        *p++ = base64[((string[i] & 0x3) << 4) |
                        ((int) (string[i + 1] & 0xF0) >> 4)];
        *p++ = base64[((string[i + 1] & 0xF) << 2) |
                        ((int) (string[i + 2] & 0xC0) >> 6)];
        *p++ = base64[string[i + 2] & 0x3F];
    }
    if (i < enc_len) {
        *p++ = base64[(string[i] >> 2) & 0x3F];
        if (i == (enc_len - 1)) {
            *p++ = base64[((string[i] & 0x3) << 4)];
            *p++ = '=';
        }
        else {
            *p++ = base64[((string[i] & 0x3) << 4) |
                            ((int) (string[i + 1] & 0xF0) >> 4)];
            *p++ = base64[((string[i + 1] & 0xF) << 2)];
        }
        *p++ = '=';
    }

    *p++ = '\0';
    return p - encoded;
}

int qcry_save_pubkey(mbedtls_pk_context *pub, const char *path, const char *fp)
{
    int ret;
    FILE *f;
    size_t len = 0;
    size_t buf_s = 16000; // FIXME: This has _got_ to be better
    unsigned char output_buf[buf_s];
    unsigned char *c = output_buf;

    /** Create path buffer depending on input length + "slashyness" */
    size_t ps = strlen(path) + strlen(fp) + strlen(".pub");
    bool slashed =  strcmp(&path[strlen(path) - 1], "/") != 0;
    if(slashed) ps += 1;
    char ppath[ps];

    /* Either copy the path with or without an extra slash into the buffer */
    if(slashed) mbedtls_snprintf(ppath, sizeof(ppath), "%s/%s.pub", path, fp);
    else        mbedtls_snprintf(ppath, sizeof(ppath), "%s%s.pub", path, fp);

    /**************** Write public key to file ****************/

    /** Clear Buffer and write into it  */
    memset(output_buf, 0, buf_s);
    ret = mbedtls_pk_write_pubkey_pem(pub, output_buf, buf_s);
    if(ret != 0) return ret;

    /** Write buffer to file handle */
    if((f = fopen(ppath, "w")) == NULL) {
        printf("FAILED\n");
        return -1;
    }

    if(fwrite(c, 1, len, f) != len) {
        printf("FAILED\n");
        fclose(f);
        return -1;
    }

    fclose(f);

    return QCRY_STATUS_OK;
}


int qcry_load_pubkey(mbedtls_pk_context **pub, const char *path, const char *fp)
{
    int ret = 0;

    /*** Malloc space for the pub and pri key values on heap ***/
    (*pub) = (mbedtls_pk_context*) calloc(sizeof(mbedtls_pk_context),  1);
    if(*pub == NULL) return EXIT_FAILURE;

    /** Create path buffer depending on input length + "slashyness" */
    size_t ps = strlen(path) + strlen(fp) + strlen(".pub");
    bool slashed =  strcmp(&path[strlen(path) - 1], "/") != 0;
    if(slashed) ps += 1;
    char ppath[ps];

    /* Either copy the path with or without an extra slash into the buffer */
    if(slashed) mbedtls_snprintf(ppath, sizeof(ppath), "%s/%s.pub", path, fp);
    else        mbedtls_snprintf(ppath, sizeof(ppath), "%s%s.pub", path, fp);

    /*** Read keys off disk and initialise the contexts ***/
    mbedtls_printf("[KEYSTORE] Parsing key for %s...", fp);
    fflush(stdout);

    ret = mbedtls_pk_parse_public_keyfile(*pub, ppath);
    if(ret != 0) {
        mbedtls_printf("FAILED! mbedtls_pk_parse_public_keyfile returned 0x%04x\n", ret);
        goto cleanup;
    }

    mbedtls_printf("OK\n");

    return QCRY_STATUS_OK;

    cleanup:
    mbedtls_pk_free(*pub);
    return ret;
}


int qcry_serialise_pubkey(unsigned char **buffer, size_t *ksize, mbedtls_pk_context *key)
{

    return QCRY_STATUS_OK;
}


int qcry_deserialise_pubkey(mbedtls_pk_context **key, size_t ksize, unsigned char *buffer)
{

    return QCRY_STATUS_OK;
}
