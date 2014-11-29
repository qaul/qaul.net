/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaullib_private.h"
#include <pj/config_site.h> // local configuration needs to be included for android
#include <pjsua-lib/pjsua.h>

#define THIS_FILE	"qaullib_voip.c"
#define SIP_USER	"qaul"


// currently active call id
static pjsua_call_id qaul_voip_callid;
static pjsua_acc_id qaul_voip_acc_id;
static pjsua_transport_id qaul_voip_trans_id;

// thread registration list
static pj_status_t rc;

/**
 * thread linked list declarations
 */
struct qaul_voip_thread_LL_item {
	struct qaul_voip_thread_LL_item *next;	// next node
	struct qaul_voip_thread_LL_item *prev;  // previous node
	pj_thread_desc 		desc;
	pj_thread_t 		*this_thread;
};
static struct qaul_voip_thread_LL_item qaul_voip_threadlist;
int qaul_voip_LL_count;
struct qaul_voip_thread_LL_item* Qaullib_Voip_LL_Add();
// todo: clean references when a call ended
void Qaullib_Voip_LL_Delete_Item(struct qaul_voip_thread_LL_item *item);
void Qaullib_Voip_LL_Clean (void);

/**
 * get caller / callee name
 */
static void Qaullib_VoipSetNameByIp(char *ip)
{
	// get caller name
	struct qaul_user_LL_item *myuseritem;
	union olsr_ip_addr my_olsrip;
	inet_pton(AF_INET, ip, &my_olsrip.v4);

	if(strlen(ip) <= MAX_IP_LEN)
		strncpy(qaul_voip_call.ip, ip, sizeof(qaul_voip_call.ip));

	printf("ip: %s\n", ip);

	if(Qaullib_User_LL_IpGetFirst(&my_olsrip, &myuseritem) == 1 && myuseritem->type == QAUL_USERTYPE_KNOWN)
		strcpy(qaul_voip_call.name, myuseritem->name);
	else
		strcpy(qaul_voip_call.name, "Unknown");
}

/**
 * write message log into DB
 */
static void Qaullib_VoipLogCall(void)
{
	time_t timestamp;
	struct qaul_msg_LL_item msg_item;

	printf("Qaullib_VoipLogCall()\n");

	if(qaul_voip_call.call_logged == 0)
	{
		qaul_voip_call.call_logged = 1;

		// fill in values
		msg_item.id = 0;

		if(qaul_voip_call.outgoing)
			msg_item.type = 13;
		else
			msg_item.type = 3;

		strncpy(msg_item.msg, "{}", sizeof(msg_item.msg));
		strncpy(msg_item.name, qaul_voip_call.name, sizeof(msg_item.name));

		// set time
		time(&timestamp);
		msg_item.time = (int)timestamp;

		// set read
		msg_item.read = 0;

		// set ip
		// todo: ipv6
		msg_item.ipv = 4;
		strncpy(msg_item.ip, qaul_voip_call.ip, sizeof(msg_item.ip));
		inet_aton(qaul_voip_call.ip, &msg_item.ip_union.v4);

	  	// save message
		Qaullib_MsgAdd(&msg_item);
	}
}

/**
 * register this thread in pjsip
 */
static void Qaullib_VoipRegisterThread(void)
{
	// get new reference
	struct qaul_voip_thread_LL_item* item = Qaullib_Voip_LL_Add();
	// register
	pj_bzero(item->desc, sizeof(item->desc));
	rc = pj_thread_register("thread", item->desc, &item->this_thread);
	if (rc != PJ_SUCCESS) {
		pjsua_perror(THIS_FILE, "...error in pj_thread_register", rc);
	}
}

/**
 *  Callback called by the library upon receiving incoming call
 */
static void on_incoming_call(pjsua_acc_id acc_id, pjsua_call_id call_id, pjsip_rx_data *rdata)
{
    pjsua_call_info ci;

    PJ_UNUSED_ARG(acc_id);
    PJ_UNUSED_ARG(rdata);

    pjsua_call_get_info(call_id, &ci);

    PJ_LOG(3,(THIS_FILE, "Incoming call from %.*s!!",
			 (int)ci.remote_info.slen,
			 ci.remote_info.ptr));

	qaul_voip_callid = call_id;
	// send ringing notice
	pjsua_call_answer(call_id, 180, NULL, NULL);
	// set qaul_voip_event
	qaul_voip_event = 2;
	qaul_voip_new_call = 1;
	qaul_voip_call.outgoing = 0;
	qaul_voip_call.connected = 0;
	qaul_voip_call.call_logged = 0;

	Qaullib_VoipSetNameByIp(rdata->pkt_info.src_name);

	// send ring
	qaul_voip_ringing = 1;
	Qaullib_Appevent_LL_Add(QAUL_EVENT_RING);
}

