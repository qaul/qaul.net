/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <qaul/qaul.h>

ql_error_t init_memory();
ql_error_t start_gui();
ql_error_t first_startup();
ql_error_t load_configuration();
ql_error_t update_app();
ql_error_t start_network();
ql_error_t start_services();
ql_error_t start_portforwarding();


ql_error_t ql_initialise(struct qaul **state, enum qaul_os os, const char *home, const char *resources)
{
	init_memory();
	start_gui(); // start on a new thread
	first_startup();
	load_configuration();
	update_app();
	start_network();
	start_services();
	start_portforwarding();
}


ql_error_t ql_shutdown(struct qaul *state)
{
	// exit library
	// e.g. Qaullib_Exit();

	// stop networking
	// TODO: this needs to be done in the client
	// TODO: port forwarding only works with administrative access.

}



ql_error_t init_memory()
{
	// TODO: all memory initialization that needs to be done first
}

ql_error_t start_gui()
{
	// start web server

	// load GUI in window
	// TODO: define hooks for client
}

ql_error_t first_startup()
{
	// first startup
	// TODO: check if files need to be copied
}

ql_error_t load_configuration()
{
	// load configuration

	// check system

	// check user rights

}

ql_error_t update_app()
{
	// update
	// TODO: check if qaul.net app needs to updated
}

ql_error_t start_network()
{
	// which networks are present
	// which network shall be started

	// start network

}

ql_error_t start_services()
{
	// start services that need to be started after the network is configured
}

ql_error_t start_portforwarding()
{
	// port forwarding only works with administrative access
}
