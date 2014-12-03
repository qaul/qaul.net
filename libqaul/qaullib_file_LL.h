/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_FILE_LL
#define _QAULLIB_FILE_LL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus


/**
 * Contains entries of the
 */
struct qaul_filediscovery_LL_item {
	struct qaul_filediscovery_LL_item *next;  /// next node
	struct qaul_filediscovery_LL_item *prev;  /// previous node

	union olsr_ip_addr ip;                    /// ip of the seeder
};

/**
 * The file table contains an array of linked lists.
 * The file entries are added to a linked list in the array according
 * to the file hash.
 * (The hash is created from the last byte of the file hash.)
 */
struct qaul_file_LL_item {
	struct qaul_file_LL_item *next;           /// next node
	struct qaul_file_LL_item *prev;           /// previous node

    int  id;                                  /// data base ID of the file entry
    int  type;                                /// type of the file, see QAUL_FILETYPE_XXX
    unsigned char hash[MAX_HASH_LEN];                  /// file hash
    char hashstr[MAX_HASHSTR_LEN +1];         /// file hash string
    char suffix[MAX_SUFFIX_LEN +1];           /// file suffix
    char description[MAX_DESCRIPTION_LEN +1]; /// file description
    int  created_at;                          /// when this file entry was created (not the date of the file!)
    int  status;                              /// status of the file, see QAUL_FILESTATUS_XXX
    int  size;                                /// file size in bytes
    int  downloaded;                          /// number of downloaded bytes, that are concluded
    int  downloaded_chunk;                    /// number of downloaded bytes, of the current chunk downloading (these bytes can be lost)

    time_t discovery_timestamp;               /// time stamp when discovery started
    int discovery_count;                      /// how many seeders were discovered
    struct qaul_filediscovery_LL_item discoveryLL; /// first item of the discovery LL (always empty)

    char adv_name[MAX_USER_LEN +1];           /// todo: to be removed, name of the advertiser
    union olsr_ip_addr adv_ip;                /// todo: to be removed, ip of the advertiser
    int adv_validip;                          /// todo: to be removed, checks if ip is valid

    int gui_notify;                           /// 0: if nothing has changed, 1: if the file status has changed
};

struct qaul_file_LL_node {
	struct qaul_file_LL_item *item;           // this node
	uint32_t                  index;          // where it is
};

/**
 * to be called once at startup to create the linked list
 */
void Qaullib_File_LL_Init (void);

/**
 * creates a new list entry
 *
 * @retval pointer to the item
 */
struct qaul_file_LL_item* Qaullib_File_LL_Add (struct qaul_file_LL_item *item);

/**
 * delete @a item from list
 */
void Qaullib_File_LL_Delete_Item (struct qaul_file_LL_item *item);

/**
 * loops through the list and checks if a file with @a hash exists.
 * If it exists, @a item will contain the pointer to it.
 *
 * @retval 1 entry exists
 * @retval 0 entry does not exist
 */
int  Qaullib_File_LL_HashSearch (char *hash, struct qaul_file_LL_item **item);

/**
 * loops through the list and checks if a file with @a hash exists.
 *
 * @retval 1 entry exists
 * @retval 0 entry does not exist
 */
int  Qaullib_File_LL_HashExists (char *hash);

/**
 * Checks if a file with @a hash exists and if it hash finished downloading.
 *
 * @retval 1 file finished downloading
 * @retval 0 file has not finished downloading
 */
int Qaullib_File_LL_FileAvailable (char *filehash);

/**
 * initializes a @a node with the first entry of the file table
 */
void Qaullib_File_LL_InitNode (struct qaul_file_LL_node *node);

/**
 * Checks whether there is a next item while looping through the whole table
 * The function links the item into the @a node
 *
 * @retval 1 there is a next item
 * @retval 0 there is no next item
 */
int  Qaullib_File_LL_NextNode (struct qaul_file_LL_node *node);

/**
 * Checks whether there is a next item ready for download
 * The function links the item into the @a node
 *
 * @retval 1 there is a next item
 * @retval 0 there is no next item
 */
int  Qaullib_File_LL_NextNodePub (struct qaul_file_LL_node *node);

/**
 * Checks whether there is a next item displayable in GUI
 * The function links the item into the @a node
 *
 * @retval 1 there is a next item
 * @retval 0 there is no next item
 */
int  Qaullib_File_LL_NextNodePriv (struct qaul_file_LL_node *node);

/**
 * Checks whether there is a next item with a publicly available binary.
 * The function links the item into the @a node
 *
 * @retval 1 there is a next item
 * @retval 0 there is no next item
 */
int  Qaullib_File_LL_NextNodePubBinaries (struct qaul_file_LL_node *node);

/**
 * process a file discovery @a msg and add the seeders @a ip to the LL
 */
void Qaullib_Filediscovery_LL_DiscoveryMsgProcessing(struct qaul_fileavailable_msg *msg, union olsr_ip_addr *ip);

/**
 * Adds the @a ip as a seeder to @a file
 */
void Qaullib_Filediscovery_LL_AddSeederIp (struct qaul_file_LL_item *file, union olsr_ip_addr *ip);

/**
 * Deletes the seeder with @a ip from the @a file list
 */
void Qaullib_Filediscovery_LL_DeleteSeederIp (struct qaul_file_LL_item *file, union olsr_ip_addr *ip);

/**
 * Sets @a ip to the best seeder for this @a file
 *
 * @retval 1 seeder found
 * @retval 0 no seeder found
 */
int  Qaullib_Filediscovery_LL_GetBestSeeder (struct qaul_file_LL_item *file, union olsr_ip_addr **ip);

/**
 * Searches the file discovery LL of @a file for @a ip
 *
 * @retval 1 ip found
 * @retval 0 ip not found
 */
int  Qaullib_Filediscovery_LL_SeederExists (struct qaul_file_LL_item *file, union olsr_ip_addr *ip);

/**
 * Sets link to next @a item
 *
 * @retval 1 next item found
 * @retval 0 no next item
 */
int  Qaullib_Filediscovery_LL_NextItem (struct qaul_file_LL_item *file,  struct qaul_filediscovery_LL_item **discovery_item);

/**
 * Empty the file discovery LL list of @a file
 */
void Qaullib_Filediscovery_LL_EmptyList (struct qaul_file_LL_item *file);

/**
 * Delete file discovery LL @a item
 */
void Qaullib_Filediscovery_LL_DeleteItem (struct qaul_filediscovery_LL_item *item);


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
