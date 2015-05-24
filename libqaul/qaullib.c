/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"

// ------------------------------------------------------------
void Qaullib_Init(const char* resourcePath)
{
	int rc, i, dbExists;

	// -------------------------------------------------
	// define global variables
	qaul_new_msg = 0;
	ipc_connected = 0;
	qaul_username_set = 0;
	qaul_locale_set = 0;
	qaul_ip_set = 0;
	qaul_gui_pagename_set = 0;
	pickFileCheck = 0;
	qaul_chunksize = 2000000000; // todo: smaller chunk size
	qaul_configured = 0;
	qaul_loading_wait = 1;
	qaul_conf_quit = 0;
	qaul_conf_debug = 0;
	qaul_conf_voip = 0;
	qaul_conf_ios = 0;
	qaul_conf_wifi_set = 0;
	qaul_conf_interface = 0;
	qaul_conf_internet = 0;
	qaul_conf_network = 0;
	qaul_web_localip_set = 0;
	qaul_UDP_socket = -1;
	qaul_UDP_started = 0;
	qaul_exe_available = 0;
	qaul_ipc_topo_request = 0;
	qaul_ipc_connected = 1;
	qaul_topo_LL_first = 0;
	qaul_appevent_LL_first = 0;
	qaul_appevent_LL_last = 0;
	qaul_conf_filedownloadfolder_set = 0;
	sprintf(qaullib_AppEventOpenURL, "http://%s:%s/", IPC_ADDR, CHAT_PORT);
	qaul_interface_configuring = 0;

	// -------------------------------------------------
	// create buffers for socket communication
	for(i=0; i<MAX_USER_CONNECTIONS; i++)
	{
		userconnections[i].conn.connected = 0;
		// get memory for buffer
	    memset(&userconnections[i].conn.buf, 0, BUFFSIZE + 1);
	}

	for(i=0; i<MAX_FILE_CONNECTIONS; i++)
	{
		fileconnections[i].conn.connected = 0;
		// get memory for buffer
	    memset(&fileconnections[i].conn.buf, 0, BUFFSIZE + 1);
	}

	// -------------------------------------------------

	printf("Qaullib_Init:\n");
	printf("path: %s\n", resourcePath);

	// set webserver path
	strcpy(webPath, resourcePath);
#ifdef WIN32
	strcat(webPath, "\\www");
#else
	strcat(webPath, "/www");
#endif

	// set db path
	strcpy(dbPath, resourcePath);
#ifdef WIN32
	strcat(dbPath, "\\qaullib.db");
#else
	strcat(dbPath, "/qaullib.db");
#endif

	// check if db exists
	dbExists = Qaullib_FileExists(dbPath);

	// configure sqlite
	// make sure sqlite is in full mutex mode for multi threading
	if(sqlite3_config(SQLITE_CONFIG_SERIALIZED) != SQLITE_OK)
		printf("SQLITE_CONFIG_SERIALIZED error\n");
	// open database
	rc = sqlite3_open(dbPath, &db);
	if( rc )
	{
		//fprintf(stderr, "Can't open database: %s\n", sqlite3_errmsg(db));
		sqlite3_close(db);
		exit(1);
	}
	// initialize Database
	Qaullib_DbInit();
	// insert filesharing
	if(dbExists == 0)
	{
		Qaullib_DbPopulateConfig();
		Qaullib_FilePopulate();
	}

	// initialize linked lists
	Qaullib_UserInit();
	Qaullib_MsgInit();
	Qaullib_FileInit();

	// initialize exe discovery
	Qaullib_ExeInit();

#ifdef WIN32
	// needs to be called before socket()
	WSADATA data;
	WSAStartup(MAKEWORD(2,2), &data);
#endif // WIN32
}

// ------------------------------------------------------------
void Qaullib_Exit(void) //destructor
{
	if(ipc_connected)
	{
		// send exit message to olsrd
		Qaullib_IpcSendCom(0);
		usleep(200000);
		printf("Qaullib exit message sent\n");
		Qaullib_IpcClose();
	}
	else printf("ipc not connected\n");

	// clean up qaullib
	sqlite3_close(db);

#ifdef WIN32
	(void) WSACleanup();
#endif // WIN32
}

