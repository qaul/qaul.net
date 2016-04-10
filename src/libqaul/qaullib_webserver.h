/*
 * qaul.net is free software
 * licensed under GPL (version 3)
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

void Ql_WwwSetName(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwSetLocale(struct mg_connection *conn, int event, void *event_data);
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
void Ql_WwwExtBinaries(struct mg_connection *conn, int event, void *event_data);
void Ql_WwwLoading(struct mg_connection *conn, int event, void *event_data);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif
