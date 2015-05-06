/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_DEFINES
#define _QAULLIB_DEFINES

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#ifdef WIN32
#else
#include <limits.h>  // for PATH_MAX
#endif

#define TIMEOUT_LASTRECEIVED 20
#define TIMEOUT_CONNECTED   600

#define MAX_SUFFIX_LEN        4
#define MAX_DESCRIPTION_LEN  80
#define MAX_HASH_LEN         20
#define MAX_HASHSTR_LEN      40
#define MAX_INTSTR_LEN       10
#define MAX_PAGE_LEN         20
#define MAX_LOCALE_LEN        6
#define MAX_VARCHAR_LEN     255
#define MAX_URL_LEN         512
#define MAX_TIME_LEN         20
#define MAX_SSID_LEN         32
#define MAX_BSSID_LEN        17
#define QAUL_MAX_PROFILE_LEN      30

#define FLAG_EXE_WIN7      0x01
#define FLAG_EXE_OSX105    0x02
#define FLAG_EXE_OSX106    0x04
#define FLAG_EXE_UBUNTU    0x08
#define FLAG_EXE_ANDROID   0x16
#define FLAG_EXE_IOS       0x32

#ifdef WIN32
#define PATH_SEPARATOR     "\\"
#define PATH_SEPARATOR_INT '\\'
#define MAX_PATH_LEN   MAX_PATH
#else
#define PATH_SEPARATOR      "/"
#define PATH_SEPARATOR_INT  '/'
#define MAX_PATH_LEN   PATH_MAX
#endif

#define MAX_USER_LEN           20
#define MAX_MESSAGE_LEN       140
#define MAX_FILENAME_LEN       46
#define MAX_IP_LEN             40
#define IPC_PORT             8112
#define BUFFSIZE             8192
#define IPC_ADDR      "127.0.0.1"
#define CHAT_PORT          "8081"
#define WEB_PORT             8081
#define VOIP_PORT            8060
#define UDP_PORT             8083
#define MAX_USER_CONNECTIONS    3
#define MAX_FILE_CONNECTIONS    5
#define MAX_TIMESTR_SIZE       80
#define MAX_HEADER_LEN       1024
#define MAX_JSON_LEN         1024

#define QAUL_WEB_THREADS        5

/**
 * qaul file sharing constants
 */
#define QAUL_FILEDISCOVERY_TIMEOUT 60

#define QAUL_FILETYPE_FILE           1
#define QAUL_FILETYPE_PROFILEIMAGE   2
#define QAUL_FILETYPE_EXECUTABLE     4

#define QAUL_FILESTATUS_COPYING     -5
#define QAUL_FILESTATUS_DELETED     -2
#define QAUL_FILESTATUS_ERROR       -1
#define QAUL_FILESTATUS_NEW          0
#define QAUL_FILESTATUS_DISCOVERING  1
#define QAUL_FILESTATUS_DISCOVERED   2
#define QAUL_FILESTATUS_DOWNLOADING  3
#define QAUL_FILESTATUS_DOWNLOADED   4
#define QAUL_FILESTATUS_MYFILE       5

/**
 * qaul chat msg constants
 */
#define MAX_MSG_COUNT               40
#define MAX_MSG_FIRST               10
#define MAX_MSG_COUNT_WEB           10
#define MAX_MSG_FIRST_WEB            5

#define QAUL_MSGTYPE_PUBLIC_IN       1
#define QAUL_MSGTYPE_PUBLIC_OUT     11
#define QAUL_MSGTYPE_PRIVATE_IN      2
#define QAUL_MSGTYPE_PRIVATE_OUT    12
#define QAUL_MSGTYPE_VOIP_IN         3
#define QAUL_MSGTYPE_VOIP_OUT       13


struct qaul_userinfo_msg
{
	union olsr_ip_addr ip;
	char               name[MAX_USER_LEN];
	char               icon[MAX_HASH_LEN];
	char               suffix[MAX_SUFFIX_LEN];
};

struct qaul_fileavailable_msg
{
	uint16_t  msgtype;
	char      hash[MAX_HASH_LEN];
	char      suffix[MAX_SUFFIX_LEN];
	uint32_t  filesize;
};

// todo: cluster exe available messages
struct qaul_exeavailable_msg
{
	uint16_t msgtype;
	uint32_t OS_flag;
	char     hash[MAX_HASH_LEN];
	char     suffix[MAX_SUFFIX_LEN];
	uint32_t filesize;
};


/********************************************//**
 * qaul olsr Messages
 *
 * message structures for the interprocess communication
 ***********************************************/

