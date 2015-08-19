/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib.h"
#include "qaullib_private.h"
#include "urlcode/urlcode.h"

#include <math.h>

// ------------------------------------------------------------
// static declarations
// ------------------------------------------------------------
static void Qaullib_WwwSetName(struct mg_connection *conn);
static void Qaullib_WwwSetLocale(struct mg_connection *conn);
static void Qaullib_WwwGetName(struct mg_connection *conn);

/**
 * request interface configuration
 */
static void Qaullib_WwwConfigInterfaceLoading(struct mg_connection *conn);

/**
 * load network interface configuration
 */
static void Qaullib_WwwConfigInterfaceGet(struct mg_connection *conn);

/**
 * save network interface configuration
 */
static void Qaullib_WwwConfigInterfaceSet(struct mg_connection *conn);

/**
 * request interface configuration for internet sharing
 */
static void Qaullib_WwwConfigInternetLoading(struct mg_connection *conn);

/**
 * load Internet configuration
 */
static void Qaullib_WwwConfigInternetGet(struct mg_connection *conn);

/**
 * save Internet configuration
 */
static void Qaullib_WwwConfigInternetSet(struct mg_connection *conn);


/**
 * load active network configuration
 */
static void Qaullib_WwwConfigNetworkGet(struct mg_connection *conn);

/**
 * load profile network configuration
 */
static void Qaullib_WwwConfigNetworkGetProfile(struct mg_connection *conn);

/**
 * save network configuration
 */
static void Qaullib_WwwConfigNetworkSet(struct mg_connection *conn);


/**
 * load file sharing configuration
 */
static void Qaullib_WwwConfigFilesGet(struct mg_connection *conn);

/**
 * save file sharing configuration
 */
static void Qaullib_WwwConfigFilesSet(struct mg_connection *conn);


/**
 * load network topology
 */
static void Qaullib_WwwGetTopology(struct mg_connection *conn);

/**
 * Quit program
 */
static void Qaullib_WwwQuit(struct mg_connection *conn);

/**
 * process call handling
 */
static void Qaullib_WwwCallStart(struct mg_connection *conn);
static void Qaullib_WwwCallEnd(struct mg_connection *conn);
static void Qaullib_WwwCallAccept(struct mg_connection *conn);
static void Qaullib_WwwCallEvent(struct mg_connection *conn);
static void Qaullib_WwwFavoriteGet(struct mg_connection *conn);
static void Qaullib_WwwFavoriteAdd(struct mg_connection *conn);
static void Qaullib_WwwFavoriteDelete(struct mg_connection *conn);
static void Qaullib_WwwSetPageName(struct mg_connection *conn);
static void Qaullib_WwwSetOpenUrl(struct mg_connection *conn);
static void Qaullib_WwwSetWifiSet(struct mg_connection *conn);
static void Qaullib_WwwGetConfig(struct mg_connection *conn);

/**
 * get messages
 * different type searches
 * type: 1 (send all new messages)
 * type: 5 (search for users)
 * type: 6 (search for this )
 */
static void Qaullib_WwwGetMsgs(struct mg_connection *conn);
static void Qaullib_WwwSendMsg(struct mg_connection *conn);
static void Qaullib_WwwGetUsers(struct mg_connection *conn);
static void Qaullib_WwwGetEvents(struct mg_connection *conn);
static void Qaullib_WwwFileList(struct mg_connection *conn);
static void Qaullib_WwwFileAdd(struct mg_connection *conn);
static void Qaullib_WwwFilePick(struct mg_connection *conn);
static void Qaullib_WwwFilePickCheck(struct mg_connection *conn);
static void Qaullib_WwwFileOpen(struct mg_connection *conn);
static void Qaullib_WwwFileDelete(struct mg_connection *conn);
static void Qaullib_WwwFileSchedule(struct mg_connection *conn);
static void Qaullib_WwwPubUsers(struct mg_connection *conn);
static void Qaullib_WwwPubFilechunk(struct mg_connection *conn);
static void Qaullib_WwwPubMsg(struct mg_connection *conn);
static void Qaullib_WwwPubInfo(struct mg_connection *conn);
static void Qaullib_WwwWebGetMsgs(struct mg_connection *conn);
static void Qaullib_WwwWebSendMsg(struct mg_connection *conn);
static void Qaullib_WwwWebGetUsers(struct mg_connection *conn);
static void Qaullib_WwwWebGetFiles(struct mg_connection *conn);
static void Qaullib_WwwExtBinaries(struct mg_connection *conn);
static void Qaullib_WwwLoading(struct mg_connection *conn);
static int  Qaullib_WwwGetMsgsCallback(void *NotUsed, int number_of_lines, char **column_value, char **column_name);
static void Qaullib_WwwFile2Json(struct mg_connection *conn, struct qaul_file_LL_item *file);


// ------------------------------------------------------------
// web server polling thread
// ------------------------------------------------------------
void *Qaullib_Www_Server(void *webserver_instance)
{
	while(webserver_instance != NULL)
	{
		mg_poll_server((struct mg_server *)webserver_instance, 100);
	}
}


