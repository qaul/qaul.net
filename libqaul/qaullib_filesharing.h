/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_FILESHARING
#define _QAULLIB_FILESHARING

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Indicates wheter advertised files shall be downloaded automatically.
 * 0: no
 * 1: yes
 */
int qaul_file_autodownload;

/**
 * maximum space available for file sharing
 * 0: unlimited
 * 1: 1KB
 */
int qaul_file_space_max;

/**
 * maximum file size that is automatically downloaded
 * 0: unlimited
 * 1: 1KB
 */
int qaul_file_size_max;

/**
 * defines chunk size of file sharing
 */
int qaul_chunksize;

/**
 * structure of available connections for downloading files
 */
struct qaul_file_connection
{
	struct qaul_wget_connection conn;
	struct qaul_file_LL_item *fileinfo;
	unsigned int chunksize;
	FILE *file;
};

/**
 * array of TCP connections for file downloading
 */
struct qaul_file_connection fileconnections[MAX_FILE_CONNECTIONS];

/**
 * Initialize file table and read files from data base.
 * Called once in qaullib_init.
 */
void Qaullib_FileInit(void);

/**
 * add existing files to DB
 * called once after installation
 *
 * fat clients contain all executables for all OSs
 * slim clients only contain information about the other versions
 * and need to download the executables from other users.
 */
void Qaullib_FilePopulate(void);

/**
 * add a new file to data base and LL
 *
 * @retval 0 on error
 * @retval 1 on success
 */
int Qaullib_FileAdd(struct qaul_file_LL_item *file_item);

/**
 * copy a file from its @a origin file path to @a destiny
 *
 * @retval file size in Bytes
 * @retval 0 error
 */
int Qaullib_FileCopy(const char* origin, const char* destiny);

/**
 * add file from @a path to file sharing and analyzes the file.
 * It creates the hashstr, the hash and suffix and fills the values into @a file_item
 *
 * @retval 0 on error
 * @retval filesize in Bytes on success
 */
int Qaullib_FileCopyNew(char *path, struct qaul_file_LL_item *file);

/**
 * copies the file to the download folder if it was set
 */
int Qaullib_FileCopyToDownloadFolder(struct qaul_file_LL_item *file);

/**
 * check if any file needs to be downloaded
 */
void Qaullib_FileCheckScheduled(void);

/**
 * flood discovery message via IPC over olsr
 */
void Qaullib_FileSendDiscoveryMsg(struct qaul_file_LL_item *file_item);

/**
 * check if file is downloading, unschedule it
 */
void Qaullib_FileStopDownload(struct qaul_file_LL_item *file_item);

/**
 * create the @a filepath out of the @a hash string and the @a suffix
 */
void Qaullib_FileCreatePath(char *filepath, char *hash, char *suffix);

/**
 * create the @a filepath to the dowload folder for the @a file_item
 * files are copied there after downloading
 */
void Qaullib_FileCreatePathToDownloadFolder(char *filepath, struct qaul_file_LL_item *file_item);

/**
 * checks if file with @a hash is available from the bytes position @a startbyte
 *
 * @retval 1 file is available
 * @retval 0 file is not available from this position
 */
int Qaullib_FileAvailable(char *hashstr, char *suffix, struct qaul_file_LL_item **file_item);

/**
 * try to download a file
 */
void Qaullib_FileConnect(struct qaul_file_LL_item *file_item);

/**
 * process downloaded bytes
 */
int Qaullib_FileDownloadProcess(struct qaul_file_connection *fileconnection, int bytes, int first);

/**
 * handle failed download
 */
void Qaullib_FileDownloadFailed(struct qaul_file_connection *fileconnection);

/**
 * check file sockets for incoming traffic
 */
//void Qaullib_FileCheckSockets(void);

/**
 * end a failed download of th @a fileconnection
 */
//void Qaullib_FileEndFailedConnection(struct qaul_file_connection *fileconnection);

/**
 * check if download @a filesize match the @a fileconnection
 *
 * @retval 1 success
 * @retval 0 error
 */
int Qaullib_FileCompareFileSize(struct qaul_file_connection *fileconnection, int filesize);

/**
 * check if file @a path exists
 *
 * @retval 1 file exists
 * @retval 0 file does not exist
 */
int Qaullib_FileExists(char *path);

/**
 * delete a file by it's database @a id
 *
 * @retval 1 success
 * @retval 0 error
 */
int Qaullib_FileDelete(struct qaul_file_LL_item *file_item);

/**
 * fill files from DB into LL
 */
void Qaullib_FileDB2LL(void);

/**
 * compares the hash with the downloaded file of @a file_item
 *
 * @retval 1 on success: hash and file match
 * @retval 0 on error: hash and file differ
 */
int Qaullib_VerifyDownload(struct qaul_file_LL_item *file_item);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
