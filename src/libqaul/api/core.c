/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <qaul/qaul.h>


ql_error_t first_startup();
ql_error_t init_library();
ql_error_t update_app();
ql_error_t start_gui();
ql_error_t start_network();



ql_error_t ql_initialise(struct qaul **state, enum qaul_os os, const char *home, const char *resources)
{
	first_startup();
	init_library();
	update_app();
	start_gui();
	start_network();
}


ql_error_t ql_shutdown(struct qaul *state)
{
	// exit library
	Qaullib_Exit();

	// stop networking
	// TODO: this needs to be done in the client
	// TODO: port forwarding only works with administrative access.

}



ql_error_t first_startup()
{
	// first startup
	// TODO: check if files need to be copied
}

ql_error_t init_library()
{
	// initialize qaullib
	Qaullib_Init();
/*
	//platform specific initializations
	Qaullib_SetConf();
	Qaullib_SetConfDownloadFolder();
	Qaullib_GetConfInt();
	Qaullib_GetConfString();
*/
}

ql_error_t update_app();
{
	// update
	// TODO: check if qaul.net app needs to updated
}

ql_error_t start_gui()
{
	// invoke configuration functions
	// startup configuration (0):
	// start web server
	Qaullib_WebserverStart();

	// load GUI in window
	// TODO: define
}

ql_error_t start_network()
{
/*
	// startup configuration (10):
	// check if you have sufficient authorization rights
	// request them if not

	// startup configuration (20):
	// check if wifi is configured manually
	Qaullib_GetConfInt("net.interface.manual");
	Qaullib_GetConfString("net.interface.name", config_interface_c);

	// check if wifi interface is available
	// start wifi interface
	// configure address
	// connect to qaul.net

	// startup configuration (30):
	Qaullib_ConfigStart();

	// check if user name was set
	// wait until user name has been set
	Qaullib_ExistsUsername();

	// startup configuration (40):
	// start olsrd routing

	// startup configuration (45):
	// connect ipc
	Qaullib_IpcConnect()

	// startup configuration (50):
	// start voip
	Qaullib_SetConfVoIP();

	// start UDP server
	Qaullib_UDP_StartServer();

	// start captive portal
	Qaullib_CaptiveStart();

	// start port forwarding
	// start timers to continuously invoke
	Qaullib_TimedCheckAppEvent();
	Qaullib_TimedSocketReceive();
	Qaullib_TimedDownload();

	// continuously update network nodes:
	Qaullib_IpcSendCom();

	// tell qaullib that configuration is finished
	Qaullib_ConfigurationFinished();
*/
}

