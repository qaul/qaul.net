/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */


#ifndef QAUL_NET_MESSAGING_H
#define QAUL_NET_MESSAGING_H


// TODO: This function needs to be handed a database state
ql_error_t qlmsg_initialise();

ql_error_t qlmsg_encode();

ql_error_t qlmsg_decode();


#endif //QAUL_NET_MESSAGING_H
