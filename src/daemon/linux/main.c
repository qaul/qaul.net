/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#include <unistd.h>
#include <getopt.h>
#define GetCurrentDir getcwd

#include "qaullib.h"
#include <QaulConfig.h>
#include "../../client/gtk/qaul_configure.h"
#include "cli_options.h"

// ------------------------------------------------------------
int main(int argc, char *argv[])
{
	char cCurrentPath[FILENAME_MAX];
        char prefix[FILENAME_MAX];

	if (!GetCurrentDir(cCurrentPath, sizeof(cCurrentPath)))
	{
		fprintf(stderr,"ERROR: couldn't get directory\n");
		return EXIT_FAILURE;
	}
	cCurrentPath[sizeof(cCurrentPath) - 1] = '\0';
	fprintf(stderr,"The current working directory is %s\n", cCurrentPath);

        snprintf(prefix, FILENAME_MAX, "%s/lib/qaul", QAUL_ROOT_PATH);

	Qaullib_Init(cCurrentPath, prefix);

	// parse cli options
	qaul_cli_options(argc, argv);

	// set configuration
	Qaullib_SetConf(QAUL_CONF_INTERFACE);
	Qaullib_SetConf(QAUL_CONF_INTERNET);
	Qaullib_SetConf(QAUL_CONF_NETWORK);
	Qaullib_SetConf(QAUL_CONF_QUIT);

	// enable debug menu
	qaul_conf_debug = 1;

	if(!Qaullib_WebserverStart())
		fprintf(stderr,"Webserver startup failed\n");

	// run configuration loop
	qaulConfigureCounter = 0;
	while(qaul_configure())
	{
		fprintf(stderr,"qaulConfigureCounter: %i\n", qaulConfigureCounter);
		usleep(500000);
	}

	// inform user
	fprintf(stderr,"\n");
	fprintf(stderr,"----------------------------------------------------------------------\n");
	fprintf(stderr,"qaul daemon has started\n");
	fprintf(stderr,"Your IP: %s\n", Qaullib_GetIP());
	fprintf(stderr,"Kill app to exit!\n");
	fprintf(stderr,"Open http://localhost:8081/qaul.html in your web browser to use it ...\n");
	fprintf(stderr,"----------------------------------------------------------------------\n\n");

	// loop variables
	int socketCounter = 0;
	int ipcCounter = 0;
	// main loop
	while (1) {
		usleep(10000);

		// get event
		int event = Qaullib_TimedCheckAppEvent();
		if(event == QAUL_EVENT_QUIT)
			fprintf(stderr,"quit app\n");
		else if(event == QAUL_EVENT_CHOOSEFILE)
			fprintf(stderr,"open file chooser\n");
		else if(event == QAUL_EVENT_OPENFILE)
			fprintf(stderr,"open file\n");

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

void qaul_startTimers(void)
{
	// don't do anything here
}
