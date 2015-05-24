/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_SQL
#define _QAULLIB_SQL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * message table
 *
 * deprecated: ipv4 ipv6  (only use text & ipv indicator)
 *
 * message types:
 * 1:  public message received
 * 2:  private message received
 * 3:  voip incoming call
 * 11: public message sent by me
 * 12: private message sent by me
 * 13: voip outgoing call
 */
static const char* sql_msg_table = "CREATE TABLE IF NOT EXISTS 'msg' ('id' INTEGER PRIMARY KEY  AUTOINCREMENT  NOT NULL , 'type' INTEGER DEFAULT 1 NOT NULL, 'name' TEXT, 'msg' TEXT, 'ip' TEXT, 'ipv' INTEGER DEFAULT 4, 'time' INTEGER DEFAULT 0, 'read' INTEGER DEFAULT 0);";

// set indexes
static const char* sql_msg_index = "CREATE INDEX IF NOT EXISTS 'myindex' ON 'msg' ('id' DESC); CREATE INDEX IF NOT EXISTS 'msg_read' ON 'msg' ('read' ASC);";

// get messages
static const char* sql_msg_get_latest  = "SELECT * FROM 'msg' ORDER BY id DESC LIMIT 40;";
static const char* sql_msg_get_archive = "SELECT * FROM 'msg' WHERE id > %i ORDER BY id DESC LIMIT 20;";
static const char* sql_msg_get_new     = "SELECT * FROM 'msg' WHERE read = 0 ORDER BY id ASC;";
static const char* sql_msg_get_user0   = "SELECT * FROM 'msg' WHERE name = \"%s\" OR msg LIKE \"%s@%s%s\" ORDER BY id DESC LIMIT 20;";
static const char* sql_msg_get_user    = "SELECT * FROM 'msg' WHERE id > %i AND ( name = \"%s\" OR  msg LIKE \"%s@%s%s\" ) ORDER BY id DESC LIMIT 20;";
static const char* sql_msg_get_tag0    = "SELECT * FROM 'msg' WHERE msg LIKE \"%s%s%s\" ORDER BY id DESC LIMIT 20;";
static const char* sql_msg_get_tag     = "SELECT * FROM 'msg' WHERE id > %i AND msg LIKE \"%s%s%s\" ORDER BY id DESC LIMIT 20;";

// update message
static const char* sql_msg_update_read = "UPDATE 'msg' SET read = 1 WHERE id = %i ;";

// insert message
static const char* sql_msg_set = "INSERT INTO 'msg' ('type','name','msg','ip','ipv','time','read') VALUES (%i,\"%s\",\"%s\",\"%s\",%i,%i,%i);";


/**
 * configuration table
 *
 * contains key value pairs
 */
static const char* sql_config_table = "CREATE TABLE IF NOT EXISTS 'config' ('id' INTEGER PRIMARY KEY  AUTOINCREMENT  NOT NULL, 'key' TEXT DEFAULT '', 'type' INTEGER DEFAULT 0, 'value' TEXT DEFAULT '', 'value_int' INTEGER DEFAULT 0, 'time' INTEGER DEFAULT CURRENT_TIMESTAMP);";

// get value
static const char* sql_config_get = "SELECT * FROM 'config' WHERE key = \"%s\";";

// update
static const char* sql_config_update = "UPDATE 'config' SET read = 1 WHERE id = %i ;";

// insert message
static const char* sql_config_set = "INSERT INTO 'config' ('key','type','value') VALUES (\"%s\",1,\"%s\");";
static const char* sql_config_set_int = "INSERT INTO 'config' ('key','type','value_int') VALUES (\"%s\",0,%i);";
static const char* sql_config_set_all = "INSERT INTO 'config' ('key','type','value','value_int') VALUES (\"%s\",%i,\"%s\",%i);";

// delete values for key
static const char* sql_config_delete = "DELETE FROM 'config' WHERE key = \"%s\";";


/**
 * user table
 * used to save and remember favorites
 */
static const char* sql_user_table = "CREATE TABLE IF NOT EXISTS 'user' ('id' INTEGER PRIMARY KEY  AUTOINCREMENT  NOT NULL, 'name' TEXT, 'ipv' INTEGER DEFAULT 4, 'ipv4' INTEGER, 'ipv6' CHAR(16), 'uid' VARCHAR(255), 'icon' VARCHAR(255), 'created_at' INTEGER DEFAULT CURRENT_TIMESTAMP);";