// ------------------------------------------------------------
// web server functions
// ------------------------------------------------------------
// event handler
char qaul_web_localip[MAX_IP_LEN +1];
int Qaullib_WwwEvent_handler(struct mg_connection *conn, enum mg_event event)
{
	char requestaddr[MAX_IP_LEN +1];
	int processed;

	processed = 0;

	if (event == MG_REQUEST)
	{
		if(qaul_web_localip_set == 0)
		{
			printf("qaul_web_localip conn->remote_ip %s\n", conn->remote_ip);
			strncpy(qaul_web_localip, conn->remote_ip, sizeof(qaul_web_localip));
			qaul_web_localip_set = 1;
		}

		// only locally accessible pages
		if(strncmp(qaul_web_localip, conn->remote_ip, sizeof(qaul_web_localip)) == 0)
		{
			// local jqm gui
			if (strcmp(conn->uri, "/getmsgs.json") == 0)
			{
				Qaullib_WwwGetMsgs(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/getevents.json") == 0)
			{
				Qaullib_WwwGetEvents(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/getusers.json") == 0)
			{
				Qaullib_WwwGetUsers(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/sendmsg") == 0)
			{
				Qaullib_WwwSendMsg(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/getname.json") == 0)
			{
				Qaullib_WwwGetName(conn);
				processed = 1;
			}
			// call handling
			else if (strcmp(conn->uri, "/call_event") == 0)
			{
				Qaullib_WwwCallEvent(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/call_start") == 0)
			{
				Qaullib_WwwCallStart(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/call_end") == 0)
			{
				Qaullib_WwwCallEnd(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/call_accept") == 0)
			{
				Qaullib_WwwCallAccept(conn);
				processed = 1;
			}
			// file handling
			else if (strcmp(conn->uri, "/file_list.json") == 0)
			{
				Qaullib_WwwFileList(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_add.json") == 0)
			{
				Qaullib_WwwFileAdd(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_pick.json") == 0)
			{
				Qaullib_WwwFilePick(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_pickcheck.json") == 0)
			{
				Qaullib_WwwFilePickCheck(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_open.json") == 0)
			{
				Qaullib_WwwFileOpen(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_delete.json") == 0)
			{
				Qaullib_WwwFileDelete(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/file_schedule.json") == 0)
			{
				Qaullib_WwwFileSchedule(conn);
				processed = 1;
			}
			// user favorites
			else if (strcmp(conn->uri, "/fav_get.json") == 0)
			{
				Qaullib_WwwFavoriteGet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/fav_add.json") == 0)
			{
				Qaullib_WwwFavoriteAdd(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/fav_del.json") == 0)
			{
				Qaullib_WwwFavoriteDelete(conn);
				processed = 1;
			}
			// configuration
			else if (strcmp(conn->uri, "/getconfig.json") == 0)
			{
				Qaullib_WwwGetConfig(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/setlocale") == 0)
			{
				Qaullib_WwwSetLocale(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/setname") == 0)
			{
				Qaullib_WwwSetName(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/setpagename") == 0)
			{
				Qaullib_WwwSetPageName(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/setopenurl.json") == 0)
			{
				Qaullib_WwwSetOpenUrl(conn);
				processed = 1;
			}

			else if (strcmp(conn->uri, "/config_interface_loading") == 0)
			{
				Qaullib_WwwConfigInterfaceLoading(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_interface_get") == 0)
			{
				Qaullib_WwwConfigInterfaceGet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_interface_set") == 0)
			{
				Qaullib_WwwConfigInterfaceSet(conn);
				processed = 1;
			}

			else if (strcmp(conn->uri, "/config_internet_loading") == 0)
			{
				Qaullib_WwwConfigInternetLoading(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_internet_get") == 0)
			{
				Qaullib_WwwConfigInternetGet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_internet_set") == 0)
			{
				Qaullib_WwwConfigInternetSet(conn);
				processed = 1;
			}

			else if (strcmp(conn->uri, "/config_network_get") == 0)
			{
				Qaullib_WwwConfigNetworkGet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_network_profile") == 0)
			{
				Qaullib_WwwConfigNetworkGetProfile(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_network_set") == 0)
			{
				Qaullib_WwwConfigNetworkSet(conn);
				processed = 1;
			}

			else if (strcmp(conn->uri, "/config_files_get") == 0)
			{
				Qaullib_WwwConfigFilesGet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/config_files_set") == 0)
			{
				Qaullib_WwwConfigFilesSet(conn);
				processed = 1;
			}

			else if (strcmp(conn->uri, "/gettopology.json") == 0)
			{
				Qaullib_WwwGetTopology(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/set_wifiset.json") == 0)
			{
				Qaullib_WwwSetWifiSet(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/quit") == 0)
			{
				Qaullib_WwwQuit(conn);
				processed = 1;
			}
			// loading
			else if (strcmp(conn->uri, "/loading.json") == 0)
			{
				Qaullib_WwwLoading(conn);
				processed = 1;
			}
		}

		// publicly accessible pages
		if(processed == 0)
		{
			// other qaul users (pub = public)
			if (strcmp(conn->uri, "/pub_users") == 0)
			{
				Qaullib_WwwPubUsers(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/pub_msg") == 0)
			{
				Qaullib_WwwPubMsg(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/pub_info.json") == 0)
			{
				Qaullib_WwwPubInfo(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/pub_filechunk") == 0)
			{
				Qaullib_WwwPubFilechunk(conn);
				processed = 1;
			}
			// web interface to test qaul
			else if (strcmp(conn->uri, "/web_getmsgs") == 0)
			{
				Qaullib_WwwWebGetMsgs(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/web_sendmsg") == 0)
			{
				Qaullib_WwwWebSendMsg(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/web_getusers") == 0)
			{
				Qaullib_WwwWebGetUsers(conn);
				processed = 1;
			}
			else if (strcmp(conn->uri, "/web_getfiles") == 0)
			{
				Qaullib_WwwWebGetFiles(conn);
				processed = 1;
			}
		}

		// external access without qaul : qaul download & info pages (ext = external)
		if (strcmp(conn->uri, "/ext_binaries.json") == 0)
		{
			Qaullib_WwwExtBinaries(conn);
			processed = 1;
		}
	}
	else if(event == MG_AUTH)
	{
		processed = 1;
	}
	else if (event == MG_HTTP_ERROR)
	{
		if(qaul_web_localip_set == 1 &&
			strncmp(qaul_web_localip, conn->remote_ip, sizeof(qaul_web_localip)) == 0
			)
		{
			// do nothing
		}
		else
		{
			// redirect to splash page
			conn->status_code = 303;
		    mg_printf(conn, "HTTP/1.1 303 See Other\r\n"
		              "Location: %s\r\n\r\n", "http://start.qaul/");
		    mg_printf(conn, "<a href=\"%s\">qaul.net &gt;&gt;</a>", "http://start.qaul/");

			processed = 1;
		}
	}
	else
	{
		processed = 0;
	}

	if(processed == 1)
		return MG_TRUE;

	return MG_FALSE;
}

// ------------------------------------------------------------
static void Qaullib_WwwSetName(struct mg_connection *conn)
{
	char username[3*MAX_USER_LEN +1];
	char protected_username[MAX_USER_LEN +1];

	// Fetch user name
	mg_get_var(conn, "n", username, sizeof(username));
	printf("user name len: %i\n", (int)strlen(username));
	memcpy(&username[MAX_USER_LEN], "\0", 1);

	if(Qaullib_StringNameProtect(protected_username, username, sizeof(protected_username)) > 0)
	{
		printf("save user name len %i: ", (int)strlen(protected_username));
		printf("%s  \n", protected_username);
		Qaullib_SetUsername(protected_username);
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwSetLocale(struct mg_connection *conn)
{
	// Fetch locale
	mg_get_var(conn, "l", qaul_locale, sizeof(qaul_locale));

	printf("save locale: %s\n", qaul_locale);
	Qaullib_SetLocale(qaul_locale);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInterfaceLoading(struct mg_connection *conn)
{
	if(QAUL_DEBUG)
		printf("Qaullib_WwwSetInterfaceLoading\n");

	// request interface json from application
	if(qaul_interface_configuring == 0)
	{
		qaul_interface_configuring = 1;
		Qaullib_Appevent_LL_Add(QAUL_EVENT_GETINTERFACES);
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInterfaceGet(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// configuration method
	mg_printf_data(conn, "\"manual\":%i,", Qaullib_GetConfInt("net.interface.manual"));

	// selected interface
	mg_printf_data(conn, "\"selected\":\"%s\",", Qaullib_GetInterface());

	// interfaces
	mg_printf_data(conn, "\"interfaces\":[");
	mg_printf_data(conn, "%s", qaul_interface_json);
	mg_printf_data(conn, "]");

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInterfaceSet(struct mg_connection *conn)
{
	char local_manual[MAX_INTSTR_LEN +1];
	int mymanual;
	char local_interface[255 +1];

	// save interface method
	mg_get_var(conn, "m", local_manual, sizeof(local_manual));
	memcpy(&local_manual[MAX_INTSTR_LEN], "\0", 1);
	mymanual = atoi(local_manual);
	Qaullib_SetInterfaceManual(mymanual);

	// if method manual, extract interface
	if(mymanual)
	{
		mg_get_var(conn, "if", local_interface, sizeof(local_interface));
		Qaullib_DbSetConfigValue("net.interface.name", local_interface);

		if(QAUL_DEBUG)
			printf("set interface %s manually\n", local_interface);
	}
	else
	{
		if(QAUL_DEBUG)
			printf("set interface automatically set\n");
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInternetLoading(struct mg_connection *conn)
{
	if(QAUL_DEBUG)
		printf("Qaullib_WwwConfigInternetLoading\n");

	// request interface json from application
	if(qaul_interface_configuring == 0)
	{
		qaul_interface_configuring = 1;
		Qaullib_Appevent_LL_Add(QAUL_EVENT_GETINTERFACES);
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInternetGet(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// configuration method
	mg_printf_data(conn, "\"share\":%i,", Qaullib_GetConfInt("internet.share"));

	// interface used by qaul
	mg_printf_data(conn, "\"used\":\"%s\",", Qaullib_GetInterface());

	// selected interfaces
	Qaullib_GetConfString("internet.interface", qaul_internet_interface);
	mg_printf_data(conn, "\"selected\":\"%s\",", qaul_internet_interface);

	// interfaces
	mg_printf_data(conn, "\"interfaces\":[");
	mg_printf_data(conn, "%s", qaul_interface_json);
	mg_printf_data(conn, "]");

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigInternetSet(struct mg_connection *conn)
{
	char local_share[MAX_INTSTR_LEN +1];

	// save interface method
	mg_get_var(conn, "share", local_share, sizeof(local_share));
	memcpy(&local_share[MAX_INTSTR_LEN], "\0", 1);
	qaul_internet_share = atoi(local_share);
	Qaullib_DbSetConfigValueInt("internet.share", qaul_internet_share);

	// if method manual, extract interface
	if(qaul_internet_share)
	{
		mg_get_var(conn, "if", qaul_internet_interface, sizeof(qaul_internet_interface));
		memcpy(&qaul_internet_interface[255], "\0", 1);
		Qaullib_DbSetConfigValue("internet.interface", qaul_internet_interface);

		if(QAUL_DEBUG)
			printf("share Internet via %s\n", qaul_internet_interface);
	}
	else
	{
		if(QAUL_DEBUG)
			printf("don't share Internet\n");
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigNetworkGet(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// profile exists
	mg_printf_data(conn, "\"available\":1,");

	// network profile
	mg_printf_data(conn, "\"profile\":\"%s\",", Qaullib_GetNetProfile());

	mg_printf_data(conn, "\"ip\":\"%s\",", Qaullib_GetIP());
	mg_printf_data(conn, "\"mask\":\"%i\",", Qaullib_GetNetMask());
	mg_printf_data(conn, "\"broadcast\":\"%s\",", Qaullib_GetNetBroadcast());
	mg_printf_data(conn, "\"gateway\":\"%s\",", Qaullib_GetNetGateway());

	mg_printf_data(conn, "\"ns1\":\"%s\",", Qaullib_GetNetNs1());
	mg_printf_data(conn, "\"ns2\":\"%s\",", Qaullib_GetNetNs2());

	mg_printf_data(conn, "\"channel\":\"%i\",", Qaullib_GetWifiChannel());
	mg_printf_data(conn, "\"ssid\":\"%s\",", Qaullib_GetInterface());
	mg_printf_data(conn, "\"bssid\":\"%s\"", Qaullib_GetInterface());

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigNetworkGetProfile(struct mg_connection *conn)
{
	char local_profile[QAUL_MAX_PROFILE_LEN +1];
	char profile_dbprotected[2*QAUL_MAX_PROFILE_LEN +1];
	char key[512];
	char value[512];

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// get profile name
	mg_get_var(conn, "p", local_profile, sizeof(local_profile));
	Qaullib_StringDbProtect(profile_dbprotected, local_profile, sizeof(profile_dbprotected));
	sprintf(key, "%s.profile", profile_dbprotected);

	// check if profile exists
	if(Qaullib_DbGetConfigValueInt(key))
	{
		// profile exists
		mg_printf_data(conn, "\"available\":1,");

		// network profile
		mg_printf_data(conn, "\"profile\":\"%s\",", local_profile);

		sprintf(key, "%s.net.ip", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"ip\":\"%s\",", value);

		sprintf(key, "%s.net.mask", profile_dbprotected);
		mg_printf_data(conn, "\"mask\":\"%i\",", Qaullib_DbGetConfigValueInt(key));

		sprintf(key, "%s.net.broadcast", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"broadcast\":\"%s\",", value);

		sprintf(key, "%s.net.gateway", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"gateway\":\"%s\",", value);

		sprintf(key, "%s.net.ns1", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"ns1\":\"%s\",", value);

		sprintf(key, "%s.net.ns2", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"ns2\":\"%s\",", value);

		sprintf(key, "%s.wifi.channel", profile_dbprotected);
		mg_printf_data(conn, "\"channel\":\"%i\",", Qaullib_DbGetConfigValueInt(key));

		sprintf(key, "%s.wifi.ssid", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"ssid\":\"%s\",", value);

		sprintf(key, "%s.net.bssid", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf_data(conn, "\"bssid\":\"%s\"", value);
	}
	else
	{
		// profile does not exist
		mg_printf_data(conn, "\"available\":0");
	}

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigNetworkSet(struct mg_connection *conn)
{
	int value_int;
	char local_profile[QAUL_MAX_PROFILE_LEN +1], profile_dbprotected[2*QAUL_MAX_PROFILE_LEN +1];
	char key[512];
	char value[255 +1], value_dbprotected[2*sizeof(value)];

	// profile
	mg_get_var(conn, "profile", local_profile, sizeof(local_profile));
	Qaullib_StringDbProtect(profile_dbprotected, local_profile, sizeof(profile_dbprotected));
	sprintf(key, "%s.profile", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, 1);
	Qaullib_DbSetConfigValue("net.profile", profile_dbprotected);

	// ip
	mg_get_var(conn, "ip", value, sizeof(MAX_IP_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ip", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("ip", value_dbprotected);

	// mask
	mg_get_var(conn, "mask", value, sizeof(MAX_INTSTR_LEN +1));
	value_int = atoi(value);
	sprintf(key, "%s.net.mask", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, value_int);
	Qaullib_DbSetConfigValue("net.mask", value_dbprotected);

	// broadcast
	mg_get_var(conn, "broadcast", value, sizeof(MAX_IP_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.broadcast", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.broadcast", value_dbprotected);

	// gateway
	mg_get_var(conn, "gateway", value, sizeof(MAX_IP_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.gateway", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.gateway", value_dbprotected);

	// ns1 DNS server
	mg_get_var(conn, "ns1", value, sizeof(MAX_IP_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ns1", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.ns1", value_dbprotected);

	// ns2 DNS server
	mg_get_var(conn, "ns2", value, sizeof(MAX_IP_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ns2", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.ns2", value_dbprotected);

	// wifi channel
	mg_get_var(conn, "channel", value, sizeof(MAX_INTSTR_LEN +1));
	value_int = atoi(value);
	sprintf(key, "%s.wifi.channel", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, value_int);
	Qaullib_DbSetConfigValueInt("wifi.channel", value_int);

	// wifi ssid
	mg_get_var(conn, "ssid", value, sizeof(MAX_SSID_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.wifi.ssid", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("wifi.ssid", value_dbprotected);

	// wifi bssid
	mg_get_var(conn, "bssid", value, sizeof(MAX_BSSID_LEN +1));
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.wifi.bssid", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("wifi.bssid", value_dbprotected);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigFilesGet(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// configuration auto download of files
	mg_printf_data(conn, "\"autodownload\":%i,", Qaullib_GetConfInt("files.autodownload"));

	// configuration auto download of files
	mg_printf_data(conn, "\"space\":\"%i\",", Qaullib_GetConfInt("files.space.max"));

	// configuration auto download of files
	mg_printf_data(conn, "\"filesize\":\"%i\"", Qaullib_GetConfInt("files.filesize.max"));

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwConfigFilesSet(struct mg_connection *conn)
{
	char local_intstr[MAX_INTSTR_LEN +1];
	int mydownload, myspace, myfilesize;

	// save download method
	mg_get_var(conn, "download", local_intstr, sizeof(local_intstr));
	mydownload = atoi(local_intstr);
	Qaullib_DbSetConfigValueInt("files.autodownload", mydownload);
	qaul_file_autodownload = mydownload;

	// if auto download is set, set boundaries
	if(mydownload)
	{
		mg_get_var(conn, "space", local_intstr, sizeof(local_intstr));
		myspace = atoi(local_intstr);
		Qaullib_DbSetConfigValueInt("files.space.max", myspace);
		qaul_file_space_max = myspace;

		mg_get_var(conn, "size", local_intstr, sizeof(local_intstr));
		myfilesize = atoi(local_intstr);
		Qaullib_DbSetConfigValueInt("files.filesize.max", myfilesize);
		qaul_file_size_max = myfilesize;

		if(QAUL_DEBUG)
			printf("download advertised files automatically, max space %i, max file size %i\n", myspace, myfilesize);
	}
	else
	{
		if(QAUL_DEBUG)
			printf("download advertised files manually\n");
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwGetTopology(struct mg_connection *conn)
{
	int add, request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	char dest_ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	first = 0;

	if(QAUL_DEBUG)
		printf("Qaullib_WwwGetTopology\n");

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// check if topology has been received
	if(qaul_ipc_topo_request == 2)
	{
		// set availability
		mg_printf_data(conn, "\"available\":1,");
		// send topology
		mg_printf_data(conn, "\"links\":[");

		// send link list
		first = 0;
		while(qaul_topo_LL_first)
		{
			if(!first)
				first = 1;
			else
				mg_printf_data(conn, ",");

			// FIXME: ipv6
			mg_printf_data(conn,
					"{\"source\":\"%s\",\"target\":\"%s\",\"lq\":\"%d\"}",
					inet_ntop(AF_INET, &qaul_topo_LL_first->src_ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
					inet_ntop(AF_INET, &qaul_topo_LL_first->dest_ip.v4.s_addr, (char *)&dest_ipbuf, sizeof(dest_ipbuf)),
					qaul_topo_LL_first->lq
					);

			// delete this item
			Qaullib_Topo_LL_Delete_Item ();
		}
		mg_printf_data(conn, "],");

		// set topo to 0, as the list has been deleted
		qaul_ipc_topo_request = 0;

		// send all users
		mg_printf_data(conn, "\"nodes\":{");
		// print this user
		mg_printf_data(conn,
				"\"%s\":{\"name\":\"%s\",\"type\":\"name\"}",
				qaul_ip_str,
				qaul_username
				);
		// loop through LL
		first = 0;
		struct qaul_user_LL_node mynode;
		Qaullib_User_LL_InitNode(&mynode);
		while(Qaullib_User_LL_NextNode(&mynode))
		{
			// send all users
			if(mynode.item->type == QAUL_USERTYPE_KNOWN)
			{
				// make sure the user name is not empty
				if(strlen(mynode.item->name) > 0)
				{
					mg_printf_data(conn, ",");
					// FIXME: ipv6
					mg_printf_data(conn,
							"\"%s\":{\"name\":\"%s\",\"type\":\"name\"}",
							inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
							mynode.item->name
							);
				}
			}
		}
		mg_printf_data(conn, "}");
	}
	// otherwise request data and send a wait response
	else
	{
		// request data if not yet requested
		if(qaul_ipc_topo_request == 0)
		{
			Qaullib_IpcSendCom(QAUL_IPCCOM_GETMESHTOPO);
			qaul_ipc_topo_request = 1;
		}

		// wait until data is available
		mg_printf_data(conn, "\"available\":0");
	}
	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwSetPageName(struct mg_connection *conn)
{
	// Fetch screen name
	mg_get_var(conn, "p", qaullib_GuiPageName, sizeof(qaullib_GuiPageName));
	qaul_gui_pagename_set = 1;

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwSetOpenUrl(struct mg_connection *conn)
{
	// Fetch url
	mg_get_var(conn, "url", qaullib_AppEventOpenURL+22, sizeof(qaullib_AppEventOpenURL)-22);

	printf("set event url to open: %s\n", qaullib_AppEventOpenURL);
	Qaullib_Appevent_LL_Add(QAUL_EVENT_OPENURL);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwSetWifiSet(struct mg_connection *conn)
{
	if(QAUL_DEBUG)
		printf("Qaullib_WwwSetWifiSet\n");

	qaul_conf_wifi_set = 1;
	qaul_gui_pagename_set = 0;

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{\"ok\":1}");
}

// ------------------------------------------------------------
static void Qaullib_WwwQuit(struct mg_connection *conn)
{
	Qaullib_Appevent_LL_Add(QAUL_EVENT_QUIT);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwCallEvent(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{\"event\":%i", qaul_voip_event);
	if(qaul_voip_event == 5)
		mg_printf_data(conn, ",\"code\":%i", qaul_voip_event_code);
	mg_printf_data(conn, "}");

	// set ring
	if(qaul_voip_ringing > 0)
	{
		Qaullib_Appevent_LL_Add(QAUL_EVENT_RING);
	}

	qaul_voip_event = 0;
}

static void Qaullib_WwwCallStart(struct mg_connection *conn)
{
	char call_ip[MAX_IP_LEN +1];
	// extract variables
	mg_get_var(conn, "ip", call_ip, sizeof(call_ip));
	// call user
	Qaullib_VoipCallStart(call_ip);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

static void Qaullib_WwwCallEnd(struct mg_connection *conn)
{
	Qaullib_VoipCallEnd();

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

static void Qaullib_WwwCallAccept(struct mg_connection *conn)
{
	Qaullib_VoipCallAccept();

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFavoriteGet(struct mg_connection *conn)
{
	int first = 0;
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// print Users
	mg_printf_data(conn, "{\"favorites\":[");
	// loop LL and check for favorites
	first = 0;
	struct qaul_user_LL_node mynode;
	Qaullib_User_LL_InitNode(&mynode);
	while(Qaullib_User_LL_NextNode(&mynode))
	{
		// check if node is favorite
		if(mynode.item->favorite > 0)
		{
			if(!first)
				first = 1;
			else
				mg_printf_data(conn, ",");
			// FIXME: ipv6
			mg_printf_data(conn,
					"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\"}",
					mynode.item->name,
					inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
					mynode.item->idstr
					);
		}
	}
	mg_printf_data(conn, "]}");
}

static void Qaullib_WwwFavoriteAdd(struct mg_connection *conn)
{
	char myname[3*MAX_USER_LEN +1];
	char myipstr[MAX_IP_LEN +1];
	char myidstr[MAX_HASHSTR_LEN +1];

	// extract variable
	mg_get_var(conn, "ip", myipstr, sizeof(myipstr));
	mg_get_var(conn, "name", myname, sizeof(myname));
	mg_get_var(conn, "id", myidstr, sizeof(myidstr));

	printf("add favorite %s \n", myname);
	Qaullib_UserFavoriteAdd(myname, myipstr, myidstr);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

static void Qaullib_WwwFavoriteDelete(struct mg_connection *conn)
{
	char myipstr[MAX_IP_LEN +1];
	char myidstr[MAX_HASHSTR_LEN +1];

	// extract variable
	mg_get_var(conn, "ip", myipstr, sizeof(myipstr));
	mg_get_var(conn, "id", myidstr, sizeof(myidstr));

	printf("delete favorite %s \n", myipstr);
	Qaullib_UserFavoriteRemove(myipstr, myidstr);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwGetConfig(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// send UI language
	if(Qaullib_ExistsLocale())
		mg_printf_data(conn, "\"locale\":\"%s\",", Qaullib_GetLocale());

	// send user name
	if(Qaullib_ExistsUsername())
		mg_printf_data(conn, "\"name\":\"%s\",", Qaullib_GetUsername());

	// send all the rest
	mg_printf_data(conn, "\"msg_max\":20,");
	if(qaul_conf_quit)
		mg_printf_data(conn, "\"c_quit\":true,");
	else
		mg_printf_data(conn, "\"c_quit\":false,");

	if(qaul_conf_debug)
		mg_printf_data(conn, "\"c_debug\":true,");
	else
		mg_printf_data(conn, "\"c_debug\":false,");

	if(qaul_conf_interface)
		mg_printf_data(conn, "\"c_interface\":true,");
	else
		mg_printf_data(conn, "\"c_interface\":false,");

	if(qaul_conf_internet)
		mg_printf_data(conn, "\"c_internet\":true,");
	else
		mg_printf_data(conn, "\"c_internet\":false,");

	if(qaul_conf_network)
		mg_printf_data(conn, "\"c_network\":true");
	else
		mg_printf_data(conn, "\"c_network\":false");

	mg_printf_data(conn, "}");
}


// ------------------------------------------------------------
static void Qaullib_WwwGetEvents(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	// get number of waiting incoming messages
	mg_printf_data(conn, "\"m_pub\":%i,", qaul_new_msg);
	mg_printf_data(conn, "\"m_priv\":%i,", qaul_new_msg);

	// get newly downloaded files
	mg_printf_data(conn, "\"files\":%i,",0);

	// check call events
	if(qaul_voip_new_call)
	{
		mg_printf_data(conn, "\"call\":%i",1);
		mg_printf_data(conn, ",\"callee\":\"%s\"",qaul_voip_call.name);
		qaul_voip_new_call = 0;
	}
	else
		mg_printf_data(conn, "\"call\":%i",0);

	mg_printf_data(conn, "}");
}


// ------------------------------------------------------------
static void Qaullib_WwwGetName(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{\"name\":\"%s\"}", qaul_username);
}

// ------------------------------------------------------------
static void Qaullib_WwwGetMsgs(struct mg_connection *conn)
{
	char buffer[10240];
	char* stmt = buffer;
	char local_type[MAX_INTSTR_LEN +1];
	char local_id[MAX_INTSTR_LEN +1];
	char local_tag[MAX_FILENAME_LEN +1];
	char local_name[MAX_USER_LEN +1];
	char timestr[MAX_TIME_LEN];
	int  id, type, count, items;
	struct qaul_msg_LL_node node;

	items = 0;

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{\"name\":\"%s\",\"messages\":[", qaul_username);

	// get variables
	// message type
	mg_get_var(conn, "t", local_type, sizeof(local_type));
	type = atoi(local_type);

	// get id
	mg_get_var(conn, "id", local_id, sizeof(local_id));
	id = atoi(local_id);

	// prepare statements
	// user related
	if(type == 5)
	{
		mg_get_var(conn, "v", local_name, sizeof(local_name));

		// prepare statement
		if(id == 0)
			sprintf(stmt, sql_msg_get_user0, local_name, "%", local_name, "%");
		else
			sprintf(stmt, sql_msg_get_user, id, local_name, "%", local_name, "%");

		items = Qaullib_MsgDB2LL(&node, stmt);
		if(items)
		{
			// set node to last item
			while(Qaullib_Msg_LL_NextItem(&node))
			{

			}
		}
	}
	// with tag
	else if(type == 6)
	{
		mg_get_var(conn, "v", local_tag, sizeof(local_tag));

		// prepare statement
		if(id == 0)
			sprintf(stmt, sql_msg_get_tag0, "%", local_tag, "%");
		else
			sprintf(stmt, sql_msg_get_tag, id, "%", local_tag, "%");

		items = Qaullib_MsgDB2LL(&node, stmt);
		if(items)
		{
			// set node to last item
			while(Qaullib_Msg_LL_NextItem(&node))
			{

			}
		}
	}
	else
	{
		items = Qaullib_Msg_LL_FirstItem (&node, id);
		qaul_new_msg = 0;
	}

	// loop through items
	if(items)
	{
		count = 0;
		do
		{
			if(count > 0)
				mg_printf_data(conn, "%s", ",");

			count++;

			mg_printf_data(conn, "{");

			mg_printf_data(conn, "\"id\":%i", node.item->id);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"type\":%i", node.item->type);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"name\":\"%s\"", node.item->name);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"msg\":\"%s\"", node.item->msg);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"ip\":\"%s\"", node.item->ip);
			mg_printf_data(conn, ",");
			Qaullib_Timestamp2Isostr(timestr, node.item->time, MAX_TIME_LEN);
			mg_printf_data(conn, "\"time\":\"%s\"", timestr);

			mg_printf_data(conn, "}");
		} while (Qaullib_Msg_LL_PrevItem(&node));
	}

	mg_printf_data(conn, "%s", "]}");
}

// ------------------------------------------------------------
static void Qaullib_WwwSendMsg(struct mg_connection *conn)
{
	char local_msg[3*MAX_MESSAGE_LEN +1];
	char local_name[3*MAX_USER_LEN +1];
	char local_type[7];
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;

	// get type
	mg_get_var(conn, "t", local_type, sizeof(local_type));
	msg_item.type = atoi(local_type);

	// get msg
	mg_get_var(conn, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(msg_item.msg, local_msg, sizeof(msg_item.msg));

	// get name
	mg_get_var(conn, "n", local_name, sizeof(local_name));
	Qaullib_StringNameProtect(msg_item.name, local_name, sizeof(msg_item.name));

	// set time
	time(&timestamp);
	msg_item.time = (int)timestamp;

	// set ip
	msg_item.ipv = 4;
	strncpy(msg_item.ip, qaul_ip_str, sizeof(msg_item.ip));
	memcpy(&msg_item.ip_union, &qaul_ip_addr, sizeof(msg_item.ip_union));

	// set read
	msg_item.read = 1;

	// send and save message
	if(msg_item.type == QAUL_MSGTYPE_PUBLIC_OUT)
		Qaullib_MsgSendPublic(&msg_item);
	else if(msg_item.type == QAUL_MSGTYPE_PRIVATE_OUT)
		Qaullib_MsgSendPrivate(&msg_item);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}


// ------------------------------------------------------------
static void Qaullib_WwwGetUsers(struct mg_connection *conn)
{
	int add, request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	first = 0;

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_var(conn, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// print newly
	mg_printf_data(conn, "{\"users\":[");
	// loop through LL
	first = 0;
	struct qaul_user_LL_node mynode;
	Qaullib_User_LL_InitNode(&mynode);
	while(Qaullib_User_LL_NextNode(&mynode))
	{
		// check if node was changed
		if(
				(mynode.item->type == QAUL_USERTYPE_KNOWN || mynode.item->type == QAUL_USERTYPE_WEB_KNOWN) &&
				(mynode.item->changed == QAUL_USERCHANGED_MODIFIED ||
				mynode.item->changed == QAUL_USERCHANGED_DELETED ||
				request_type > 0)
				)
		{
			// make sure the user name is not empty
			if(strlen(mynode.item->name) > 0)
			{
				if(!first)
					first = 1;
				else
					mg_printf_data(conn, ",");

				// FIXME: ipv6
				mg_printf_data(conn,
						"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\",\"lq\":%i,\"add\":%i}",
						mynode.item->name,
						inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
						mynode.item->idstr,
						Qaullib_UserLinkcost2Img(mynode.item->lq),
						mynode.item->changed
						);
			}

			if(mynode.item->changed == QAUL_USERCHANGED_DELETED)
			{
				if(request_type != 2)
					mynode.item->changed = QAUL_USERCHANGED_CACHED;
			}
			else
			{
				if(request_type != 2)
					mynode.item->changed = QAUL_USERCHANGED_UNCHANGED;
			}
		}
	}
	mg_printf_data(conn, "]}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFileList(struct mg_connection *conn)
{
	int firstitem, request_type;
	char request_type_char[MAX_INTSTR_LEN +1];
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNode(&mynode);

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_var(conn, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");
	mg_printf_data(conn, "\"files\":[");

	// loop through files
	firstitem = 1;
	if(request_type > 0)
	{
		// default behaviour (all nodes)
		while(Qaullib_File_LL_NextNode(&mynode))
		{
			if(firstitem)
				firstitem = 0;
			else
				mg_printf_data(conn, ",");

			Qaullib_WwwFile2Json(conn, mynode.item);
			if(request_type == 1)
				mynode.item->gui_notify = 0;
		}
	}
	else
	{
		// default behaviour (only updated nodes)
		while(Qaullib_File_LL_NextNodeGuiPriv(&mynode))
		{
			if(firstitem)
				firstitem = 0;
			else
				mg_printf_data(conn, ",");

			Qaullib_WwwFile2Json(conn, mynode.item);
			mynode.item->gui_notify = 0;
		}
	}


	mg_printf_data(conn, "]");
	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFileAdd(struct mg_connection *conn)
{
	char buffer[1024];
	char* stmt = buffer;
	char *error_exec=NULL;
	int advertise, size;
	time_t timestamp;
	struct qaul_file_LL_item file_item;
	struct qaul_file_LL_item *existing_file;
	char local_advertise[2];
	char local_path[MAX_PATH_LEN +1];
	char local_msg[MAX_MESSAGE_LEN +1];
	union olsr_message *m = (union olsr_message *)buffer;

	printf("Qaullib_WwwFileAdd\n");

	// get path
	mg_get_var(conn, "p", local_path, sizeof(local_path));
	// get msg
	mg_get_var(conn, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(file_item.description, local_msg, sizeof(file_item.description));
	// get advertise
	mg_get_var(conn, "a", local_advertise, sizeof(local_advertise));
	advertise = atoi(local_advertise);

	// copy file into directory & make hash
	file_item.size = Qaullib_FileCopyNew(local_path, &file_item);

	if(file_item.size > 0)
	{
		printf("Qaullib_WwwFileAdd hashstr: %s \n", file_item.hashstr);

		// add file
		file_item.type = QAUL_FILETYPE_FILE;
		file_item.status = QAUL_FILESTATUS_MYFILE;
		sprintf(file_item.adv_name, "%s", "");
		memset(&file_item.adv_ip, 0, sizeof(file_item.adv_ip));
		file_item.adv_validip = 0;
		file_item.downloaded = 0;
		file_item.downloaded_chunk = 0;
		time(&timestamp);
		file_item.created_at = (int)timestamp;

		// check if file already exists
		if(Qaullib_File_LL_HashSearch(file_item.hash, &existing_file))
		{
			if(existing_file->status == QAUL_FILESTATUS_DELETED)
			{
				// delete from LL
				Qaullib_File_LL_Delete_Item(existing_file);

				// add the file again
				Qaullib_FileAdd(&file_item);
			}
		}
		else
		{
			Qaullib_FileAdd(&file_item);
		}

		// FIXME: make ipv6 compatible
		// pack chat into olsr message
		if(advertise)
		{
			printf("send advertise message\n");

			// ipv4 only at the moment
			memset(&m->v4.originator, 0, sizeof(m->v4.originator));
			m->v4.olsr_msgtype = QAUL_CHAT_MESSAGE_TYPE;
			memcpy(&m->v4.message.chat.name, qaul_username, MAX_USER_LEN);
			// create message
			strncpy(local_msg, file_item.hashstr, sizeof(file_item.hashstr));
			if(strlen(file_item.suffix) > 0)
			{
				strcat(local_msg, ".");
				strcat(local_msg, file_item.suffix);
			}
			strcat(local_msg, " ");
			strcat(local_msg, file_item.description);

			memcpy(&m->v4.message.chat.msg, local_msg, MAX_MESSAGE_LEN);
			size = sizeof( struct qaul_chat_msg);
			size = size + sizeof(struct olsrmsg);
			m->v4.olsr_msgsize = htons(size);

			printf("olsr message: name: %s, msg: %s, size: %i, status: %i\n", qaul_username, local_msg, size, file_item.status);

			// send package
			Qaullib_IpcSend(m);
		}

		// todo: check whether sending was successful...
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");
	mg_printf_data(conn, "\"success\":%i", 1);
	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFilePick(struct mg_connection *conn)
{
	printf("Qaullib_WwwFilePick\n");

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// open file picker
	pickFileCheck = 1;
	Qaullib_Appevent_LL_Add(QAUL_EVENT_CHOOSEFILE);

	// deliver answer
	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFilePickCheck(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// deliver answer
	mg_printf_data(conn, "{\"picked\":%i", pickFileCheck);
	if(pickFileCheck == 2)
	{
		int backslash = '\\';
		if(strchr(pickFilePath, backslash))
		{
			mg_printf_data(conn, ",\"path\":\"");
			// protect backslashes
			int i;
			for(i=0; i<strlen(pickFilePath); i++)
			{
				if(pickFilePath[i] == backslash)
					mg_printf_data(conn, "\\");
				mg_printf_data(conn, "%c", pickFilePath[i]);
			}
			mg_printf_data(conn, "\",");
		}
		else
			mg_printf_data(conn, ",\"path\":\"%s\",", pickFilePath);
		char *local_file = strrchr(pickFilePath, PATH_SEPARATOR_INT);
		mg_printf_data(conn, "\"name\":\"%s\",", local_file+1);
		// FIXME: use correct size and date
		mg_printf_data(conn, "\"size\":1024,");
		mg_printf_data(conn, "\"create\":\"2012-02-02 19:59:42\"");
		pickFileCheck = 0;
	}

	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFileOpen(struct mg_connection *conn)
{
	char hashstr[MAX_HASHSTR_LEN +1];
	char hash[MAX_HASH_LEN];
	char old_path[MAX_PATH_LEN +1];
	struct qaul_file_LL_item *file_item;

	if(QAUL_DEBUG)
		printf("Qaullib_WwwFileOpen\n");

	// get file variable
	mg_get_var(conn, "f", hashstr, sizeof(hashstr));
	memcpy(&hashstr[MAX_HASHSTR_LEN], "\0", 1);

	// get file
	if(Qaullib_StringToHash(hashstr, hash))
	{
		// check if file is in file sharing
		if(
			Qaullib_File_LL_HashSearch(hash, &file_item) &&
			file_item->status >= QAUL_FILESTATUS_DOWNLOADED
			)
		{
			if(qaul_conf_filedownloadfolder_set)
			{
				if(QAUL_DEBUG)
					printf("qaul_conf_filedownloadfolder_set\n");

				// get path
				Qaullib_FileCreatePathToDownloadFolder(qaullib_AppEventOpenPath, file_item);

				if(QAUL_DEBUG)
					printf("path to download folder: %s\n", qaullib_AppEventOpenPath);

				// check if file exists
				if(Qaullib_FileExists(qaullib_AppEventOpenPath))
				{
					// open file
					Qaullib_Appevent_LL_Add(QAUL_EVENT_OPENFILE);
				}
				else
				{
					Qaullib_FileCreatePath(old_path, file_item->hashstr, file_item->suffix);

					if(Qaullib_FileCopy(old_path, qaullib_AppEventOpenPath))
					{
						// open file
						Qaullib_Appevent_LL_Add(QAUL_EVENT_OPENFILE);
					}
				}
			}
			else
			{
				Qaullib_FileCreatePath(qaullib_AppEventOpenPath, file_item->hashstr, file_item->suffix);
				// open file
				Qaullib_Appevent_LL_Add(QAUL_EVENT_OPENFILE);
			}
		}
	}

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}

// ------------------------------------------------------------
static void Qaullib_WwwFileDelete(struct mg_connection *conn)
{
	char local_hashstr[MAX_HASHSTR_LEN +1];
	unsigned char local_hash[MAX_HASH_LEN];
	struct qaul_file_LL_item *file_item;

	printf("Qaullib_WwwFileDelete\n");

	// get file variable
	mg_get_var(conn, "hash", local_hashstr, sizeof(local_hashstr));

	printf("hashstr %s\n", local_hashstr);
	// delete file
	// todo: delete file (by hash)
	if(Qaullib_StringToHash(local_hashstr, local_hash))
	{
		if(Qaullib_File_LL_HashSearch(local_hash, &file_item))
		{
			printf("3\n");
			Qaullib_FileDelete(file_item);
		}
	}
	printf("4\n");

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");

	printf("5\n");
}

// ------------------------------------------------------------
static void Qaullib_WwwFileSchedule(struct mg_connection *conn)
{
	char buffer[1024];
	char *stmt;
	char *error_exec;
	time_t timestamp;
	struct qaul_file_LL_item file_item;
	struct qaul_file_LL_item *existing_file;
	char local_size[MAX_INTSTR_LEN +1];
	char local_ip[MAX_IP_LEN +1];
	char local_description[3*MAX_DESCRIPTION_LEN +1];
	char local_adv_name[3*MAX_USER_LEN +1];

	stmt = buffer;
	error_exec = NULL;

	printf("Qaullib_WwwFileSchedule\n");

	// get hash
	mg_get_var(conn, "hash", file_item.hashstr, sizeof(file_item.hashstr));
	Qaullib_StringToHash(file_item.hashstr, file_item.hash);
	// get suffix
	mg_get_var(conn, "suffix", file_item.suffix, sizeof(file_item.suffix));
	// get description
	mg_get_var(conn, "description", local_description, sizeof(local_description));
	Qaullib_StringMsgProtect(file_item.description, local_description, sizeof(file_item.description));
	// get size
	mg_get_var(conn, "size", local_size, sizeof(local_size));
	file_item.size = atoi(local_size);
	if(file_item.size <= 0)
		file_item.size = 1024;
	// get advertised by
	mg_get_var(conn, "ip", local_ip, sizeof(local_ip));


	mg_get_var(conn, "name", local_adv_name, sizeof(local_adv_name));
	Qaullib_StringNameProtect(file_item.adv_name, local_adv_name, sizeof(file_item.adv_name));

	// add file
	file_item.type = QAUL_FILETYPE_FILE;
	file_item.status = QAUL_FILESTATUS_NEW;
	time(&timestamp);
	file_item.created_at = timestamp;
	file_item.downloaded = 0;
	file_item.downloaded_chunk = 0;

	// check if file already exists
	if(Qaullib_File_LL_HashSearch(file_item.hash, &existing_file))
	{
		if(existing_file->status == QAUL_FILESTATUS_DELETED)
		{
			// delete from LL
			Qaullib_File_LL_Delete_Item(existing_file);

			// add the file again
			Qaullib_FileAdd(&file_item);
		}
	}
	else
		Qaullib_FileAdd(&file_item);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");
	mg_printf_data(conn, "\"success\":%i", 1);
	mg_printf_data(conn, "}");

	// check all scheduled files
	Qaullib_FileCheckScheduled();
}

// ------------------------------------------------------------
static void Qaullib_WwwPubUsers(struct mg_connection *conn)
{
	char buf[sizeof(struct qaul_userinfo_msg)];
	struct qaul_userinfo_msg *msg = (struct qaul_userinfo_msg *) buf;

	// send your user name
	memcpy(&msg->ip, &qaul_ip_addr, sizeof(union olsr_ip_addr));
	memcpy(&msg->name, qaul_username, MAX_USER_LEN);
	memcpy(&msg->icon, "\0", 1);
	memcpy(&msg->suffix, "\0", 1);
	// send message
	mg_write(conn, buf, (size_t) sizeof(struct qaul_userinfo_msg));

	// send all other known user names
	// loop through LL
	struct qaul_user_LL_node mynode;
	Qaullib_User_LL_InitNode(&mynode);
	while(Qaullib_User_LL_NextNode(&mynode))
	{
		if(mynode.item->type == QAUL_USERTYPE_KNOWN && mynode.item->changed < QAUL_USERCHANGED_DELETED)
		{
			memcpy(&msg->ip, &mynode.item->ip, sizeof(union olsr_ip_addr));
			memcpy(&msg->name, mynode.item->name, MAX_USER_LEN);
			memcpy(&msg->icon, "\0", 1);
			memcpy(&msg->suffix, "\0", 1);
			// send info
			mg_write(conn, buf, (size_t) sizeof(struct qaul_userinfo_msg));
		}
	}
}

// ------------------------------------------------------------
static void Qaullib_WwwPubMsg(struct mg_connection *conn)
{
	//int length;
	char encoded_msg[3*MAX_MESSAGE_LEN +1];
	char encoded_name[3*MAX_USER_LEN +1];
	char *local_msg;
	char *local_name;
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;

	// Fetch Message
	// fixme: memory leak at Qaullib_UrlDecode()?
	msg_item.id = 0;
	msg_item.type = QAUL_MSGTYPE_PRIVATE_IN;

	// get msg
	mg_get_var(conn, "m", encoded_msg, sizeof(encoded_msg));
	local_msg = Qaullib_UrlDecode(encoded_msg);
	strncpy(msg_item.msg, local_msg, sizeof(msg_item.msg));
	free(local_msg);

	// get name
	mg_get_var(conn, "n", encoded_name, sizeof(encoded_name));
	local_name = Qaullib_UrlDecode(encoded_name);
	strncpy(msg_item.name, local_name, sizeof(msg_item.name));
	free(local_name);

	// set time
	time(&timestamp);
	msg_item.time = (int)timestamp;

	// set read
	msg_item.read = 0;

	// set ip
	// todo: ipv6
	msg_item.ipv = 4;
	strncpy(msg_item.ip, conn->remote_ip, sizeof(msg_item.ip));
	inet_pton(AF_INET, conn->remote_ip, &msg_item.ip_union.v4);

  	// save Message
	Qaullib_MsgAdd(&msg_item);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// json callback
	mg_printf_data(conn, "%s", "abc({})");
}

// ------------------------------------------------------------
static void Qaullib_WwwPubInfo(struct mg_connection *conn)
{
	int firstitem;
	struct qaul_file_LL_node mynode;
	char qaul_username_json[2* MAX_USER_LEN +1];

	Qaullib_StringJsonProtect(qaul_username_json, qaul_username, sizeof(qaul_username_json));
	Qaullib_File_LL_InitNode(&mynode);

	printf("Qaullib_WwwGetFiles\n");

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "abc({");
	mg_printf_data(conn, "\"name\":\"%s\",", qaul_username_json);

	mg_printf_data(conn, "\"files\":[");
	// loop through files
	firstitem = 1;
	while(Qaullib_File_LL_NextNodePub(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf_data(conn, ",");

		Qaullib_WwwFile2Json(conn, mynode.item);
	}
	mg_printf_data(conn, "]");
	mg_printf_data(conn, "})");
}

// ------------------------------------------------------------
static void Qaullib_WwwLoading(struct mg_connection *conn)
{
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");

	if(qaul_loading_wait == 1)
	{
		// wait
		mg_printf_data(conn, "\"change\":0");
	}
	else if(Qaullib_ExistsLocale() == 0)
	{
		// show set user name
		mg_printf_data(conn, "\"change\":1,\"page\":\"#page_config_locale\"");
	}
	else if(qaul_conf_ios == 1 && qaul_conf_wifi_set == 0)
	{
		// show open wifi page
		mg_printf_data(conn, "\"change\":1,\"page\":\"#page_iphone\"");
	}
	else if(Qaullib_ExistsUsername() == 0)
	{
		// show set user name
		mg_printf_data(conn, "\"change\":1,\"page\":\"#page_config_name\"");
	}
	else if(qaul_interface_configuring)
	{
		// wait until interface is configured
		if(qaul_interface_configuring == 2)
		{
			mg_printf_data(conn, "\"change\":1,\"page\":\"#page_config_interface\"");
			qaul_interface_configuring = 0;
		}
		else
		{
			mg_printf_data(conn, "\"change\":0");
		}
	}
	else if(qaul_configured == 1 && qaul_gui_pagename_set == 1)
	{
		// show configured page name
		mg_printf_data(conn, "\"change\":1,\"page\":\"#%s\"", qaullib_GuiPageName);
	}
	else if(qaul_configured == 1)
	{
		// configuration finished, show chat
		mg_printf_data(conn, "\"change\":1,\"page\":\"#page_chat\"");
	}
	// TODO: show error message if an error occurred
	else
	{
		// still loading
		mg_printf_data(conn, "\"change\":0");
	}
	mg_printf_data(conn, "}");
}

// ------------------------------------------------------------
static void Qaullib_WwwPubFilechunk(struct mg_connection *conn)
{
	char local_hash[MAX_HASHSTR_LEN +1];
	char local_suffix[MAX_SUFFIX_LEN +1];
	char local_chunkpos[MAX_INTSTR_LEN +1];
	char local_file[MAX_PATH_LEN +1];
	int  chunkpos, mychunksize;
	struct qaul_file_LL_item *myfile;
	union qaul_inbuf msgbuf;


	size_t len = 0 ;
    char buffer[BUFSIZ] = { '\0' } ;

	if(QAUL_DEBUG)
		printf("Qaullib_WwwPubFilechunk\n");

	// get hash
	mg_get_var(conn, "h", local_hash, sizeof(local_hash));
	// get suffix
	mg_get_var(conn, "s", local_suffix, sizeof(local_suffix));
	// get chunk starting position
	mg_get_var(conn, "c", local_chunkpos, sizeof(local_chunkpos));
	chunkpos = atoi(local_chunkpos);

	if(QAUL_DEBUG)
		printf("Qaullib_WwwPubFilechunk request %s.%s %i\n", local_hash, local_suffix, chunkpos);

	// check if file exists
	if(Qaullib_FileAvailable(local_hash, local_suffix, &myfile))
	{
		printf("Qaullib_WwwPubFilechunk size: %i\n", myfile->size);

		// check if file is big enough
		if(myfile->size < chunkpos)
		{
			printf("Qaullib_WwwPubFilechunk size smaller than chunkpos\n");

			msgbuf.filechunk.type = htonl(3);
			msgbuf.filechunk.filesize = htonl(myfile->size);
			mg_write(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
			return;
		}

		// send file chunk
		Qaullib_FileCreatePath(local_file, local_hash, local_suffix);
		FILE* sendfile = fopen(local_file, "rb") ;
		if(sendfile != NULL)
		{
			printf("Qaullib_WwwPubFilechunk send chunk\n");

			// send type
			msgbuf.filechunk.type = htonl(1);
			// send file size
			msgbuf.filechunk.filesize = htonl(myfile->size);
			// send chunk size
			if(chunkpos + qaul_chunksize > myfile->size)
				mychunksize = myfile->size - chunkpos;
			else
				mychunksize = qaul_chunksize;
			msgbuf.filechunk.chunksize = htonl(mychunksize);

			printf("chunkpos %i, filesize %i, chunksize %i, BUFSIZ %i, iterations %i\n", chunkpos, myfile->size, mychunksize, BUFSIZ, (int) ceil(mychunksize/BUFSIZ));

			// TODO: send chunk hash
			memcpy(msgbuf.filechunk.chunkhash, "01234567890123456789", MAX_HASH_LEN);

			// send buf
			mg_write(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));

			// loop through file and send it
			fseek(sendfile, chunkpos, SEEK_SET);
			int i;
	        for(i=0; BUFSIZ*i < mychunksize; i++)
	        {
	        	int mybuf = BUFSIZ;
	        	if(BUFSIZ*(i+1)>mychunksize)
	        	{
	        		mybuf = mychunksize - (BUFSIZ*i);
	        	}
	        	len = fread( buffer, 1, mybuf, sendfile);
	        	mg_write(conn, buffer, len);
	        }
			/*
	        while( (len = fread( buffer, 1, BUFSIZ, sendfile)) > 0 )
	        {
	            mg_write(conn, buffer, len);
	        }
	        */
	        // close file
	        fclose(sendfile);
		}
		else
		{
			printf("Qaullib_WwwPubFilechunk sendfile == NULL\n");

			msgbuf.filechunk.type = htonl(0);
			mg_write(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
		}
	}
	else
	{
		printf("Qaullib_WwwPubFilechunk send error\n");

		// send error
		msgbuf.filechunk.type = htonl(0);
		mg_write(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
	}
}


// ------------------------------------------------------------
static void Qaullib_WwwWebGetMsgs(struct mg_connection *conn)
{
	char local_id[MAX_INTSTR_LEN +1];
	char timestr[MAX_TIME_LEN];
	int  id, count, items;
	struct qaul_msg_LL_node node;

	items = 0;

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{\"messages\":[");

	// get variables
	// get id
	mg_get_var(conn, "id", local_id, sizeof(local_id));
	id = atoi(local_id);

	// loop through items
	if(Qaullib_Msg_LL_FirstWebItem(&node, id))
	{
		count = 0;
		while (Qaullib_Msg_LL_PrevWebItem(&node))
		{
			if(count > 0)
				mg_printf_data(conn, "%s", ",");

			count++;

			mg_printf_data(conn, "{");

			mg_printf_data(conn, "\"id\":%i", node.item->id);
			mg_printf_data(conn, ",");
			if(node.item->type == QAUL_MSGTYPE_PUBLIC_OUT)
				mg_printf_data(conn, "\"type\":%i", QAUL_MSGTYPE_PUBLIC_IN);
			else
				mg_printf_data(conn, "\"type\":%i", node.item->type);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"name\":\"%s\"", node.item->name);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"msg\":\"%s\"", node.item->msg);
			mg_printf_data(conn, ",");
			mg_printf_data(conn, "\"ip\":\"%s\"", node.item->ip);
			mg_printf_data(conn, ",");
			Qaullib_Timestamp2Isostr(timestr,  node.item->time, MAX_TIME_LEN);
			mg_printf_data(conn, "\"time\":\"%s\"", timestr);

			mg_printf_data(conn, "}");
		}
	}
	mg_printf_data(conn, "%s", "]}");
}


// ------------------------------------------------------------
static void Qaullib_WwwWebSendMsg(struct mg_connection *conn)
{
	char buffer[1024];
	char *stmt;
	char *error_exec;
	char local_msg[3*MAX_MESSAGE_LEN +1];
	char local_name[3*MAX_USER_LEN +1];
	char local_type[7];
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;
	struct qaul_user_LL_item *user_item;
	unsigned char userid[MAX_HASH_LEN];

	if(QAUL_DEBUG)
		printf("Qaullib_WwwWebSendMsg\n");

	// fill in data
	msg_item.id = 0;
	msg_item.type = QAUL_MSGTYPE_PUBLIC_IN;

	// get msg
	mg_get_var(conn, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(msg_item.msg, local_msg, sizeof(msg_item.msg));

	// get name
	mg_get_var(conn, "n", local_name, sizeof(local_name));
	Qaullib_StringNameProtect(msg_item.name, local_name, sizeof(msg_item.name));

	// check if name is web name, add [WEB] otherwise
	if(Qaullib_UserCheckWebUserName(msg_item.name) == 0)
	{
		if(strlen(msg_item.name) +5 > MAX_USER_LEN)
			strcpy(msg_item.name +MAX_USER_LEN -5, "[WEB]");
		else
			strcat(msg_item.name,"[WEB]");
	}

	// set time
	time(&timestamp);
	msg_item.time = (int)timestamp;

	// set ip
	// todo: ipv6
	msg_item.ipv = 4;
	strncpy(msg_item.ip, conn->remote_ip, sizeof(msg_item.ip));
	//inet_pton(AF_INET, conn->remote_ip, &msg_item.ip_union.v4);
	memcpy((char *)&msg_item.ip_union, (char *)&qaul_ip_addr, sizeof(msg_item.ip_union));

	// set read
	msg_item.read = 0;

	// send and save message
	Qaullib_MsgSendPublicWeb(&msg_item);

	// everything went fine
	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{}");
}


// ------------------------------------------------------------
static void Qaullib_WwwWebGetUsers(struct mg_connection *conn)
{
	int request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	first = 0;

	if(QAUL_DEBUG)
		printf("Qaullib_WwwWebGetUsers\n");

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_var(conn, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	// print newly
	mg_printf_data(conn, "{\"users\":[");
	// loop through LL
	first = 0;
	struct qaul_user_LL_node mynode;
	Qaullib_User_LL_InitNode(&mynode);
	while(Qaullib_User_LL_NextNode(&mynode))
	{
		// only use named nodes
		if(mynode.item->type == QAUL_USERTYPE_KNOWN || mynode.item->type == QAUL_USERTYPE_WEB_KNOWN )
		{
			// make sure the user name is not empty
			if(strlen(mynode.item->name) > 0 && mynode.item->changed <= QAUL_USERCHANGED_MODIFIED)
			{
				if(!first)
					first = 1;
				else
					mg_printf_data(conn, ",");

				// FIXME: ipv6
				mg_printf_data(conn,
						"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\",\"lq\":%i,\"add\":1}",
						mynode.item->name,
						inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
						mynode.item->idstr,
						Qaullib_UserLinkcost2Img(mynode.item->lq)
						);
			}
		}
	}

	// add host user
	if(first)
		mg_printf_data(conn, ",");

	mg_printf_data(conn,
							"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\",\"lq\":%i,\"add\":1}",
							qaul_username,
							qaul_ip_str,
							"yourhostuserxxxxxxxxxxxxxxx",
							4
							);

	mg_printf_data(conn, "]}");
}


// ------------------------------------------------------------
static void Qaullib_WwwWebGetFiles(struct mg_connection *conn)
{
	int firstitem, request_type;
	char request_type_char[MAX_INTSTR_LEN +1];
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNode(&mynode);

	printf("Qaullib_WwwWebGetFiles\n");

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_var(conn, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");
	mg_printf_data(conn, "\"files\":[");

	// loop through files
	firstitem = 1;
	// select downloaded files
	while(Qaullib_File_LL_NextNodePub(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf_data(conn, ",");

		Qaullib_WwwFile2Json(conn, mynode.item);
	}

	mg_printf_data(conn, "]");
	mg_printf_data(conn, "}");
}


// ------------------------------------------------------------
static void Qaullib_WwwFile2Json(struct mg_connection *conn, struct qaul_file_LL_item *file)
{
	char timestr[MAX_TIME_LEN];
	double perc;

	if(QAUL_DEBUG)
		printf("Qaullib_WwwFile2Json %s status: %i\n    downloaded: %i, downloaded_chunk: %i, size: %i\n",
				file->hashstr,
				file->status,
				file->downloaded,
				file->downloaded_chunk,
				file->size
				);

	mg_printf_data(conn, "\n{");
	mg_printf_data(conn, "\"hash\":\"%s\",", file->hashstr);
	mg_printf_data(conn, "\"size\":%i,", file->size);
	mg_printf_data(conn, "\"suffix\":\"%s\",", file->suffix);
	mg_printf_data(conn, "\"description\":\"%s\",", file->description);

	Qaullib_Timestamp2Isostr(timestr, file->created_at, MAX_TIME_LEN);
	mg_printf_data(conn, "\"time\":\"%s\",", timestr);
	mg_printf_data(conn, "\"status\":%i,", file->status);
	if(file->size > 0)
	{
		if(file->downloaded >= file->size)
			mg_printf_data(conn, "\"downloaded\":100");
		else
		{
			perc = file->downloaded + file->downloaded_chunk;
			perc = perc *100;
			perc = perc / file->size;
			printf("downloaded percentage: %.0f\n", floor(perc));
			mg_printf_data(conn, "\"downloaded\":%.0f", floor(perc));
		}
	}
	else
		mg_printf_data(conn, "\"downloaded\":0");

	mg_printf_data(conn, "}");
}


// ------------------------------------------------------------
static void Qaullib_WwwExtBinaries(struct mg_connection *conn)
{
	int firstitem;
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNode(&mynode);

	printf("Qaullib_WwwGetFiles\n");

	// send header
	mg_send_status(conn, 200);
	mg_send_header(conn, "Content-Type", "application/json; charset=utf-8");

	mg_printf_data(conn, "{");
	mg_printf_data(conn, "\"name\":\"%s\",", qaul_username);

	// loop through files
	mg_printf_data(conn, "\"files\":[");
	firstitem = 1;
	while(Qaullib_File_LL_NextNodePubBinaries(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf_data(conn, ",");

		Qaullib_WwwFile2Json(conn, mynode.item);
	}
	mg_printf_data(conn, "\n]");
	mg_printf_data(conn, "\n}");
}
