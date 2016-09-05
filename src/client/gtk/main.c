/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#define GetCurrentDir getcwd

#include <stdio.h> // defines FILENAME_MAX
#include <stdlib.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/stat.h>
#include <string.h>
#include <glib.h>
#include <glib/gstdio.h>
#include "qaullib.h"

#include <gtk/gtk.h>
#include <webkit/webkit.h>

#include <QaulConfig.h>
#include "structures.h"
#include "qaul_configure.h"
#include "configure.h"
#include "networkmanager_configuration.h"


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
gint qaulConfigureTimer;
char network_json_txt[MAX_JSON_LEN +1];

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
	char qaulHomePath[MAX_PATH_LEN];
	char qaulResourcePath[MAX_PATH_LEN];
	char qaulTmpPath[MAX_PATH_LEN];
	char qaulTmpPath2[MAX_PATH_LEN];

	qaulConfigureCounter = 0;
	qaulTimerEvents = 0;
	qaulTimerSockets = 0;
	qaulTimerTopology = 0;
	network_interface_found = 0;

	// initialize glib types
	// TODO this is only needed for GLib prior 2.36
	g_type_init();

	// set paths
	sprintf(qaulHomePath, "%s/.qaul", (char*)g_get_home_dir());
	printf ("qaul.net home directory is %s\n", qaulHomePath);
	sprintf(qaulResourcePath, "%s/lib/qaul", QAUL_ROOT_PATH);
	printf ("qaul.net resource directory is %s\n", qaulResourcePath);

	// create qaul user directory
	if(!g_file_test(qaulHomePath, G_FILE_TEST_EXISTS))
	{
		// create directory
		// http://www.gnu.org/software/libc/manual/html_node/Permission-Bits.html
		if(g_mkdir(qaulHomePath, S_IRUSR|S_IWUSR|S_IXUSR)== -1)
			printf("qaul.net home directory %s creation error.\n", qaulHomePath);
	}
	// check if we have to update
	sprintf(qaulTmpPath, "%s/%s", qaulHomePath, QAUL_VERSION);
	if(!g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
	{
		printf("Update user folder to qaul.net version %s\n", QAUL_VERSION);
		// copy www folder
		sprintf(qaulTmpPath, "%s/files", qaulResourcePath);
		sprintf(qaulTmpPath2, "%s/files", qaulHomePath);
		if(!qaul_copyDirectory(qaulTmpPath, qaulTmpPath2))
			printf("qaul copy directory error. source: %s target: %s\n", qaulTmpPath, qaulTmpPath2);
		// TODO: update data base
		// remove old data base if it exists
		sprintf(qaulTmpPath, "%s/qaullib.db", qaulHomePath);
		if(g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
			if(g_remove(qaulTmpPath) == -1)
				printf("qaul.net database %s removal error\n", qaulTmpPath);
		// create qaul version file
		sprintf(qaulTmpPath, "%s/%s", qaulHomePath, QAUL_VERSION);
		if(!g_file_test(qaulTmpPath, G_FILE_TEST_EXISTS))
			if(!g_creat(qaulTmpPath, S_IRUSR|S_IWUSR) == -1)
				printf("qaul.net version file %s creation error\n", qaulTmpPath);
	}

	Qaullib_Init(qaulHomePath, qaulResourcePath);
	// set configuration
	Qaullib_SetConf(QAUL_CONF_INTERFACE);
	Qaullib_SetConf(QAUL_CONF_INTERNET);
	Qaullib_SetConf(QAUL_CONF_NETWORK);

	// enable debug menu
	qaul_conf_debug = 1;

	if(!Qaullib_WebserverStart())
		printf("Webserver startup failed!\n");

	// start configuration timer
	qaulConfigureTimer = g_timeout_add(500, qaul_configure, NULL);

	// open window
	gtk_init(&argc,&argv);

    // Create a window that will contain the browser instance
    qaulMainWindow = gtk_window_new(GTK_WINDOW_TOPLEVEL);
    gtk_window_set_default_size(GTK_WINDOW(qaulMainWindow), 400, 592);
    gtk_window_set_title(GTK_WINDOW(qaulMainWindow), "qaul.net - قول");
    sprintf(qaulTmpPath, "%s/lib/qaul/icons/qaul_app_icon.png", QAUL_ROOT_PATH);
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
		printf("[quit] qaul_stopInternetSharing\n");
		qaul_stopGateway();
		printf("[quit] qaul_olsrdStop\n");
		qaul_olsrdStop();

		// stop network
		printf("[quit] qaul_networkStop\n");
		qaul_networkStop();
	}

	// stop timers
	if(qaulTimerEvents)
		g_source_remove(qaulTimerEvents);
	if(qaulTimerSockets)
		g_source_remove(qaulTimerSockets);
	if(qaulTimerTopology)
		g_source_remove(qaulTimerTopology);
}

void qaul_startTimers(void)
{
	qaulTimerEvents = g_timeout_add(10, qaul_timerSocket, NULL);
	qaulTimerSockets = g_timeout_add(100, qaul_timerEvent, NULL);
	qaulTimerTopology = g_timeout_add(5000, qaul_timerTopology, NULL);
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
            if(qaul_getInterfacesJson(network_json_txt))
            {
            	// set Interfaces
            	Qaullib_SetInterfaceJson(network_json_txt);
            }
        }
        else if(myEvent == QAUL_EVENT_GATEWAY_START)
        {
        	// configure firewall
        	qaul_startGateway();
        	// restart olsrd
        	qaul_olsrdStop();
        	qaul_olsrdStart();
        }
        else if(myEvent == QAUL_EVENT_GATEWAY_STOP)
        {
        	// configure firewall
        	qaul_stopGateway();
        	// restart olsrd
        	qaul_olsrdStop();
        	qaul_olsrdStart();
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
