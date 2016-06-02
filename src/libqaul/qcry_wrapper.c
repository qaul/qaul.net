#include <qaullib/qcry_wrapper.h>
#include "crypto/qcry_context.h"

#include <stdio.h>
#include <string.h>

// Include public key crypto headers
#include "mbedtls/pk.h"
#include "mbedtls/entropy.h"
#include "mbedtls/ctr_drbg.h"

int qcry_devel_init()
{
    printf("Wrapper INIT\n");

    printf("This example will do public key cryptography with RSA...\n");

    char *to_encrypt = "This is some super super secrit data: PANCAKES!";
    size_t to_encrypt_len = strlen(to_encrypt);

    /******************************************
     * INITIALISE THE CORE CRYPTO FUNCTIONS
     *
     *****************************************/
    int ret = 0;
    mbedtls_pk_context pk;

    mbedtls_pk_init(&pk);

    if( ( ret = mbedtls_pk_parse_public_keyfile( &pk, "our-key.pub" ) ) != 0 )
    {
        printf( " failed\n  ! mbedtls_pk_parse_public_keyfile returned -0x%04x\n", -ret );
        goto exit;
    }

    /******************************************
     * INITIALISE THE ENTROPY AND RANDOM NUMBER GENERATOR
     *****************************************/
    mbedtls_entropy_context entropy;
    mbedtls_entropy_init(&entropy);

    /** Fields for random number generator */
    mbedtls_ctr_drbg_context ctr_drbg;
    char *personalization = "my_app_specific_string";
    size_t per_s = strlen(personalization);

    /** Initialise the RNG */
    mbedtls_ctr_drbg_init(&ctr_drbg);
    mbedtls_ctr_drbg_set_prediction_resistance(&ctr_drbg, MBEDTLS_CTR_DRBG_PR_ON);

    unsigned char buf[MBEDTLS_MPI_MAX_SIZE];
    size_t olen = 0;

    /*
     * Calculate the RSA encryption of the data.
     */
    printf( "\n  . Generating the encrypted value" );
    fflush( stdout );

    ret = mbedtls_pk_encrypt(&pk, to_encrypt, to_encrypt_len, buf, &olen, sizeof(buf), mbedtls_ctr_drbg_random, &ctr_drbg);
    if( ret != 0 )
    {
        printf( " failed\n  ! mbedtls_pk_encrypt returned -0x%04x\n", -ret );
        goto exit;
    }

    printf("Successfully encrypted data!!\n");
    return 0;

    exit:
    printf("Exiting because of an error!\n");
    return 255;

    }
