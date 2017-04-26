/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/*
#include <sys/types.h>
#include <netinet/in.h>
#include <unistd.h>
#include <fcntl.h>
#include <arpa/inet.h>
#include <signal.h>

#include <sys/types.h>
#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <sys/time.h>
#include <time.h>
#include <math.h>
#include <stdio.h>
#include <string.h>

#include <stdlib.h>
#include <errno.h>
#include <stdarg.h>
*/
/*
#ifdef WIN32
#include <winsock2.h>
#else
// manpage says: fd_set is in sys/select.h with posix (at least with the Android-NDK)
#include <sys/select.h>
#endif

// OpenBSD wants this here
#include <sys/types.h>
#include <sys/socket.h>
*/

#include "qaul_olsrd_plugin.h"

#include <fcntl.h> // F_GETFL F_SETFL O_NONBLOCK
#include <unistd.h>

#include "qaul_ipc.h"
#include "qaul_msg.h"
#include "qaul_messages.h"
#include "qaul_net.h"

// ------------------------------------------------------------
// defines
#ifdef WIN32
#define close(x) closesocket(x)
#define perror(x) WinSockPError(x)
void WinSockPError(const char *);
#endif

#ifndef MSG_NOSIGNAL
#define MSG_NOSIGNAL 0
#endif

#define QAUL_IPC_PORT 8112
#define BUFFSIZE 8192

static int ipc_sock = -1;
static int ipc_conn = -1;
static int ipc_active = 0;
/*
union ipc_inbuf{
    char buf[BUFFSIZE + 1];
    union olsr_message msg;
};
struct {
	int size = 0;
	union ipc_inbuf buf;
} ipc_inbuf_struct;
*/
// ------------------------------------------------------------
// structs & variables
union olsr_message *qaul_in_msg;



// ------------------------------------------------------------
// outgoing messages
// ------------------------------------------------------------
// forward olsr message via ipc socket
// (smaller footprint, clearer interface)

void qaul_ipc_msg2gui(union olsr_message *m)
{
	if(ipc_active)
	{
		int size;
		size = (int) ntohs(m->v4.olsr_msgsize);

		OLSR_PRINTF(1, "[Qaul] Message to gui\n");

		if (send(ipc_conn,(const char *)m, size, MSG_NOSIGNAL) < 0)
		{
			OLSR_PRINTF(1, "[Qaul] IPC connection lost!\n");
			CLOSE(ipc_conn);
			ipc_active = false;
		}
	}
}


// ------------------------------------------------------------
// incoming messages
// ------------------------------------------------------------

void qaul_ipc_receive(void *foo __attribute__ ((unused)))
{
	int bytes, tmp_len;
	char *tmp;
	union {
	char buf[BUFFSIZE + 1];
	union olsr_message msg;
	} inbuf;


	if(ipc_active)
	{
		memset(&inbuf, 0, sizeof(BUFFSIZE + 1));

		//OLSR_PRINTF(1, "[Qaul] ipc_sock = %i\n", ipc_sock);
		bytes = recv(ipc_conn, (char *)&inbuf, BUFFSIZE, 0);
		if (bytes == 0)
		{
			OLSR_PRINTF(1, "[Qaul] socket closed\n");
		}
		else if(bytes > 0)
		{
			tmp = (char *)&inbuf.msg;
			qaul_in_msg = &inbuf.msg;

			OLSR_PRINTF(1, "[Qaul] received bytes: %i size: %i type: %i\n", bytes, ntohs(inbuf.msg.v4.olsr_msgsize),inbuf.msg.v4.olsr_msgtype);

			// do it as often as needed until all messages are out of the buffer.
			if (bytes > 0 && ntohs(inbuf.msg.v4.olsr_msgsize) <= bytes)
			{
				while (bytes > 0 && ntohs(qaul_in_msg->v4.olsr_msgsize) <= bytes)
				{
					OLSR_PRINTF(1, "[Qaul] proceed\n");

					// proceed
					qaul_ipc_evaluate(qaul_in_msg);

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
						bytes = recv(ipc_conn, (char *)&inbuf.buf[bytes], tmp_len - bytes, 0);
						tmp = (char *)&inbuf.msg;
						qaul_in_msg = (union olsr_message *)tmp;
					}
				}
			}
		}
	}
}

