/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * UDP server and client for file sharing messages
 */

#include "qaullib_private.h"

// ------------------------------------------------------------
int  Qaullib_UDP_StartServer(void)
{
#ifdef WIN32
	int On = 1;
	unsigned long Len;
#else
	int flags;
#endif

	struct sockaddr_in myAddr;
	int option, option_status;

	// todo: ipv6
	myAddr.sin_family = AF_INET;
	myAddr.sin_port = htons(UDP_PORT);
	myAddr.sin_addr.s_addr = htonl(INADDR_ANY);

	qaul_UDP_socket = socket(PF_INET, SOCK_DGRAM, 0);

	if(qaul_UDP_socket == INVALID_SOCKET)
	{
		printf("unable to create UDP socket\n");
		return 0;
	}

	qaul_UDP_started = bind(qaul_UDP_socket, (struct sockaddr *)&myAddr, sizeof(myAddr));
	if(qaul_UDP_started < 0)
	{
		qaul_UDP_started = 0;
		perror("UDP socket bind error\n");
		return 0;
	}

    // Setting socket non-blocking
#ifdef WIN32
    if (WSAIoctl(qaul_UDP_socket, FIONBIO, &On, sizeof(On), NULL, 0, &Len, NULL, NULL) < 0) {
		fprintf(stderr, "Error while making socket non-blocking!\n");
		exit(1);
    }
#else
    if ((flags = fcntl(qaul_UDP_socket, F_GETFL, 0)) < 0) {
		fprintf(stderr, "Error getting socket flags!\n");
		exit(1);
    }

    if (fcntl(qaul_UDP_socket, F_SETFL, flags | O_NONBLOCK) < 0) {
		fprintf(stderr, "Error setting socket flags!\n");
		exit(1);
    }
#endif

	printf("UDP server started\n");
	qaul_UDP_started = 1;

	return 1;
}

// ------------------------------------------------------------
void Qaullib_UDP_SendFileavailableMsg(struct qaul_fileavailable_msg *msg, union olsr_ip_addr *ip)
{
	struct sockaddr_in destAddr;
	int status;

	destAddr.sin_family = AF_INET;
	destAddr.sin_port = htons(UDP_PORT);
	// todo: ipv6
	destAddr.sin_addr.s_addr = ip->v4.s_addr;

	if(QAUL_DEBUG)
	{
		char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
		inet_ntop(destAddr.sin_family, &destAddr.sin_addr, (char *)&ipbuf, MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN));
		printf("Qaullib_UDP_SendFileavailableMsg to: %s\n", ipbuf);

		inet_ntop(AF_INET, &ip->v4, (char *)&ipbuf, MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN));
		printf("Qaullib_UDP_SendFileavailableMsg 2 to: %s\n", ipbuf);
	}

	status = sendto(
					qaul_UDP_socket,
					(char *)msg,
					sizeof(struct qaul_fileavailable_msg),
					0,
					(struct sockaddr *)&destAddr,
					sizeof(destAddr)
					);

	if(status < 0)
		printf("Qaullib_UDP_SendFileavailableMsg Error sending file available message: %i\n", status);
}

// ------------------------------------------------------------
void Qaullib_UDP_SendExeavailableMsg(struct qaul_exeavailable_msg *msg, union olsr_ip_addr *ip)
{
	struct sockaddr_in destAddr;
	int status;

	destAddr.sin_family = AF_INET;
	destAddr.sin_port = htons(UDP_PORT);
	// todo: ipv6
	destAddr.sin_addr.s_addr = ip->v4.s_addr;

	if(QAUL_DEBUG)
		printf("Qaullib_UDP_SendExeavailableMsg \n");

	status = sendto(
					qaul_UDP_socket,
					(char *)msg,
					sizeof(struct qaul_exeavailable_msg),
					0,
					(struct sockaddr *)&destAddr,
					sizeof(destAddr)
					);

	if(status < 0)
		printf("Qaullib_UDP_SendExeavailableMsg Error sending file available message: %i\n", status);
}

// ------------------------------------------------------------
void Qaullib_UDP_CheckSocket(void)
{
	char buffer[1024];
	struct qaul_fileavailable_msg *fileavailable;
	struct qaul_exeavailable_msg *exeavailable;
	struct sockaddr_in sourceAddr;
	union olsr_ip_addr olsrSourceAddr;
	int received, i;
	uint8_t msgtype;
	socklen_t socklen;

	fileavailable = (struct qaul_fileavailable_msg *)buffer;
	exeavailable = (struct qaul_exeavailable_msg *)buffer;
	memset(&sourceAddr,0,sizeof(sourceAddr));

	if(qaul_UDP_started)
	{
		socklen = sizeof(struct sockaddr_in);
		received = 1;
		while(received > 0)
		{
			received = recvfrom(
								qaul_UDP_socket,
								buffer,
								1024,
								0,
								(struct sockaddr *)&sourceAddr,
								&socklen
								);

			if(received > 0)
			{
				if(QAUL_DEBUG)
					printf("Qaullib_UDP_CheckSocket message received\n");

				// check which message we received
				uint16_t msgtype = ntohs(fileavailable->msgtype);

				if(msgtype == QAUL_FILEAVAILABLE_MESSAGE_TYPE && received >= sizeof(struct qaul_fileavailable_msg))
				{
					if(QAUL_DEBUG)
						printf("QAUL_FILEAVAILABLE_MESSAGE_TYPE received\n");

					// todo: ipv6
					memcpy(&olsrSourceAddr.v4.s_addr, &sourceAddr.sin_addr.s_addr, sizeof(olsrSourceAddr.v4.s_addr));
					// add discovery to LL
					Qaullib_Filediscovery_LL_DiscoveryMsgProcessing(fileavailable, &olsrSourceAddr);
				}
				else if(msgtype == QAUL_EXEAVAILABLE_MESSAGE_TYPE && received >= sizeof(struct qaul_exeavailable_msg))
				{
					if(QAUL_DEBUG)
						printf("QAUL_EXEAVAILABLE_MESSAGE_TYPE received\n");

					Qaullib_ExeProcessAvailableMsg(exeavailable);
				}
				else
				{
					if(QAUL_DEBUG)
						printf("Qaullib_UDP_CheckSocket unknown message type: %i\n", msgtype);
				}
			}
		}
	}
}

