/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"


/**
 * add message to data base
 *
 * @retval 0 on error
 * @retval 1 on success
 */
static int Qaullib_MsgAdd2DB(struct qaul_msg_LL_item *msg_item);


// ------------------------------------------------------------
void Qaullib_MsgInit(void)
{
	struct qaul_msg_LL_node node;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgInit\n");

	// initialize LL
	Qaullib_File_LL_Init();

	// insert messages from DB to LL
	node.item = 0;
	Qaullib_MsgDB2LL(&node, sql_msg_get_latest);
	qaul_msg_LL_first = node.item;
}

// ------------------------------------------------------------
int Qaullib_MsgDB2LL(struct qaul_msg_LL_node *node, const char *stmt)
{
	sqlite3_stmt *ppStmt;
	char *error_exec=NULL;
	struct qaul_msg_LL_item myitem;
	struct qaul_msg_LL_node tmp_node;
	int count;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgDB2LL\n");

	// init node
	tmp_node.item = 0;

	// Select rows from database
	if( sqlite3_prepare_v2(db, stmt, -1, &ppStmt, NULL) != SQLITE_OK )
	{
		printf("SQLite error: %s\n", sqlite3_errmsg(db));
		return 0;
	}

	count = 0;
	while (sqlite3_step(ppStmt) == SQLITE_ROW)
	{
		// For each column
		int jj;
		for(jj=0; jj < sqlite3_column_count(ppStmt); jj++)
		{
		    if(strcmp(sqlite3_column_name(ppStmt,jj), "id") == 0)
			{
		    	myitem.id = sqlite3_column_int(ppStmt, jj);
			}
		    else if(strcmp(sqlite3_column_name(ppStmt,jj), "type") == 0)
			{
		    	myitem.type = sqlite3_column_int(ppStmt, jj);
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "name") == 0)
			{
				sprintf(myitem.name, "%s", sqlite3_column_text(ppStmt, jj));
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "msg") == 0)
			{
				sprintf(myitem.msg, "%s", sqlite3_column_text(ppStmt, jj));
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "ipv") == 0)
			{
		    	myitem.ipv = sqlite3_column_int(ppStmt, jj);
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "ip") == 0)
			{
				sprintf(myitem.ip, "%s", sqlite3_column_text(ppStmt, jj));
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "time") == 0)
			{
		    	myitem.time = sqlite3_column_int(ppStmt, jj);
			}
			else if(strcmp(sqlite3_column_name(ppStmt,jj), "read") == 0)
			{
		    	myitem.read = sqlite3_column_int(ppStmt, jj);
			}
		}

		// add it to LL
		Qaullib_Msg_LL_AddNext(&myitem, &tmp_node);

		if(count == 0)
			node->item = tmp_node.item;

		count++;
	}
	sqlite3_finalize(ppStmt);

	if(count > 0)
		return 1;

	return 0;
}

// ------------------------------------------------------------
int Qaullib_MsgAdd(struct qaul_msg_LL_item *item)
{
	struct qaul_msg_LL_item msg_item;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgAdd\n");

	// protect & validate item values
	msg_item.id = item->id;
	msg_item.type = item->type;
	Qaullib_StringMsgProtect(msg_item.msg, item->msg, sizeof(msg_item.msg));
	Qaullib_StringNameProtect(msg_item.name, item->name, sizeof(msg_item.name));
	msg_item.time = item->time;
	msg_item.read = item->read;
	msg_item.ipv = item->ipv;
	strncpy(msg_item.ip, item->ip, sizeof(msg_item.ip));
	memcpy(&msg_item.ip_union, &item->ip_union, sizeof(msg_item.ip_union));

	// add to DB
	Qaullib_MsgAdd2DB(&msg_item);

	// add to LL
	Qaullib_Msg_LL_Add(&msg_item);

	// set new message flag
	if(msg_item.type != QAUL_MSGTYPE_VOIP_IN && msg_item.type != QAUL_MSGTYPE_VOIP_OUT)
		qaul_new_msg++;

	if(msg_item.type == QAUL_MSGTYPE_PUBLIC_IN || msg_item.type == QAUL_MSGTYPE_PRIVATE_IN)
	{
		// check if user is in data base
		Qaullib_UserCheckUser(&msg_item.ip_union, msg_item.name);

		// check for file download
		if(qaul_file_autodownload == 1)
			Qaullib_MsgCheckFile(&msg_item);
	}

	return 1;
}

