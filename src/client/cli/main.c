
#include <qaul/qaul.h>
int main() {}


///*
// * qaul.net is free software
// * licensed under GPL (version 3)
// */
//
//#include <stdio.h> // defines FILENAME_MAX
//#include <stdlib.h>
//#include <unistd.h>
//#define GetCurrentDir getcwd
//
//// #include "qaullib.h"
//#include <QaulConfig.h>
//
//// ------------------------------------------------------------
//// int main(int argc, char *argv[])
//// {
//// 	char cCurrentPath[FILENAME_MAX];
////         char prefix[FILENAME_MAX];
//
//// 	if (!GetCurrentDir(cCurrentPath, sizeof(cCurrentPath)))
//// 	{
//// 		fprintf(stderr,"ERROR: couldn't get directory\n");
//// 		return EXIT_FAILURE;
//// 	}
//// 	cCurrentPath[sizeof(cCurrentPath) - 1] = '\0';
//// 	fprintf(stderr,"The current working directory is %s\n", cCurrentPath);
//
////         snprintf(prefix, FILENAME_MAX, "%s/lib/qaul", QAUL_ROOT_PATH);
//
//// 	Qaullib_Init(cCurrentPath, prefix);
//// 	// enable debug menu
//// 	qaul_conf_debug = 1;
//
//// 	if(!Qaullib_WebserverStart())
//// 		fprintf(stderr,"Webserver startup failed\n");
//// 	Qaullib_ConfigStart();
//
//// 	fprintf(stderr,"----------------------------------------------------\n");
//// 	fprintf(stderr," config started\n");
//// 	fprintf(stderr,"----------------------------------------------------\n");
//// 	// The invoking of Qaullib_GetIP() is mandatory to load the IP.
//// 	fprintf(stderr,"IP: %s\n\n", Qaullib_GetIP());
//
//// 	// wait until user name is set
//// 	int username_flag = 0;
//// 	while(Qaullib_ExistsUsername() == 0)
//// 	{
//// 		if(username_flag == 0)
//// 		{
//// 			username_flag = 1;
//// 			fprintf(stderr,"Waiting until a username is set ...\n");
//// 			fprintf(stderr,"Open http://localhost:8081/qaul.html in your web browser to set it ...\n\n");
//// 		}
//// 		sleep(1);
//// 	}
//// 	fprintf(stderr,"Username successfully set!\n");
//
//// 	if(!Qaullib_IpcConnect())
//// 		fprintf(stderr,"Ipc connection failed!\n");
//// 	Qaullib_SetConfVoIP();
//// 	if(!Qaullib_UDP_StartServer())
//// 		fprintf(stderr,"UDP server failed!\n");
//// 	if(!Qaullib_CaptiveStart())
//// 		fprintf(stderr,"Captive portal failed!\n");
//// 	Qaullib_ConfigurationFinished();
//
//// 	// test config
//// 	fprintf(stderr,"Your IP: %s\n", Qaullib_GetIP());
//// 	fprintf(stderr,"Qaul started\n");
//
//// 	// loop variables
//// 	int socketCounter = 0;
//// 	int ipcCounter = 0;
//
//// 	fprintf(stderr,"Kill app to exit!\n");
//
//// 	// main loop
//// 	while (1) {
//// 		usleep(10000);
//
//// 		// get event
//// 		int event = Qaullib_TimedCheckAppEvent();
//// 		if(event == QAUL_EVENT_QUIT)
//// 			fprintf(stderr,"quit app\n");
//// 		else if(event == QAUL_EVENT_CHOOSEFILE)
//// 			fprintf(stderr,"open file chooser\n");
//// 		else if(event == QAUL_EVENT_OPENFILE)
//// 			fprintf(stderr,"open file\n");
//
//// 		// check sockets
//// 		if(socketCounter >= 10)
//// 		{
//// 			Qaullib_TimedSocketReceive();
//// 			socketCounter = 0;
//// 		}
//// 		else
//// 			socketCounter++;
//
//// 		// get network node IPs
//// 		// schedule downloads
//// 		if(ipcCounter >= 500)
//// 		{
//// 			Qaullib_IpcSendCom(1);
//// 			Qaullib_TimedDownload();
//// 			ipcCounter = 0;
//// 		}
//// 		else
//// 			ipcCounter++;
//// 	}
//// }
