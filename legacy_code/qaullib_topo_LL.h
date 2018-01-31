/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef _QAULLIB_TOPO_LL
#define _QAULLIB_TOPO_LL

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * The topology table contains a simple linked list with the received
 * topology from olsr.
 * New entries are put in front.
 * After every use the list is deleted.
 */

struct qaul_topo_LL_item {
	struct qaul_topo_LL_item *next;           /// next node
	union olsr_ip_addr src_ip;                /// source ip address
	union olsr_ip_addr dest_ip;               /// destination address
	float              lq;                    /// link quality
};

/**
 * Pointer to the first entry of the topo list.
 * If list is empty this pointer is null.
 */
struct qaul_topo_LL_item *qaul_topo_LL_first;

/**
 * add a new list entry with @a src_ip, @a dest_ipin, and @a lq to the topology table.
 */
void Qaullib_Topo_LL_Add (union olsr_ip_addr *src_ip, union olsr_ip_addr *dest_ip, float lq);

/**
 * delete first list entry
 */
void Qaullib_Topo_LL_Delete_Item ();


#ifdef __cplusplus
}
#endif // __cplusplus

#endif
