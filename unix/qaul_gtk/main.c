/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#define GetCurrentDir getcwd

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#include <unistd.h>
#include "../../libqaul/qaullib.h"
#include "../../libqaul/qaullib_private.h"
#include "network.h"

#include <gtk/gtk.h>
#include <webkit/webkit.h>

#include <QaulConfig.h>

#define MAX_PATH_LEN   PATH_MAX

/// GUI
GtkWidget *qaulMainWindow;

static void destroyWindowCb(GtkWidget* widget, GtkWidget* window);
static gboolean closeWebViewCb(WebKitWebView* webView, GtkWidget* window);
GdkPixbuf *create_pixbuf(const gchar * filename);

/// utilities
int qaul_copyDirectory(char* source, char* target);

/// runs on start up after opening the window
void qaul_onstartup(void);

/// executed before quitting the application
void qaul_onquit(void);

/// configure computer for qaul.net
int qaulConfigureCounter;
gint qaulConfigureTimer;
gboolean qaul_configure(gpointer data);
DBusConnection*	network_dbus_connection;
qaul_dbus_connection_settings network_settings;
qaul_dbus_device_properties network_device;
int network_interface_found;
char network_json_txt[MAX_JSON_LEN +1];

/// configuration tasks
void qaul_olsrdStart(void);
void qaul_olsrdStop(void);
void qaul_startPortForwarding(void);
void qaul_stopPortForwarding(void);

/// timers
gint qaulTimerEvents;
gint qaulTimerSockets;
gint qaulTimerTopology;
gboolean qaul_timerEvent(gpointer data);
gboolean qaul_timerSocket(gpointer data);
gboolean qaul_timerTopology(gpointer data);

// ------------------------------------------------------------

