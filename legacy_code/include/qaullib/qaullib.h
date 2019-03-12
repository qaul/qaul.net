/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB
#define _QAULLIB

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

#define QAUL_EVENT_QUIT        99
#define QAUL_EVENT_CHOOSEFILE 100
#define QAUL_EVENT_OPENFILE   101
#define QAUL_EVENT_OPENURL    102
#define QAUL_EVENT_NOTIFY     103
#define QAUL_EVENT_RING       104
#define QAUL_EVENT_GETINTERFACES 105
#define QAUL_EVENT_GATEWAY_START 106
#define QAUL_EVENT_GATEWAY_STOP  107

#define QAUL_ERROR_NOWIFI       1
#define QAUL_ERROR_NOROOT       2

#define QAUL_CONF_QUIT          1
#define QAUL_CONF_IOS           2
#define QAUL_CONF_INTERFACE     3
#define QAUL_CONF_INTERNET      4
#define QAUL_CONF_NETWORK       5

#define QAUL_CHECK_WIFI_SET     1

#define MAX_USER_LEN           20
#define MAX_PASSPHRASE_LEN	 1024
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

int qaul_conf_debug;

/**
 * configuration procedure in the qaul
 *
 * initialize qaullib
 *   @see Qaullib_Init()
 *
 *   platform specific initializations
 *   @see Qaullib_SetConf()
 *   @see Qaullib_SetConfDownloadFolder()
 *   @see Qaullib_GetConfInt()
 *   @see Qaullib_GetConfString()
 *
 * invoke configuration functions
 *
 * startup configuration (0):
 *   start web server
 *   @see Qaullib_WebserverStart()
 *
 * startup configuration (10):
 *   check if you have sufficient authorization rights
 *   request them if not
 *
 * startup configuration (20):
 *   check if wifi is configured manually
 *   Qaullib_GetConfInt("net.interface.manual")
 *   Qaullib_GetConfString("net.interface.name", config_interface_c)
 *
 *   check if wifi interface is available
 *   start wifi interface
 *   configure address
 *   connect to qaul.net
 *
 * startup configuration (30):
 *   @see Qaullib_ConfigStart()
 *
 *   check if user name was set
 *   wait until user name has been set
 *   @see Qaullib_ExistsUsername()
 *
 * startup configuration (40):
 *   start olsrd routing
 *
 * startup configuration (45):
 *   connect ipc
 *   @see Qaullib_IpcConnect()
 *
 * startup configuration (50):
 *   start voip
 *   @see Qaullib_SetConfVoIP()
 *
 *   start UDP server
 *   @see Qaullib_UDP_StartServer()
 *
 *   start captive portal
 *   @see Qaullib_CaptiveStart()
 *
 *   start port forwarding
 *   start timers to continuously invoke
 *   @see Qaullib_TimedCheckAppEvent()
 *   @see Qaullib_TimedSocketReceive()
 *   @see Qaullib_TimedDownload()
 *   continuously update network nodes:
 *   @see Qaullib_IpcSendCom()
 *
 *   tell qaullib that configuration is finished
 *   @see Qaullib_ConfigurationFinished()
 */

/**
 * initialize qaullib
 * invoke this function once at the beginning, before any other qaullib function
 * @a resourcePath is the absolute path to the directory containing the the www folder
 * @a homePath is the absolute path to the qaul home directory, containing the database etc.
 */
void Qaullib_Init(const char* homePath, const char* resourcePath);

/**
 * configure qaullib
 * enable Quit button in GUI
 * this is only used if the window has no exit button e.g. in android
 *
 * THIS FUNCTION IS DEPRECATED, USE @see Qaullib_SetConf() instead
 */
void Qaullib_SetConfQuit(void);

/**
 * platform specific configurations
 * QAUL_CONF_QUIT          for a quit button
 * QAUL_CONF_IOS           for the net configure screen on iOS
 * QAUL_CONF_INTERFACE     to be able to configure the network
 *                         interfaces manually
 * QAUL_CONF_INTERNET      to be able to configure Internet sharing
 * QAUL_CONF_NETWORK       to be able to configure a custom network
 */
