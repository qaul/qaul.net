#include <stdio.h>
#include <stdlib.h>
#include <stdarg.h>
#include <unistd.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/types.h>
#include <android/log.h>
#include "log.h"

#include "wpa_ctrl.h"
#include "wifi_config.h"

static struct wpa_ctrl *control_connection = NULL;

static void wpa_close_control_connection(void) {
  if (control_connection == NULL) 
    return;
  
  wpa_ctrl_close(control_connection);
  
  control_connection = NULL;
}

static struct wpa_ctrl *wpa_open_control_connection(const char *ifname)
{
  if (ifname == NULL) {
    return NULL;
  }
  
  if (control_connection != NULL) {
    wpa_close_control_connection();
  }
  
  control_connection = wpa_ctrl_open(ifname);
  return control_connection;
}

void wpa_message_callback(char *message, size_t length) 
{
  android_syslog(ANDROID_LOG_INFO, "%s\n", message);
}

static void wpa_print_results(char *command) 
{
  char buffer[4096];
  size_t length;
  int ret;
  
  if (control_connection == NULL) {
    android_syslog(ANDROID_LOG_ERROR, "Error: Not connected to wpa_supplicant");
    return;
  }
  
  length = sizeof(buffer) - 1;
  ret = wpa_ctrl_request(control_connection, command, strlen(command), buffer, &length, wpa_message_callback);

  if (ret == -2) {
    android_syslog(ANDROID_LOG_ERROR, "'%s' command timed out.", command);
    return;
  }
  else if (ret < 0) {
    android_syslog(ANDROID_LOG_ERROR, "'%s' command failed", command);
    return;
  }
  
  buffer[length] = '\0';
  printf("%s", buffer);

  return;
}

int main(int argc, char *argv[])
{

  if (argc != 3)
    return 1;
  else if (wpa_open_control_connection(argv[1]) == NULL)
    return 2;
  else 
    wpa_print_results(argv[2]);
 
  wpa_close_control_connection();
  return 0;
 
}
