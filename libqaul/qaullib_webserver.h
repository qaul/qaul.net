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
int Qaullib_WwwEvent_handler(struct mg_connection *conn, enum mg_event event);

/**
 * run server polling in separate thread
 */
void *Qaullib_Www_Server(void *qaul_webserver_instance);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
