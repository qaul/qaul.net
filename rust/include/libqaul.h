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
