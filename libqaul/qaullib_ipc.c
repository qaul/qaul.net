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
int Qaullib_IpcConnect(void)
{

#ifdef WIN32
  int On = 1;
  unsigned long Len;
#else
  int flags;
#endif

  ipc_connected = 0;

  // fill in the socket structure with host information
  struct sockaddr_in saddr;
  saddr.sin_family = AF_INET;
  saddr.sin_port = htons(IPC_PORT);
  inet_aton("127.0.0.1", &saddr.sin_addr); // numeric IP only

  if ((ipc_socket = socket(PF_INET, SOCK_STREAM, 0)) == -1)
  {
	  perror("socket");
#ifdef WIN32
      int err = WSAGetLastError();
#else
      int err = -1;
#endif
      return err;
  }

  if(qaul_ipc_connected == 1)
	  printf("[qaullib] IPC: Attempting connect...");

  // connect to PORT on HOST
  if ((ipc_conn = connect(ipc_socket, (struct sockaddr *)&saddr, sizeof(saddr))) < 0)
  {
	  if(qaul_ipc_connected == 1)
	  {
		  fprintf(stderr, "Error connecting %d - %s\n", errno, strerror(errno));
		  qaul_ipc_connected = 0;
	  }
	  else if(QAUL_DEBUG)
		  printf(".");

	  // close socket if there was an error
	  Qaullib_IpcClose();
	  //return (int)errno;
  }
  else
  {
    printf("Connected!! socket: %i connection: %i\n",ipc_socket,ipc_conn);
    qaul_ipc_connected = 1;

    // Setting socket non-blocking
#ifdef WIN32
    if (WSAIoctl(ipc_socket, FIONBIO, &On, sizeof(On), NULL, 0, &Len, NULL, NULL) < 0)
    {
      fprintf(stderr, "Error while making socket non-blocking!\n");
    }
#else
    if ((flags = fcntl(ipc_socket, F_GETFL, 0)) < 0)
    {
      fprintf(stderr, "Error getting socket flags!\n");
    }

    if (fcntl(ipc_socket, F_SETFL, flags | O_NONBLOCK) < 0)
    {
      fprintf(stderr, "Error setting socket flags!\n");
    }
#endif
    ipc_connected = 1;

    // send user hello message
    Qaullib_IpcSendUserhello();

    return 1;
  }
  return 0;
}


// ------------------------------------------------------------
int Qaullib_IpcClose(void)
{
  if (close(ipc_socket))
    return 1;

  return 0;
}

// ------------------------------------------------------------
void Qaullib_IpcReceive(void)
{
	int bytes, tmp_len;
	char *tmp;
	union
	{
		char buf[BUFFSIZE + 1];
		union olsr_message msg;
	} inbuf;

	if (!ipc_connected)
	{
		if(qaul_ipc_connected == 1)
			printf("Connection closed, try to reconnect ...\n");

		// connect to the application
		Qaullib_IpcConnect();
	}
	else
	{
		memset(&inbuf, 0, BUFFSIZE + 1);

		bytes = recv(ipc_socket, (char *)&inbuf, BUFFSIZE, 0);
		if (bytes == 0)
		{
		  shutdown(ipc_socket, SHUT_RDWR);
		  printf("bytes == 0: close connection\n");
		  //set_net_info("Disconnected from server...", 1);
		  ipc_connected = 0;
		  close(ipc_socket);
		}

		if (bytes > 0)
		{
			//printf("bytes: %i msg-size: %i type: %i\n", bytes, (int) ntohs(inbuf.msg.v4.olsr_msgsize), (int) inbuf.msg.v4.olsr_msgtype);

			tmp = (char *)&inbuf.msg;
			qaul_in_msg = &inbuf.msg;

			// do it as often as needed until all messages are out of the buffer.
			if (bytes > 0 && ntohs(inbuf.msg.v4.olsr_msgsize) <= bytes)
			{
				while (bytes > 0 && ntohs(qaul_in_msg->v4.olsr_msgsize) <= bytes)
				{
					//printf("read out bytes: %i %i\n", bytes, ntohs(qaul_in_msg->v4.olsr_msgsize));

					// proceed
					Qaullib_IpcEvaluateMessage(qaul_in_msg);

					// copy buffer to new location
					tmp_len = ntohs(qaul_in_msg->v4.olsr_msgsize);
					qaul_in_msg = (union olsr_message *)&tmp[tmp_len];
					tmp = &tmp[tmp_len];
					if (tmp_len == 0)
						break;
					bytes = bytes - tmp_len;
					tmp_len = ntohs(qaul_in_msg->v4.olsr_msgsize);

					// Copy to start of buffer
					if (tmp_len > bytes) {
						// Copy the buffer
						memcpy(&inbuf, tmp, bytes);
						bytes = recv(ipc_socket, (char *)&inbuf.buf[bytes], tmp_len - bytes, 0);
						tmp = (char *)&inbuf.msg;
						qaul_in_msg = (union olsr_message *)tmp;
					}
				}
			}
		}
	}
}


