/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_NET_MESSAGING_H
#define QAUL_NET_MESSAGING_H

#include <qaul/error.h>
#include <qaul/mod/structures.h>

// TODO: This function needs to be handed a database state
ql_error_t qlmsg_initialise();

/**
 * Encode a ql_message struct into a messagepack binary string
 *
 * @param msg
 * @param msgpack
 * @return
 */
ql_error_t qlmsg_encode(struct ql_message *msg, unsigned char **msgpack);


/**
 * Decode a messagepack binary string into a ql_message struct
 *
 * @param msgpack
 * @param msg
 * @return
 */
ql_error_t qlmsg_decode(const unsigned char *msgpack, struct ql_message **msg);


#endif //QAUL_NET_MESSAGING_H
