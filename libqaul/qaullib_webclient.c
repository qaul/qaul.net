/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"

#ifdef WIN32
#define close(x) closesocket(x)
#undef errno
#define errno WSAGetLastError()
#undef strerror
#define strerror(x) StrError(x)
#define perror(x) WinSockPError(x)
#endif

#ifndef MSG_NOSIGNAL
#define MSG_NOSIGNAL 0
#endif

// ------------------------------------------------------------
void *Qaullib_WgetRunThread(void *wget_connection)
{
	struct qaul_wget_connection *conn;
	int success;

	conn = (struct qaul_wget_connection *)wget_connection;

	if(QAUL_DEBUG)
		printf("Qaullib_WgetRunThread create type: %i\n", conn->type);

	// check for something to do
	while( 1 )
	{
		if(conn->connected)
		{
			if(QAUL_DEBUG)
				printf("Qaullib_WgetRunThread new connection %i\n", conn->type);

			// connect to address
			success = Qaullib_WgetConnect(conn);
			if(success)
			{
				// send header
				success = Qaullib_WgetSendHeader(conn);
				if(success)
				{
					// process the response
					Qaullib_WgetDownload(conn);
				}
				else
				{
					printf("send header failed ( type: %i )\n", conn->type);
					Qaullib_WgetFailed(conn);
				}
			}
			else
			{
				printf("connect failed ( type: %i )\n", conn->type);
				Qaullib_WgetFailed(conn);
			}
		}

		sleep(1);
	}
}

// ------------------------------------------------------------
void Qaullib_WgetFailed(struct qaul_wget_connection *myConn)
{
	// close connection
	Qaullib_WgetClose(myConn);

	if(QAUL_DEBUG)
		printf("Qaullib_WgetFailed ( type: %i )\n", myConn->type);

	// close the rest
	if(myConn->type == QAUL_WGET_USER)
		Qaullib_UserDownloadFailed((struct qaul_user_connection *)myConn->download_ref);
	else if(myConn->type == QAUL_WGET_FILE)
		Qaullib_FileDownloadFailed((struct qaul_file_connection *)myConn->download_ref);
}

// ------------------------------------------------------------
int Qaullib_WgetConnect(struct qaul_wget_connection *myConn)
{
	int inet;
	myConn->connected = 0;
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];

#ifdef WIN32
  int On = 1;
  unsigned long Len;
#else
  int flags;
#endif

  if(QAUL_DEBUG)
	  printf("Qaullib_WgetConnect ( type: %i )\n", myConn->type);

  if (!myConn->socket)
  {
	if ((myConn->socket = socket(myConn->ip.sin_family, SOCK_STREAM, 0)) == -1)
	{
      perror("socket");
#ifdef WIN32
      int err = WSAGetLastError();
#else
      int err = -1;
#endif
      printf("[qaullib] socket error %i\n", err);
      return -1;
    }
  }

  if(QAUL_DEBUG)
  {
	  printf("Attempting connect... ");
	  inet_ntop(AF_INET, &myConn->ip.sin_addr, (char *)&ipbuf, MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN));
	  printf("to %s\n", ipbuf);
  }


  // connect to PORT on HOST
  if (connect(myConn->socket, (struct sockaddr *)&myConn->ip, sizeof(struct sockaddr)) < 0)
  {
    fprintf(stderr, "Error connecting %d - %s\n", errno, strerror(errno));
    return 0;
  }
  else
  {
	  char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	  inet_ntop(myConn->ip.sin_family, &myConn->ip.sin_addr, (char *)&ipbuf, MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN));
	  printf("Connected!! socket: %i ip: %s\n", myConn->socket, ipbuf);

	if(QAUL_DEBUG)
		printf("connected\n");

	myConn->connected = 1;
	myConn->connected_at = time(NULL);
	myConn->lastreceived_at = time(NULL);
	myConn->received = 0;

	return 1;
  }

  if(QAUL_DEBUG)
  	printf("not connected\n");

  return 0;
}


// ------------------------------------------------------------
int Qaullib_WgetClose(struct qaul_wget_connection *myConn)
{
	int success;
	myConn->connected = 0;

	// if connection close fails, the response is not 0
	success = close(myConn->socket);
	myConn->socket = 0;
	if (success > 0)
	{
	  printf("Qaullib_WgetClose connection closing error: %i", success);
	  return 0;
	}
	return 1;
}

// ------------------------------------------------------------
int Qaullib_WgetSendHeader(struct qaul_wget_connection *myConn)
{
	int size;

	size = (int) strlen(myConn->header);
	if (send(myConn->socket, myConn->header, size, MSG_NOSIGNAL) < 0)
	{
		printf("Qaullib_WgetSendHeader tcp header error: connection lost!\n");
		Qaullib_WgetClose(myConn);
		return 0;
	}
	return 1;
}

// ------------------------------------------------------------
void Qaullib_WgetDownload(struct qaul_wget_connection *myConn)
{
	int bytes, first;

	first = 1;

	do
	{
		bytes = recv(myConn->socket, (char *)&myConn->buf +myConn->bufpos, BUFFSIZE -myConn->bufpos, 0);

		// proceed bytes
		if(bytes >= 0)
		{
			if(myConn->type == QAUL_WGET_USER)
			{
				if(!Qaullib_UserDownloadProcess((struct qaul_user_connection *)myConn->download_ref, bytes))
					break;
			}
			else if(myConn->type == QAUL_WGET_FILE)
			{
				if(!Qaullib_FileDownloadProcess((struct qaul_file_connection *)myConn->download_ref, bytes, first))
					break;
			}
			else
			{
				if(QAUL_DEBUG)
					printf("No connection type set\n");
			}
		}
		else
		{
			printf("Qaullib_WgetDownload recv error: %i\n", bytes);

			if(myConn->type == QAUL_WGET_USER)
				Qaullib_UserDownloadFailed((struct qaul_user_connection *)myConn->download_ref);
			else if(myConn->type == QAUL_WGET_FILE)
				Qaullib_FileDownloadFailed((struct qaul_file_connection *)myConn->download_ref);
			else
				printf("No connection type set\n");
		}

		first = 0;
	}while(bytes > 0);

	// close connection
	Qaullib_WgetClose(myConn);
}

// ------------------------------------------------------------
int Qaullib_WgetReceive(struct qaul_wget_connection *myConn)
{
	char *tmp;
	int bytes = 0;

	if (myConn->connected)
	{
		// check for timeouts
		if(myConn->lastreceived_at < time(NULL) - TIMEOUT_LASTRECEIVED ||
		   myConn->connected_at < time(NULL) - TIMEOUT_CONNECTED)
		{
			printf("Qaullib_WgetReceive socket received time out: connected %i lastreceived %i now %i\n",
					(int)myConn->lastreceived_at,
					(int)myConn->connected_at,
					(int)time(NULL));
			// close connection
			Qaullib_WgetClose(myConn);
			return -1;
		}

		bytes = recv(myConn->socket, (char *)&myConn->buf +myConn->bufpos, BUFFSIZE -myConn->bufpos, 0);

		// connection was closed
		if (bytes == 0)
		{
			printf("Qaullib_WgetReceive bytes == 0: close connection\n");
			Qaullib_WgetClose(myConn);
			return 0;
		}
		// data has been received
		else if (bytes > 0)
		{
			myConn->lastreceived_at = time(NULL);
		}
	}
	return bytes;
}