// set indexes
static const char* sql_user_index = "CREATE INDEX IF NOT EXISTS 'user_name' ON 'user' ('name' ASC); CREATE INDEX IF NOT EXISTS 'user_ip' ON 'user' ('ipv4' ASC);";

// get users
static const char* sql_user_get_all = "SELECT * FROM 'user' ORDER BY name ASC;";

// check if user exists
static const char* sql_user_check_ipv4 = "SELECT id FROM 'user' WHERE ipv = 4 AND ipv4 = %i;";
static const char* sql_user_check_ipv6 = "SELECT id FROM 'user' WHERE ipv = 6 AND ipv6 = \"%s\";";

// update user
static const char* sql_user_update_lastseen = "UPDATE 'user' SET lastseen_at = DATETIME('now', 'localtime') WHERE id = %i ;";
static const char* sql_user_update_nameicon = "UPDATE 'user' SET  lastseen_at = DATETIME('now', 'localtime'), name = \"%s\", icon = \"%s\", online = %i WHERE id = %i ;";
static const char* sql_user_update_name = "UPDATE 'user' SET lastseen_at = DATETIME('now', 'localtime'), name = \"%s\", online = %i WHERE id = %i ;";

// insert user
static const char* sql_user_set_ip = "INSERT INTO 'user' ('name','icon','ipv','ipv4','ipv6','uid') VALUES (\"%s\",\"%s\",%i,%i,\"%s\",\"%s\");";
static const char* sql_user_set_ipv4 = "INSERT INTO 'user' ('name','icon','ipv','ipv4','uid') VALUES (\"%s\",\"%s\",4,%i,\"%s\");";
static const char* sql_user_set_ipv6 = "INSERT INTO 'user' ('name','icon','ipv','ipv6','uid') VALUES (\"%s\",\"%s\",6,\"%s\",\"%s\");";

// delete users
static const char* sql_user_delete_id = "DELETE FROM 'user' WHERE id = %i;";
static const char* sql_user_delete_uid = "DELETE FROM 'user' WHERE uid = \"%s\";";
static const char* sql_user_delete_ipv4 = "DELETE FROM 'user' WHERE ipv4 = %i;";


/**
 * file table
 */
static const char* sql_file_table = "CREATE TABLE IF NOT EXISTS 'file' ('id' INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL, 'type' INTEGER NOT NULL DEFAULT 1, 'hash' TEXT, 'suffix' CHAR(5), 'description' TEXT, 'size' INTEGER, 'status' INTEGER DEFAULT 0, 'favorite' INTEGER DEFAULT 0, 'created_at' INTEGER DEFAULT 0, 'adv_name' TEXT DEFAULT '', 'adv_ip' TEXT DEFAULT '', 'geolon' REAL DEFAULT 0, 'geolat' REAL DEFAULT 0, 'requests' INTEGER DEFAULT 0, 'downloaded' FLOAT DEFAULT 0);";

// set indexes
static const char* sql_file_index = "CREATE INDEX IF NOT EXISTS 'file_hash' ON 'file' ('hash' DESC); CREATE INDEX IF NOT EXISTS 'file_suffix' ON 'file' ('suffix' DESC);";

// get files
static const char* sql_file_get_everything = "SELECT * FROM 'file' ORDER BY status ASC, id DESC;";
static const char* sql_file_get_hash = "SELECT * FROM 'file' WHERE hash = \"%s\" LIMIT 1;";
static const char* sql_file_get_id = "SELECT * FROM 'file' WHERE id = %i LIMIT 1;";

// update file
static const char* sql_file_update_status = "UPDATE 'file' SET status = %i WHERE hash = \"%s\" ;";
static const char* sql_file_update_downloaded = "UPDATE 'file' SET downloaded = %i WHERE hash = \"%s\" ;";
static const char* sql_file_update_size = "UPDATE 'file' SET size = %i WHERE hash = \"%s\" ;";
static const char* sql_file_update_favorite = "UPDATE 'file' SET favorite = %i WHERE hash = \"%s\" ;";

// insert file
static const char* sql_file_add = "INSERT INTO 'file' ('hash','suffix','description','size','status','type','adv_name','adv_ip','created_at') VALUES (\"%s\",\"%s\",\"%s\",%i,%i,%i,\"%s\",\"%s\",%i);";

