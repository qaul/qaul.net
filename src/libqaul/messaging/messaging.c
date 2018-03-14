/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <qaul/mod/messaging.h>
#include <msgpack/object.h>


ql_error_t qlmsg_initialise()
{

    return SUCCESS;
}


ql_error_t qlmsg_encode(struct ql_message *msg, unsigned char **msgpack)
{


    return SUCCESS;
}

ql_error_t qlmsg_decode(const unsigned char *msgpack, struct ql_message **msg)
{

}