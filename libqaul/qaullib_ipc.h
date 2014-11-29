/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * IPC / interprocess communication between qaullib and olsrd_qaul plugin
 *
 * functions in the public API
 *   int Qaullib_IpcConnect(void);
 *   void Qaullib_IpcSendCom(int commandId);
 * @see qaullib.h
 */

#ifndef _QAULLIB_IPC
#define _QAULLIB_IPC

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * close interprocess communication connection
 */
int Qaullib_IpcClose(void);

/**
 * check if something has been received
 */
void Qaullib_IpcReceive(void);

/**
 * evaluate message type to process the message
 */
void Qaullib_IpcEvaluateMessage(union olsr_message *msg);

/**
 * process chat message
 */
void Qaullib_IpcEvaluateChat(union olsr_message *msg);

/**
 * check message type of @a msg
 */
void Qaullib_IpcEvaluateCom(union olsr_message *msg);

/**
 * check if user exists in user table, create it if not
 */
void Qaullib_IpcEvaluateTopo(union olsr_message *msg);

/**
 * write network topology links into linked list
 */
void Qaullib_IpcEvaluateMeshtopo(union olsr_message *msg);

/**
 * process user hello message
 *
 * fill in new user name to user LL
 */
void Qaullib_IpcEvaluateUserhello(union olsr_message *msg);

/**
 * process file discover message
 *
 * check if you store this file, then answer,
 * otherwise ignore this message
 */
void Qaullib_IpcEvaluateFilediscover(union olsr_message *msg);

/**
 * process executable discover message
 *
 * check if you store the requested executables, then answer,
 * otherwise ignore this message.
 */
void Qaullib_IpcEvaluateExediscover(union olsr_message *msg);

/**
 * send message @msg over ipc
 */
void Qaullib_IpcSend(union olsr_message *msg);

/**
 * send user hello message
 */
void Qaullib_IpcSendUserhello(void);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
