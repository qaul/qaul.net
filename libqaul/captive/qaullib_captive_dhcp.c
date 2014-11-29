/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * Minimal DHCP Implementation
 */

#include "../qaullib_private.h"
#include "qaullib_captive.h"

// --------------------------------------------------------------------------

#define DHCPDISCOVER                    1
#define DHCPOFFER                       2
#define DHCPREQUEST                     3
#define DHCPACK                         5

#define DHCP_MAX_LL                   300
#define DHCP_LEASE_TIME              1200

#ifdef WIN32
#define DHCP_PORT                      67
#else
#define DHCP_PORT                    8067
#endif

// --------------------------------------------------------------------------
struct qaul_dhcp_LL_item{
	struct qaul_dhcp_LL_item *next;
	struct qaul_dhcp_LL_item *prev;
	uint32_t          ip;      // ipv4 address
	char              mac[16]; // Hardware address
	time_t            time;    // time when last checked
};

struct qaul_dhcp_LL_item Qaul_dhcp_LL_root;
int qaul_dhcp_LL_count;

void Qaullib_Captive_Dhcp_LL_Init(void);
void Qaullib_Dhcp_LL_Add (char *ip, char *mac);
void Qaullib_Dhcp_LL_Delete_Item (struct qaul_dhcp_LL_item *item);
void Qaullib_Dhcp_LL_Clean (void);
int Qaullib_Dhcp_LL_Search_Ip (uint32_t ip, struct qaul_dhcp_LL_item **item);
int Qaullib_Dhcp_LL_Search_Mac (char *mac, struct qaul_dhcp_LL_item **item);
int Qaullib_Dhcp_LL_Search_IpMAC (uint32_t ip, char *mac, struct qaul_dhcp_LL_item **item);
int Qaullib_MAC_Compare (char *mac, struct qaul_dhcp_LL_item *item);
int Qaullib_DHCP_Options(char *options, char *ip, char *request_ip);


typedef struct
{
    uint8_t  dp_op;                           /* packet opcode type */
    uint8_t  dp_htype;                        /* hardware addr type */
    uint8_t  dp_hlen;                         /* hardware addr length */
    uint8_t  dp_hops;                         /* gateway hops */
    uint32_t dp_xid;                          /* transaction ID */
    uint16_t dp_secs;                         /* seconds since boot began */
    uint16_t dp_flags;
    uint8_t  dp_ciaddr[4];                    /* client IP address */
    uint8_t  dp_yiaddr[4];                    /* 'your' IP address */
    uint8_t  dp_siaddr[4];                    /* server IP address */
    uint8_t  dp_giaddr[4];                    /* gateway IP address */
    uint8_t  dp_chaddr[16];                   /* client hardware address */
    uint8_t  dp_legacy[192];
    uint8_t  dp_magic[4];
    uint8_t  dp_options[275];                 /* options area */
                                              /* as of RFC2131 it is variable length */
}DHCP_TYPE;


