#include "hardware_legacy_stub.h"
int wifi_load_driver() { return -1; }
int wifi_unload_driver() { return -1; }
int wifi_start_supplicant() { return -1; }
int wifi_stop_supplicant() { return -1; }
int wifi_connect_to_supplicant() { return -1; }
void wifi_close_supplicant_connection() { }
int wifi_wait_for_event(char *buf, size_t len) { return -1; }
int wifi_command(const char *command, char *reply, size_t *reply_len)  { return -1; }