void qaul_ipc_evaluate(union olsr_message *msg)
{
	OLSR_PRINTF(1, "[Qaul] IPC message arrived! Message type: %i\n", msg->v4.olsr_msgtype);
	switch(msg->v4.olsr_msgtype)
	{
		case QAUL_CHAT_MESSAGE_TYPE:
			OLSR_PRINTF(1, "[QAUL] send chat message\n");
			qaul_qaulmsg_send_all(msg);
			break;
		case QAUL_IPCCOM_MESSAGE_TYPE:
			OLSR_PRINTF(1, "[QAUL] check topology\n");
			qaul_ipc_evaluate_com(msg);
			break;
		case QAUL_USERHELLO_MESSAGE_TYPE:
			OLSR_PRINTF(1, "[QAUL] send user hello message\n");
			qaul_qaulmsg_send_all(msg);
			break;
		case QAUL_FILEDISCOVER_MESSAGE_TYPE:
			OLSR_PRINTF(1, "[QAUL] send file discover message\n");
			qaul_qaulmsg_send_all(msg);
			break;
		case QAUL_EXEDISCOVER_MESSAGE_TYPE:
			OLSR_PRINTF(1, "[QAUL] send exe discover message\n");
			qaul_qaulmsg_send_all(msg);
			break;
		default:
			OLSR_PRINTF(1, "not a known message type\n");
			break;
	}
}

void qaul_ipc_evaluate_com(union olsr_message *msg)
{
	struct qaul_ipc_msg *ipcCom = ( struct qaul_ipc_msg *)ARM_NOWARN_ALIGN(&msg->v4.message);
	switch(ipcCom->type)
	{
		case QAUL_IPCCOM_QUIT:
			// exit olsrd
			OLSR_PRINTF(1, "[Qaul] ipc EXIT command received!\n");
			olsr_exit("[Qaul] exit message received", EXIT_FAILURE);
			break;
		case QAUL_IPCCOM_GETTOPO:
			OLSR_PRINTF(1, "[Qaul] ipc GET TOPO command received: 1\n");
			qaul_net_topo2gui();
			break;
		case QAUL_IPCCOM_GETMESHTOPO:
			OLSR_PRINTF(1, "[Qaul] ipc GET MESHTOPO command received: 2\n");
			qaul_net_meshtopo2gui();
			break;
		default:
			OLSR_PRINTF(1, "[Qaul] not a known ipc command: %i\n", ipcCom->type);
			break;
	}
}


// ------------------------------------------------------------
// ipc connection
// ------------------------------------------------------------

void qaul_ipc_accept(int fd, void *data __attribute__ ((unused)), unsigned int flags __attribute__ ((unused)))
{
#ifdef WIN32
  int On = 1;
  unsigned long Len;
#else
  //int myflags;
#endif

  socklen_t addrlen;
  struct sockaddr_in pin;
  char *addr;

  addrlen = sizeof(struct sockaddr_in);

  if ((ipc_conn = accept(fd, (struct sockaddr *)&pin, &addrlen)) == -1) {
    perror("[Qaul] IPC accept");
    olsr_exit("[Qaul] IPC accept", EXIT_FAILURE);
  } else {
    OLSR_PRINTF(1, "[Qaul] Front end connected\n");
    addr = inet_ntoa(pin.sin_addr);
    if (qaul_ipc_check_allowed_ip((union olsr_ip_addr *)&pin.sin_addr.s_addr)) {
        OLSR_PRINTF(1, "[Qaul] Connection from %s\n", addr);

        // make the socket non blocking
        // Setting socket non-blocking
#ifdef WIN32
        if (WSAIoctl(ipc_conn, FIONBIO, &On, sizeof(On), NULL, 0, &Len, NULL, NULL) < 0) {
          fprintf(stderr, "Error while making socket non-blocking!\n");
          exit(1);
        }
#else
        if (fcntl(ipc_conn, F_SETFL, flags | O_NONBLOCK) < 0) {
          fprintf(stderr, "Error setting socket flags!\n");
          exit(1);
        }
#endif
        ipc_active = true;
    } else {
      OLSR_PRINTF(1, "[Qaul] Front end-connection from foregin host(%s) not allowed!\n", addr);
      OLSR_PRINTF(1, "[Qaul] OLSR: Front end-connection from foregin host(%s) not allowed!\n", addr);
      CLOSE(ipc_conn);
    }
  }
}

