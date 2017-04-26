/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _OLSRD_PLUGIN_QAUL
#define _OLSRD_PLUGIN_QAUL

#include <sys/types.h>
#include <netinet/in.h>
#include <sys/socket.h>
#include <sys/times.h>
#include <arpa/inet.h>
#include <sys/time.h>
#include <time.h>
#include <math.h>
#include <locale.h>
#include <wchar.h>

#include "olsr.h"
#include "plugin_util.h"
#include "olsr_types.h"
#include "neighbor_table.h"
#include "two_hop_neighbor_table.h"
#include "tc_set.h"
#include "hna_set.h"
#include "mid_set.h"
#include "mpr_selector_set.h"
#include "routing_table.h"


/**
 * qaul message types
 */
#define QAUL_CHAT_MESSAGE_TYPE 	        222
#define QAUL_CHAT_PARSER_TYPE	        QAUL_CHAT_MESSAGE_TYPE
#define QAUL_IPCCOM_MESSAGE_TYPE        223
#define QAUL_IPCTOPO_MESSAGE_TYPE       224
#define QAUL_USERHELLO_MESSAGE_TYPE     225
#define QAUL_USERHELLO_PARSER_TYPE	    QAUL_USERHELLO_MESSAGE_TYPE
#define QAUL_FILEDISCOVER_MESSAGE_TYPE  226
#define QAUL_FILEDISCOVER_PARSER_TYPE   QAUL_FILEDISCOVER_MESSAGE_TYPE
#define QAUL_FILEAVAILABLE_MESSAGE_TYPE 227
#define QAUL_FILEAVAILABLE_PARSER_TYPE  QAUL_FILEAVAILABLE_MESSAGE_TYPE
#define QAUL_EXEDISCOVER_MESSAGE_TYPE   228
#define QAUL_EXEDISCOVER_PARSER_TYPE    QAUL_EXEDISCOVER_MESSAGE_TYPE
#define QAUL_EXEAVAILABLE_MESSAGE_TYPE  229
#define QAUL_EXEAVAILABLE_PARSER_TYPE   QAUL_EXEAVAILABLE_MESSAGE_TYPE
#define QAUL_IPCMESHTOPO_MESSAGE_TYPE   230

/**
 * IPC messages
 */
#define QAUL_IPCCOM_QUIT                0
#define QAUL_IPCCOM_GETTOPO             1
#define QAUL_IPCCOM_GETMESHTOPO         2
#define QAUL_IPCCOM_MESHTOPO_SENT        3


#define PLUGIN_NAME    "qaul.net mesh plugin"
#define PLUGIN_VERSION "0.1"
#define PLUGIN_AUTHOR  "http://qaul.net project"
#define MOD_DESC PLUGIN_NAME " " PLUGIN_VERSION " by " PLUGIN_AUTHOR
#define PLUGIN_INTERFACE_VERSION 5

// global variables
extern int qaul_ipc_port;


struct allowed_net {
  struct olsr_ip_prefix prefix;
  struct allowed_net *next;
};

/****************************************************************************
 *                Functions that the plugin MUST provide                    *
 ****************************************************************************/

/* Initialization function */
int olsrd_plugin_init(void);

int olsrd_plugin_interface_version(void);

#endif

