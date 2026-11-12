#import "GeneratedPluginRegistrant.h"

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * test function
 */
char *hello(void);

/**
 * start libqaul in an own thread
 *
 * This function initializes and starts libqaul.
 * It needs to be called before any other function
 * of this API.
 */
void start(const char *s);

/**
 * start libqaul on desktop operating systems
 *
 * This function supports the following systems:
 * linux, macOS, windows
 *
 * libqaul will create and find the common paths
 * to the data storage location. Therefore no path has
 * to be provided.
 */
void start_desktop(void);

/**
 * check if libqaul finished initializing
 *
 * Returns 1 when it finished, otherwise 0.
 *
 * 1: initialization finished
 * 0: libqaul not initialized
 *
 * Don't send any messages to libqaul before it finished initializing.
 */
int32_t initialized(void);

/**
 * Yields the total number of messages queued to be received.
 */
int32_t receivequeue(void);

/**
 * send RPC messages to libqaul
 *
 * returns 0 on success and negative numbers on failure
 *
 * 0  : success
 * -1 : pointer is null
 * -2 : message is too big
 */
int32_t send_rpc_to_libqaul(const uint8_t *message, uint32_t message_length);

/**
 * receive RPC messages from libqaul
 *
 * You need to provide the pointer to a buffer and declare
 * the length of a buffer.
 * If a message was received, this function copies the message
 * into the buffer.
 *
 * The function returns the length of the message.
 * The return value '0' means no message was received.
 *
 * A negative value is an error.
 * -1 : an error occurred
 * -2 : buffer to small
 * -3 : buffer pointer is null
 */
int32_t receive_rpc_from_libqaul(unsigned char *buffer, uint32_t buffer_length);





/* *************         BLE (ble_module/src/ios/ffi.rs)            ****************/


/**
 * A peripherals CoreBluetooth UUID, 16 raw bytes.
 * Matches rusts #[repr(C)] Address([u8; 16]).
 */
typedef struct { uint8_t bytes[16]; } QaulBleAddress;

/**
 * Rust -> Swift. action:
 * 0 Start,
 * 1 Stop,
 * 2 Connect,
 * 3 Disconnect,
 * 4 Transmit
 * bytes is only valid during the call: copy it before returning.
 * Called from Rust's BLE thread, not the main thread.
 */
typedef void (*QaulBleActionCallback)(uint8_t action, QaulBleAddress addr,
                                      const uint8_t *bytes, uint32_t len);

/** Register the callback for rust once at startup. NULL clears it. */
void qaul_ble_set_callback(QaulBleActionCallback cb);

/** Swift -> Rust radio events */
void qaul_ble_powered(bool on);
void qaul_ble_frame_received(QaulBleAddress addr, const uint8_t *bytes, uint32_t len);
void qaul_ble_frame_sent(QaulBleAddress addr);
void qaul_ble_subscribed(QaulBleAddress addr, uint32_t mtu);
void qaul_ble_advert(QaulBleAddress addr, const uint8_t *id_prefix, uint32_t len, int16_t rssi);
void qaul_ble_peer_id_read(QaulBleAddress addr, const uint8_t *id, uint32_t len);
void qaul_ble_connect_failed(QaulBleAddress addr);
void qaul_ble_gone(QaulBleAddress addr);