// ------------------------------------------------------------
int Qaullib_MsgCheckFile(struct qaul_msg_LL_item *item)
{
	struct qaul_file_LL_item file_item;
	struct qaul_file_LL_item *existing_file;
	time_t timestamp;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgCheckFile\n");

	// check if message contains file advertising information
	if(Qaullib_MsgCheckFileFindHash(item->msg, &file_item) == 1)
	{
		printf("Qaullib_MsgCheckFile [1] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

		// set file size to dummy value
		file_item.size = 1024;

		// set new values
		file_item.type = QAUL_FILETYPE_FILE;
		file_item.status = QAUL_FILESTATUS_NEW;
		time(&timestamp);
		file_item.created_at = (int)timestamp;
		file_item.downloaded = 0;
		file_item.downloaded_chunk = 0;
		Qaullib_StringToHash(file_item.hashstr, file_item.hash);

		// todo: deprecated, to be removed
		strncpy(file_item.adv_name, item->name, sizeof(file_item.adv_name));
		file_item.adv_validip = 0;
		memcpy(&file_item.adv_ip, &item->ip_union, sizeof(file_item.adv_ip));

		printf("Qaullib_MsgCheckFile [2] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

		// check if file already exists
		if(Qaullib_File_LL_HashSearch(file_item.hash, &existing_file))
		{
			printf("Qaullib_MsgCheckFile [3] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

			if(existing_file->status == QAUL_FILESTATUS_DELETED)
			{
				// delete from LL
				Qaullib_File_LL_Delete_Item(existing_file);

				// add the file again
				Qaullib_FileAdd(&file_item);
			}
		}
		else
		{
			printf("Qaullib_MsgCheckFile [4] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

			Qaullib_FileAdd(&file_item);
		}

		printf("Qaullib_MsgCheckFile [5] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

		// check all scheduled files
		Qaullib_FileCheckScheduled();

		printf("Qaullib_MsgCheckFile [6] %s.%s size[%i] status[%i]\n", file_item.hashstr, file_item.suffix, file_item.size, file_item.status);

		return 1;
	}

	return 0;
}

int Qaullib_MsgCheckFileFindHash(char *msg, struct qaul_file_LL_item *file)
{
	int msglen, i, pattern_count, desc_count, msg_found;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgCheckFileFindHash msg: %s\n", msg);

	msglen = (int)strlen(msg);
	pattern_count = 0;
	desc_count = 0;
	msg_found = -1;

	for(i=0; i < msglen; i++)
	{
		if(pattern_count < MAX_HASHSTR_LEN && Qaullib_ValidateCharASCIILetterOrNumber(&msg[i]) == 1)
		{
			memcpy(&file->hashstr[pattern_count], &msg[i], 1);
			pattern_count++;
		}
		else if(pattern_count == MAX_HASHSTR_LEN && memcmp(&msg[i], ".", 1) == 0)
		{
			memcpy(&file->hashstr[pattern_count], "\0", 1);
			pattern_count++;
		}
		else if(pattern_count > MAX_HASHSTR_LEN)
		{
			if(msg_found >= 0)
			{
				if(desc_count <= MAX_DESCRIPTION_LEN)
					memcpy(&file->description[desc_count], &msg[i], 1);

				msg_found = 1;
				pattern_count++;
				desc_count++;
			}
			else if(pattern_count <= (MAX_HASHSTR_LEN + 1 + MAX_SUFFIX_LEN) && Qaullib_ValidateCharASCIILetterOrNumber(&msg[i]) == 1)
			{
				memcpy(&file->suffix[pattern_count -MAX_HASHSTR_LEN -1], &msg[i], 1);
				pattern_count++;
			}
			else if(memcmp(&msg[i], " ", 1) == 0 || memcmp(&msg[i], "|", 1) == 0)
			{
				memcpy(&file->suffix[pattern_count -MAX_HASHSTR_LEN -1], "\0", 1);
				msg_found = 0;
			}
			else
				pattern_count = 0;
		}
		else
		{
			pattern_count = 0;
		}
	}

	if(msg_found == 1)
	{
		printf("Qaullib_MsgCheckFileFindHash %s.%s size[%i] status[%i]\n", file->hashstr, file->suffix, file->size, file->status);

		if(desc_count < MAX_DESCRIPTION_LEN)
			memcpy(&file->description[desc_count], "\0", 1);
		else
			memcpy(&file->description[MAX_DESCRIPTION_LEN], "\0", 1);

		return 1;
	}

	return 0;
}

// ------------------------------------------------------------
int Qaullib_MsgSendPublic(struct qaul_msg_LL_item *item)
{
	char buffer[1024];
	union olsr_message *m;
	int size;

	m = (union olsr_message *)buffer;

	// pack chat into olsr message
	// TODO: ipv6
	memset(&m->v4.originator, 0, sizeof(m->v4.originator));
	m->v4.olsr_msgtype = QAUL_CHAT_MESSAGE_TYPE;
	memcpy(&m->v4.message.chat.name, item->name, MAX_USER_LEN);
	memcpy(&m->v4.message.chat.msg, item->msg, MAX_MESSAGE_LEN);
	memset(&m->v4.originator, 0, sizeof(m->v4.originator));
	size = sizeof(struct qaul_chat_msg);
	size = size + sizeof(struct olsrmsg);
	m->v4.olsr_msgsize = htons(size);

	// send package
	Qaullib_IpcSend(m);

	// save message
	Qaullib_MsgAdd(item);

	return 1;
}

// ------------------------------------------------------------
int Qaullib_MsgSendPublicWeb(struct qaul_msg_LL_item *item)
{
	char buffer[1024];
	union olsr_message *m;
	int size;

	m = (union olsr_message *)buffer;

	// pack chat into olsr message
	// TODO: ipv6
	memset(&m->v4.originator, 0, sizeof(m->v4.originator));
	m->v4.olsr_msgtype = QAUL_CHAT_MESSAGE_TYPE;
	memcpy(&m->v4.message.chat.name, item->name, MAX_USER_LEN);
	memcpy(&m->v4.message.chat.msg, item->msg, MAX_MESSAGE_LEN);
	memcpy(&m->v4.originator, &item->ip_union, sizeof(m->v4.originator));
	size = sizeof(struct qaul_chat_msg);
	size = size + sizeof(struct olsrmsg);
	m->v4.olsr_msgsize = htons(size);

	// send package
	Qaullib_IpcSend(m);

	// save message
	Qaullib_MsgAdd(item);

	return 1;
}

// ------------------------------------------------------------
int Qaullib_MsgSendPrivate(struct qaul_msg_LL_item *item)
{
	// save message
	Qaullib_MsgAdd(item);

	return 1;
}

// ------------------------------------------------------------
// helper functions
// ------------------------------------------------------------
static int Qaullib_MsgAdd2DB(struct qaul_msg_LL_item *item)
{
	char buffer[1024];
	char *stmt;
	char *error_exec;
	char msg_dbprotected[2*MAX_MESSAGE_LEN +1];
	char name_dbprotected[2*MAX_USER_LEN +1];

	error_exec = NULL;
	stmt = buffer;

	if(QAUL_DEBUG)
		printf("Qaullib_MsgAdd2DB\n");

	// protect message for data base
	Qaullib_StringDbProtect(msg_dbprotected, item->msg, sizeof(msg_dbprotected));
	Qaullib_StringDbProtect(name_dbprotected, item->name, sizeof(name_dbprotected));

  	// save Message to database
	sprintf(stmt,
			sql_msg_set,
			item->type,
			name_dbprotected,
			msg_dbprotected,
			item->ip,
			item->ipv,
			item->time,
			item->read
	);

	if(QAUL_DEBUG)
		printf("statement: %s\n", stmt);

	if(sqlite3_exec(db, stmt, NULL, NULL, &error_exec) != SQLITE_OK)
	{
		// execution failed
		printf("SQLite error: %s\n",error_exec);
		sqlite3_free(error_exec);
		error_exec=NULL;
		return 0;
	}

	// get message id
	item->id = (int)sqlite3_last_insert_rowid(db);

	return 1;
}