// todo: remove this
static const char* sql_file_set = "INSERT INTO 'file' ('hash','suffix','description','size','status','type','adv_name','adv_ip','created_at') VALUES (\"%s\",\"%s\",\"%s\",%i,5,1,'','',%i);";
static const char* sql_file_schedule = "INSERT INTO 'file' ('hash','suffix','description','size','status','type','adv_name','adv_ip','created_at') VALUES (\"%s\",\"%s\",\"%s\",%i,0,1,\"%s\",\"%s\",%i);";

// delete files
static const char* sql_file_delete_hash = "DELETE FROM 'file' WHERE hash = \"%s\";";


/********************************************//**
 * populate configuration
 ***********************************************/
#define MAX_POPULATE_CONFIG 13
#define CONFIG_TYPE_INT      0
#define CONFIG_TYPE_STR      1

struct qaul_populate_config_struct
{
	char key[MAX_VARCHAR_LEN +1];
	int  type;
	char value[MAX_VARCHAR_LEN +1];
	int  value_int;
};

static struct qaul_populate_config_struct qaul_populate_config[MAX_POPULATE_CONFIG] = {
	{"net.profile",          CONFIG_TYPE_STR, "qaul",              0},
	{"net.protocol",         CONFIG_TYPE_INT, "",                  4},
	{"net.mask",             CONFIG_TYPE_INT, "",                  8},
	{"net.broadcast",        CONFIG_TYPE_STR, "10.255.255.255",    0},
	{"net.gateway",          CONFIG_TYPE_STR, "0.0.0.0",           0},
	{"wifi.channel",         CONFIG_TYPE_INT, "",                 11},
	{"wifi.ssid",            CONFIG_TYPE_STR, "qaul.net",          0},
	{"wifi.bssid_set",       CONFIG_TYPE_INT, "",                  0},
	{"wifi.bssid",           CONFIG_TYPE_STR, "02:11:87:88:D6:FF", 0},
	{"net.interface.manual", CONFIG_TYPE_INT, "",                  0},
	{"net.interface.name",   CONFIG_TYPE_STR, "",                  0},
	{"net.ns1",              CONFIG_TYPE_STR, "5.45.96.220",       0},
	{"net.ns2",              CONFIG_TYPE_STR, "185.82.22.133",     0}
};

/********************************************//**
 * populate file sharing
 ***********************************************
 * configuration keys, flags
 * exe.1  : ubuntu & debian 32 Bit (QT client)
 * exe.2  : OSX 10.5
 * exe.4  : OSX 10.7 - 10.9
 * exe.8  : Windows 7 / 8
 * exe.16 : Android
 * exe.32 : iOS
 * exe.64 : Ubuntu & Debian 32 Bit (GTK client)
 * exe.128: Ubuntu & Debian 64 Bit (GTK client)
 ***********************************************/

struct qaul_populate_file_struct
{
	uint32_t  OS_flag;
	int  type;
	int  size;
    char hashstr[MAX_HASHSTR_LEN +1];
    char suffix[MAX_SUFFIX_LEN +1];
    char description[MAX_DESCRIPTION_LEN +1];
    int  max_size;
};


#define MAX_POPULATE_FILE 5

static struct qaul_populate_file_struct qaul_populate_file[MAX_POPULATE_FILE] = {
//	{1,  4, 0, "0000000000000000000000000000000000000000", "gz",  "ubuntu & debian 32 Bit", 10000000},
//  {2,  4, 0, "0000000000000000000000000000000000000000", "zip", "OSX 10.5",               4000000},
	{4,	 4, 0, "0000000000000000000000000000000000000000", "dmg", "OSX 10.6 - 10.9",        10000000},
	{8,  4, 0, "0000000000000000000000000000000000000000", "exe", "Windows 7 / 8",          8000000},
	{16, 4, 0, "0000000000000000000000000000000000000000", "apk", "Android",                5000000},
//	{32, 4, 0, "0000000000000000000000000000000000000000", "deb", "iOS",                    5000000},
	{64, 4, 0, "0000000000000000000000000000000000000000", "deb", "Ubuntu & Debian 32 Bit", 5000000},
	{128,4, 0, "0000000000000000000000000000000000000000", "deb", "Ubuntu & Debian 64 Bit", 5000000}
};


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