int main(int argc, char *argv[])
{
	char qaulUserPath[MAX_PATH_LEN];
	char qaulTmpPath[MAX_PATH_LEN];
	char qaulTmpPath2[MAX_PATH_LEN];

	qaulConfigureCounter = 0;
	qaulTimerEvents = 0;
	qaulTimerSockets = 0;
	qaulTimerTopology = 0;
	network_interface_found = 0;

	// initialize glib types
	g_type_init();

	// set paths
	sprintf(qaulUserPath, "%s/.qaul", (char*)g_get_home_dir());
	printf ("qaul.net home directory is %s\n", qaulUserPath);

	// create qaul user directory
	if(!g_file_test(qaulUserPath, G_FILE_TEST_EXISTS))
	{
		// create directory
		// http://www.gnu.org/software/libc/manual/html_node/Permission-Bits.html
		if(g_mkdir(qaulUserPath, S_IRUSR|S_IWUSR|S_IXUSR)== -1)
			printf("qaul.net home directory %s creation error.\n", qaulUserPath);
	}
	// check if we have to update
	sprintf(qaulTmpPath, "%s/%s", qaulUserPath, QAUL_VERSION);
	if(!g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
	{
		printf("Update user folder to qaul.net version %s\n", QAUL_VERSION);
		// copy www folder
		sprintf(qaulTmpPath, "%s/www", QAUL_ROOT_PATH);
		sprintf(qaulTmpPath2, "%s/www", qaulUserPath);
		if(!qaul_copyDirectory(qaulTmpPath, qaulTmpPath2))
			printf("qaul copy directory error. source: %s target: %s\n", qaulTmpPath, qaulTmpPath2);
		// TODO: update data base
		// remove old data base if it exists
		sprintf(qaulTmpPath, "%s/qaullib.db", qaulUserPath);
		if(g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
			if(g_remove(qaulTmpPath) == -1)
				printf("qaul.net database %s removal error\n", qaulTmpPath);
		// create qaul version file
		sprintf(qaulTmpPath, "%s/%s", qaulUserPath, QAUL_VERSION);
		if(!g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
			if(!g_creat(qaulTmpPath, S_IRUSR|S_IWUSR) == -1)
				printf("qaul.net version file %s creation error\n", qaulTmpPath);
	}

	Qaullib_Init(qaulUserPath);
	// set configuration
	Qaullib_SetConf(QAUL_CONF_INTERFACE);
	// enable debug menu
	qaul_conf_debug = 1;

	if(!Qaullib_WebserverStart())
		printf("Webserver startup failed\n");

	// initialize dbus connection
	qaul_dbus_init(&network_dbus_connection);
	// start configuration timer
	qaulConfigureTimer = g_timeout_add(500, qaul_configure, NULL);

	// open window
	gtk_init(&argc,&argv);

    // Create a window that will contain the browser instance
    qaulMainWindow = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_default_size(GTK_WINDOW(qaulMainWindow), 400, 592);
    gtk_window_set_title(GTK_WINDOW(qaulMainWindow), "qaul.net - قول");
    sprintf(qaulTmpPath, "%s/app_icon.png", QAUL_ROOT_PATH);
    gtk_window_set_icon(GTK_WINDOW(qaulMainWindow), create_pixbuf(qaulTmpPath));

    // Create a browser instance
    WebKitWebView *webView = WEBKIT_WEB_VIEW(webkit_web_view_new());

    // Create a scrollable area, and put the browser instance into it
    GtkWidget *scrolledWindow = gtk_scrolled_window_new(NULL, NULL);
    gtk_scrolled_window_set_policy(GTK_SCROLLED_WINDOW(scrolledWindow),
            GTK_POLICY_AUTOMATIC, GTK_POLICY_AUTOMATIC);
    gtk_container_add(GTK_CONTAINER(scrolledWindow), GTK_WIDGET(webView));

    // Set up callbacks so that if either the main window or the browser instance is
    // closed, the program will exit
    g_signal_connect(qaulMainWindow, "destroy", G_CALLBACK(destroyWindowCb), NULL);
    g_signal_connect(webView, "close-web-view", G_CALLBACK(closeWebViewCb), qaulMainWindow);

    // Put the scrollable area into the main window
    gtk_container_add(GTK_CONTAINER(qaulMainWindow), scrolledWindow);

    // Load a web page into the browser instance
    webkit_web_view_load_uri(webView, "http://127.0.0.1:8081/qaul.html");

    // Make sure that when the browser area becomes visible, it will get mouse
    // and keyboard events
    gtk_widget_grab_focus(GTK_WIDGET(webView));

    // Make sure the main window and all its contents are visible
    gtk_widget_show_all(qaulMainWindow);

    // Run the main GTK+ event loop
    gtk_main();

    return 0;
}

static void destroyWindowCb(GtkWidget* widget, GtkWidget* window)
{
    qaul_onquit();
    gtk_main_quit();
}

static gboolean closeWebViewCb(WebKitWebView* webView, GtkWidget* window)
{
    gtk_widget_destroy(window);
    return TRUE;
}

GdkPixbuf *create_pixbuf(const gchar * filename)
{
   GdkPixbuf *pixbuf;
   GError *error = NULL;
   pixbuf = gdk_pixbuf_new_from_file(filename, &error);
   if(!pixbuf) {
      fprintf(stderr, "%s\n", error->message);
      g_error_free(error);
   }

   return pixbuf;
}

int qaul_copyDirectory(char* source, char* target)
{
	// g_file_copy: https://developer.gnome.org/gio/stable/GFile.html#g-file-copy
	// Permissions: http://www.gnu.org/software/libc/manual/html_node/Permission-Bits.html
	// glib-file-utilities: https://developer.gnome.org/glib/2.37/glib-File-Utilities.html
	const gchar *fileName;
	char sourcePath[MAX_PATH_LEN], targetPath[MAX_PATH_LEN];
	GFile *sourceFile, *targetFile;
	GFileEnumerator* enumerator;
	GFileInfo *fileInfo;
	GError *error = NULL;

	printf("qaul_copyDirectory: %s -> %s\n", source, target);

	// test if source path is a directory
	if(g_file_test(source, G_FILE_TEST_IS_DIR))
	{
		// check if directory exists & create it otherwise
		if(!g_file_test(target, G_FILE_TEST_EXISTS) && g_mkdir(target, S_IRUSR|S_IWUSR|S_IXUSR)==-1)
		{
			printf("qaul.net home directory %s creation error.\n", target);
			return 0;
		}
		else
		{
			// get directories files
			sourceFile = g_file_new_for_path(source);
			enumerator = g_file_enumerate_children(sourceFile, "*", G_FILE_QUERY_INFO_NONE, NULL, &error);

			// loop through the directories files
			fileInfo = g_file_enumerator_next_file(enumerator, NULL, &error);
			while(fileInfo != NULL)
			{
				// copy regular files
				if(g_file_info_get_file_type(fileInfo) == G_FILE_TYPE_REGULAR)
				{
					// get source file
					fileName = g_file_info_get_name(fileInfo);
					sourceFile = g_file_get_child(g_file_enumerator_get_container(enumerator), fileName);
					// create target path
					sprintf(targetPath, "%s/%s", target, fileName);
					targetFile = g_file_new_for_path(targetPath);
					if(!g_file_copy(sourceFile, targetFile, G_FILE_COPY_OVERWRITE, NULL, NULL, NULL, &error))
					{
						printf("qaul_copyDirectory copy file error: %s\n", (char*)error->message);
					}
				}
				// recursively copy directories
				else if(g_file_info_get_file_type(fileInfo) == G_FILE_TYPE_DIRECTORY)
				{
					fileName = g_file_info_get_name(fileInfo);
					sprintf(sourcePath, "%s/%s", source, fileName);
					sprintf(targetPath, "%s/%s", target, fileName);
					if(!qaul_copyDirectory(sourcePath, targetPath))
						printf("qaul_copyDirectory error copying %s -> %s", sourcePath, targetPath);
				}

				// free file info
				g_object_unref(fileInfo);
				fileInfo = g_file_enumerator_next_file(enumerator, NULL, &error);
			}
		}
		return 1;
	}
	else
		printf("qaul_copyDirectoryRecursively source is not a directory error: %s\n", source);
/*
	else
	{
		sourcefile = g_file_new_for_path(source);
		targetfile = g_file_new_for_path(target);
		if(!g_file_copy(sourcefile, targetfile, G_FILE_COPY_OVERWRITE, NULL, NULL, NULL, &error))
		{
			printf("qaul_copyDirectory error: %s\n", (char*)error->message);
			return 0;
		}
	}
*/
	return 0;
}

void qaul_onquit(void)
{
	// stop configuration
	if(qaulConfigureCounter < 60)
		g_source_remove(qaulConfigureTimer);
	else
	{
		// stop services
		printf("[quit] qaul_stopPortForwarding\n");
		qaul_stopPortForwarding();
		printf("[quit] qaul_olsrdStop\n");
		qaul_olsrdStop();

		// stop network
		// deactivate connection
		if(qaul_network_connection_deactivate(network_dbus_connection, &network_settings))
			printf("[quit] connection deactivated\n");
		else
			printf("[quit] connection not deactivated\n");

		// delete connection settings
		if(qaul_network_settings_delete(network_dbus_connection, &network_settings))
			printf("[quit] connection settings deleted\n");
		else
			printf("[quit] connection settings not deleted\n");
	}

	// stop timers
	if(qaulTimerEvents)
		g_source_remove(qaulTimerEvents);
	if(qaulTimerSockets)
		g_source_remove(qaulTimerSockets);
	if(qaulTimerTopology)
		g_source_remove(qaulTimerTopology);
}

gboolean qaul_configure(gpointer data)
{
    // initialize qaul library
    if(qaulConfigureCounter == 0)
    {
        // everything is fine
        Qaullib_ConfigStart();
        qaulConfigureCounter = 3;
    }

    // check authorization
    if(qaulConfigureCounter == 10)
    {
        // nothing to be done here
    	qaulConfigureCounter = 20;
    }

    // TODO: enable networking

    // get network interface
    if(qaulConfigureCounter == 20)
    {
        // check if interface has been configured manually
    	if(Qaullib_GetInterfaceManual())
    	{
    		printf("[configure] interface manually configured\n");
    		if(qaul_network_device_get_by_interface(Qaullib_GetInterface(), network_dbus_connection, &network_device))
    			network_interface_found = 1;
    		else
    			printf("[configure] manually configured interface \"%s\" not found\n", Qaullib_GetInterface());
    	}
    	// find wifi interface
    	else
    	{
    		if(qaul_network_find_wifi(network_dbus_connection, &network_device))
    			network_interface_found = 1;
    		else
    			printf("[configure] no wifi interface found\n");
    	}

    	// TODO: enable wifi

    	qaulConfigureCounter = 21;
    }

    // configure network interface
    if(qaulConfigureCounter == 21)
    {
        if(network_interface_found)
        {
        	printf("[configure] network interface %s\n", network_device.interface);

        	// get network configuration
        	strncpy(network_settings.ipv4_address, Qaullib_GetIP(), sizeof(network_settings.ipv4_address));
        	Qaullib_GetConfString("net.gateway", network_settings.ipv4_gateway);
        	network_settings.ipv4_netmask = Qaullib_GetConfInt("net.mask");
        	strncpy(network_settings.ipv4_dns1, "5.45.96.220", sizeof(network_settings.ipv4_dns1));
        	strncpy(network_settings.ipv4_dns2, "185.82.22.133", sizeof(network_settings.ipv4_dns2));
        	network_settings.wifi_channel = Qaullib_GetConfInt("wifi.channel");
        	Qaullib_GetConfString("wifi.ssid", network_settings.wifi_ssid);

        	// add network configuration
        	if(qaul_network_settings_add(network_dbus_connection, &network_settings, &network_device))
        	{
        		printf("[configure] network connection setting added: %s\n", network_settings.dbus_connection_path);

        		// activate configuration
        		if(qaul_network_connection_activate(network_dbus_connection, &network_settings, &network_device))
        			printf("[configure] network connection activated: %s\n", network_settings.dbus_active_connection_path);
        		else
        			printf("[configure] network connection not activated\n");
        	}
        	else
        		printf("[configure] network connection settings not added\n");
        }

    	qaulConfigureCounter = 29;
    }

    // check if username is set
    if(qaulConfigureCounter == 30)
    {
        if(Qaullib_ExistsUsername())
			qaulConfigureCounter = 40;
        else
        {
            // wait
            qaulConfigureCounter--;
        }
    }

    // start olsrd
    if(qaulConfigureCounter == 40)
    {
        printf("[configure] start olsrd \n");

        // start olsrd
        qaul_olsrdStart();

        qaulConfigureCounter = 44;
    }

    // connect ipc
    if(qaulConfigureCounter == 45)
    {
        printf("[configure] connect ipc \n");
        Qaullib_IpcConnect();
        qaulConfigureCounter = 46;
    }

    // start captive portal
    if(qaulConfigureCounter == 46)
    {
    	printf("[configure] start captive portal \n");
    	Qaullib_SetConfVoIP();
        Qaullib_UDP_StartServer();
        Qaullib_CaptiveStart();

        // configure firewall
        qaul_startPortForwarding();

        qaulConfigureCounter = 50;
    }

    // start timers
    if(qaulConfigureCounter == 50)
    {
        printf("[configure] timers \n");

		// start timers
		qaulTimerEvents = g_timeout_add(10, qaul_timerSocket, NULL);
		qaulTimerSockets = g_timeout_add(100, qaul_timerEvent, NULL);
		qaulTimerTopology = g_timeout_add(5000, qaul_timerTopology, NULL);

        Qaullib_ConfigurationFinished();

        qaulConfigureCounter = 60;
    }

    // end configuration
	if(qaulConfigureCounter == 60)
	{
		printf("[configure] finished \n");
		return FALSE;
	}

	qaulConfigureCounter++;
	return TRUE;
}


void qaul_olsrdStart(void)
{
	char command[255];
	sprintf(command, "%s/bin/qaulhelper startolsrd no %s", QAUL_ROOT_PATH, network_device.interface);
	system(command);
}

void qaul_olsrdStop(void)
{
	char command[255];
	sprintf(command, "%s/bin/qaulhelper stopolsrd", QAUL_ROOT_PATH);
	system(command);
}

void qaul_startPortForwarding(void)
{
	char command[255];
	sprintf(command, "%s/bin/qaulhelper startportforwarding %s %s", QAUL_ROOT_PATH, network_device.interface, network_settings.ipv4_address);
	system(command);
}

void qaul_stopPortForwarding(void)
{
	char command[255];
	sprintf(command, "%s/bin/qaulhelper stopportforwarding", QAUL_ROOT_PATH);
	system(command);
}

gboolean qaul_timerEvent(gpointer data)
{
    int myEvent;
    gchar myFilePath[MAX_URL_LEN +8];
	GError *myError;
	GtkWidget *myDialog;
	char *myFileChoose;

    myEvent = Qaullib_TimedCheckAppEvent();

    if(myEvent > 0)
    {
		printf("qaul_timerEvent: event [%i] found \n", myEvent);

    	if(myEvent == QAUL_EVENT_CHOOSEFILE)
        {
			printf("QAUL_EVENT_CHOOSEFILE \n");
    		myDialog = gtk_file_chooser_dialog_new("Choose File",
								  GTK_WINDOW(qaulMainWindow),
								  GTK_FILE_CHOOSER_ACTION_OPEN,
								  GTK_STOCK_CANCEL,
								  GTK_RESPONSE_CANCEL,
								  GTK_STOCK_OPEN,
								  GTK_RESPONSE_ACCEPT,
								  NULL);
			if (gtk_dialog_run(GTK_DIALOG(myDialog)) == GTK_RESPONSE_ACCEPT)
			{
				myFileChoose = gtk_file_chooser_get_filename(GTK_FILE_CHOOSER(myDialog));
				Qaullib_FilePicked(2, myFileChoose);
				g_free(myFileChoose);
			}
			gtk_widget_destroy(myDialog);
        }
        else if(myEvent == QAUL_EVENT_OPENFILE)
        {
			printf("QAUL_EVENT_OPENFILE \n");

        	// open file in default program
            myError = NULL;
            g_snprintf(myFilePath, MAX_URL_LEN +8, "file://%s", Qaullib_GetAppEventOpenPath());
            if(gtk_show_uri(gdk_screen_get_default(), myFilePath, gtk_get_current_event_time(), &myError))
				printf("Open file %s\n", myFilePath);
			else
				printf("Error opening file %s\n", myFilePath);

        	if(myError != NULL)
        	{
        		g_printerr("Error QAUL_EVENT_OPENFILE: %s\n", myError->message);
        		g_error_free(myError);
        	}
        }
        else if(myEvent == QAUL_EVENT_OPENURL)
        {
			printf("QAUL_EVENT_OPENURL \n");

        	// open URL in default browser
        	myError = NULL;
            if(gtk_show_uri(
					NULL,
					Qaullib_GetAppEventOpenURL(),
					GDK_CURRENT_TIME,
					&myError
				))
				printf("Open URL %s\n", Qaullib_GetAppEventOpenURL());
			else
				printf("Error opening URL %s\n", Qaullib_GetAppEventOpenURL());

        	if(myError != NULL)
        	{
        		g_printerr("Error QAUL_EVENT_OPENURL: %s\n", myError->message);
        		g_error_free(myError);
        	}
        }
        else if(myEvent == QAUL_EVENT_QUIT)
        {
			printf("QAUL_EVENT_QUIT \n");

            // quit application
			qaul_onquit();
			gtk_main_quit();
        }
        else if(myEvent == QAUL_EVENT_NOTIFY || myEvent == QAUL_EVENT_RING)
        {
			printf("QAUL_EVENT_NOTIFY || QAUL_EVENT_RING \n");

            // play beep
            gdk_beep();
        }
        else if(myEvent == QAUL_EVENT_GETINTERFACES)
        {
			printf("QAUL_EVENT_GETINTERFACES \n");

            // search for Interfaces
            if(qaul_network_devices_json(network_dbus_connection, network_json_txt))
            {
            	// set Interfaces
            	Qaullib_SetInterfaceJson(network_json_txt);
            }
        }
    }

	return TRUE;
}

gboolean qaul_timerSocket(gpointer data)
{
    Qaullib_TimedSocketReceive();
	return TRUE;
}

gboolean qaul_timerTopology(gpointer data)
{
    Qaullib_IpcSendCom(1);
    Qaullib_TimedDownload();
	return TRUE;
}
