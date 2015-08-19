/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_EXEDISCOVERY
#define _QAULLIB_EXEDISCOVERY

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


struct qaul_exe_struct
{
	uint32_t  OS_flag;
	int  size;
    char hashstr[MAX_HASHSTR_LEN +1];
    char hash[MAX_HASH_LEN];
    char suffix[MAX_SUFFIX_LEN +1];
    char description[MAX_DESCRIPTION_LEN +1];
    int  max_size;
    int  discovered;
    time_t discovery_timestamp;
};

struct qaul_exe_struct qaul_exe_array[MAX_POPULATE_FILE];

/**
 * initialize exe discovery
 */
void Qaullib_ExeInit(void);

/**
 * check if a discovery message needs to be sent
 */
void Qaullib_ExeScheduleDiscovery(void);

/**
 * process an incoming exe available message
 */
void Qaullib_ExeProcessAvailableMsg(struct qaul_exeavailable_msg *msg);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