/**
 * qaul message types
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


struct qaul_chat_msg
{
  char name[MAX_USER_LEN];
  char msg[MAX_MESSAGE_LEN];
};

/**
 * IPC message
 *
 * 0: quit olsrd
 */
union qaul_ipc
{
	//char string[250];
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
	uint32_t lq;
};

struct qaul_userhello_msg
{
	char name[MAX_USER_LEN];
	char icon[MAX_HASH_LEN];
	char suffix[MAX_SUFFIX_LEN];
};

struct qaul_filechunk_msg
{
	uint32_t type;  // type 0 => error: file does not exists
					// type 1 => chunk
					// type 3 => error: requested chunk position is bigger than file
	uint32_t filesize;
	uint32_t chunksize;
	char     chunkhash[MAX_HASH_LEN];
	//char chunk;
};

struct qaul_filediscover_msg
{
	char hash[MAX_HASH_LEN];
};

struct qaul_exediscover_msg
{
	uint32_t OS_flag;
};

/**
 * olsrd messages
 */

/**
 * Hello info
 */
struct hellinfo {
  olsr_u8_t link_code;
  olsr_u8_t reserved;
  olsr_u16_t size;
  olsr_u32_t neigh_addr[1];            /* neighbor IP address(es) */
};

/**
 * The HELLO message
 */
struct hellomsg {
  olsr_u16_t reserved;
  olsr_u8_t htime;
  olsr_u8_t willingness;
  struct hellinfo hell_info[1];
};

/*
 * IPv6
 */
struct hellinfo6 {
  olsr_u8_t link_code;
  olsr_u8_t reserved;
  olsr_u16_t size;
  struct in6_addr neigh_addr[1];       /* neighbor IP address(es) */
};

struct hellomsg6 {
  olsr_u16_t reserved;
  olsr_u8_t htime;
  olsr_u8_t willingness;
  struct hellinfo6 hell_info[1];
};

/**
 * Topology Control packet
 */
struct neigh_info {
  olsr_u32_t addr;
};

struct olsr_tcmsg {
  olsr_u16_t ansn;
  olsr_u16_t reserved;
  struct neigh_info neigh[1];
};

/*
 * IPv6
 */
struct neigh_info6 {
  struct in6_addr addr;
};

struct olsr_tcmsg6 {
  olsr_u16_t ansn;
  olsr_u16_t reserved;
  struct neigh_info6 neigh[1];
};

/**
 * OLSR message (several can exist in one OLSR packet)
 */

union olsr_msg_union {
    struct hellomsg hello;
    struct olsr_tcmsg tc;
    struct hnamsg hna;
    struct midmsg mid;

    // my messages
    struct qaul_chat_msg         chat;
    struct qaul_ipc_msg          ipc;
    struct qaul_node_msg         node;
    struct qaul_userhello_msg    userhello;
    struct qaul_filediscover_msg filediscover;
    struct qaul_exediscover_msg  exediscover;
};

struct olsrmsg {
	olsr_u8_t olsr_msgtype;
	olsr_u8_t olsr_vtime;
	olsr_u16_t olsr_msgsize;
	olsr_u32_t originator;
	olsr_u8_t ttl;
	olsr_u8_t hopcnt;
	olsr_u16_t seqno;

	union olsr_msg_union message;
};

/*
 *IPv6
 */

struct olsrmsg6 {
  olsr_u8_t olsr_msgtype;
  olsr_u8_t olsr_vtime;
  olsr_u16_t olsr_msgsize;
  struct in6_addr originator;
  olsr_u8_t ttl;
  olsr_u8_t hopcnt;
  olsr_u16_t seqno;

  union olsr_msg_union message;
};

union qaul_inbuf
{
	char                          buf[BUFFSIZE + 1];
	struct qaul_userinfo_msg      userinfo;
	struct qaul_filechunk_msg     filechunk;
	struct qaul_userhello_msg     userhello;
	struct qaul_filediscover_msg  filediscover;
	struct qaul_exediscover_msg   exediscover;
};

/*
 * Generic OLSR packet
 */

struct olsr {
  olsr_u16_t olsr_packlen;             /* packet length */
  olsr_u16_t olsr_seqno;
  struct olsrmsg olsr_msg[1];          /* variable messages */
};

struct olsr6 {
  olsr_u16_t olsr_packlen;             /* packet length */
  olsr_u16_t olsr_seqno;
  struct olsrmsg6 olsr_msg[1];         /* variable messages */
};

/* IPv4 <-> IPv6 compability */

union olsr_message {
  struct olsrmsg v4;
  struct olsrmsg6 v6;
};

union olsr_packet {
  struct olsr v4;
  struct olsr6 v6;
};


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
