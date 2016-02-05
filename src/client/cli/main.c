/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifdef WIN32
#include <windows.h>
#else
#endif

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#ifdef WINDOWS
    #include <direct.h>
    #define GetCurrentDir _getcwd
#else
    #include <unistd.h>
    #define GetCurrentDir getcwd
 #endif

#include "qaullib.h"
#include <QaulConfig.h>


// ------------------------------------------------------------
int main(int argc, char *argv[])
{
	char cCurrentPath[FILENAME_MAX];

	if (!GetCurrentDir(cCurrentPath, sizeof(cCurrentPath)))
	{
		printf ("ERROR: couldn't get directory\n");
		return EXIT_FAILURE;
	}
	cCurrentPath[sizeof(cCurrentPath) - 1] = '\0';
	printf ("The current working directory is %s\n", cCurrentPath);

	// TODO: set resource path in accordance with to platform
	Qaullib_Init(cCurrentPath, "/usr/local/lib/qaul");
	// enable debug menu
	qaul_conf_debug = 1;

	if(!Qaullib_WebserverStart())
		printf("Webserver startup failed\n");
	Qaullib_ConfigStart();

	printf("----------------------------------------------------\n");
	printf(" config started\n");
	printf("----------------------------------------------------\n");
	// The invoking of Qaullib_GetIP() is mandatory to load the IP.
	printf("IP: %s\n", Qaullib_GetIP());

	// wait until user name is set
	int username_flag = 0;
	while(Qaullib_ExistsUsername() == 0)
	{
		if(username_flag == 0)
		{
			username_flag = 1;
			printf("waiting until user name is set ...\n");
			printf("open web browser with http://localhost:8081/qaul.html to set it ...\n");
		}
		sleep(1);
	}
	printf("user name successfully set!\n");

	if(!Qaullib_IpcConnect())
		printf("Ipc connection failed\n");
	Qaullib_SetConfVoIP();
	if(!Qaullib_UDP_StartServer())
		printf("UDP server failed\n");
	if(!Qaullib_CaptiveStart())
		printf("Captive portal failed\n");
	Qaullib_ConfigurationFinished();

	// test config
	printf("IP: %s\n", Qaullib_GetIP());
	printf("Qaul started\n");

	// loop variables
	int socketCounter = 0;
	int ipcCounter = 0;

	printf("kill app to exit!\n");

	// main loop
	while (1) {
		usleep(10000);

		// get event
		int event = Qaullib_TimedCheckAppEvent();
		if(event == QAUL_EVENT_QUIT)
			printf("quit app\n");
		else if(event == QAUL_EVENT_CHOOSEFILE)
			printf("open file chooser\n");
		else if(event == QAUL_EVENT_OPENFILE)
			printf("open file\n");

		// check sockets
		if(socketCounter >= 10)
		{
			Qaullib_TimedSocketReceive();
			socketCounter = 0;
		}
		else
			socketCounter++;

		// get network node IPs
		// schedule downloads
		if(ipcCounter >= 500)
		{
			Qaullib_IpcSendCom(1);
			Qaullib_TimedDownload();
			ipcCounter = 0;
		}
		else
			ipcCounter++;
	}
}