void *Qaullib_DHCP_Server(void *server_ip)
{
	//char buf[sizeof(DHCP_TYPE)];
	DHCP_TYPE DHCP_Buffer;
	struct sockaddr_in myAddr;
	struct sockaddr_in sourceAddr;
	struct sockaddr_in destinationAddr;
	int status, sendstatus, option, addr_valid, DHCPSocket;
	socklen_t sourcelen;
	struct sock_struct *sockptr;
	char * option_ptr;
	uint32_t leasetime = htonl(DHCP_LEASE_TIME);
	time_t last_checked = time(NULL);
	sourcelen = sizeof(struct sockaddr_in);

	DHCPSocket = -1;
	char NewIP[4] = {10,13,13,13};
	char New_subnet[] = {1,4,255,0,0,0};
	char New_offer[] = {53,1,DHCPOFFER};
	char New_ack[] = {53,1,DHCPACK};
	char magic_cookie[] = {0x63,0x82,0x53,0x63};
	char dhcp_option_end[] = {255}; // closes the option field
	char dhcp_option_leasetime[] = {51,4,0,0,0,0}; // lease time in seconds
	memcpy( &dhcp_option_leasetime[2], (char *)&leasetime, 4 );
	char dhcp_option_router[] = {3,4,10,0,0,1}; // set it to this devices address
	memcpy( &dhcp_option_router[2], server_ip, 4);
	char dhcp_option_nameserver[] = {6,4,10,0,0,1};  // set it to this devices address
	memcpy( &dhcp_option_nameserver[2], server_ip, 4);
	char dhcp_option_dhcpserver[] = {54,4,10,0,0,1};  // set it to this devices address
	memcpy( &dhcp_option_dhcpserver[2], server_ip, 4);
	char dhcp_option_broadcast[] = {28,4,10,255,255,255};  // set broadcast address


	memset(&myAddr,0,sizeof(myAddr));
	memset(&sourceAddr,0,sizeof(sourceAddr));
	memset(&destinationAddr,0,sizeof(destinationAddr));

	myAddr.sin_family = AF_INET;
	myAddr.sin_port = htons(DHCP_PORT); // 67 DHCP server port // 8067 for firewall redirection
	myAddr.sin_addr.s_addr = htonl(INADDR_ANY);
	//memcpy(&myAddr.sin_addr.s_addr, server_ip, 4);

	DHCPSocket = socket(PF_INET, SOCK_DGRAM, 0);

	if(DHCPSocket == INVALID_SOCKET)
		printf("unable to create DHCPSocket\n");

	status = bind(DHCPSocket, (struct sockaddr *)&myAddr, sizeof(myAddr));

	if(status < 0)
		perror("bind DHCPSocket error");

	// set socket options
	// set broadcast flag
	option = 1;
	sendstatus = setsockopt(DHCPSocket, SOL_SOCKET, SO_BROADCAST,  (const char*)&option, sizeof(option));
	// set reuse flag
	option = 1;
	sendstatus = setsockopt(DHCPSocket, SOL_SOCKET, SO_REUSEADDR,  (const char*)&option, sizeof(option));
	//// bind a socket to a device name (might not work on all systems):
	//optval2 = "eth1"; // 4 bytes long, so 4, below:
	//setsockopt(s2, SOL_SOCKET, SO_BINDTODEVICE, optval2, 4);

	// initialize linked list
	Qaullib_Captive_Dhcp_LL_Init();

	printf("DHCP server started\n");

	while ( 1 )
	{
		// remove unused dhcp entries from list
		if(last_checked < time(NULL) -60)
		{
			Qaullib_Dhcp_LL_Clean ();
			last_checked = time(NULL);
		}

		status = recvfrom( DHCPSocket, (char *)&DHCP_Buffer, sizeof( DHCP_Buffer ), 0 , (struct sockaddr *)&sourceAddr, &sourcelen);

		if ( status > 0 )
		{
			switch ( DHCP_Buffer.dp_options[2] )
			{
				case DHCPDISCOVER:
				   printf("DHCPDISCOVER, status = %i, sourcelen = %i\n", status, sourcelen);

				   printf("[DHCP] qaul_dhcp_LL_count %i, DHCP_MAX_LL %i\n", qaul_dhcp_LL_count, DHCP_MAX_LL);

				   if(qaul_dhcp_LL_count < DHCP_MAX_LL)
				   {
					   // create new ip
					   Qaullib_Captive_CreateIP((char *)&NewIP);
					   // write ip into linked-list
					   Qaullib_Dhcp_LL_Add ((char *)&NewIP, (char *)&DHCP_Buffer.dp_chaddr);

					   DHCP_Buffer.dp_op = DHCPOFFER;
					   memset( &DHCP_Buffer.dp_options, 0, sizeof( DHCP_Buffer.dp_options ));
					   memcpy( &DHCP_Buffer.dp_yiaddr, NewIP, 4 );
					   memcpy( &DHCP_Buffer.dp_magic, magic_cookie, 4 );
					   option_ptr = (char *)&DHCP_Buffer.dp_options;
					   memcpy( option_ptr, New_offer, 3 );
					   option_ptr += 3;
					   memcpy( option_ptr, New_subnet, 6 );
					   option_ptr += 6;
					   memcpy( option_ptr, dhcp_option_router, 6 );
					   option_ptr += 6;
					   memcpy( option_ptr, dhcp_option_nameserver, 6 );
					   option_ptr += 6;
					   memcpy( option_ptr, dhcp_option_broadcast, 6 );
					   option_ptr += 6;
					   memcpy( option_ptr, dhcp_option_dhcpserver, 6 );
					   option_ptr += 6;
					   // lease time: 5 min.
					   memcpy( option_ptr, dhcp_option_leasetime, 6 );
					   option_ptr += 6;
					   memcpy( option_ptr, dhcp_option_end, 1 );
					   option_ptr += 1;

					   destinationAddr.sin_port = htons(68);
					   destinationAddr.sin_family = AF_INET;

					   // 255.255.255.255 is not working as OSX is not broadcasting
					   // via wifi interface, we have to explicitly use 10.255.255.255
					   destinationAddr.sin_addr.s_addr = inet_addr("10.255.255.255");
					   //destinationAddr.sin_addr.s_addr = htonl(INADDR_BROADCAST);
					   sendstatus = sendto( DHCPSocket,
											(char *)&DHCP_Buffer,
											sizeof(DHCP_Buffer),
											0,
											(struct sockaddr *)&destinationAddr,
											sizeof(destinationAddr));

					   if(sendstatus < 0)
						   perror("sendto error");
				   }
				   break;

				case DHCPREQUEST:
					printf("DHCPREQUEST, status = %i, sourcelen = %i\n", status, sourcelen);

					uint32_t client_ip;
					if(Qaullib_DHCP_Options( (char *)&DHCP_Buffer.dp_options, server_ip, (char *)&client_ip) < 1)
						break;
					//else
					//	printf("option 54, address found\n");

					// check if address is in dhcp list
					struct qaul_dhcp_LL_item *ll_item;
					if(Qaullib_Dhcp_LL_Search_IpMAC (client_ip, (char *)&DHCP_Buffer.dp_chaddr, &ll_item))
					{
						ll_item->time = time(NULL);
					}
					else
					{
						//printf("ip address not found\n");
						break;
					}

					// check if address already exists
					if(Qaullib_Captive_IpExists((char *)&client_ip))
					{
						// propose new address if not valid
						Qaullib_Captive_CreateIP((char *)&client_ip);
					}

					DHCP_Buffer.dp_op = DHCPOFFER;
					memset( &DHCP_Buffer.dp_options, 0, sizeof( DHCP_Buffer.dp_options ));
					memcpy( &DHCP_Buffer.dp_yiaddr, NewIP, 4 );
					memcpy( DHCP_Buffer.dp_magic, magic_cookie, 4 );
					option_ptr = (char *)&DHCP_Buffer.dp_options;
					memcpy( option_ptr, New_ack, 3 );
					option_ptr += 3;
					memcpy( option_ptr, New_subnet, 6 );
					option_ptr += 6;
					memcpy( option_ptr, dhcp_option_router, 6 );
					option_ptr += 6;
					memcpy( option_ptr, dhcp_option_nameserver, 6 );
					option_ptr += 6;
					memcpy( option_ptr, dhcp_option_broadcast, 6 );
					option_ptr += 6;
					memcpy( option_ptr, dhcp_option_dhcpserver, 6 );
					option_ptr += 6;
					// lease time: 5 min.
					memcpy( option_ptr, dhcp_option_leasetime, 2 );
					memcpy( option_ptr+2, (char *)&leasetime, 4 );
					option_ptr += 6;
					memcpy( option_ptr, dhcp_option_end, 1 );
					option_ptr += 1;

					destinationAddr.sin_port = htons(68);
					destinationAddr.sin_family = AF_INET;

					// 255.255.255.255 is not working as OSX is not broadcasting
					// via wifi interface, we have to explicitly use 10.255.255.255
					destinationAddr.sin_addr.s_addr=inet_addr("10.255.255.255");
					//destinationAddr.sin_addr.s_addr = htonl(INADDR_BROADCAST);
					sendstatus = sendto( DHCPSocket,
										(char *)&DHCP_Buffer,
										sizeof(DHCP_Buffer),
										0,
										(struct sockaddr *)&destinationAddr,
										sizeof( destinationAddr ));

					if(sendstatus < 0)
						perror("sendto error");

					break;

				default:
				   printf("default\n");
				   break;
			}
		}
	}
}

