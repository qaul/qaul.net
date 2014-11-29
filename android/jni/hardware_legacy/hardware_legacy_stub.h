#include <stdlib.h>
int wifi_load_driver();
int wifi_unload_driver();
int wifi_start_supplicant();
int wifi_stop_supplicant();
int wifi_connect_to_supplicant();
void wifi_close_supplicant_connection();
int wifi_wait_for_event(char *buf, size_t len);
int wifi_command(const char *command, char *reply, size_t *reply_len);