bool qaul_ipc_check_allowed_ip(const union olsr_ip_addr *addr)
{
  struct ip_prefix_list *ipcn;

  if (addr->v4.s_addr == ntohl(INADDR_LOOPBACK)) {
    return true;
  }

  /* check nets */
  for (ipcn = olsr_cnf->ipc_nets; ipcn != NULL; ipcn = ipcn->next) {
    if (ip_in_net(addr, &ipcn->net)) {
      return true;
    }
  }

  return false;
}

// ------------------------------------------------------------
// initialize ipc
// ------------------------------------------------------------

int qaul_ipc_init(void)
{
/**/
#ifdef WIN32
  int On = 1;
  unsigned long Len;
#else
  int flags;
#endif

  //int flags;
  struct sockaddr_in mysin;
  int port;
  int yes = 1;

  // Add parser function
  // would forward all OLSRD Messages to GUI
  //olsr_parser_add_function(&frontend_msgparser, PROMISCUOUS);

  /* get an internet domain socket */
  if ((ipc_sock = socket(AF_INET, SOCK_STREAM, 0)) == -1) {
    perror("[Qaul] IPC socket");
    olsr_exit("[Qaul] IPC socket", EXIT_FAILURE);
  }

  if (setsockopt(ipc_sock, SOL_SOCKET, SO_REUSEADDR, (char *)&yes, sizeof(yes)) < 0) {
    perror("[Qaul] SO_REUSEADDR failed");
    return 0;
  }

  /* complete the socket structure */
  port = qaul_ipc_port != 0? qaul_ipc_port : QAUL_IPC_PORT;
  memset(&mysin, 0, sizeof(mysin));
  mysin.sin_family = AF_INET;
  mysin.sin_addr.s_addr = INADDR_ANY;
  mysin.sin_port = htons(port);

  /* bind the socket to the port number */
  if (bind(ipc_sock, (struct sockaddr *)&mysin, sizeof(mysin)) == -1) {
    perror("[Qaul] bind IPC error");
    OLSR_PRINTF(1, "[Qaul] Will retry in 5 seconds...\n");
    sleep(5);
    if (bind(ipc_sock, (struct sockaddr *)&mysin, sizeof(mysin)) == -1) {
      perror("[Qaul] bind IPC error");
      olsr_exit("[Qaul] exit due to IPC bind error", EXIT_FAILURE);
    }
    OLSR_PRINTF(1, "[Qaul] OK\n");
  }

  // show that we are willing to listen
  if (listen(ipc_sock, olsr_cnf->ipc_connections) == -1) {
    perror("[Qaul] IPC listen");
    olsr_exit("[Qaul] IPC listen", EXIT_FAILURE);
  }


    // Setting socket non-blocking
#ifdef WIN32
    if (WSAIoctl(ipc_sock, FIONBIO, &On, sizeof(On), NULL, 0, &Len, NULL, NULL) < 0) {
      fprintf(stderr, "Error while making socket non-blocking!\n");
      exit(1);
    }
#else
    if ((flags = fcntl(ipc_sock, F_GETFL, 0)) < 0) {
      fprintf(stderr, "Error getting socket flags!\n");
      exit(1);
    }

    if (fcntl(ipc_sock, F_SETFL, flags | O_NONBLOCK) < 0) {
      fprintf(stderr, "Error setting socket flags!\n");
      exit(1);
    }
#endif
/**/

  // Register the socket with the socket parser
  add_olsr_socket(ipc_sock, &qaul_ipc_accept, NULL, NULL, SP_PR_READ);

	// check for new messages
	olsr_start_timer(1 * MSEC_PER_SEC, 0, OLSR_TIMER_PERIODIC, &qaul_ipc_receive, NULL, 0);

  //return ipc_sock;
  return 1;
}


// ------------------------------------------------------------
// shutdown ipc
// ------------------------------------------------------------
int qaul_ipc_shutdown(void)
{
  OLSR_PRINTF(1, "[QAUL] Shutting down IPC...\n");
  CLOSE(ipc_sock);
  CLOSE(ipc_conn);

  return 1;
}

