/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <math.h>
#include "qaullib.h"
#include "qaullib_private.h"
#include "urlcode/urlcode.h"
#include "qaullib_crypto.h"
#include "qaullib_file_LL.h"
#include "qaullib/logging.h"

// ------------------------------------------------------------
// static declarations
// ------------------------------------------------------------

static int  Qaullib_WwwGetMsgsCallback(void *NotUsed, int number_of_lines, char **column_value, char **column_name);
void Ql_WwwFile2Json(struct mg_connection *conn, struct qaul_file_LL_item *file);

/**
 * checks if the IP matches the local IP used by the user interface
 *
 * @return 0: is a different IP
 * @return 1: is local IP
 */
static int Ql_Www_IsLocalIP(struct mg_connection *conn);

// event handler
union olsr_ip_addr qaul_web_localip;

// ------------------------------------------------------------
// web server polling thread
// ------------------------------------------------------------
void *Ql_Www_Server(void *webserver_instance)
{
	while(webserver_instance != NULL)
	{
		mg_mgr_poll((struct mg_mgr *)webserver_instance, 100);
	}
}

// ------------------------------------------------------------
void Ql_Www_ServerStop(struct mg_mgr *webserver_instance)
{
	mg_mgr_free(webserver_instance);
}

// ------------------------------------------------------------
// helper functions
// ------------------------------------------------------------
static int Ql_Www_IsLocalIP(struct mg_connection *conn)
{
	if(memcmp(&qaul_web_localip.v4.s_addr, &conn->sa.sin.sin_addr, sizeof(qaul_web_localip.v4.s_addr)) == 0)
		return 1;
	else
		return 0;
}

// ------------------------------------------------------------
// web server functions
// ------------------------------------------------------------
/*
 * declaration of internal mongoose web server function
 */
int mg_uri_to_local_path(struct http_message *hm,
                                     const struct mg_serve_http_opts *opts,
                                     char **local_path,
                                     struct mg_str *remainder);

void Ql_WwwEvent_handler(struct mg_connection *conn, int event, void *event_data)
{
	char requestaddr[MAX_IP_LEN +1];
	int processed, redirect;
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	struct mg_str path_info;
	char *path = NULL;
	struct http_message *hm = (struct http_message *) event_data;
	redirect = 0;

	if (event == MG_EV_HTTP_REQUEST)
	{
		if(qaul_web_localip_set == 0)
		{
			memcpy(&qaul_web_localip.v4.s_addr, &conn->sa.sin.sin_addr, sizeof(qaul_web_localip.v4.s_addr));
			printf("qaul_web_localip conn->sa.sin.sin_addr %s\n",
					inet_ntop(AF_INET, &conn->sa.sin.sin_addr, (char *)&ipbuf, sizeof(ipbuf)));
			qaul_web_localip_set = 1;
		}

		// check if captive portal redirect is needed
		if(Ql_Www_IsLocalIP(conn) == 0)
		{
			if( !mg_normalize_uri_path(&hm->uri, &hm->uri) ||
				mg_uri_to_local_path(hm, &ql_webserver_options, &path, &path_info) == 0 ||
				Qaullib_FileExists(path)==0 )
			{
				// redirect to splash page
				mg_printf(conn, "HTTP/1.1 303 See Other\r\n"
						  "Location: %s\r\n\r\n", "http://start.qaul/");
				mg_printf(conn, "<a href=\"%s\">qaul.net &gt;&gt;</a>", "http://start.qaul/");

				conn->flags |= MG_F_SEND_AND_CLOSE;
				return;
			}
		}

		// serve pages
		mg_serve_http(conn, hm, ql_webserver_options);
	}
}