void Qaullib_SetConf(int conf);

/**
 * platform specific checker
 * QAUL_CHECK_WIFI_SET
 */
int Qaullib_CheckConf(int conf);

/**
 * configure qaullib
 * enable VoIP
 */
void Qaullib_SetConfVoIP(void);

/**
 * set the download folder
 * all downloaded files will be copied into this folder after download.
 */
void Qaullib_SetConfDownloadFolder(const char *path);

/**
 * set @a locale UI language
 */
void Qaullib_SetLocale(const char* locale);

/**
 * get configuration for @key from DB
 *
 * @retval configuration string value, "" if nothing is found
 */
int Qaullib_GetConfString(const char *key, char *value);

/**
 * get configuration for @key from DB
 *
 * @retval configuration integer value, 0 if nothing is found
 */
int Qaullib_GetConfInt(const char *key);

/**
 * save configuration string @value for @key  to DB
 */
void Qaullib_SetConfString(const char *key, const char *value);

/**
 * save configuration integer @value for @key  to DB
 */
void Qaullib_SetConfInt(const char *key, int value);

/**
 * start web server
 *
 * @retval 1 web server started successfully
 * @retval 0 web server start error
 */
int Qaullib_WebserverStart(void);

/**
 * start GUI configuration (language, user name)
 * make sure all data is copied successfully
 */
void Qaullib_ConfigStart(void);

/**
 * start interprocess communication with olsrd_qaul plugin
 *
 * @retval 1 successfully connected
 * @retval 0 IPC connection failed
 */
int Qaullib_IpcConnect(void);

/**
 * check if user name exists
 *
 * @retval 1 user name exists
 * @retval 0 user name not set yet
 */
int Qaullib_ExistsUsername(void);

/**
 * get network profile name
 *
 * @retval string string name, such as "qaul"
 */
const char* Qaullib_GetNetProfile(void);

/**
 * get IP protocol version
 *
 * @retval 4 IPv4
 * @retval 6 IPv6
 */
int Qaullib_GetNetProtocol(void);

/**
 * get network mask as integer
 *
 * @retval  8 = "255.0.0.0"
 * @retval 16 = "255.255.0.0"
 * @retval 24 = "255.255.255.0"
 */
int Qaullib_GetNetMask(void);

/**
 * get network mask as IPv4 string
 *
 * translate the network mask integer into a string
 * e.g.: 8 => "255.0.0.0"
 *
 * @retval	IPv4 string of network mask
 */
const char* Qaullib_GetNetMaskString(void);

/**
 * get network broadcast address
 *
 * @retval string of broadcast address such as "10.255.255.255"
 */
const char* Qaullib_GetNetBroadcast(void);

/**
 * get network gateway
 *
 * @retval string of gateway e.g. "0.0.0.0"
 */
const char* Qaullib_GetNetGateway(void);

/**
 * get wifi ssid name
 *
 * @retval string of ssid name e.g. "qaul.net"
 */
const char* Qaullib_GetWifiSsid(void);

/**
 * check if the wifi bss id is set
 *
 * @retval 1 bss id is set
 * @retval 0 bss id not set
 */
int Qaullib_GetWifiBssIdSet(void);

/**
 * get wifi ibss cell id
 *
 * @retval string of bss id e.g. "B6:B5:B3:F5:AB:E4"
 */
const char* Qaullib_GetWifiBssId(void);

/**
 * get wifi channel
 *
 * @retval wifi channel integer
 */
int Qaullib_GetWifiChannel(void);

/**
 * get IP
 *
 * This function returns the IP.
 * If the IP is unknown, it gets the IP from the data base.
 * If there is no IP configured yet, it generates an IP
 * and writes it into the data base.
 *
 * @retval string of IP e.g. "10.33.234.12"
 */
const char* Qaullib_GetIP(void);

/**
 * set @ip
 *
 * @retval 1 sucess
 * @retval 0 error
 */
int Qaullib_SetIP(char* ip);

/**
 * get NS1 (DNS name server 1)
 *
 * @retval string of NS1 e.g. "5.45.96.220"
 */
