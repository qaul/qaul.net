/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_QAUL_MESSAGES
#define _OLSRD_QAUL_MESSAGES

/**
 * buffer length definition
 */
#define MAX_USER_LEN      20
#define MAX_MESSAGE_LEN  140
#define MAX_IP_LEN        40
#define MAX_FILE_LEN      33
#define MAX_FILENAME_LEN  46
#define MAX_HASH_LEN      20
#define MAX_HASHSTR_LEN   40
#define MAX_SUFFIX_LEN     4

/**
 * message definitions
 */
#define QAUL_CHAT_MESSAGE_TYPE 	        222
#define QAUL_CHAT_PARSER_TYPE	        QAUL_CHAT_MESSAGE_TYPE
#define QAUL_IPCCOM_MESSAGE_TYPE        223
#define QAUL_IPCTOPO_MESSAGE_TYPE       224
#define QAUL_USERHELLO_MESSAGE_TYPE     225
#define QAUL_USERHELLO_PARSER_TYPE	    QAUL_USERHELLO_MESSAGE_TYPE
#define QAUL_FILEDISCOVER_MESSAGE_TYPE  226
#define QAUL_FILEDISCOVER_PARSER_TYPE   QAUL_FILEDISCOVER_MESSAGE_TYPE
#define QAUL_FILEAVAILABLE_MESSAGE_TYPE 227
#define QAUL_FILEAVAILABLE_PARSER_TYPE  QAUL_FILEAVAILABLE_MESSAGE_TYPE
#define QAUL_EXEDISCOVER_MESSAGE_TYPE   228
#define QAUL_EXEDISCOVER_PARSER_TYPE    QAUL_EXEDISCOVER_MESSAGE_TYPE
#define QAUL_EXEAVAILABLE_MESSAGE_TYPE  229
#define QAUL_EXEAVAILABLE_PARSER_TYPE   QAUL_EXEAVAILABLE_MESSAGE_TYPE
#define QAUL_IPCMESHTOPO_MESSAGE_TYPE   230

/**
 * IPC messages
 */
#define QAUL_IPCCOM_QUIT                0
#define QAUL_IPCCOM_GETTOPO             1
#define QAUL_IPCCOM_GETMESHTOPO         2
#define QAUL_IPCCOM_MESHTOPO_SENT       3

// message to send
struct qaul_chat_msg
{
	char name[MAX_USER_LEN];
	char msg[MAX_MESSAGE_LEN];
};

struct qaul_userhello_msg
{
	char name[MAX_USER_LEN];
	char icon[MAX_HASH_LEN];
	char suffix[MAX_SUFFIX_LEN];
};

struct qaul_filediscover_msg
{
	char hash[MAX_HASH_LEN];
};

struct qaul_fileavailable_msg
{
	char     hash[MAX_HASH_LEN];
	char     suffix[MAX_SUFFIX_LEN];
	uint32_t filesize;
};

struct qaul_exediscover_msg
{
	uint32_t platform;
};

// todo: cluster exeavailable messages
struct qaul_exeavailable_msg
{
	uint32_t platform;
	char     hash[MAX_HASH_LEN];
	char     suffix[MAX_SUFFIX_LEN];
	uint32_t filesize;
};

union qaul_ipc
{
	int integer;
};

struct qaul_ipc_msg
{
	int type;
	union qaul_ipc msg;
};

struct qaul_node_msg
{
	union olsr_ip_addr ip;
	union olsr_ip_addr gateway;
	float           lq;
};

/*
 * todo: bundle the topo messages together
struct qaul_topomsg
{
	int count;
	struct qaul_topo topo[];
};
*/
#endif