// ------------------------------------------------------------
void Qaullib_IpcEvaluateMessage(union olsr_message *msg)
{
	//printf("message arrived: %i\n", msg->v4.olsr_msgtype);
	switch(msg->v4.olsr_msgtype)
	{
		case QAUL_CHAT_MESSAGE_TYPE:
			Qaullib_IpcEvaluateChat(msg);
			break;
		case QAUL_IPCCOM_MESSAGE_TYPE:
			Qaullib_IpcEvaluateCom(msg);
			break;
		case QAUL_IPCTOPO_MESSAGE_TYPE:
			Qaullib_IpcEvaluateTopo(msg);
			break;
		case QAUL_IPCMESHTOPO_MESSAGE_TYPE:
			Qaullib_IpcEvaluateMeshtopo(msg);
			break;
		case QAUL_USERHELLO_MESSAGE_TYPE:
			Qaullib_IpcEvaluateUserhello(msg);
			break;
		case QAUL_FILEDISCOVER_MESSAGE_TYPE:
			Qaullib_IpcEvaluateFilediscover(msg);
			break;
		case QAUL_EXEDISCOVER_MESSAGE_TYPE:
			Qaullib_IpcEvaluateExediscover(msg);
			break;
		default:
			printf("not a known message type\n");
			break;
	}
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateChat(union olsr_message *msg)
{
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	char chat_msg[MAX_MESSAGE_LEN +1];
	char chat_user[MAX_USER_LEN +1];
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;

	// fill in values
	msg_item.id = 0;
	msg_item.type = QAUL_MSGTYPE_PUBLIC_IN;

	// get msg
	memcpy(&msg_item.msg, msg->v4.message.chat.msg, MAX_MESSAGE_LEN);
	memcpy(&msg_item.msg[MAX_MESSAGE_LEN], "\0", 1);

	// get name
	memcpy(&msg_item.name, msg->v4.message.chat.name, MAX_USER_LEN);
	memcpy(&msg_item.name[MAX_USER_LEN], "\0", 1);

	// set time
	time(&timestamp);
	msg_item.time = (int)timestamp;

	// set read
	msg_item.read = 0;

	// set ip
	// todo: ipv6
	msg_item.ipv = 4;
	strncpy(msg_item.ip, inet_ntop(AF_INET, &msg->v4.originator, (char *)&ipbuf, sizeof(ipbuf)), sizeof(msg_item.ip));
	memcpy(&msg_item.ip_union.v4, &msg->v4.originator, sizeof(msg_item.ip_union.v4));

  	// save Message
	Qaullib_MsgAdd(&msg_item);
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateCom(union olsr_message *msg)
{
	switch(msg->v4.message.ipc.type)
	{
		case QAUL_IPCCOM_MESHTOPO_SENT:
			qaul_ipc_topo_request = 2;
			break;
		default:
			printf("not a known message type\n");
			break;
	}
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateTopo(union olsr_message *msg)
{
	float linkcost;

	// check if user has our ip
	if(memcmp(&qaul_ip_addr.v4, &msg->v4.message.node.ip.v4, sizeof(msg->v4.message.node.ip.v4)) == 0)
		return;

	memcpy(&linkcost, &msg->v4.message.node.lq, sizeof(linkcost));
	// check if user exists, create it if not
	Qaullib_UserTouchIp(&msg->v4.message.node.ip, linkcost);
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateMeshtopo(union olsr_message *msg)
{
	if(QAUL_DEBUG)
		printf("Qaullib_IpcEvaluateMeshtopo\n");

	// create new entry
	Qaullib_Topo_LL_Add(&msg->v4.message.node.ip, &msg->v4.message.node.gateway, msg->v4.message.node.lq);
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateUserhello(union olsr_message *msg)
{
	if(QAUL_DEBUG)
		printf("Qaullib_IpcEvaluateUserhello()\n");
	// todo: ipv6
	union olsr_ip_addr ip;
	memcpy(&ip.v4, &msg->v4.originator, sizeof(msg->v4.originator));

	Qaullib_UserAdd(	&ip,
						msg->v4.message.userhello.name,
						msg->v4.message.userhello.icon,
						msg->v4.message.userhello.suffix);
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateFilediscover(union olsr_message *msg)
{
	char buffer[1024];
	char* stmt = buffer;
	char *error_exec = NULL;
	char hash[MAX_HASH_LEN];
	struct qaul_file_LL_item *file_item;
	struct qaul_fileavailable_msg fileavailable_msg;
	union olsr_ip_addr ip;

	if(QAUL_DEBUG)
		printf("Qaullib_IpcEvaluateFilediscover\n");

	// todo: ipv6
	// check if hash exists
	if(Qaullib_File_LL_HashSearch(msg->v4.message.filediscover.hash, &file_item))
	{
		// check if file is available
		if(file_item->status >= QAUL_FILESTATUS_DOWNLOADED)
		{
			// generate the file available message
			fileavailable_msg.msgtype = htons(QAUL_FILEAVAILABLE_MESSAGE_TYPE);
			memcpy(&fileavailable_msg.hash, file_item->hash, MAX_HASH_LEN);
			memcpy(&fileavailable_msg.suffix, file_item->suffix, MAX_SUFFIX_LEN);
			fileavailable_msg.filesize = htonl(file_item->size);

			// set the ip address
			// todo: ipv6
			memcpy(&ip.v4, &msg->v4.originator, sizeof(msg->v4.originator));

			Qaullib_UDP_SendFileavailableMsg(&fileavailable_msg, &ip);
		}
	}
}

// ------------------------------------------------------------
void Qaullib_IpcEvaluateExediscover(union olsr_message *msg)
{
	struct qaul_exeavailable_msg exeavailable_msg;
	int  i;
	uint32_t OS_flags;
	union olsr_ip_addr ip;

	if(QAUL_DEBUG)
		printf("Qaullib_IpcEvaluateExediscover \n");

	OS_flags = ntohl(msg->v4.message.exediscover.OS_flag);

	// todo: ipv6
	// check if OS flag exists
	for(i=0; i<MAX_POPULATE_FILE; i++)
	{
		if(
			(qaul_exe_array[i].OS_flag & OS_flags) &&
			qaul_exe_array[i].discovered > 0
			)
		{
			if(QAUL_DEBUG)
				printf("flag %i found (for %s)\n", qaul_exe_array[i].OS_flag, qaul_exe_array[i].hashstr);

			// check if file is available
			if(Qaullib_File_LL_FileAvailable(qaul_exe_array[i].hash))
			{
				if(QAUL_DEBUG)
					printf("file available: %s\n", qaul_exe_array[i].hashstr);

				// send exe available message
				exeavailable_msg.msgtype = htons(QAUL_EXEAVAILABLE_MESSAGE_TYPE);
				exeavailable_msg.OS_flag = htonl(qaul_exe_array[i].OS_flag);
				memcpy(exeavailable_msg.hash, qaul_exe_array[i].hash, MAX_HASH_LEN);
				memcpy(exeavailable_msg.suffix, qaul_exe_array[i].suffix, MAX_SUFFIX_LEN);
				exeavailable_msg.filesize = htonl(qaul_exe_array[i].size);

				memcpy(&ip.v4, &msg->v4.originator, sizeof(msg->v4.originator));

				Qaullib_UDP_SendExeavailableMsg(&exeavailable_msg, &ip);
			}
		}
	}
}

// ------------------------------------------------------------
void Qaullib_IpcSendCom(int commandId)
{
	char buffer[1024];
	union olsr_message *msg = (union olsr_message *)buffer;
	int size;

	// pack chat into olsr message
	// ipv4 only at the moment
	memset(&msg->v4.originator, 0, sizeof(msg->v4.originator));
    msg->v4.olsr_msgtype = QAUL_IPCCOM_MESSAGE_TYPE;
    msg->v4.message.ipc.type = commandId;
    size = sizeof( struct qaul_ipc_msg);
	size = size + sizeof(struct olsrmsg);
    msg->v4.olsr_msgsize = htons(size);

	// send package
	Qaullib_IpcSend(msg);
}


// ------------------------------------------------------------
void Qaullib_IpcSend(union olsr_message *msg)
{
	int size;
	size = (int) ntohs(msg->v4.olsr_msgsize);

	if (send(ipc_socket,(const char *)msg, size, MSG_NOSIGNAL) < 0)
	{
		printf("[qaullib] IPC connection lost!\n");
		CLOSE(ipc_socket);
		ipc_connected = 0;
	}
}

// ------------------------------------------------------------
void Qaullib_IpcSendUserhello(void)
{
	char buffer[1024];
	int size;
	union olsr_message *m = (union olsr_message *)buffer;

	printf("send user hello message \n");

	// send user hello message
	// todo: ipv6
	memset(&m->v4.originator, 0, sizeof(m->v4.originator));
	m->v4.olsr_msgtype = QAUL_USERHELLO_MESSAGE_TYPE;
	memcpy(&m->v4.message.userhello.name, qaul_username, MAX_USER_LEN);
	//memcpy(&m->v4.message.userhello.icon, "\0", 1);
	memset(&m->v4.message.userhello.icon, 0, sizeof(m->v4.message.userhello.icon));
	memcpy(&m->v4.message.userhello.suffix, "\0", 1);
	size = sizeof( struct qaul_userhello_msg);
	size = size + sizeof(struct olsrmsg);
	m->v4.olsr_msgsize = htons(size);

	// send package
	Qaullib_IpcSend(m);
}
