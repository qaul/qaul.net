/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

 /*
  * qaul olsrd plugin
  */

#include <stdio.h>
#include <string.h>

#include "../../src/olsrd_plugin.h"

#include "qaul_olsrd_plugin.h"
#include "qaul_msg.h"
#include "qaul_ipc.h"
#include "qaul_net.h"

#define PLUGIN_INTERFACE_VERSION 5

// ipc plugin
int qaul_ipc_port = 8112;
struct allowed_net *qaul_ipc_allowed_nets = NULL;


/****************************************************************************
 *                Functions that the plugin MUST provide                    *
 ****************************************************************************/

/**
 * Plugin interface version
 * Used by main olsrd to check plugin interface version
 */
int
olsrd_plugin_interface_version(void)
{
  return PLUGIN_INTERFACE_VERSION;
}

static int
add_plugin_access(const char *value, void *data, set_plugin_parameter_addon addon __attribute__ ((unused)))
{
  struct olsr_ip_prefix prefix;
  struct allowed_net **my_allowed_nets = data;
  struct allowed_net *an;

  if (olsr_string_to_prefix(olsr_cnf->ip_version, &prefix, value)) {
    fprintf(stderr, "[QAUL] unknown access restriction parameter: %s!\n", value);
    exit(0);
  }

  an = olsr_malloc(sizeof(*an), __func__);
  if (an == NULL) {
    fprintf(stderr, "[QAUL] register param net out of memory!\n");
    exit(0);
  }

  an->prefix = prefix;
  an->next = *my_allowed_nets;
  *my_allowed_nets = an;
  return 0;
}



/**
 * Register parameters from config file
 * Called for all plugin parameters
 */
// FIXME: is this still needed?
static const struct olsrd_plugin_parameters plugin_parameters[] = {
  // ipc params
  {.name = "qaul_ipc_port",.set_plugin_parameter = &set_plugin_port,.data = &qaul_ipc_port},
  {.name = "qaul_ipc_host",.set_plugin_parameter = &add_plugin_access,.data = &qaul_ipc_allowed_nets},
  {.name = "qaul_ipc_net",.set_plugin_parameter = &add_plugin_access,.data = &qaul_ipc_allowed_nets},
};

void
olsrd_get_plugin_parameters(const struct olsrd_plugin_parameters **params, int *size)
{
  *params = plugin_parameters;
  *size = sizeof(plugin_parameters) / sizeof(*plugin_parameters);
}


// ------------------------------------------------------------
// initialize  plugin
// ------------------------------------------------------------

int
olsrd_plugin_init(void)
{
  // init chat
  qaul_msg_init();

  // init ipc
  qaul_ipc_init();

  return 1;
}




/****************************************************************************
 *       Optional private constructor and destructor functions              *
 ****************************************************************************/

/* attention: make static to avoid name clashes */

static void my_init(void) __attribute__ ((constructor));
static void my_fini(void) __attribute__ ((destructor));

/**
 * Optional Private Constructor
 */
static void
my_init(void)
{
  printf("*** QAUL: constructor\n");
  setlocale(LC_CTYPE, "de_DE.UTF-8");
}

/**
 * Optional Private Destructor
 */
static void
my_fini(void)
{
  printf("*** QAUL: destructor\n");
}

