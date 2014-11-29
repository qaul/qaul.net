/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "../qaullib_private.h"
#include "qaullib_captive.h"


// ---------------------------------------------------------------------
pthread_t qaul_dhcp_thread;
pthread_t qaul_dns_thread;
int qaul_captive_running = 0;
char qaul_ip_captive[4];

// ---------------------------------------------------------------------

void *Qaullib_DHCP_Server(void *server_ip);
void *Qaullib_DNS_Server(void *server_ip);

// ---------------------------------------------------------------------
int Qaullib_CaptiveStart(void)
{
	struct sockaddr_in sa;

	if(QAUL_DEBUG)
		printf("Qaullib_CaptiveStart\n");

	if(qaul_captive_running == 0 && qaul_exe_available == 1)
	{
		if(QAUL_DEBUG)
			printf("start captive portal\n");

		//inet_pton(AF_INET, qaul_ip_str, &(sa.sin_addr));
		//memcpy( &qaul_ip_captive, &(sa.sin_addr.s_addr), 4);
		memcpy( &qaul_ip_captive, &(qaul_ip_addr.v4.s_addr), 4);

		qaullib_pthread_start((qaullib_thread_func_t) Qaullib_DHCP_Server, &qaul_ip_captive);
		qaullib_pthread_start((qaullib_thread_func_t) Qaullib_DNS_Server, &qaul_ip_captive);

		qaul_captive_running = 1;
	}

	return qaul_captive_running;
}

// ---------------------------------------------------------------------
void Qaullib_Captive_CreateIP(char *ip)
{
	srand(time(NULL));
	ip[0] = 10;
	ip[1] = rand()%255;
	ip[2] = rand()%255;
	ip[3] = (rand()%254)+1;
}

// ---------------------------------------------------------------------
int Qaullib_Captive_IpExists(char *ip)
{
	int exists;
	// create ip
	union olsr_ip_addr myip;
	memcpy(&myip.v4.s_addr, ip, sizeof(struct in_addr));


	// lock DB
	pthread_mutex_lock( &qaullib_mutex_userLL );
	// check for ip
	exists = Qaullib_User_LL_IpExists(&myip);
	// unlock DB
	pthread_mutex_unlock( &qaullib_mutex_userLL );

	return exists;
}



