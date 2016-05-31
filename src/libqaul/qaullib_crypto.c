/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include "qaullib_crypto.h"
#include "mbedtls/sha1.h"
#include "qaullib/logging.h"


// ------------------------------------------------------------
int Ql_sha1_file(char *filepath, unsigned char *hash)
{
    int ret;
    FILE *f;
    size_t n;
	mbedtls_sha1_context ctx;
    unsigned char buf[1024];

    Ql_log_debug("Ql_sha1_file %s", filepath);

    ret = 1;
    if((f = fopen(filepath, "rb")) == NULL)
    {
    	Ql_log_error("Ql_sha1_file failed to open file: %s\n", filepath);
    	return 0;
    }

	mbedtls_sha1_init(&ctx);
	mbedtls_sha1_starts(&ctx);

    while((n = fread(buf, 1, sizeof(buf), f)) > 0)
    {
    	mbedtls_sha1_update(&ctx, buf, n);
    }

    if(ferror(f) != 0)
    {
    	Ql_log_error("Ql_sha1_file file error on file: %s\n", filepath);
    	ret = 0;
        goto cleanup;
    }

	mbedtls_sha1_finish(&ctx, hash);

cleanup:
    fclose(f);
    mbedtls_sha1_free(&ctx);

	return ret;
}

// ------------------------------------------------------------
int Ql_HashToString(unsigned char *hash, char *string)
{
	int i;

	Ql_log_debug("Ql_HashToString");

	// FIXME: big-endian / little-endian
	for(i=0;i<MAX_HASH_LEN;i++)
	{
		sprintf(string+(i*2),"%02x",hash[i]);
	}
	return 1;
}

// ------------------------------------------------------------
int Ql_StringToHash(char *string, unsigned char *hash)
{
	int i, j;
	uint8_t mybyte;

	// fill hash with zeros
	memset(hash, 0, MAX_HASH_LEN);

	for(i=0;i<MAX_HASH_LEN;i++)
	{
		mybyte = 0;

		for(j=0;j<2;j++)
		{
			if(strncmp(string+i*2+j,"0",1)==0)
				mybyte |= 0;
			else if(strncmp(string+i*2+j,"1",1)==0)
				mybyte |= 1;
			else if(strncmp(string+i*2+j,"2",1)==0)
				mybyte |= 2;
			else if(strncmp(string+i*2+j,"3",1)==0)
				mybyte |= 3;
			else if(strncmp(string+i*2+j,"4",1)==0)
				mybyte |= 4;
			else if(strncmp(string+i*2+j,"5",1)==0)
				mybyte |= 5;
			else if(strncmp(string+i*2+j,"6",1)==0)
				mybyte |= 6;
			else if(strncmp(string+i*2+j,"7",1)==0)
				mybyte |= 7;
			else if(strncmp(string+i*2+j,"8",1)==0)
				mybyte |= 8;
			else if(strncmp(string+i*2+j,"9",1)==0)
				mybyte |= 9;
			else if(strncmp(string+i*2+j,"a",1)==0 || strncmp(string+i*2+j,"A",1)==0)
				mybyte |= 10;
			else if(strncmp(string+i*2+j,"b",1)==0 || strncmp(string+i*2+j,"B",1)==0)
				mybyte |= 11;
			else if(strncmp(string+i*2+j,"c",1)==0 || strncmp(string+i*2+j,"C",1)==0)
				mybyte |= 12;
			else if(strncmp(string+i*2+j,"d",1)==0 || strncmp(string+i*2+j,"D",1)==0)
				mybyte |= 13;
			else if(strncmp(string+i*2+j,"e",1)==0 || strncmp(string+i*2+j,"E",1)==0)
				mybyte |= 14;
			else if(strncmp(string+i*2+j,"f",1)==0 || strncmp(string+i*2+j,"F",1)==0)
				mybyte |= 15;
			else
				return 0;

			if(j==0)
				mybyte = mybyte << 4;
			else
				memcpy(hash+i, &mybyte, 1);
		}
	}
	return 1;
}
