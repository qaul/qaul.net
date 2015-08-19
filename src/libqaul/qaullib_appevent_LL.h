/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_APPEVENT_LL
#define _QAULLIB_APPEVENT_LL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * The app event linked list. This is a FIFO linked list.
 * New entries are put last.
 * Entries are pulled from front and deleted after a pull.
 */

struct qaul_appevent_LL_item {
	struct qaul_appevent_LL_item *next;        /// next node
	int                           event;       /// event
};

/**
 * Pointer to the first item of the list.
 * If list is empty this pointer is null.
 */
struct qaul_appevent_LL_item *qaul_appevent_LL_first;

/**
 * Pointer to the last item of the list.
 * If list is empty this pointer is null.
 */
struct qaul_appevent_LL_item *qaul_appevent_LL_last;

/**
 * Add a new list entry with @a event to the app event table.
 */
void Qaullib_Appevent_LL_Add (int event);

/**
 * Return and delete first app event.
 */
int Qaullib_Appevent_LL_Get ();


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
