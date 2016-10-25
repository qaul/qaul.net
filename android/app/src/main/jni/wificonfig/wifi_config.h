
#ifndef _WIFI_CONFIG_H
#define _WIFI_CONFIG_H

/**
 * Close the wpa_ctrl connection.
 */
static void wpa_close_control_connection(void);

/**
 * Open the wpa_ctrl connection.
 *
 * @param wpa_directory the directory containing the interface to open.
 * @return wpa_ctrl a static pointer to the actual wpa_supplicant connection.
 */
static struct wpa_ctrl *wpa_open_control_connection(const char *wpa_directory);

/**
 * Simple callback function for printing wpa_ctrl messages.
 */
static void wpa_message_callback(char *message, size_t length);

/**
 * Print the wifi scan results.
 */
static void wpa_print_scan_results(void);

#endif /* _WIFI_CONFIG_H */
