/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_WEBCLIENT
#define _QAULLIB_WEBCLIENT

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * call back functions
 */
//void (*qaullib_wget_process)(struct qaul_wget_connection *, void *, int);
//void (*qaullib_wget_end)(struct qaul_wget_connection *, void *);
//void (*qaullib_wget_failed)(struct qaul_wget_connection *, void *);
#define QAUL_WGET_USER 1
#define QAUL_WGET_FILE 2

/**
 * structure of web client connection
 */
// FIXME: ipv6
struct qaul_wget_connection
{
	struct sockaddr_in ip;
	int connected;
	int socket;
	int max_ttl;
	int max_lastreceived;
	time_t connected_at;
	time_t lastreceived_at;
	int received;
	int bufsize;
	int bufpos;

	int type;  // todo: do this with call back functions
	void *download_ref;
//	qaullib_wget_process download_process;
//	qaullib_wget_end download_end;
//	qaullib_wget_failed download_failed;

	char header[MAX_HEADER_LEN +1];
	union qaul_inbuf buf;
};

/**
 * start and run the webclient thread
 */
void *Qaullib_WgetRunThread(void *connection);

/**
 * connection @a myConn failed
 */
void Qaullib_WgetFailed(struct qaul_wget_connection *myConn);

/**
 * connect the @a myConn info
 *
 * @retval -1 socket error
 * @retval  0 not able to connect
 * @retval  1 successfully connected
 */
int Qaullib_WgetConnect(struct qaul_wget_connection *myConn);

/**
 * close the @a myConn connection
 *
 * @retval 1 successfully closed
 * @retval 0 error in closing
 */
int Qaullib_WgetClose(struct qaul_wget_connection *myConn);

/**
 * send @a header via @a myConn connection
 *
 * @retval 1 successfully sent
 * @retval 0 connection error
 */
int Qaullib_WgetSendHeader(struct qaul_wget_connection *myConn);

/**
 * download from connection
 */
void Qaullib_WgetDownload(struct qaul_wget_connection *myConn);

/**
 * check if connection @a myConn received something
 *
 * @returnval received Bytes
 */
int Qaullib_WgetReceive(struct qaul_wget_connection *myConn);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