// ------------------------------------------------------------
// configuration
// ------------------------------------------------------------
void Qaullib_ConfigStart(void)
{
	qaul_loading_wait = 0;
}

// deprecated use Qaullib_SetConf() instead
void Qaullib_SetConfQuit(void)
{
	qaul_conf_quit = 1;
}

void Qaullib_SetConf(int conf)
{
	if(conf == QAUL_CONF_QUIT)
		qaul_conf_quit = 1;
	else if(conf == QAUL_CONF_IOS)
		qaul_conf_ios = 1;
	else if(conf == QAUL_CONF_INTERFACE)
		qaul_conf_interface = 1;
	else if(conf == QAUL_CONF_INTERNET)
		qaul_conf_internet = 1;
	else if(conf == QAUL_CONF_NETWORK)
		qaul_conf_network = 1;
}

int Qaullib_CheckConf(int conf)
{
	if(conf == QAUL_CHECK_WIFI_SET)
		return qaul_conf_wifi_set;

	return 0;
}

void Qaullib_SetConfVoIP(void)
{
	if(Qaullib_VoipStart())
		qaul_conf_voip = 1;
}

void Qaullib_SetConfDownloadFolder(const char *path)
{
	if(QAUL_DEBUG)
		printf("Qaullib_SetConfDownloadFolder\n");

	if(strlen(path) <= MAX_PATH_LEN +1)
	{
		if(QAUL_DEBUG)
			printf("Qaullib_SetConfDownloadFolder set path: %s\n", path);

		strncpy(qaullib_FileDownloadFolderPath, path, MAX_PATH_LEN);
		memcpy(&qaullib_FileDownloadFolderPath[MAX_PATH_LEN], "\0", 1);
		qaul_conf_filedownloadfolder_set = 1;
	}
}

int Qaullib_ExistsLocale(void)
{
	// if user name is set return it
	if (qaul_locale_set) return qaul_locale_set;

	// check if user name is in Database
	qaul_locale_set = Qaullib_DbGetConfigValue("locale", qaul_locale);
	return qaul_locale_set;
}

const char* Qaullib_GetLocale(void)
{
	// if user name is set return it
	if (qaul_locale_set) return qaul_locale;

	// check if user name is in Database
	if (Qaullib_DbGetConfigValue("locale", qaul_locale)) return qaul_locale;

	// no user name found
	return "";
}

void Qaullib_SetLocale(const char* locale)
{
	Qaullib_DbSetConfigValue("locale", locale);
	strcpy(qaul_locale, locale);
	qaul_locale_set = 1;
}

// ------------------------------------------------------------
// timed functions
// ------------------------------------------------------------
void Qaullib_TimedSocketReceive(void)
{
	// check ipc socket
	Qaullib_IpcReceive();

	// check user & file sockets
	//Qaullib_UserCheckSockets();
	//Qaullib_FileCheckSockets();

	// check UDP sockets
	Qaullib_UDP_CheckSocket();

	// check web server
	//mg_poll_server(qaul_webserver_instance, 1024);
}

int Qaullib_TimedCheckAppEvent(void)
{
	return Qaullib_Appevent_LL_Get();
}

void Qaullib_TimedDownload(void)
{
	// download user names
	Qaullib_UserCheckNonames();
	// discover executables for download
	Qaullib_ExeScheduleDiscovery();
	// discover and download scheduled files
	Qaullib_FileCheckScheduled();
	// delete users
	Qaullib_User_LL_Clean();
}

// ------------------------------------------------------------
// App Events
// ------------------------------------------------------------
const char* Qaullib_GetAppEventOpenPath(void)
{
	return qaullib_AppEventOpenPath;
}

// ------------------------------------------------------------
const char* Qaullib_GetAppEventOpenURL(void)
{
	return qaullib_AppEventOpenURL;
}