// http://www.iana.org/assignments/bootp-dhcp-parameters/bootp-dhcp-parameters.xml
int Qaullib_DHCP_Options(char *options, char *ip, char *request_ip)
{
	uint8_t dhcp_end = 255;
	uint8_t dhcp_serverip = 54;
	uint8_t dhcp_requestip = 50;
	uint8_t dhcp_pad = 0;
	int myreturn = 0;

	int i;
	for(i=3; i<275; )
	{
		// stop on breakpoint
		if(strncmp(options +i, (char *)&dhcp_end, 1) == 0)
		{
			break;
		}
		// is option 54: server ip
		else if(strncmp(options +i, (char *)&dhcp_serverip, 1) ==0)
		{
			if(strncmp(options +i+2, ip, 4) != 0) return 0;
			i+=6;
			myreturn++;
		}
		// is option 50: requested ip
		else if(strncmp(options +i, (char *)&dhcp_requestip, 1) ==0)
		{
			memcpy( request_ip, options +i+2, 4);
			i+=6;
			myreturn++;
		}
		// small option
		else if(strncmp(options +i, (char *)&dhcp_pad, 1) ==0)
		{
			i++;
		}
		// normal option
		else
		{
			i += options[i+1] +2;
		}
#ifdef _WIN32
		Sleep(100);
#endif
	}
	return myreturn;
}



