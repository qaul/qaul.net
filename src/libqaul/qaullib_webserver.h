/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaullib web server using mongoose web server.
 *
 * The web server is reachable via port 8081. If all the binary installers are present
 * port 80 is forwarded to 8081. The web server delivers the static web pages from
 * the globally installed www directory and the shared files from the files folder
 * in the users .qaul folder in the users home directory.
 *
 * In this file are the functions that create the dynamic pages.
 *
 * link to some static pages:
 * captive portal installer download page: http://localhost:8081/
 * qaul.net GUI: http://localhost:8081/qaul.html
 * qaul.net web client: http://localhost:8081/qaul_web.html
 */

#ifndef _QAULLIB_WEBSERVER
#define _QAULLIB_WEBSERVER

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * call back function for web server
 */
void Ql_WwwEvent_handler(struct mg_connection *conn, int event, void *event_data);

/**
 * run server polling in separate thread
 */
void *Ql_Www_Server(void *webserver_instance);

/**
 * stop web server and free memory
 */
void Ql_Www_ServerStop(struct mg_mgr *webserver_instance);


/**
 * dynamic web pages:
 */

/**
 * store local user name in data base
 */
void Ql_WwwSetName(struct mg_connection *conn, int event, void *event_data);

/**
 * store GUI language in data base
 */
void Ql_WwwSetLocale(struct mg_connection *conn, int event, void *event_data);

/**
 * returns JSON with local user name.
 *
 * JSON example:
 * {"name":"MYUSERNAME"}
 */
void Ql_WwwGetName(struct mg_connection *conn, int event, void *event_data);

/**
 * request interface configuration
 */
void Ql_WwwConfigInterfaceLoading(struct mg_connection *conn, int event, void *event_data);

/**
 * load network interface configuration
 */
void Ql_WwwConfigInterfaceGet(struct mg_connection *conn, int event, void *event_data);

/**
 * save network interface configuration
 */
void Ql_WwwConfigInterfaceSet(struct mg_connection *conn, int event, void *event_data);

/**
 * request interface configuration for Internet sharing
 */
void Ql_WwwConfigInternetLoading(struct mg_connection *conn, int event, void *event_data);

/**
 * load Internet configuration
 */
void Ql_WwwConfigInternetGet(struct mg_connection *conn, int event, void *event_data);

/**
 * save Internet configuration
 */
void Ql_WwwConfigInternetSet(struct mg_connection *conn, int event, void *event_data);


/**
 * load active network configuration
 */
void Ql_WwwConfigNetworkGet(struct mg_connection *conn, int event, void *event_data);

/**
 * load profile network configuration
 */
void Ql_WwwConfigNetworkGetProfile(struct mg_connection *conn, int event, void *event_data);

/**
 * save network configuration
 */
void Ql_WwwConfigNetworkSet(struct mg_connection *conn, int event, void *event_data);

/**
 * load file sharing configuration
 */
void Ql_WwwConfigFilesGet(struct mg_connection *conn, int event, void *event_data);

/**
 * save file sharing configuration
 */
void Ql_WwwConfigFilesSet(struct mg_connection *conn, int event, void *event_data);

/**
 * load network topology
 */
void Ql_WwwGetTopology(struct mg_connection *conn, int event, void *event_data);

/**
 * Quit program
 */
void Ql_WwwQuit(struct mg_connection *conn, int event, void *event_data);

/**
 * process call handling
 */
void Ql_WwwCallStart(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwCallEnd(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwCallAccept(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwCallEvent(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFavoriteGet(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFavoriteAdd(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFavoriteDelete(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwSetPageName(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwSetOpenUrl(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwSetWifiSet(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwGetConfig(struct mg_connection *conn, int event, void *event_data);

/**
 * get messages
 * different type searches
 * type: 1 (send all new messages)
 * type: 5 (search for users)
 * type: 6 (search for this )
 */
void Ql_WwwGetMsgs(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwSendMsg(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwGetUsers(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwGetEvents(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFileList(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFileAdd(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFilePick(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFilePickCheck(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFileOpen(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFileDelete(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwFileSchedule(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwPubUsers(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwPubFilechunk(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwPubMsg(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwPubInfo(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwWebGetMsgs(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwWebSendMsg(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwWebGetUsers(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwWebGetFiles(struct mg_connection *conn, int event, void *event_data);

/**
 * upload a file as a web user.
 *
 * upload variables:
 * m: file description message
 * f: file data
 *
 * this function returns an empty JSON object: {}
 */
void Ql_WwwWebFileUpload(struct mg_connection *conn, int event, void *event_data);

/**
 * returns a JSON object with an array of all available qaul.net binary installers.
 *
 * JSON example:
 * {"name":"MYUSERNAME",
 * "files":[
 * {"hash":"d065b3b913fbdd4437befff77629e967be628138","size":3151794,"suffix":"deb","description":"Ubuntu & Debian 32 Bit","time":"2016-02-05 14:26:49","status":5,"downloaded":0},
 * {"hash":"4c8aa1ce39c9f6795ae6e1f3c7951dc54996d57d","size":3130836,"suffix":"deb","description":"Ubuntu & Debian 64 Bit","time":"2016-02-05 14:26:49","status":5,"downloaded":0}
 * ]}
 */
void Ql_WwwExtBinaries(struct mg_connection *conn, int event, void *event_data);

/**
 * loading function is checked by GUI in loading screen.
 * It returns a JSON object stating whether the loading screen should change
 * and if yes to what page.
 *
 * JSON examples:
 * {"change":0}
 * {"change":1,"page":"#page_config_locale"}
 */
void Ql_WwwLoading(struct mg_connection *conn, int event, void *event_data);

/**
 * Crypto access functions
 */
void Ql_WwwCryGetInfo(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwCryInitialise(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwCryCreateUsr(struct mg_connection *conn, int event, void *event_data);

/*
 * iOS/OSX captive portal checking fix
 *
 * OSX checks for captive portals, if this page is not returned, OSX users can't download binaries
 * via the captive portal.
 *
 * checked URLs:
 * http://captive.apple.com/hotspot-detect.html
 * /library/test/success.html
 */
void Ql_WwwCaptivePortalDetectionOsx(struct mg_connection *conn, int event, void *event_data);

/**
 * iOS/OSX captive portal whitelisting
 *
 * Whitelist a specific IP address
 */
void Ql_WwwCaptiveWhitelist(struct mg_connection *conn, int event, void *event_data);



#ifdef __cplusplus
}
#endif // __cplusplus

#endif