// ------------------------------------------------------------
int Qaullib_WebserverStart(void)
{
	if(QAUL_DEBUG)
		printf("Qaullib_WebserverStart \n");

	//qaul_webserver_instance = mg_create_server(options, (mg_handler_t) Qaullib_WwwEvent_handler);
	qaul_webserver_instance = mg_create_server(NULL, (mg_handler_t) Qaullib_WwwEvent_handler);
	mg_set_option(qaul_webserver_instance, "listening_port", CHAT_PORT);
	mg_set_option(qaul_webserver_instance, "document_root", webPath);
	//mg_set_option(qaul_webserver_instance, "num_threads", "60");

	mg_start_thread(Qaullib_Www_Server, qaul_webserver_instance);

	return 1;
}

// ------------------------------------------------------------
// SQLite functions
// ------------------------------------------------------------
int Qaullib_DbInit(void)
{
	// create msg-table
	if(sqlite3_exec(db, sql_msg_table, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating message table failed \n");
	}
	else if(sqlite3_exec(db, sql_msg_index, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating message index failed \n");
	}

	// create config-table
	if(sqlite3_exec(db, sql_config_table, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating config table failed \n");
	}

	// create user table
	if(sqlite3_exec(db, sql_user_table, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating user table failed \n");
	}
	else if(sqlite3_exec(db, sql_user_index, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating user index failed \n");
	}

	// create file table
	if(sqlite3_exec(db, sql_file_table, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating file table failed \n");
	}
	else if(sqlite3_exec(db, sql_file_index, NULL, NULL, NULL) != SQLITE_OK)
	{
		printf("creating file index failed \n");
	}

	return 1;
}

// ------------------------------------------------------------
// string protection functionality

int Qaullib_StringDescription2Filename(char *filename, struct qaul_file_LL_item *file, int buffer_size)
{
	int i, j;

	if(QAUL_DEBUG)
		printf("Qaullib_StringDescription2Filename\n");

	printf("strlen: %i\n", (int)strlen(file->description));

	j=0;
	// convert description to file name
	for(i=0; i<(int)strlen(file->description); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(file->description +i, "\0", 1) == 0)
		{
			break;
		}
		else if(
				memcmp(file->description +i, "\"", 1)==0 ||
				memcmp(file->description +i, "'", 1)==0 ||
				memcmp(file->description +i, " ", 1)==0 ||
				memcmp(file->description +i, "<", 1)==0 ||
				memcmp(file->description +i, ">", 1)==0 ||
				memcmp(file->description +i, "\\", 1)==0 ||
				memcmp(file->description +i, "/", 1)==0 ||
				memcmp(file->description +i, ":", 1)==0 ||
				memcmp(file->description +i, "|", 1)==0 ||
				memcmp(file->description +i, "?", 1)==0 ||
				memcmp(file->description +i, "*", 1)==0 ||
				memcmp(file->description +i, ".", 1)==0 ||
				memcmp(file->description +i, "~", 1)==0 ||
				memcmp(file->description +i, "$", 1)==0 ||
				memcmp(file->description +i, "^", 1)==0
				)
		{
			memcpy(filename +j, "_", 1);
			j++;
		}
		else
		{
			memcpy(filename +j, file->description +i, 1);
			j++;
		}
	}

	if(j < buffer_size -1)
	{
		memcpy(filename +j, "_", 1);
		j++;
	}

	// add short hash part
	for(i=0; i<5; i++)
	{
		if(j >= buffer_size -1)
			break;

		memcpy(filename +j, file->hashstr +i, 1);
		j++;
	}

	// add suffix
	if(j < buffer_size -1)
	{
		memcpy(filename +j, ".", 1);
		j++;
	}
	for(i=0; i<strlen(file->suffix); i++)
	{
		if(j >= buffer_size -1)
			break;

		memcpy(filename +j, file->suffix +i, 1);
		j++;
	}

	memcpy(filename +j, "\0", 1);
	return j;
}

int Qaullib_StringMsgProtect(char *protected_string, char *unprotected_string, int buffer_size)
{
	int i, j;

	j=0;
	for(i=0; i<strlen(unprotected_string); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(unprotected_string +i, "\0", 1)==0)
		{
			break;
		}
		else if(memcmp(unprotected_string +i, "\"", 1)==0)
		{
			memcpy(protected_string +j, "'", 1);
			j++;
		}
		else
		{
			memcpy(protected_string +j, unprotected_string +i, 1);
			j++;
		}
	}

	memcpy(protected_string +j, "\0", 1);
	return j;
}

int Qaullib_StringNameProtect(char *protected_string, char *unprotected_string, int buffer_size)
{
	int i, j;

	if(QAUL_DEBUG)
		printf("Qaullib_StringNameControl\n");

	j=0;
	for(i=0; i<strlen(unprotected_string); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(unprotected_string +i, "\0", 1)==0)
		{
			break;
		}
		else if(memcmp(unprotected_string +i, "\"", 1)==0)
		{
			memcpy(protected_string +j, "'", 1);
			j++;
		}
		else if(memcmp(unprotected_string +i, " ", 1)==0)
		{
			memcpy(protected_string +j, "_", 1);
			j++;
		}
		else
		{
			memcpy(protected_string +j, unprotected_string +i, 1);
			j++;
		}
	}

	memcpy(protected_string +j, "\0", 1);
	return j;
}

int Qaullib_StringJsonProtect(char *protected_string, char *unprotected_string, int buffer_size)
{
	int i, j;

	if(QAUL_DEBUG)
		printf("Qaullib_StringJsonProtect\n");

	j=0;
	for(i=0; i<strlen(unprotected_string); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(unprotected_string +i, "\0", 1)==0)
		{
			break;
		}
		else if(memcmp(unprotected_string +i, "\"", 1)==0)
		{
			memcpy(protected_string +j, "'", 1);
			j++;
		}
		else if(memcmp(unprotected_string +i, "\\", 1)==0)
		{
			if(j < buffer_size -2)
			{
				memcpy(protected_string +j, "\\", 1);
				j++;
				memcpy(protected_string +j, "\\", 1);
				j++;
			}
		}
		else
		{
			memcpy(protected_string +j, unprotected_string +i, 1);
			j++;
		}
	}

	memcpy(protected_string +j, "\0", 1);
	return j;
}

int Qaullib_StringDbProtect(char *protected_string, char *unprotected_string, int buffer_size)
{
	int i, j;

	j=0;
	for(i=0; i<strlen(unprotected_string); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(unprotected_string +i, "\0", 1)==0)
		{
			break;
		}
		else if(memcmp(unprotected_string +i, "\"", 1)==0)
		{
			memcpy(protected_string +j, "'", 1);
			j+=2;
		}
		else if(memcmp(unprotected_string +i, "\\", 1)==0)
		{
			if(j < buffer_size -2)
			{
				memcpy(protected_string +j, "\\\\", 2);
				j+=2;
			}
			else
				break;
		}
		else
		{
			memcpy(protected_string +j, unprotected_string +i, 1);
			j++;
		}
	}

	memcpy(protected_string +j, "\0", 1);

	return j;
}

int Qaullib_StringDbUnprotect(char *unprotected_string, char *protected_string, int buffer_size)
{
	int i, j;

	if(QAUL_DEBUG)
		printf("Qaullib_StringDbUnprotect\n");

	j=0;
	for(i=0; i<strlen(protected_string); i++)
	{
		if(j >= buffer_size -1)
			break;

		if(memcmp(protected_string +i, "\0", 1)==0)
		{
			break;
		}
		else if(memcmp(protected_string +i, "\\", 1)==0)
		{
			i++;
			memcpy(unprotected_string +j, protected_string +i, 1);
			j++;
		}
		else
		{
			memcpy(unprotected_string +j, protected_string +i, 1);
			j++;
		}
	}

	memcpy(unprotected_string +j, "\0", 1);
	return j;
}

// ------------------------------------------------------------
// get and set configuration
void Qaullib_DbSetConfigValue(const char* key, const char* value)
{
	char buffer[10240];
	char *stmt = buffer;
	char *error_exec=NULL;

	if(QAUL_DEBUG)
		printf("Qaullib_DbSetConfigValue %s %s\n", key, value);

	// delete old entries (if exist)
	sprintf(stmt, sql_config_delete, key);
	if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
	{
		printf("SQLite error: %s\n",error_exec);
		sqlite3_free(error_exec);
		error_exec=NULL;
	}

	// insert new value
	sprintf(stmt, sql_config_set, key, value);

	if(QAUL_DEBUG)
		printf("stmt: %s\n", stmt);

	if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
	{
		printf("SQLite error: %s\n", error_exec);
		sqlite3_free(error_exec);
		error_exec=NULL;
	}
}

void Qaullib_DbSetConfigValueInt(const char* key, int value)
{
	char buffer[10240];
	char *stmt = buffer;
	char *error_exec=NULL;

	if(QAUL_DEBUG)
		printf("Qaullib_DbSetConfigValueInt\n");

	// delete old entries (if exist)
	sprintf(stmt, sql_config_delete, key);
	if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
	{
		printf("SQLite error: %s\n",error_exec);
		sqlite3_free(error_exec);
		error_exec=NULL;
	}

	// insert new value
	sprintf(stmt, sql_config_set_int, key, value);

	if(QAUL_DEBUG)
		printf("stmt: %s\n", stmt);

	if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
	{
		printf("SQLite error: %s\n", error_exec);
		sqlite3_free(error_exec);
		error_exec=NULL;
	}
}

int Qaullib_DbGetConfigValue(const char* key, char *value)
{
	sqlite3_stmt *ppStmt;
	char buffer[10240];
	char *stmt = buffer;
	int value_found = 0;

	// check if key exists in Database
	sprintf(stmt, sql_config_get, key);
	if( sqlite3_prepare_v2(db, stmt, -1, &ppStmt, NULL) != SQLITE_OK )
	{
		printf("SQLite error: %s\n",sqlite3_errmsg(db));
	}
	else
	{
		// For each row returned
		while (sqlite3_step(ppStmt) == SQLITE_ROW)
		{
		  // For each collumn
		  int jj;
		  for(jj=0; jj < sqlite3_column_count(ppStmt); jj++)
		  {
				if(strcmp(sqlite3_column_name(ppStmt,jj), "value") == 0)
				{
					sprintf(value,"%s",sqlite3_column_text(ppStmt, jj));
					value_found = 1;
					break;
				}
		  }
		}
		sqlite3_finalize(ppStmt);
	}
	return value_found;
}

int Qaullib_DbGetConfigValueInt(const char* key)
{
	sqlite3_stmt *ppStmt;
	char buffer[1024];
	char *stmt = buffer;
	int myvalue = 0;

	sprintf(stmt, sql_config_get, key);
	if( sqlite3_prepare_v2(db, stmt, -1, &ppStmt, NULL) != SQLITE_OK )
	{
		printf("SQLite error: %s\n",sqlite3_errmsg(db));
	}
	else
	{
		// For each row returned
		while (sqlite3_step(ppStmt) == SQLITE_ROW)
		{
			// For each collumn
			int jj;
			for(jj=0; jj < sqlite3_column_count(ppStmt); jj++)
			{
				if(strcmp(sqlite3_column_name(ppStmt,jj), "value_int") == 0)
				{
					myvalue = sqlite3_column_int(ppStmt, jj);
					return myvalue;
				}
			}
		}
		sqlite3_finalize(ppStmt);
	}
	return myvalue;
}

// ------------------------------------------------------------
void Qaullib_DbPopulateConfig(void)
{
	char buffer[10240];
	char *stmt = buffer;
	char *error_exec=NULL;
	int i;

	// loop trough entries
	for(i=0; i<MAX_POPULATE_CONFIG; i++)
	{
		// write entry into DB
		sprintf(stmt,
				sql_config_set_all,
				qaul_populate_config[i].key,
				qaul_populate_config[i].type,
				qaul_populate_config[i].value,
				qaul_populate_config[i].value_int
				);

		if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
		{
			printf("SQLite error: %s\n",error_exec);
			sqlite3_free(error_exec);
			error_exec=NULL;
		}
	}
}

// ------------------------------------------------------------
// configure user name
char* Qaullib_GetUsername(void)
{
	// if username is set return it
	if(qaul_username_set)
		return qaul_username;

	// check if username is in Database
	if(Qaullib_DbGetConfigValue("username", qaul_username))
		return qaul_username;

	// no username found
	return "";
}

int Qaullib_ExistsUsername(void)
{
	// if username is set return it
	if(qaul_username_set)
		return qaul_username_set;

	// check if username is in Database
	qaul_username_set = Qaullib_DbGetConfigValue("username", qaul_username);

	return qaul_username_set;
}

int Qaullib_SetUsername(char* name)
{
	int size;
	char namebuf[MAX_USER_LEN*2 +1];

	Qaullib_StringDbProtect(namebuf, name, sizeof(namebuf));
	Qaullib_DbSetConfigValue("username", name);
	strncpy(qaul_username, name, MAX_USER_LEN);
	memcpy(&qaul_username[MAX_USER_LEN], "\0", 1);
	qaul_username_set = 1;
	return 1;
}

// ------------------------------------------------------------
void Qaullib_FilePicked(int check, const char* path)
{
	strncpy(pickFilePath, path, MAX_PATH_LEN);
	memcpy(&pickFilePath[MAX_PATH_LEN], "\0", 1);
	pickFileCheck = check;
}

// ------------------------------------------------------------
// configure IP
const char* Qaullib_GetIP(void)
{
	// if IP is set return it
	if (qaul_ip_set)
		return qaul_ip_str;

	qaul_ip_version = AF_INET;
	qaul_ip_size = sizeof(struct in_addr);

	// check if username is in Database
	if (Qaullib_DbGetConfigValue("ip", qaul_ip_str))
	{
		// create ip bin
		// FIXME: ipv6
		inet_pton(AF_INET, qaul_ip_str, &qaul_ip_addr.v4);

		qaul_ip_set = 1;
		// return string
		return qaul_ip_str;
	}

	// create new IP
	Qaullib_CreateIP(qaul_ip_str);
	// write IP into config
	Qaullib_SetIP(qaul_ip_str);
	// return IP
	return qaul_ip_str;
}

int Qaullib_SetIP(const char* IP)
{
	Qaullib_DbSetConfigValue("ip", IP);
	strcpy(qaul_ip_str, IP);
	// create ip bin
	// FIXME: ipv6
	inet_pton(AF_INET, qaul_ip_str, &qaul_ip_addr.v4);

	qaul_ip_set = 1;
	return 1;
}

void Qaullib_CreateIP(char* IP)
{
	// todo: take network-card-number or processor-number into account
	srand(time(NULL));
	int rand1 = rand()%255;
	int rand2 = rand()%255;
	int rand3 = (rand()%254)+1;

	sprintf(IP,"10.%i.%i.%i",rand1,rand2,rand3);
}


// ------------------------------------------------------------
int Qaullib_GetConfString(const char *key, char *value)
{
	memcpy(value, "\0", 1);
	Qaullib_DbGetConfigValue(key, value);

	return strlen(value);
}

int Qaullib_GetConfInt(const char *key)
{
	return Qaullib_DbGetConfigValueInt(key);
}

void Qaullib_SetConfString(const char *key, const char *value)
{
	Qaullib_DbSetConfigValue(key, value);
}

void Qaullib_SetConfInt(const char *key, int value)
{
	Qaullib_DbSetConfigValueInt(key, value);
}

// ------------------------------------------------------------
const char* Qaullib_GetNetProfile(void)
{
	if (Qaullib_GetConfString("net.profile", qaul_net_profile))
	{
		return qaul_net_profile;
	}
	return "custom";
}

int Qaullib_GetNetProtocol(void)
{
	int protocol = Qaullib_DbGetConfigValueInt("net.protocol");
	if (protocol > 0)
	{
		return protocol;
	}
	else return 4;
}

int Qaullib_GetNetMask(void)
{
	return Qaullib_DbGetConfigValueInt("net.mask");
}

const char* Qaullib_GetNetBroadcast(void)
{
	if (Qaullib_DbGetConfigValue("wifi.ssid", qaul_net_broadcast))
	{
		return qaul_net_ssid;
	}
	return "";
}

const char* Qaullib_GetNetGateway(void)
{
	if (Qaullib_DbGetConfigValue("net.gateway", qaul_net_gateway))
	{
		return qaul_net_gateway;
	}
	return "0.0.0.0";
}

const char* Qaullib_GetNetNs1(void)
{
	if (Qaullib_DbGetConfigValue("net.ns1", qaul_net_ns1))
	{
		return qaul_net_ns1;
	}
	return "5.45.96.220";
}

const char* Qaullib_GetNetNs2(void)
{
	if (Qaullib_DbGetConfigValue("net.ns2", qaul_net_ns2))
	{
		return qaul_net_ns2;
	}
	return "185.82.22.133";
}

const char* Qaullib_GetWifiSsid(void)
{
	if (Qaullib_DbGetConfigValue("wifi.ssid", qaul_net_ssid))
	{
		return qaul_net_ssid;
	}
	return "";
}

int Qaullib_GetWifiBssIdSet(void)
{
	return Qaullib_DbGetConfigValueInt("wifi.bssid_set");
}

const char* Qaullib_GetWifiBssId(void)
{
	if (Qaullib_DbGetConfigValue("wifi.bssid", qaul_net_bssid))
	{
		return qaul_net_bssid;
	}
	return "";
}

int Qaullib_GetWifiChannel(void)
{
	return Qaullib_DbGetConfigValueInt("wifi.channel");
}

int Qaullib_GetInterfaceManual(void)
{
	return Qaullib_DbGetConfigValueInt("net.interface.manual");
}

void Qaullib_SetInterfaceManual(int value)
{
	Qaullib_DbSetConfigValueInt("net.interface.manual", value);
}

const char* Qaullib_GetInterface(void)
{
	if (Qaullib_DbGetConfigValue("net.interface.name", qaul_net_interface))
	{
		return qaul_net_interface;
	}
	return "";
}

void Qaullib_SetInterfaceJson(const char *json)
{
	if(QAUL_DEBUG)
		printf("Qaullib_SetInterfaceJson\n");

	if(strlen(json) <= MAX_JSON_LEN)
	{
		if(QAUL_DEBUG)
				printf("strlen(json) <= MAX_JSON_LEN\n");

		strncpy(qaul_interface_json, json, MAX_JSON_LEN);
		memcpy(&qaul_interface_json[MAX_JSON_LEN], "\0", 1);
	}
	else
		memcpy(&qaul_interface_json[0], "\0", 1);

	qaul_interface_configuring = 2;
}

// ------------------------------------------------------------
void Qaullib_ConfigurationFinished(void)
{
	qaul_configured = 1;
}

// ------------------------------------------------------------
int Qaullib_Timestamp2Isostr(char *str_buffer, int timestamp, int str_buffer_size)
{
    time_t     now;
    struct tm *ts;

	if(QAUL_DEBUG)
		printf("sizes int: %lu, size time_t: %lu\n", sizeof(timestamp), sizeof(now));

    // convert integer to time_t
    now = 1 * timestamp;

    // Format and print the time, "yyyy-mm-dd hh:mm:ss"
    ts = localtime(&now);
	strftime(str_buffer, str_buffer_size, "%Y-%m-%d %H:%M:%S", ts);

	if(QAUL_DEBUG)
		printf("str_buffer %s\n", str_buffer);

    return 1;
}