// ---------------------------------------------------------------------
void Qaullib_Captive_Dhcp_LL_Init(void)
{
	if(qaul_dhcp_LL_count > 0)
	{
		// TODO: empty list
	}
	else
	{
		qaul_dhcp_LL_count = 0;
		Qaul_dhcp_LL_root.next = &Qaul_dhcp_LL_root;
		Qaul_dhcp_LL_root.prev = &Qaul_dhcp_LL_root;
	}
}

// --------------------------------------------------------------------------

void Qaullib_Dhcp_LL_Add (char *ip, char *mac)
{
	struct qaul_dhcp_LL_item *new_item;
	new_item = (struct qaul_dhcp_LL_item *)malloc(sizeof(struct qaul_dhcp_LL_item));

	if(QAUL_DEBUG)
		printf("Qaullib_Dhcp_LL_Add\n");

	// fill in content
	new_item->time = time(NULL);
	memcpy((char *)&new_item->ip, ip, 4);
	memcpy(new_item->mac, mac, 16);

	// lock
	pthread_mutex_lock( &qaullib_mutex_DhcpLL );

	// create links
	new_item->prev = &Qaul_dhcp_LL_root;
	new_item->next = Qaul_dhcp_LL_root.next;
	Qaul_dhcp_LL_root.next = new_item;
	new_item->next->prev = new_item;

	// unlock
	pthread_mutex_unlock( &qaullib_mutex_DhcpLL );

	qaul_dhcp_LL_count++;
}

void Qaullib_Dhcp_LL_Delete_Item (struct qaul_dhcp_LL_item *item)
{
	if(QAUL_DEBUG)
		printf("Qaullib_Dhcp_LL_Delete_Item\n");

	item->prev->next = item->next;
	item->next->prev = item->prev;
	free(item);
	qaul_dhcp_LL_count--;
}

void Qaullib_Dhcp_LL_Clean (void)
{
	struct qaul_dhcp_LL_item *element;
	for(element = Qaul_dhcp_LL_root.next; element != &Qaul_dhcp_LL_root; element = element->next)
	{
		if(element->time < time(NULL) -60)
		{
			element = element->prev;
			Qaullib_Dhcp_LL_Delete_Item (element->next);
		}
	}
}

int Qaullib_Dhcp_LL_Search_Ip (uint32_t ip, struct qaul_dhcp_LL_item **item)
{
	struct qaul_dhcp_LL_item *element;
	for(element = Qaul_dhcp_LL_root.next; element != &Qaul_dhcp_LL_root; element = element->next)
	{
		if(element->ip == ip)
		{
			*item = element;
			return 1;
		}
	}
	return 0;
}

int Qaullib_Dhcp_LL_Search_IpMAC (uint32_t ip, char *mac, struct qaul_dhcp_LL_item **item)
{
	if(Qaullib_Dhcp_LL_Search_Ip (ip, item))
	{
		if(Qaullib_MAC_Compare (mac, *item))
			return 1;
		else
			printf("MAC not matched\n");
	}
	return 0;
}

int Qaullib_MAC_Compare (char *mac, struct qaul_dhcp_LL_item *item)
{
	if(strncmp(mac, item->mac, 16) == 0) return 1;
	return 0;
}