/**
 * Callback called by the library when call's state has changed
 */
static void on_call_state(pjsua_call_id call_id, pjsip_event *e)
{
	printf("on_call_state\n");

	pjsua_call_info ci;
    PJ_UNUSED_ARG(e);
    pjsua_call_get_info(call_id, &ci);

	// write state to qaul_voip_event & qaul_voip_event_code
    // unused:
    // PJSIP_INV_STATE_NULL
    // PJSIP_INV_STATE_CALLING
    // PJSIP_INV_STATE_INCOMING
    if(ci.state == PJSIP_INV_STATE_EARLY)
    {
    	// set ringing event only when this user is the caller
    	if(qaul_voip_call.outgoing)
    	{
    		qaul_voip_event = 1;
    		qaul_voip_ringing = 1;
    		Qaullib_Appevent_LL_Add(QAUL_EVENT_RING);
    	}
    }
    else
    	qaul_voip_ringing = 0;

    if(ci.state == PJSIP_INV_STATE_CONNECTING)
    {
    	qaul_voip_event = 3;
    }
    else if(ci.state == PJSIP_INV_STATE_CONFIRMED)
    {
    	qaul_voip_event = 4;
    }
    else if(ci.state == PJSIP_INV_STATE_DISCONNECTED)
    {
    	qaul_voip_event = 5;
    	qaul_voip_event_code = 487;

    	Qaullib_VoipLogCall();

    	printf("PJSIP_INV_STATE_DISCONNECTED\n");
    }

    PJ_LOG(3,(THIS_FILE, "Call %d state=%.*s", call_id,
			 (int)ci.state_text.slen,
			 ci.state_text.ptr));
}

/**
 * Callback called by the library when call's media state has changed
 */
static void on_call_media_state(pjsua_call_id call_id)
{
	printf("on_call_media_state\n");

	pjsua_call_info ci;

    pjsua_call_get_info(call_id, &ci);

    if (ci.media_status == PJSUA_CALL_MEDIA_ACTIVE)
    {
		// When media is active, connect call to sound device.
		pjsua_conf_connect(ci.conf_slot, 0);
		pjsua_conf_connect(0, ci.conf_slot);
    }
}

void Qaullib_VoipCallStart(char* ip)
{
	pj_status_t status;
	char buffer[256];
	char* stmt = buffer;

	// register this thread
	Qaullib_VoipRegisterThread();

	// check if another call is in progress
	if(pjsua_call_get_count() == 0)
	{
		qaul_voip_call.outgoing = 1;
		qaul_voip_call.call_logged = 0;
		Qaullib_VoipSetNameByIp(ip);

		// create uri
		sprintf(stmt, "sip:%s@%s:%i", SIP_USER, ip, VOIP_PORT);
		pj_str_t uri = pj_str(stmt);

		// set user name in msg_data
		pjsua_msg_data my_data;
		pjsip_generic_string_hdr my_hdr;
		pj_str_t hname = pj_str("qaul_name");
		pj_str_t hvalue = pj_str("qaul");
		pjsua_msg_data_init(&my_data);
		pjsip_generic_string_hdr_init2(&my_hdr, &hname, &hvalue);
		pj_list_push_back(&my_data.hdr_list, &my_hdr);

		status = pjsua_call_make_call(qaul_voip_acc_id, &uri, 0, NULL, &my_data, &qaul_voip_callid);
		if (status != PJ_SUCCESS)
		{
			pjsua_perror(THIS_FILE, "Error making call", status);
			qaul_voip_event = 5;
		}

		printf("Qaullib_VoipCallStart qaul_voip_callid %i\n", (int)qaul_voip_callid);
	}
}

void Qaullib_VoipCallAccept(void)
{
	Qaullib_VoipRegisterThread();
	pjsua_call_answer(qaul_voip_callid, 200, NULL, NULL);
}

void Qaullib_VoipCallEnd(void)
{
	printf("Qaullib_VoipCallEnd()\n");

	Qaullib_VoipRegisterThread();
	pjsua_call_hangup_all();
	Qaullib_VoipLogCall();
}

