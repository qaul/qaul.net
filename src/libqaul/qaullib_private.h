/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_PRIVATE
#define _QAULLIB_PRIVATE

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


#include <stdlib.h>
#include <assert.h>
#include <stdio.h>
#include <string.h>
#include <errno.h>
#include <fcntl.h>
#include <time.h>
#include <stdint.h>

#ifdef WIN32
#include <winsock2.h>
#define SHUT_RDWR 2
#else
#include <sys/select.h> // fd_set (Android-NDK)
#include <netinet/in.h> // IPv6 address format in6_addr

// OpenBSD:
#include <sys/types.h>
#include <sys/socket.h>
#include <arpa/inet.h>
#endif

#include "qaullib.h"
#include "olsrd_types.h"
#include "qaullib_defines.h"
#include "qaullib_validate.h"
#include "qaullib_webclient.h"
#include "qaullib_topo_LL.h"
#include "qaullib_appevent_LL.h"
#include "qaullib_user_LL.h"
#include "qaullib_user.h"
#include "qaullib_file_LL.h"
#include "qaullib_filesharing.h"
#include "qaullib_msg_LL.h"
#include "qaullib_messaging.h"
#include "qaullib_threads.h"
#include "olsrd/mantissa.h"
#include "mongoose/mongoose.h"
#include "qaullib_webserver.h"
#include "sqlite/sqlite3.h"
#include "qaullib_sql.h"
#include "qaullib_exediscovery.h"
#include "qaullib_ipc.h"
#include "qaullib_voip.h"
#include "qaullib_udp_communication.h"

#ifndef WIN32
#include <unistd.h>             // close()
#endif


/**
 * for debugging messages switch this to 1
 */
#define QAUL_DEBUG              1

/**
 * global variables and structures
 */
struct mg_server *qaul_webserver_instance;
sqlite3 *db;
char dbPath[MAX_PATH_LEN +1];
char webPath[MAX_PATH_LEN +1];
char qaullib_AppEventOpenPath[MAX_PATH_LEN +1];
int qaul_gui_pagename_set;
char qaullib_GuiPageName[MAX_PAGE_LEN +1];
struct sockaddr_in pin;
int ipc_socket, ipc_conn;
int qaul_new_msg;
int ipc_connected;
int qaul_loading_wait;
int qaul_conf_quit;
int qaul_conf_debug;
int qaul_conf_voip;
int qaul_conf_ios;
int qaul_conf_wifi_set;
int qaul_conf_interface;
int qaul_conf_internet;
int qaul_conf_network;
int qaul_web_localip_set;
int qaul_exe_available;
int qaul_ipc_topo_request;
int qaul_ipc_connected;
char qaullib_AppEventOpenURL[MAX_URL_LEN +1];

char qaullib_FileDownloadFolderPath[MAX_PATH_LEN +1];
int  qaul_conf_filedownloadfolder_set;

union olsr_message *qaul_in_msg;


/********************************************//**
 * configuration variables
 ***********************************************/
char qaul_username[MAX_USER_LEN +1];
int qaul_username_set;
// ip
int qaul_ip_version;                // IP version 4/6
int qaul_ip_size;
int qaul_ip_set;
char qaul_ip_str[MAX_IP_LEN +1];    // string of the IP
union olsr_ip_addr qaul_ip_addr;	// binary IP address
// network
char qaul_net_profile[255 +1];      // profile name
char qaul_net_broadcast[MAX_IP_LEN +1]; // broadcast address
char qaul_net_gateway[MAX_IP_LEN +1]; // string of the gateway IP
char qaul_net_ns1[MAX_IP_LEN +1];     // DNS 1 IP address
char qaul_net_ns2[MAX_IP_LEN +1];     // DNS 2 IP address
char qaul_net_ssid[255 +1];           // string of the SSID name
char qaul_net_bssid[17 +1];           // string of the BSSID
char qaul_net_interface[255 +1];      // string of the interface
int  qaul_interface_configuring;
char qaul_interface_json[MAX_JSON_LEN +1]; // json string of the actual interfaces
// internet sharing
int  qaul_internet_share;
char qaul_internet_interface[255 +1]; // string of the interface to share the Internet

// locale i18n
int qaul_locale_set;
char qaul_locale[MAX_LOCALE_LEN +1];
// finished
int qaul_configured;

/**
 * buffer and checker to write file path of
 * @see Qaullib_FilePicked(int, const char*)
 */
char pickFilePath[MAX_PATH_LEN +1];
int pickFileCheck;

/**
 * create a save file name from the file @a description the @a hashstr of the file and
 * the @a suffix
 *
 * @retval returns the string size of the file name
 */
int Qaullib_StringDescription2Filename(char *filename, struct qaul_file_LL_item *file, int buffer_size);

/**
 * checks and protects a message string
 *
 * @retval protected string size
 */
int Qaullib_StringMsgProtect(char *protected_string, char *unprotected_string, int buffer_size);

/**
 * checks and protects a user name string
 *
 * @retval protected string size
 */
int Qaullib_StringNameProtect(char *protected_string, char *unprotected_string, int buffer_size);

/**
 * protects a string for that it is save to send it via Json
 *
 * @retval the size of the protected string
 */
int Qaullib_StringJsonProtect(char *protected_string, char *unprotected_string, int buffer_size);

/**
 * protects a string to be ready to write it to the data base
 *
 * @retval the size of the protected string
 */
int Qaullib_StringDbProtect(char *protected_string, char *unprotected_string, int buffer_size);

/**
 * removes the protection characters from a protected data base string
 *
 * @retval the size of the unprotected string
 */
int Qaullib_StringDbUnprotect(char *unprotected_string, char *protected_string, int buffer_size);

/**
 * set IP @a protocol version (4 | 6)
 */
void Qaullib_SetProtocol(int protocol);

/**
 * set @a IP address
 *
 * @retval 1 success
 * @retval 0 error
 */
int Qaullib_SetIP(const char* IP);

/**
 * create an @a IP
 */
void Qaullib_CreateIP(char* IP);

/**
 * init the data base and check if table exists
 * called once at the beginning
 */
int  Qaullib_DbInit(void);

/**
 * insert a @a value for a configuration @a key
 */
void Qaullib_DbSetConfigValue(const char* key, const char* value);

/**
 * insert a @a value for a configuration @a key
 */
void Qaullib_DbSetConfigValueInt(const char* key, int value);

/**
 * get config @a value from data base for @a key
 */
int  Qaullib_DbGetConfigValue(const char* key, char *value);

/**
 * write default configuration into config table
 */
void Qaullib_DbPopulateConfig(void);

/**
 * check if a UI language was selected by the user
 * if not show the language selection screen.
 */
int Qaullib_ExistsLocale(void);

/**
 * @retval UI language selection
 */
const char* Qaullib_GetLocale(void);

/**
 * set @a locale UI language
 */
void Qaullib_SetLocale(const char* locale);

/**
 * convert a unix time stamp into an local ISO date format
 *
 * @retval 1 success
 * @retval 0 error
 */
int Qaullib_Timestamp2Isostr(char *isostr, int timestamp, int buffer_size);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