const char* Qaullib_GetNetNs1(void);

/**
 * get NS2 (DNS name server 2)
 *
 * @retval string of NS2 e.g. "185.82.22.133"
 */
const char* Qaullib_GetNetNs2(void);

/**
 * check if the interface is set manually
 *
 * @retval 1 interface is set manually
 * @retval 0 interface is set automatically
 */
int Qaullib_GetInterfaceManual(void);

/**
 * save interface configuration method as integer @value to DB
 */
void Qaullib_SetInterfaceManual(int value);

/**
 * get network interface name
 *
 * @retval string of network interface name e.g. "wlan0"
 */
const char* Qaullib_GetInterface(void);

/**
 * set network interface @name
 */
void Qaullib_SetInterface(const char* name);

/**
 * set @json string of all available interfaces for the GUI
 */
void Qaullib_SetInterfaceJson(const char *json);

/**
 * check if this computer is configured as a gateway to the Internet
 *
 * @retval 1 gateway is configured
 * @retval 0 no gateway
 */
int Qaullib_IsGateway(void);

/**
 * get Internet gateway interface name
 *
 * @retval string of Internet gateway interface name e.g. "eth0"
 */
const char* Qaullib_GetGatewayInterface(void);


/**
 * start captive portal
 *
 * @retval 1 sucessfully started
 * @retval 0 failed
 */
int Qaullib_CaptiveStart(void);

/**
 * Start UDP Server on port 8083
 *
 * @retval 1 sucessfully started
 * @retval 0 failed
 */
int Qaullib_UDP_StartServer(void);


/**
 * tell qaullib to exit waiting screen
 */
void Qaullib_ConfigurationFinished(void);


/**
 * timed events must be called regularly from the qaul application
 */

/**
 * check if an event has occurred that needs action from the qaul application
 * to be called every 10ms / 100ms
 *
 * @retval QAUL_EVENT_QUIT          quit app
 * @retval QAUL_EVENT_CHOOSEFILE    open file picker
 * @retval QAUL_EVENT_OPENFILE      open file
 * @retval QAUL_EVENT_OPENURL       open url in external web browser
 * @retval QAUL_EVENT_NOTIFY        notify user about incoming message
 * @retval QAUL_EVENT_RING          play ring tone
 * @retval QAUL_EVENT_GETINTERFACES inform qaullib about all available network interfaces
 */
int  Qaullib_TimedCheckAppEvent(void);

/**
 * check sockets for incoming traffic
 * to be called every 100ms
 */
void Qaullib_TimedSocketReceive(void);

/**
 * check if a file or a username needs to be downloaded
 * to be called every 3s
 */
void Qaullib_TimedDownload(void);

/**
 * send ipc command
 * check for network topology every 5 seconds
 *
 * @param commandId == 0: exit command for olsrd
 * @param commandId == 1: check network topology / nodes
 */
void Qaullib_IpcSendCom(int commandId);

/**
 * get the path of the file to open
 * to be called after Qaullib_TimedCheckEvent() received QAUL_EVENT_CHOOSEFILE
 */
const char* Qaullib_GetAppEventOpenPath(void);

/**
 * get the URL of the page to open
 * to be called after Qaullib_TimedCheckEvent() received QAUL_EVENT_OPENURL
 */
const char* Qaullib_GetAppEventOpenURL(void);

/**
 * tell qaullib @a check if the user has picked a file
 * and send the @a path to the file.
 */
void Qaullib_FilePicked(int check, const char* path);

// Some functions required to interact with the static crypto state
// in qaullib.c


/**
 * invoke this function before exiting qaul
 */
void Qaullib_Exit(void);


/**
 * Additional helper functions
 * needed for the CLI configuration, until qaul.net has a proper
 * client / deamon architecture
 */

/**
 * checks and protects a user name string
 *
 * @retval protected string size
 */
int Qaullib_StringNameProtect(char *protected_string, char *unprotected_string, int buffer_size);


/**
 * set user name
 */
int Qaullib_SetUsername(char* name);



#ifdef __cplusplus
}
#endif // __cplusplus

#endif