// ------------------------------------------------------------
void Ql_WwwSetName(struct mg_connection *conn, int event, void *event_data)
{
	char username[3*MAX_USER_LEN +1];
	char protected_username[MAX_USER_LEN +1];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// Fetch user name
	mg_get_http_var(&hm->body, "n", username, sizeof(username));
	printf("user name len: %i\n", (int)strlen(username));
	memcpy(&username[MAX_USER_LEN], "\0", 1);

	if(Qaullib_StringNameProtect(protected_username, username, sizeof(protected_username)) > 0)
	{
		printf("save user name len %i: ", (int)strlen(protected_username));
		printf("%s  \n", protected_username);
		Qaullib_SetUsername(protected_username);
	}

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwSetLocale(struct mg_connection *conn, int event, void *event_data)
{
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// Fetch locale
	mg_get_http_var(&hm->body, "l", qaul_locale, sizeof(qaul_locale));

	printf("save locale: %s\n", qaul_locale);
	Qaullib_SetLocale(qaul_locale);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInterfaceLoading(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// request interface json from application
	if(qaul_interface_configuring == 0)
	{
		qaul_interface_configuring = 1;
		Qaullib_Appevent_LL_Add(QAUL_EVENT_GETINTERFACES);
	}

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInterfaceGet(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// configuration method
	mg_printf(conn, "\"manual\":%i,", Qaullib_GetConfInt("net.interface.manual"));

	// selected interface
	mg_printf(conn, "\"selected\":\"%s\",", Qaullib_GetInterface());

	// interfaces
	mg_printf(conn, "\"interfaces\":[");
	mg_printf(conn, "%s", qaul_interface_json);
	mg_printf(conn, "]");

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInterfaceSet(struct mg_connection *conn, int event, void *event_data)
{
	char local_manual[MAX_INTSTR_LEN +1];
	int mymanual;
	char local_interface[255 +1];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// save interface method
	mg_get_http_var(&hm->body, "m", local_manual, sizeof(local_manual));
	memcpy(&local_manual[MAX_INTSTR_LEN], "\0", 1);
	mymanual = atoi(local_manual);
	Qaullib_SetInterfaceManual(mymanual);

	// if method manual, extract interface
	if(mymanual)
	{
		mg_get_http_var(&hm->body, "if", local_interface, sizeof(local_interface));
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
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInternetLoading(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// request interface json from application
	if(qaul_interface_configuring == 0)
	{
		qaul_interface_configuring = 1;
		Qaullib_Appevent_LL_Add(QAUL_EVENT_GETINTERFACES);
	}
	qaul_internet_configuring = 1;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInternetGet(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// configuration method
	mg_printf(conn, "\"share\":%i,", Qaullib_IsGateway());

	// interface used by qaul
	mg_printf(conn, "\"used\":\"%s\",", Qaullib_GetInterface());

	// selected interfaces
	mg_printf(conn, "\"selected\":\"%s\",", Qaullib_GetGatewayInterface());

	// interfaces
	mg_printf(conn, "\"interfaces\":[");
	mg_printf(conn, "%s", qaul_interface_json);
	mg_printf(conn, "]");

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigInternetSet(struct mg_connection *conn, int event, void *event_data)
{
	char local_share[MAX_INTSTR_LEN +1];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// save interface method
	mg_get_http_var(&hm->body, "share", local_share, sizeof(local_share));
	memcpy(&local_share[MAX_INTSTR_LEN], "\0", 1);
	qaul_internet_share = atoi(local_share);
	Qaullib_DbSetConfigValueInt("internet.share", qaul_internet_share);

	// if method manual, extract interface
	if(qaul_internet_share)
	{
		mg_get_http_var(&hm->body, "if", qaul_internet_interface, sizeof(qaul_internet_interface));
		memcpy(&qaul_internet_interface[255], "\0", 1);
		Qaullib_DbSetConfigValue("internet.interface", qaul_internet_interface);

		Qaullib_Appevent_LL_Add(QAUL_EVENT_GATEWAY_START);

		if(QAUL_DEBUG)
			printf("share Internet via %s\n", qaul_internet_interface);
	}
	else
	{
		Qaullib_Appevent_LL_Add(QAUL_EVENT_GATEWAY_STOP);

		if(QAUL_DEBUG)
			printf("don't share Internet\n");
	}

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigNetworkGet(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// profile exists
	mg_printf(conn, "\"available\":1,");

	// network profile
	mg_printf(conn, "\"profile\":\"%s\",", Qaullib_GetNetProfile());

	mg_printf(conn, "\"ip\":\"%s\",", Qaullib_GetIP());
	mg_printf(conn, "\"mask\":\"%i\",", Qaullib_GetNetMask());
	mg_printf(conn, "\"broadcast\":\"%s\",", Qaullib_GetNetBroadcast());
/*
	mg_printf(conn, "\"gateway\":\"%s\",", Qaullib_GetNetGateway());

	mg_printf(conn, "\"ns1\":\"%s\",", Qaullib_GetNetNs1());
	mg_printf(conn, "\"ns2\":\"%s\",", Qaullib_GetNetNs2());
*/
	mg_printf(conn, "\"channel\":\"%i\",", Qaullib_GetWifiChannel());
	mg_printf(conn, "\"ssid\":\"%s\",", Qaullib_GetWifiSsid());
	mg_printf(conn, "\"bssid\":\"%s\"", Qaullib_GetWifiBssId());

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigNetworkGetProfile(struct mg_connection *conn, int event, void *event_data)
{
	char local_profile[QAUL_MAX_PROFILE_LEN +1];
	char profile_dbprotected[2*QAUL_MAX_PROFILE_LEN +1];
	char key[512];
	char value[512];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// get profile name
	mg_get_http_var(&hm->query_string, "p", local_profile, sizeof(local_profile));
	Qaullib_StringDbProtect(profile_dbprotected, local_profile, sizeof(profile_dbprotected));
	sprintf(key, "%s.profile", profile_dbprotected);

	// check if profile exists
	if(Qaullib_DbGetConfigValueInt(key))
	{
		// profile exists
		mg_printf(conn, "\"available\":1,");

		// network profile
		mg_printf(conn, "\"profile\":\"%s\",", local_profile);

		sprintf(key, "%s.net.ip", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"ip\":\"%s\",", value);

		sprintf(key, "%s.net.mask", profile_dbprotected);
		mg_printf(conn, "\"mask\":\"%i\",", Qaullib_DbGetConfigValueInt(key));

		sprintf(key, "%s.net.broadcast", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"broadcast\":\"%s\",", value);
/*
		sprintf(key, "%s.net.gateway", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"gateway\":\"%s\",", value);

		sprintf(key, "%s.net.ns1", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"ns1\":\"%s\",", value);

		sprintf(key, "%s.net.ns2", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"ns2\":\"%s\",", value);
*/
		sprintf(key, "%s.wifi.channel", profile_dbprotected);
		mg_printf(conn, "\"channel\":\"%i\",", Qaullib_DbGetConfigValueInt(key));

		sprintf(key, "%s.wifi.ssid", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"ssid\":\"%s\",", value);

		sprintf(key, "%s.wifi.bssid", profile_dbprotected);
		Qaullib_DbGetConfigValue(key, value);
		mg_printf(conn, "\"bssid\":\"%s\"", value);
	}
	else
	{
		// profile does not exist
		mg_printf(conn, "\"available\":0");
	}

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigNetworkSet(struct mg_connection *conn, int event, void *event_data)
{
	int value_int;
	char local_profile[QAUL_MAX_PROFILE_LEN +1], profile_dbprotected[2*QAUL_MAX_PROFILE_LEN +1];
	char key[512];
	char value[255 +1], value_dbprotected[2*sizeof(value)];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// profile
	mg_get_http_var(&hm->body, "profile", local_profile, sizeof(local_profile));
	Qaullib_StringDbProtect(profile_dbprotected, local_profile, sizeof(profile_dbprotected));
	sprintf(key, "%s.profile", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, 1);
	Qaullib_DbSetConfigValue("net.profile", profile_dbprotected);

	// ip
	mg_get_http_var(&hm->body, "ip", value, MAX_IP_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ip", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("ip", value_dbprotected);

	// mask
	mg_get_http_var(&hm->body, "mask", value, MAX_INTSTR_LEN +1);
	value_int = atoi(value);
	sprintf(key, "%s.net.mask", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, value_int);
	Qaullib_DbSetConfigValueInt("net.mask", value_int);

	// broadcast
	mg_get_http_var(&hm->body, "broadcast", value, MAX_IP_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.broadcast", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.broadcast", value_dbprotected);

/*
	// TODO: allow manual gatway definition
	//       this could be done as a special option with the
	//       possibility to also select between available
	//       dynamic gateways.

	// gateway
	mg_get_http_var(&hm->body, "gateway", value, MAX_IP_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.gateway", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.gateway", value_dbprotected);

	// ns1 DNS server
	mg_get_http_var(&hm->body, "ns1", value, MAX_IP_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ns1", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.ns1", value_dbprotected);

	// ns2 DNS server
	mg_get_http_var(&hm->body, "ns2", value, MAX_IP_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.net.ns2", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("net.ns2", value_dbprotected);
*/
	// wifi channel
	mg_get_http_var(&hm->body, "channel", value, MAX_INTSTR_LEN +1);
	value_int = atoi(value);
	sprintf(key, "%s.wifi.channel", profile_dbprotected);
	Qaullib_DbSetConfigValueInt(key, value_int);
	Qaullib_DbSetConfigValueInt("wifi.channel", value_int);

	// wifi ssid
	mg_get_http_var(&hm->body, "ssid", value, MAX_SSID_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.wifi.ssid", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("wifi.ssid", value_dbprotected);

	// wifi bssid
	mg_get_http_var(&hm->body, "bssid", value, MAX_BSSID_LEN +1);
	Qaullib_StringDbProtect(value_dbprotected, value, sizeof(value_dbprotected));
	sprintf(key, "%s.wifi.bssid", profile_dbprotected);
	Qaullib_DbSetConfigValue(key, value_dbprotected);
	Qaullib_DbSetConfigValue("wifi.bssid", value_dbprotected);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigFilesGet(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// configuration auto download of files
	mg_printf(conn, "\"autodownload\":%i,", Qaullib_GetConfInt("files.autodownload"));

	// configuration auto download of files
	mg_printf(conn, "\"space\":\"%i\",", Qaullib_GetConfInt("files.space.max"));

	// configuration auto download of files
	mg_printf(conn, "\"filesize\":\"%i\"", Qaullib_GetConfInt("files.filesize.max"));

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwConfigFilesSet(struct mg_connection *conn, int event, void *event_data)
{
	char local_intstr[MAX_INTSTR_LEN +1];
	int mydownload, myspace, myfilesize;
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// save download method
	mg_get_http_var(&hm->body, "download", local_intstr, sizeof(local_intstr));
	mydownload = atoi(local_intstr);
	Qaullib_DbSetConfigValueInt("files.autodownload", mydownload);
	qaul_file_autodownload = mydownload;

	// if auto download is set, set boundaries
	if(mydownload)
	{
		mg_get_http_var(&hm->body, "space", local_intstr, sizeof(local_intstr));
		myspace = atoi(local_intstr);
		Qaullib_DbSetConfigValueInt("files.space.max", myspace);
		qaul_file_space_max = myspace;

		mg_get_http_var(&hm->body, "size", local_intstr, sizeof(local_intstr));
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
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwGetTopology(struct mg_connection *conn, int event, void *event_data)
{
	int add, request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	char dest_ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	first = 0;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// check if topology has been received
	if(qaul_ipc_topo_request == 2)
	{
		// set availability
		mg_printf(conn, "\"available\":1,");
		// send topology
		mg_printf(conn, "\"links\":[");

		// send link list
		first = 0;
		while(qaul_topo_LL_first)
		{
			if(!first)
				first = 1;
			else
				mg_printf(conn, ",");

			// FIXME: ipv6
			mg_printf(conn,
					"{\"source\":\"%s\",\"target\":\"%s\",\"lq\":\"%i\"}",
					inet_ntop(AF_INET, &qaul_topo_LL_first->src_ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
					inet_ntop(AF_INET, &qaul_topo_LL_first->dest_ip.v4.s_addr, (char *)&dest_ipbuf, sizeof(dest_ipbuf)),
					(int)(qaul_topo_LL_first->lq *1)
					);

			// delete this item
			Qaullib_Topo_LL_Delete_Item ();
		}
		mg_printf(conn, "],");

		// set topo to 0, as the list has been deleted
		qaul_ipc_topo_request = 0;

		// send all users
		mg_printf(conn, "\"nodes\":{");
		// print this user
		mg_printf(conn,
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
					mg_printf(conn, ",");
					// FIXME: ipv6
					mg_printf(conn,
							"\"%s\":{\"name\":\"%s\",\"type\":\"name\"}",
							inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
							mynode.item->name
							);
				}
			}
		}
		mg_printf(conn, "}");
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
		mg_printf(conn, "\"available\":0");
	}
	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwSetPageName(struct mg_connection *conn, int event, void *event_data)
{
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// Fetch screen name
	mg_get_http_var(&hm->query_string, "p", qaullib_GuiPageName, sizeof(qaullib_GuiPageName));
	qaul_gui_pagename_set = 1;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwSetOpenUrl(struct mg_connection *conn, int event, void *event_data)
{
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// Fetch url
	mg_get_http_var(&hm->body, "url", qaullib_AppEventOpenURL+22, sizeof(qaullib_AppEventOpenURL)-22);

	printf("set event url to open: %s\n", qaullib_AppEventOpenURL);
	Qaullib_Appevent_LL_Add(QAUL_EVENT_OPENURL);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwSetWifiSet(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	qaul_conf_wifi_set = 1;
	qaul_gui_pagename_set = 0;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{\"ok\":1}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwQuit(struct mg_connection *conn, int event, void *event_data)
{
	Qaullib_Appevent_LL_Add(QAUL_EVENT_QUIT);

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwCallEvent(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{\"event\":%i", qaul_voip_event);
	if(qaul_voip_event == 5)
		mg_printf(conn, ",\"code\":%i", qaul_voip_event_code);
	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;

	// set ring
	if(qaul_voip_ringing > 0)
	{
		Qaullib_Appevent_LL_Add(QAUL_EVENT_RING);
	}

	qaul_voip_event = 0;
}

void Ql_WwwCallStart(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	char call_ip[MAX_IP_LEN +1];
	struct http_message *hm = (struct http_message *) event_data;

	// extract variables
	mg_get_http_var(&hm->query_string, "ip", call_ip, sizeof(call_ip));
	// call user
	Qaullib_VoipCallStart(call_ip);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

void Ql_WwwCallEnd(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	Qaullib_VoipCallEnd();

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

void Ql_WwwCallAccept(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	Qaullib_VoipCallAccept();

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFavoriteGet(struct mg_connection *conn, int event, void *event_data)
{
	int first = 0;
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// print Users
	mg_printf(conn, "{\"favorites\":[");
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
				mg_printf(conn, ",");
			// FIXME: ipv6
			mg_printf(conn,
					"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\"}",
					mynode.item->name,
					inet_ntop(AF_INET, &mynode.item->ip.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)),
					mynode.item->idstr
					);
		}
	}
	mg_printf(conn, "]}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

void Ql_WwwFavoriteAdd(struct mg_connection *conn, int event, void *event_data)
{
	char myname[3*MAX_USER_LEN +1];
	char myipstr[MAX_IP_LEN +1];
	char myidstr[MAX_HASHSTR_LEN +1];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// extract variable
	mg_get_http_var(&hm->body, "ip", myipstr, sizeof(myipstr));
	mg_get_http_var(&hm->body, "name", myname, sizeof(myname));
	mg_get_http_var(&hm->body, "id", myidstr, sizeof(myidstr));

	printf("add favorite %s \n", myname);
	Qaullib_UserFavoriteAdd(myname, myipstr, myidstr);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

void Ql_WwwFavoriteDelete(struct mg_connection *conn, int event, void *event_data)
{
	char myipstr[MAX_IP_LEN +1];
	char myidstr[MAX_HASHSTR_LEN +1];
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// extract variable
	mg_get_http_var(&hm->body, "ip", myipstr, sizeof(myipstr));
	mg_get_http_var(&hm->body, "id", myidstr, sizeof(myidstr));

	printf("delete favorite %s \n", myipstr);
	Qaullib_UserFavoriteRemove(myipstr, myidstr);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwGetConfig(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// send UI language
	if(Qaullib_ExistsLocale())
		mg_printf(conn, "\"locale\":\"%s\",", Qaullib_GetLocale());

	// send user name
	if(Qaullib_ExistsUsername())
		mg_printf(conn, "\"name\":\"%s\",", Qaullib_GetUsername());

	// send all the rest
	mg_printf(conn, "\"msg_max\":20,");
	if(qaul_conf_quit)
		mg_printf(conn, "\"c_quit\":true,");
	else
		mg_printf(conn, "\"c_quit\":false,");

	if(qaul_conf_debug)
		mg_printf(conn, "\"c_debug\":true,");
	else
		mg_printf(conn, "\"c_debug\":false,");

	if(qaul_conf_interface)
		mg_printf(conn, "\"c_interface\":true,");
	else
		mg_printf(conn, "\"c_interface\":false,");

	if(qaul_conf_internet)
		mg_printf(conn, "\"c_internet\":true,");
	else
		mg_printf(conn, "\"c_internet\":false,");

	if(qaul_conf_network)
		mg_printf(conn, "\"c_network\":true");
	else
		mg_printf(conn, "\"c_network\":false");

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwGetEvents(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	// get number of waiting incoming messages
	mg_printf(conn, "\"m_pub\":%i,", qaul_new_msg);
	mg_printf(conn, "\"m_priv\":%i,", qaul_new_msg);

	// get newly downloaded files
	mg_printf(conn, "\"files\":%i,",0);

	// check call events
	if(qaul_voip_new_call)
	{
		mg_printf(conn, "\"call\":%i",1);
		mg_printf(conn, ",\"callee\":\"%s\"",qaul_voip_call.name);
		qaul_voip_new_call = 0;
	}
	else
		mg_printf(conn, "\"call\":%i",0);

	mg_printf(conn, "}");
	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwGetName(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{\"name\":\"%s\"}", qaul_username);
	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwGetMsgs(struct mg_connection *conn, int event, void *event_data)
{
	char buffer[10240];
	char* stmt = buffer;
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	char local_type[MAX_INTSTR_LEN +1];
	char local_id[MAX_INTSTR_LEN +1];
	char local_tag[MAX_FILENAME_LEN +1];
	char local_name[MAX_USER_LEN +1];
	char timestr[MAX_TIME_LEN];
	int  id, type, count, items;
	struct qaul_msg_LL_node node;
	struct http_message *hm = (struct http_message *) event_data;

	items = 0;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{\"name\":\"%s\",\"messages\":[", qaul_username);

	// get variables
	// message type
	mg_get_http_var(&hm->query_string, "t", local_type, sizeof(local_type));
	type = atoi(local_type);

	// get id
	mg_get_http_var(&hm->query_string, "id", local_id, sizeof(local_id));
	id = atoi(local_id);

	// prepare statements
	// user related
	if(type == 5)
	{
		mg_get_http_var(&hm->query_string, "v", local_name, sizeof(local_name));

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
		mg_get_http_var(&hm->query_string, "v", local_tag, sizeof(local_tag));

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
				mg_printf(conn, "%s", ",");

			count++;

			mg_printf(conn, "{");

			mg_printf(conn, "\"id\":%i", node.item->id);
			mg_printf(conn, ",");
			mg_printf(conn, "\"type\":%i", node.item->type);
			mg_printf(conn, ",");
			mg_printf(conn, "\"name\":\"%s\"", node.item->name);
			mg_printf(conn, ",");
			mg_printf(conn, "\"msg\":\"%s\"", node.item->msg);
			mg_printf(conn, ",");
			mg_printf(conn, "\"ip\":\"%s\"", inet_ntop(AF_INET, &node.item->ip_union.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)));
			mg_printf(conn, ",");
			Qaullib_Timestamp2Isostr(timestr, node.item->time, MAX_TIME_LEN);
			mg_printf(conn, "\"time\":\"%s\"", timestr);

			mg_printf(conn, "}");
		} while (Qaullib_Msg_LL_PrevItem(&node));
	}

	mg_printf(conn, "%s", "]}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwSendMsg(struct mg_connection *conn, int event, void *event_data)
{
	char local_msg[3*MAX_MESSAGE_LEN +1];
	char local_name[3*MAX_USER_LEN +1];
	char local_type[7];
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get type
	mg_get_http_var(&hm->body, "t", local_type, sizeof(local_type));
	msg_item.type = atoi(local_type);

	// get msg
	mg_get_http_var(&hm->body, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(msg_item.msg, local_msg, sizeof(msg_item.msg));

	// get name
	mg_get_http_var(&hm->body, "n", local_name, sizeof(local_name));
	Qaullib_StringNameProtect(msg_item.name, local_name, sizeof(msg_item.name));

	// set time
	time(&timestamp);
	msg_item.time = (int)timestamp;

	// set ip
	msg_item.ipv = 4;
	memcpy(&msg_item.ip_union, &qaul_ip_addr, sizeof(msg_item.ip_union));

	// set read
	msg_item.read = 1;

	// send and save message
	if(msg_item.type == QAUL_MSGTYPE_PUBLIC_OUT)
		Qaullib_MsgSendPublic(&msg_item);
	else if(msg_item.type == QAUL_MSGTYPE_PRIVATE_OUT)
		Qaullib_MsgSendPrivate(&msg_item);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwGetUsers(struct mg_connection *conn, int event, void *event_data)
{
	int add, request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	struct http_message *hm = (struct http_message *) event_data;
	first = 0;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_http_var(&hm->query_string, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// print newly
	mg_printf(conn, "{\"users\":[");
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
					mg_printf(conn, ",");

				// FIXME: ipv6
				mg_printf(conn,
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
	mg_printf(conn, "]}");

	conn->flags |= MG_F_SEND_AND_CLOSE;

	printf("Ql_WwwGetUsers end\n");
}

// ------------------------------------------------------------
void Ql_WwwFileList(struct mg_connection *conn, int event, void *event_data)
{
	int firstitem, request_type;
	char request_type_char[MAX_INTSTR_LEN +1];
	struct qaul_file_LL_node mynode;
	struct http_message *hm = (struct http_message *) event_data;
	Qaullib_File_LL_InitNode(&mynode);

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_http_var(&hm->query_string, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");
	mg_printf(conn, "\"files\":[");

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
				mg_printf(conn, ",");

			Ql_WwwFile2Json(conn, mynode.item);
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
				mg_printf(conn, ",");

			Ql_WwwFile2Json(conn, mynode.item);
			mynode.item->gui_notify = 0;
		}
	}


	mg_printf(conn, "]");
	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFileAdd(struct mg_connection *conn, int event, void *event_data)
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
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get path
	mg_get_http_var(&hm->body, "p", local_path, sizeof(local_path));
	// get msg
	mg_get_http_var(&hm->body, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(file_item.description, local_msg, sizeof(file_item.description));
	// get advertise
	mg_get_http_var(&hm->body, "a", local_advertise, sizeof(local_advertise));
	advertise = atoi(local_advertise);

	// copy file into directory & make hash
	file_item.size = Qaullib_FileCopyNew(local_path, &file_item);

	if(file_item.size > 0)
	{
		printf("Ql_WwwFileAdd hashstr: %s \n", file_item.hashstr);

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
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");
	mg_printf(conn, "\"success\":%i", 1);
	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFilePick(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// open file picker
	pickFileCheck = 1;
	Qaullib_Appevent_LL_Add(QAUL_EVENT_CHOOSEFILE);

	// deliver answer
	mg_printf(conn, "{}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFilePickCheck(struct mg_connection *conn, int event, void *event_data)
{
	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// deliver answer
	mg_printf(conn, "{\"picked\":%i", pickFileCheck);
	if(pickFileCheck == 2)
	{
		int backslash = '\\';
		if(strchr(pickFilePath, backslash))
		{
			mg_printf(conn, ",\"path\":\"");
			// protect backslashes
			int i;
			for(i=0; i<strlen(pickFilePath); i++)
			{
				if(pickFilePath[i] == backslash)
					mg_printf(conn, "\\");
				mg_printf(conn, "%c", pickFilePath[i]);
			}
			mg_printf(conn, "\",");
		}
		else
			mg_printf(conn, ",\"path\":\"%s\",", pickFilePath);
		char *local_file = strrchr(pickFilePath, PATH_SEPARATOR_INT);
		mg_printf(conn, "\"name\":\"%s\",", local_file+1);
		// FIXME: use correct size and date
		mg_printf(conn, "\"size\":1024,");
		mg_printf(conn, "\"create\":\"2012-02-02 19:59:42\"");
		pickFileCheck = 0;
	}

	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFileOpen(struct mg_connection *conn, int event, void *event_data)
{
	char hashstr[MAX_HASHSTR_LEN +1];
	char hash[MAX_HASH_LEN];
	char old_path[MAX_PATH_LEN +1];
	struct qaul_file_LL_item *file_item;
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get file variable
	mg_get_http_var(&hm->query_string, "f", hashstr, sizeof(hashstr));
	memcpy(&hashstr[MAX_HASHSTR_LEN], "\0", 1);

	// get file
	if(Ql_StringToHash(hashstr, hash))
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
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwFileDelete(struct mg_connection *conn, int event, void *event_data)
{
	char local_hashstr[MAX_HASHSTR_LEN +1];
	unsigned char local_hash[MAX_HASH_LEN];
	struct qaul_file_LL_item *file_item;
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	// get file variable
	mg_get_http_var(&hm->query_string, "hash", local_hashstr, sizeof(local_hashstr));

	printf("hashstr %s\n", local_hashstr);
	// delete file
	// todo: delete file (by hash)
	if(Ql_StringToHash(local_hashstr, local_hash))
	{
		if(Qaullib_File_LL_HashSearch(local_hash, &file_item))
		{
			Qaullib_FileDelete(file_item);
		}
	}

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");

	conn->flags |= MG_F_SEND_AND_CLOSE;}

// ------------------------------------------------------------
void Ql_WwwFileSchedule(struct mg_connection *conn, int event, void *event_data)
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
	struct http_message *hm = (struct http_message *) event_data;

	if(Ql_Www_IsLocalIP(conn) == 0)
		return;

	stmt = buffer;
	error_exec = NULL;

	printf("Ql_WwwFileSchedule\n");

	// get hash
	mg_get_http_var(&hm->body, "hash", file_item.hashstr, sizeof(file_item.hashstr));
	Ql_StringToHash(file_item.hashstr, file_item.hash);
	// get suffix
	mg_get_http_var(&hm->body, "suffix", file_item.suffix, sizeof(file_item.suffix));
	// get description
	mg_get_http_var(&hm->body, "description", local_description, sizeof(local_description));
	Qaullib_StringMsgProtect(file_item.description, local_description, sizeof(file_item.description));
	// get size
	mg_get_http_var(&hm->body, "size", local_size, sizeof(local_size));
	file_item.size = atoi(local_size);
	if(file_item.size <= 0)
		file_item.size = 1024;
	// get advertised by
	mg_get_http_var(&hm->body, "ip", local_ip, sizeof(local_ip));


	mg_get_http_var(&hm->body, "name", local_adv_name, sizeof(local_adv_name));
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
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");
	mg_printf(conn, "\"success\":%i", 1);
	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;

	// check all scheduled files
	Qaullib_FileCheckScheduled();
}

// ------------------------------------------------------------
void Ql_WwwPubUsers(struct mg_connection *conn, int event, void *event_data)
{
	char buf[sizeof(struct qaul_userinfo_msg)];
	struct qaul_userinfo_msg *msg = (struct qaul_userinfo_msg *) buf;

	// send your user name
	memcpy(&msg->ip, &qaul_ip_addr, sizeof(union olsr_ip_addr));
	memcpy(&msg->name, qaul_username, MAX_USER_LEN);
	memcpy(&msg->icon, "\0", 1);
	memcpy(&msg->suffix, "\0", 1);
	// send message
	mg_send(conn, buf, sizeof(struct qaul_userinfo_msg));

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
			mg_send(conn, buf, sizeof(struct qaul_userinfo_msg));
		}
	}

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwPubMsg(struct mg_connection *conn, int event, void *event_data)
{
	char encoded_msg[3*MAX_MESSAGE_LEN +1];
	char encoded_name[3*MAX_USER_LEN +1];
	char *local_msg;
	char *local_name;
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;
	int len;
	struct http_message *hm = (struct http_message *) event_data;

	// Fetch Message
	// fixme: memory leak at Qaullib_UrlDecode()?
	msg_item.id = 0;
	msg_item.type = QAUL_MSGTYPE_PRIVATE_IN;

	// get msg
	len = mg_get_http_var(&hm->query_string, "m", encoded_msg, sizeof(encoded_msg));
	local_msg = Qaullib_UrlDecode(encoded_msg);
	strncpy(msg_item.msg, local_msg, sizeof(msg_item.msg));
	free(local_msg);

	// get name
	len = mg_get_http_var(&hm->query_string, "n", encoded_name, sizeof(encoded_name));
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
	msg_item.ip_union.v4 = conn->sa.sin.sin_addr;

  	// save Message
	Qaullib_MsgAdd(&msg_item);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// json callback
	mg_printf(conn, "%s", "abc({})");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwPubInfo(struct mg_connection *conn, int event, void *event_data)
{
	int firstitem;
	struct qaul_file_LL_node mynode;
	char qaul_username_json[2* MAX_USER_LEN +1];

	Qaullib_StringJsonProtect(qaul_username_json, qaul_username, sizeof(qaul_username_json));
	Qaullib_File_LL_InitNode(&mynode);

	printf("Ql_WwwGetFiles\n");

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "abc({");
	mg_printf(conn, "\"name\":\"%s\",", qaul_username_json);

	mg_printf(conn, "\"files\":[");
	// loop through files
	firstitem = 1;
	while(Qaullib_File_LL_NextNodePub(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf(conn, ",");

		Ql_WwwFile2Json(conn, mynode.item);
	}
	mg_printf(conn, "]");
	mg_printf(conn, "})");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwLoading(struct mg_connection *conn, int event, void *event_data)
{
	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");

	if(qaul_loading_wait == 1)
	{
		// wait
		mg_printf(conn, "\"change\":0");
	}
	else if(Qaullib_ExistsLocale() == 0)
	{
		// show set user name
		mg_printf(conn, "\"change\":1,\"page\":\"#page_config_locale\"");
	}
	else if(qaul_conf_ios == 1 && qaul_conf_wifi_set == 0)
	{
		// show open wifi page
		mg_printf(conn, "\"change\":1,\"page\":\"#page_iphone\"");
	}
	else if(Qaullib_ExistsUsername() == 0)
	{
		// show set user name
		mg_printf(conn, "\"change\":1,\"page\":\"#page_config_name\"");
	}
	else if(qaul_interface_configuring)
	{
		// wait until interface is configured
		if(qaul_interface_configuring == 2)
		{
			if(qaul_internet_configuring)
				mg_printf(conn, "\"change\":1,\"page\":\"#page_config_internet\"");
			else
				mg_printf(conn, "\"change\":1,\"page\":\"#page_config_interface\"");

			qaul_interface_configuring = 0;
			qaul_internet_configuring = 0;
		}
		else
		{
			mg_printf(conn, "\"change\":0");
		}
	}
	else if(qaul_configured == 1 && qaul_gui_pagename_set == 1)
	{
		// show configured page name
		mg_printf(conn, "\"change\":1,\"page\":\"#%s\"", qaullib_GuiPageName);
	}
	else if(qaul_configured == 1)
	{
		// configuration finished, show chat
		mg_printf(conn, "\"change\":1,\"page\":\"#page_chat\"");
	}
	// TODO: show error message if an error occurred
	else
	{
		// still loading
		mg_printf(conn, "\"change\":0");
	}
	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}

// ------------------------------------------------------------
void Ql_WwwPubFilechunk(struct mg_connection *conn, int event, void *event_data)
{
	char local_hash[MAX_HASHSTR_LEN +1];
	char local_suffix[MAX_SUFFIX_LEN +1];
	char local_chunkpos[MAX_INTSTR_LEN +1];
	char local_file[MAX_PATH_LEN +1];
	int  chunkpos, mychunksize;
	struct qaul_file_LL_item *myfile;
	union qaul_inbuf msgbuf;
	struct http_message *hm = (struct http_message *) event_data;

	size_t len = 0 ;
    char buffer[BUFSIZ] = { '\0' } ;

	if(QAUL_DEBUG)
		printf("Ql_WwwPubFilechunk\n");

	// get hash
	mg_get_http_var(&hm->query_string, "h", local_hash, sizeof(local_hash));
	// get suffix
	mg_get_http_var(&hm->query_string, "s", local_suffix, sizeof(local_suffix));
	// get chunk starting position
	mg_get_http_var(&hm->query_string, "c", local_chunkpos, sizeof(local_chunkpos));
	chunkpos = atoi(local_chunkpos);

	if(QAUL_DEBUG)
		printf("Ql_WwwPubFilechunk request %s.%s %i\n", local_hash, local_suffix, chunkpos);

	// check if file exists
	if(Qaullib_FileAvailable(local_hash, local_suffix, &myfile))
	{
		printf("Ql_WwwPubFilechunk size: %i\n", myfile->size);

		// check if file is big enough
		if(myfile->size < chunkpos)
		{
			printf("Ql_WwwPubFilechunk size smaller than chunkpos\n");

			msgbuf.filechunk.type = htonl(3);
			msgbuf.filechunk.filesize = htonl(myfile->size);
			mg_send(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
			return;
		}

		// send file chunk
		Qaullib_FileCreatePath(local_file, local_hash, local_suffix);
		FILE* sendfile = fopen(local_file, "rb") ;
		if(sendfile != NULL)
		{
			printf("Ql_WwwPubFilechunk send chunk\n");

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
			mg_send(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));

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
	        	mg_send(conn, buffer, len);
	        }
	        // close file
	        fclose(sendfile);
		}
		else
		{
			printf("Ql_WwwPubFilechunk sendfile == NULL\n");

			msgbuf.filechunk.type = htonl(0);
			mg_send(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
		}
	}
	else
	{
		printf("Ql_WwwPubFilechunk send error\n");

		// send error
		msgbuf.filechunk.type = htonl(0);
		mg_send(conn, msgbuf.buf, sizeof(struct qaul_filechunk_msg));
	}

	// end connection
	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwWebGetMsgs(struct mg_connection *conn, int event, void *event_data)
{
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	char local_id[MAX_INTSTR_LEN +1];
	char timestr[MAX_TIME_LEN];
	int  id, count, items;
	struct qaul_msg_LL_node node;
	struct http_message *hm = (struct http_message *) event_data;

	items = 0;

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{\"messages\":[");

	// get variables
	// get id
	mg_get_http_var(&hm->query_string, "id", local_id, sizeof(local_id));
	id = atoi(local_id);

	// loop through items
	if(Qaullib_Msg_LL_FirstWebItem(&node, id))
	{
		count = 0;
		while (Qaullib_Msg_LL_PrevWebItem(&node))
		{
			if(count > 0)
				mg_printf(conn, "%s", ",");

			count++;

			mg_printf(conn, "{");

			mg_printf(conn, "\"id\":%i", node.item->id);
			mg_printf(conn, ",");
			if(node.item->type == QAUL_MSGTYPE_PUBLIC_OUT)
				mg_printf(conn, "\"type\":%i", QAUL_MSGTYPE_PUBLIC_IN);
			else
				mg_printf(conn, "\"type\":%i", node.item->type);
			mg_printf(conn, ",");
			mg_printf(conn, "\"name\":\"%s\"", node.item->name);
			mg_printf(conn, ",");
			mg_printf(conn, "\"msg\":\"%s\"", node.item->msg);
			mg_printf(conn, ",");
			mg_printf(conn, "\"ip\":\"%s\"", inet_ntop(AF_INET, &node.item->ip_union.v4.s_addr, (char *)&ipbuf, sizeof(ipbuf)));
			mg_printf(conn, ",");
			Qaullib_Timestamp2Isostr(timestr,  node.item->time, MAX_TIME_LEN);
			mg_printf(conn, "\"time\":\"%s\"", timestr);

			mg_printf(conn, "}");
		}
	}
	mg_printf(conn, "%s", "]}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwWebSendMsg(struct mg_connection *conn, int event, void *event_data)
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
	struct http_message *hm = (struct http_message *) event_data;

	Ql_log_debug("Ql_WwwWebSendMsg");

	// fill in data
	msg_item.id = 0;
	msg_item.type = QAUL_MSGTYPE_PUBLIC_IN;

	// get msg
	mg_get_http_var(&hm->body, "m", local_msg, sizeof(local_msg));
	Qaullib_StringMsgProtect(msg_item.msg, local_msg, sizeof(msg_item.msg));

	// get name
	mg_get_http_var(&hm->body, "n", local_name, sizeof(local_name));
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
	memcpy((char *)&msg_item.ip_union, (char *)&qaul_ip_addr, sizeof(msg_item.ip_union));

	// set read
	msg_item.read = 0;

	// send and save message
	Qaullib_MsgSendPublicWeb(&msg_item);

	// everything went fine
	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwWebGetUsers(struct mg_connection *conn, int event, void *event_data)
{
	int request_type, first;
	char request_type_char[MAX_INTSTR_LEN +1];
	char ipbuf[MAX(INET6_ADDRSTRLEN, INET_ADDRSTRLEN)];
	struct http_message *hm = (struct http_message *) event_data;
	first = 0;

	if(QAUL_DEBUG)
		printf("Ql_WwwWebGetUsers\n");

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_http_var(&hm->query_string, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	// print newly
	mg_printf(conn, "{\"users\":[");
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
					mg_printf(conn, ",");

				// FIXME: ipv6
				mg_printf(conn,
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
		mg_printf(conn, ",");

	mg_printf(conn,
							"{\"name\":\"%s\",\"ip\":\"%s\",\"id\":\"%s\",\"lq\":%i,\"add\":1}",
							qaul_username,
							qaul_ip_str,
							"yourhostuserxxxxxxxxxxxxxxx",
							4
							);

	mg_printf(conn, "]}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwWebGetFiles(struct mg_connection *conn, int event, void *event_data)
{
	int firstitem, request_type;
	char request_type_char[MAX_INTSTR_LEN +1];
	struct qaul_file_LL_node mynode;
	struct http_message *hm = (struct http_message *) event_data;
	Qaullib_File_LL_InitNode(&mynode);

	printf("Ql_WwwWebGetFiles\n");

	// get variable r (0=just updates, 1=all, 2=all and don't update gui_notify)
	request_type = 0;
	mg_get_http_var(&hm->query_string, "r", request_type_char, sizeof(request_type_char));
	request_type = atoi(request_type_char);

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");
	mg_printf(conn, "\"files\":[");

	// loop through files
	firstitem = 1;
	// select downloaded files
	while(Qaullib_File_LL_NextNodePub(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf(conn, ",");

		Ql_WwwFile2Json(conn, mynode.item);
	}

	mg_printf(conn, "]");
	mg_printf(conn, "}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}


// ------------------------------------------------------------
void Ql_WwwFile2Json(struct mg_connection *conn, struct qaul_file_LL_item *file)
{
	char timestr[MAX_TIME_LEN];
	double perc;

	if(QAUL_DEBUG)
		printf("Ql_WwwFile2Json %s status: %i\n    downloaded: %i, downloaded_chunk: %i, size: %i\n",
				file->hashstr,
				file->status,
				file->downloaded,
				file->downloaded_chunk,
				file->size
				);

	mg_printf(conn, "\n{");
	mg_printf(conn, "\"hash\":\"%s\",", file->hashstr);
	mg_printf(conn, "\"size\":%i,", file->size);
	mg_printf(conn, "\"suffix\":\"%s\",", file->suffix);
	mg_printf(conn, "\"description\":\"%s\",", file->description);

	Qaullib_Timestamp2Isostr(timestr, file->created_at, MAX_TIME_LEN);
	mg_printf(conn, "\"time\":\"%s\",", timestr);
	mg_printf(conn, "\"status\":%i,", file->status);
	if(file->size > 0)
	{
		if(file->downloaded >= file->size)
			mg_printf(conn, "\"downloaded\":100");
		else
		{
			perc = file->downloaded + file->downloaded_chunk;
			perc = perc *100;
			perc = perc / file->size;
			printf("downloaded percentage: %.0f\n", floor(perc));
			mg_printf(conn, "\"downloaded\":%.0f", floor(perc));
		}
	}
	else
		mg_printf(conn, "\"downloaded\":0");

	mg_printf(conn, "}");
}


// ------------------------------------------------------------
void Ql_WwwExtBinaries(struct mg_connection *conn, int event, void *event_data)
{
	int firstitem;
	struct qaul_file_LL_node mynode;
	Qaullib_File_LL_InitNode(&mynode);

	printf("Ql_WwwExtBinaries\n");

	// send header
	mg_printf(conn, "HTTP/1.1 200 OK\r\n"
	             	"Content-Type: application/json; charset=utf-8\r\n"
					"\r\n");

	mg_printf(conn, "{");
	mg_printf(conn, "\"name\":\"%s\",", qaul_username);

	// loop through files
	mg_printf(conn, "\"files\":[");
	firstitem = 1;
	while(Qaullib_File_LL_NextNodePubBinaries(&mynode))
	{
		if(firstitem)
			firstitem = 0;
		else
			mg_printf(conn, ",");

		Ql_WwwFile2Json(conn, mynode.item);
	}
	mg_printf(conn, "\n]");
	mg_printf(conn, "\n}");

	conn->flags |= MG_F_SEND_AND_CLOSE;
}