int Qaullib_VoipStart(void)
{
    pj_status_t status;
	char buffer[256];
	char* stmt = buffer;

	qaul_voip_event = 0;
	qaul_voip_event_code = 400;
	qaul_voip_callid = 0;
	qaul_voip_new_call = 0;

	// init thread list
	qaul_voip_LL_count = 0;
	qaul_voip_threadlist.next = &qaul_voip_threadlist;
	qaul_voip_threadlist.prev = &qaul_voip_threadlist;

    // Create pjsua first!
    status = pjsua_create();
    if (status != PJ_SUCCESS)
    {
    	pjsua_perror(THIS_FILE, "Error in pjsua_create()", status);
    	return 0;
    }

    // Init pjsua
    {
		pjsua_config cfg;
		pjsua_logging_config log_cfg;

		pjsua_config_default(&cfg);
		cfg.cb.on_incoming_call = &on_incoming_call;
		cfg.cb.on_call_media_state = &on_call_media_state;
		cfg.cb.on_call_state = &on_call_state;

		cfg.max_calls = 1;

		pjsua_logging_config_default(&log_cfg);
		log_cfg.console_level = 4;

		status = pjsua_init(&cfg, &log_cfg, NULL);
		if (status != PJ_SUCCESS)
		{
			pjsua_perror(THIS_FILE, "Error in pjsua_init()", status);
			return 0;
		}
    }

    // Add UDP transport.
    {
		pjsua_transport_config cfg;

		pjsua_transport_config_default(&cfg);
		cfg.port = VOIP_PORT;
		cfg.public_addr = pj_str(qaul_ip_str); // set public address
		status = pjsua_transport_create(PJSIP_TRANSPORT_UDP, &cfg, &qaul_voip_trans_id);
		if (status != PJ_SUCCESS)
		{
			pjsua_perror(THIS_FILE, "Error creating transport", status);
			return 0;
		}
    }

    // Initialization is done, now start pjsua
    status = pjsua_start();
    if (status != PJ_SUCCESS)
    {
    	pjsua_perror(THIS_FILE, "Error starting pjsua", status);
    	return 0;
    }

	// create local account
	status = pjsua_acc_add_local(qaul_voip_trans_id, PJ_TRUE, &qaul_voip_acc_id);
	if (status != PJ_SUCCESS)
		pjsua_perror(THIS_FILE, "account creation error", status);

	// modify account information
	pjsua_acc_config myconfig;
	pj_pool_t *mypool = pjsua_pool_create("tmp-pjsua", 1000, 1000);

	pjsua_acc_get_config(qaul_voip_acc_id, mypool, &myconfig);
	myconfig.contact_params = pj_str(";qaulname=XYZ"); // set additional custom headers of struct pjsua_acc_config
	status = pjsua_acc_modify(qaul_voip_acc_id, &myconfig);

	pj_pool_release(mypool);

    return 1;
}

void Qaullib_VoipStop(void)
{
	// Destroy pjsua
	pjsua_destroy();
}


/********************************************//**
 * thread list functions
 ***********************************************/

struct qaul_voip_thread_LL_item* Qaullib_Voip_LL_Add ()
{
	struct qaul_voip_thread_LL_item *new_item;
	new_item = (struct qaul_voip_thread_LL_item *)malloc(sizeof(struct qaul_voip_thread_LL_item));

	if(QAUL_DEBUG)
		printf("Qaullib_Voip_LL_Add\n");

	// create links
	new_item->prev = &qaul_voip_threadlist;
	new_item->next = qaul_voip_threadlist.next;
	qaul_voip_threadlist.next = new_item;
	new_item->next->prev = new_item;

	qaul_voip_LL_count++;
	return new_item;
}

// todo: clean references when a call ended
void Qaullib_Voip_LL_Delete_Item (struct qaul_voip_thread_LL_item *item)
{
	if(QAUL_DEBUG)
		printf("Qaullib_Voip_LL_Delete_Item\n");

	item->prev->next = item->next;
	item->next->prev = item->prev;

	qaul_voip_LL_count--;
	free(item);
}

void Qaullib_Voip_LL_Clean (void)
{
	// clean everything up to 5 references
	while(qaul_voip_LL_count > 5)
	{
		Qaullib_Voip_LL_Delete_Item(qaul_voip_threadlist.prev);
	}
}
