/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#import "AppDelegate.h"
#import "qaullib.h"

@implementation AppDelegate

@synthesize qaulWindow;
@synthesize qaulWebView;

- (void)dealloc
{
    [super dealloc];
}
	
- (void)applicationDidFinishLaunching:(NSNotification *)aNotification
{
    NSLog(@"applicationDidFinishLaunching");
    
    // initialize variables at startup
    qaulStarted = 0;
    qaulConfigWifi = [[QaulConfigWifi alloc] init];
    qaulWifiInterface = nil;
    qaulWifiInterfaceSet = FALSE;
    qaulWifiInterfaceConfigurable = FALSE;
    qaulServiceConfigured = FALSE;
    qaulInterfaceManual = FALSE;
    
    // start qaul configuration
	[self init_app];
}

// quit qaul.net app when window is closed
- (BOOL)applicationShouldTerminateAfterLastWindowClosed:(NSApplication *)theApplication 
{
    return YES;
}

// -------------------------------------------------------------------------
// first startup tasks
// -------------------------------------------------------------------------
- (void)copyFilesAtFirstStartup
{
	NSFileManager *filemgr;
	NSError *dError;    
	NSString *qaulApplicationPath;
    
    NSLog(@"CopyFilesAtFirstStartup 1");
    
    // check if files have been copied
    filemgr =[NSFileManager defaultManager];
    
    NSLog(@"CopyFilesAtFirstStartup 2");
    
    qaulApplicationPath = [[NSBundle mainBundle] resourcePath];
    NSLog(@"CopyFilesAtFirstStartup");
    NSLog(@"qaulResourcePath: %@, qaulApplicationPath: %@", qaulResourcePath, qaulApplicationPath);    
    
	// check if data base exists
	if(![filemgr fileExistsAtPath:[NSString stringWithFormat:@"%@/qaullib.db", qaulResourcePath]])
	{
        NSLog(@"First startup: copy files to document directory");
        
        // create resource folder
        if (![filemgr createDirectoryAtPath:qaulResourcePath withIntermediateDirectories:NO attributes:nil error:&dError])
            NSLog(@"Create qaulResourcePath directory error: %@", dError);
        
		if(![filemgr copyItemAtPath:[NSString stringWithFormat:@"%@/www", qaulApplicationPath] toPath:[NSString stringWithFormat:@"%@/www", qaulResourcePath] error:&dError])
			NSLog(@"Error: %@", dError);
		else
			NSLog(@"Sucessfully copied");
	}
	else
		NSLog(@"Not first startup");
	
	[filemgr release];
}

- (void)createDirectory:(NSString *)directoryName atFilePath:(NSString *)filePath
{
    NSString *filePathAndDirectory = [filePath stringByAppendingPathComponent:directoryName];
    NSError *error;
    
    if (![[NSFileManager defaultManager] createDirectoryAtPath:filePathAndDirectory
                                   withIntermediateDirectories:NO
                                                    attributes:nil
                                                         error:&error])
    {
        NSLog(@"Create directory error: %@", error);
    }
}

// -------------------------------------------------------------------------
// quit application when window is closed
// -------------------------------------------------------------------------
- (void)endAlert:(id)sheet returnCode:(int)returnCode contextInfo:(void *)contextInfo
{
	if (returnCode == NSAlertFirstButtonReturn)
	{
		// quit application
		[NSApp terminate:nil];
    }
}

- (void)applicationWillTerminate:(NSNotification *)notification
{
	NSLog(@"Application should Terminate Event");
	
	// TODO: stop timers
    
	// stop library
	Qaullib_Exit();
	
	// stop olsrd 
	if(![qaulConfigWifi stopOlsrd]) 
		NSLog(@"olsrd not killed");
	// stop portforwarding 
	if(![qaulConfigWifi stopPortForwarding]) 
		NSLog(@"portforwarding not removed");
	
	usleep(50000);
	// stop wifi
    if (qaulWifiInterfaceConfigurable)
    {
        if(![qaulConfigWifi stopAirport:qaulWifiInterface])
            NSLog(@"airport not stopped");
    }
	
	// change location
	[qaulConfigWifi deleteNetworkProfile];
	usleep(50000);
}

// -------------------------------------------------------------------------
// start library, wifi, routing
// -------------------------------------------------------------------------
- (void)init_app 
{
	NSLog(@"init_app");
	BOOL success;
    
	// check if it was initialized
	if(qaulStarted == 0)
	{
        // copy files at first startup
        qaulResourcePath = [NSString stringWithFormat:@"%@/qaul.net", [NSSearchPathForDirectoriesInDomains(NSLibraryDirectory, NSUserDomainMask, YES) objectAtIndex:0]];
        [self copyFilesAtFirstStartup];
        
		// init qaullib
		NSLog(@"initialize app");		
		Qaullib_Init([qaulResourcePath UTF8String]);
		
        // set Configuration
        Qaullib_SetConf(QAUL_CONF_INTERFACE);
		
		// set Download path
		NSString *downloadPath = [NSHomeDirectory() stringByAppendingPathComponent:@"/Downloads"];
		NSLog(@"path to download folder: %@", downloadPath);		
		Qaullib_SetConfDownloadFolder([downloadPath UTF8String]);
        
		qaulStarted = 1;
	}
	
	// start web server
	if(qaulStarted == 1)
	{
		// start web server
		Qaullib_WebserverStart();
        NSLog(@"qaullib web server start");
        
        qaulStarted = 2;
    }
    
	// set tab to web view
	if(qaulStarted == 2)
	{
		// change to chat view
		// BUG: does not work anymore if URL is:
        //      http://127.0.0.1:8081/qaul.html
        //      it has to be "localhost"
        NSString *myUrl = @"http://localhost:8081/qaul.html";
        NSLog(@"set URL %@", myUrl);
		//[qaulWebView setMainFrameURL:myUrl];
        [[qaulWebView mainFrame] loadRequest:[NSURLRequest requestWithURL:[NSURL URLWithString:myUrl]]];
		
		// start user configuration
		Qaullib_ConfigStart();
		
		qaulStarted = 3;
	}
    
	// get configuration
	if(qaulStarted == 3)
	{
		// check saved interface configuration
		if(Qaullib_GetInterfaceManual())
		{
			NSLog(@"interface is set manually");
            qaulInterfaceName = [NSString stringWithFormat:@"%s", Qaullib_GetInterface()];
            qaulInterfaceManual = true;
            NSLog(@"interface name is %@", qaulInterfaceName);
		}
		
		qaulStarted = 10;
	}
    
	// check authorization 
	if(qaulStarted == 10)
	{
        // tasks that need authorization are now executed by external qaulhelper
		qaulStarted = 20;
	}
    
	// create new network profile
	// we need to create a new network profile that we can 
	// trash afterwards, because the routing table gets messed up
	// otherwise.
	if(qaulStarted == 20)
	{
		success = [qaulConfigWifi createNetworkProfile];
		qaulStarted = 21;
	}
	
	// search Wifi interface
	if(qaulStarted == 21)
	{
		NSLog(@"loop through interfaces");
		// -----------------------------------
		// configure & start up wifi
        //		qaulInterfacesAll = (NSArray *) SCNetworkInterfaceCopyAll ();
        //		en = [qaulInterfacesAll objectEnumerator];
		SCNetworkInterfaceRef inter;
		SCPreferencesRef prefRef;
		prefRef = SCPreferencesCreate(kCFAllocatorSystemDefault, (CFStringRef)@"XXX", NULL);
		NSEnumerator *e = [(NSArray *)SCNetworkSetCopyServices(SCNetworkSetCopyCurrent(prefRef)) objectEnumerator];
		SCNetworkServiceRef service;
        
        // check if manually selected interface is present
		if(qaulInterfaceManual)
		{
            NSLog(@"check if manually selected interface is present");
            while (service = (SCNetworkServiceRef)[e nextObject])
            {
                inter = SCNetworkServiceGetInterface(service);
                
                NSString *myinterface = [NSString stringWithFormat:@"%@", SCNetworkInterfaceGetBSDName(inter)];
                NSLog(@"%@ <=> %@", myinterface, qaulInterfaceName);
                if([myinterface isEqualToString:qaulInterfaceName])
                {
                    NSLog(@"manual interface found: %@", myinterface);
                    // TODO: multiple interfaces
                    qaulServiceId = service;
                    qaulServiceFound = TRUE;
                    qaulWifiInterface = inter;
                    qaulWifiInterfaceSet = TRUE;
                    
                    // check if it is a Wifi
                    CFStringRef typesMy = SCNetworkInterfaceGetInterfaceType(inter);
                    if (typesMy == kSCNetworkInterfaceTypeIEEE80211)
                        qaulWifiInterfaceConfigurable = TRUE;
                    
                    // enable service if disabled
                    if(!SCNetworkServiceGetEnabled(qaulServiceId))
                    {
                        NSLog(@"enable Service");
                        if(!SCNetworkServiceSetEnabled(qaulServiceId, TRUE))
                            NSLog(@"service couldn't be enabled");
                        else
                            qaulServiceConfigured = true;
                    }
                    else
                        NSLog(@"SCNetworkServiceEnabled");
                    
                    // get Service Name
                    qaulServiceName = SCNetworkServiceGetName(qaulServiceId);
                    NSLog(@"service name: %@, interface: %@", qaulServiceName, SCNetworkInterfaceGetBSDName(inter));
                    
                    break;
                }
            }
            
            // fallback solution:
            // set interface configuration to auto,
            // if the interface was not found
            if (!qaulWifiInterfaceSet)
                Qaullib_SetInterfaceManual(0);
        }
        
		// fallback: select interface automatically
		if(!qaulWifiInterfaceSet)
		{
            NSLog(@"loop through services");
            while (service = (SCNetworkServiceRef)[e nextObject])
            {
                inter = SCNetworkServiceGetInterface(service);
                
                // check if it is a Wi-Fi interface
                CFStringRef typesMy = SCNetworkInterfaceGetInterfaceType(inter);
                if (typesMy == kSCNetworkInterfaceTypeIEEE80211)
                {
                    qaulServiceId = service;
                    qaulServiceFound = TRUE;
                    qaulWifiInterface = inter;
                    qaulWifiInterfaceSet = TRUE;
                    qaulWifiInterfaceConfigurable = TRUE;
                    
                    // enable service if disabled
                    if(!SCNetworkServiceGetEnabled(qaulServiceId))
                    {
                        NSLog(@"enable Service");
                        if(!SCNetworkServiceSetEnabled(qaulServiceId, TRUE))
                            NSLog(@"service couldn't be enabled");
                        else
                            qaulServiceConfigured = true;
                    }
                    else
                        NSLog(@"SCNetworkServiceEnabled");
                    
                    // get Service Name
                    qaulServiceName = SCNetworkServiceGetName(qaulServiceId);
                    NSLog(@"service name: %@, interface: %@", qaulServiceName, SCNetworkInterfaceGetBSDName(inter));
                    
                    break;                
                }            
            }
        }
        
		qaulStarted = 22;
	}
	
	// enable Service
	if(qaulStarted == 22)
	{
		
		// check if the Service is enabled
		if(!qaulServiceConfigured)
		{
			NSLog(@"Service not activated!");
			
			// TODO: create deleted Services
			// create it
			//SCNetworkSetAddService();
			//service = SCNetworkServiceCreate (prefRef, inter);
			// configure it
			//[service release];
			
		}
		
		qaulStarted = 23;
	}
	
	// switch airport on
	if(qaulStarted == 23)
	{
		NSLog(@"switch airport on");
		// switch on airport via cli
        if (qaulWifiInterfaceConfigurable) {
            success = [qaulConfigWifi startAirport:qaulWifiInterface];
            if(success)
                NSLog(@"startAirport success!!");
            else
                NSLog(@"startAirport no success");
        }
		
		qaulStarted = 24;
	}
	
	// set IP
	if(qaulStarted == 24)
	{
		// set IP
		NSString *myip = [NSString stringWithFormat:@"%s",Qaullib_GetIP()];
		success = [qaulConfigWifi setAddress:myip service:qaulServiceId];
		if(success)
			NSLog(@"setAddress success!!");
		else 
			NSLog(@"setAddress no success");		
		
		qaulStarted = 25;
	}
	
	if (qaulStarted == 25)
	{
		if(floor(NSAppKitVersionNumber) > NSAppKitVersionNumber10_5)  
			[self startDelay: 15.0f];
		else 
			[self startDelay: 3.0f];
	}
	
	// configure airport
	if(qaulStarted == 26)
	{
		if (qaulWifiInterfaceConfigurable)
        {
            // channel selection is buggy on many devices, default channel depends on OS, channel 1 usually works
            
            // get bssid and channel
            char config_ibss_c[32 +1];
            Qaullib_GetConfString("wifi.ssid", config_ibss_c);
            NSString *myibss = [NSString stringWithFormat:@"%s", config_ibss_c];
            
            success = [qaulConfigWifi connect2network:myibss channel:Qaullib_GetConfInt("wifi.channel") interface:qaulWifiInterface service:qaulServiceId];
            if(success)
                NSLog(@"connect2network success!!");
            else
                NSLog(@"connect2network no success");
        }
		
		qaulStarted = 29;
	}
	
	// usleep(7000000);
	if(qaulStarted == 29) 
		[self startDelay: 3.0f];
    
	// check username
	if(qaulStarted == 30)
	{
		// wait until username is set
		NSLog(@"check Username");
		if(Qaullib_ExistsUsername()) 
			qaulStarted = 40;
		else 
		{
			qaulStarted = 29;
			[self startDelay: 0.5f];
		}
	}
	
	// start olsrd
	if(qaulStarted == 40)
	{
		success = [qaulConfigWifi startOlsrd:qaulWifiInterface];
		if(success) 
			NSLog(@"olsrd start success!!");
		else 
			NSLog(@"olsrd start no success");
		
		qaulStarted = 44;
	}	
    
	if(qaulStarted == 44) 
		[self startDelay: 2.0f];
	
	// connect ipc
	if(qaulStarted == 45)
	{
		Qaullib_IpcConnect();
		
		qaulStarted = 46;
	}
	
	// connect captive
	if(qaulStarted == 46)
	{
		// start VoIP
		Qaullib_SetConfVoIP();
		
		// start 
		Qaullib_UDP_StartServer();
		
		// start captive portal
		Qaullib_CaptiveStart();
		
		// start port forwarding
		success = [qaulConfigWifi startPortForwarding:qaulWifiInterface];
        
        NSLog(@"captive portal configured");
		
		qaulStarted = 50;
	}
	
	// finished
	if(qaulStarted == 50)
	{
		// start timer
		[self startTimer];
		
		// set configuration to finished
		Qaullib_ConfigurationFinished();
		
		qaulStarted = 60;
	}
}

// -------------------------------------------------------------------------
// JSON files for configuration
// -------------------------------------------------------------------------
- (void)createInterfaceJson
{
    NSString* jsonString;
    NSString* oldString;
    NSString* newString;
    int ifType, ifNum;
    SCNetworkInterfaceRef inter_service;
    SCPreferencesRef prefRef;
    prefRef = SCPreferencesCreate(kCFAllocatorSystemDefault, (CFStringRef)@"XXX", NULL);
    NSEnumerator *e = [(NSArray *)SCNetworkSetCopyServices(SCNetworkSetCopyCurrent(prefRef)) objectEnumerator];
    SCNetworkServiceRef service;
    
    NSLog(@"createInterfaceJson");
    
    ifNum = 0;
    jsonString = [NSString stringWithFormat:@""];
    
    // loop through services
    while (service = (SCNetworkServiceRef)[e nextObject])
    {
        ifType = 0;            
        inter_service = SCNetworkServiceGetInterface(service);        
        
        // check if this service has a BSD interface
        if(SCNetworkInterfaceGetBSDName(inter_service))
        {
            // check if it is a Wi-Fi interface
            CFStringRef typesMy = SCNetworkInterfaceGetInterfaceType(inter_service);
            if (typesMy == kSCNetworkInterfaceTypeIEEE80211)
                ifType = 1;
            
            newString = [NSString stringWithFormat:@"{\"name\":\"%@\",\"ui_name\":\"%@\",\"type\":%i}",
                         SCNetworkInterfaceGetBSDName(inter_service),
                         SCNetworkServiceGetName(service),
                         ifType
                         ];
            
            if(ifNum == 0)
                jsonString = newString;
            else
            {
                oldString = jsonString;
                jsonString = [NSString stringWithFormat:@"%@,%@", oldString, newString];
            }
            
            ifNum++;
        }
    }
    
    // write interfaces to qaullib
    NSLog(@"interface json UTF-8: %s", [jsonString UTF8String]);
    Qaullib_SetInterfaceJson([jsonString UTF8String]);
}

// -------------------------------------------------------------------------
// Timer
// -------------------------------------------------------------------------
- (void)startTimer
{
	[qaullibTimer invalidate];
    qaullibTimer = [NSTimer scheduledTimerWithTimeInterval: 0.1 target:self selector:
                    @selector(checkIpcMessage:) userInfo:nil repeats:YES];
    [qaullibTimer fire];
	
	[qaullibTimer2 invalidate];
    qaullibTimer2 = [NSTimer scheduledTimerWithTimeInterval: 3.0 target:self selector:
                     @selector(checkIpcTopology:) userInfo:nil repeats:YES];
    [qaullibTimer2 fire];
	
	[qaullibTimer3 invalidate];
    qaullibTimer3 = [NSTimer scheduledTimerWithTimeInterval: 0.01 target:self selector:
					 @selector(checkAppEvents:) userInfo:nil repeats:YES];
    [qaullibTimer3 fire];
}

- (void)stopTimer
{
	[qaullibTimer invalidate];
    qaullibTimer = nil;
    
	[qaullibTimer2 invalidate];
    qaullibTimer2 = nil;
	
	[qaullibTimer3 invalidate];
    qaullibTimer3 = nil;
}

- (void)checkIpcMessage:(NSTimer *)theTimer
{
	Qaullib_TimedSocketReceive();
}

- (void)checkIpcTopology:(NSTimer *)theTimer
{
	Qaullib_IpcSendCom(1);
	[self performSelector:@selector(checkNames) withObject:nil afterDelay:2];
}

- (void)checkAppEvents:(NSTimer *)theTimer
{
	int appEvent = Qaullib_TimedCheckAppEvent();
	if (appEvent > 0)
	{
		NSLog([NSString stringWithFormat:@"appEvent found: %i\n", appEvent]);
        
		if(appEvent == QAUL_EVENT_CHOOSEFILE)
		{
			// show file picker
			int i; // Loop counter.
			
			// Create the File Open Dialog class.
			NSOpenPanel* openDlg = [NSOpenPanel openPanel];
			
			// Enable the selection of files in the dialog.
			[openDlg setCanChooseFiles:YES];
			
			// Disable the selection of multiple files in the dialog.
			[openDlg setAllowsMultipleSelection:NO];
			
			// Enable the selection of directories in the dialog.
			//[openDlg setCanChooseDirectories:YES];
			
			// Display the dialog.  If the OK button was pressed,
			// process the files.
			if ( [openDlg runModalForDirectory:nil file:nil] == NSOKButton )
			{
				// Get an array containing the full filenames of all
				// files and directories selected.
				NSArray* files = [openDlg filenames];
				
				// Loop through all the files and process them.
				for( i = 0; i < [files count]; i++ )
				{
					NSString* fileName = [files objectAtIndex:i];
					
					// Do something with the filename.
					NSLog(@"filename: %@\n", fileName);
					Qaullib_FilePicked(2, [fileName UTF8String]);
				}
			}			
		}
		else if(appEvent == QAUL_EVENT_OPENFILE)
		{
			// open file
			[[NSWorkspace sharedWorkspace] openFile:[NSString stringWithFormat:@"%s", Qaullib_GetAppEventOpenPath()]];
		}
		else if(appEvent == QAUL_EVENT_OPENURL)
		{
			// open url
			[[NSWorkspace sharedWorkspace] openURL:[NSURL URLWithString:[NSString stringWithFormat:@"%s", Qaullib_GetAppEventOpenURL()]]];
		}
		else if(appEvent == QAUL_EVENT_QUIT)
		{
			[NSApp terminate:nil];
		}
		else if(appEvent == QAUL_EVENT_NOTIFY || appEvent == QAUL_EVENT_RING)
		{
			NSLog(@"QAUL_EVENT_NOTIFY or QAUL_EVENT_RING received\n");
			NSBeep();
		}
		else if(appEvent == QAUL_EVENT_GETINTERFACES)
		{
			NSLog(@"QAUL_EVENT_GETINTERFACES received\n");
            [self createInterfaceJson];            
		}
	}
}

- (void)checkNames
{
	NSLog(@"check usernames by tcp");
	Qaullib_TimedDownload();	
}


// -------------------------------------------------------------------------
// Wait Timer
// -------------------------------------------------------------------------
- (void)startDelay:(NSTimeInterval)secs 
{
	NSLog(@"start Delay %f s\n",secs);
	[self performSelector:@selector(fireDelay) withObject:nil afterDelay:secs];	
}

- (void)fireDelay
{
	NSLog(@"Delay fired\n");
	qaulStarted++;
	[self init_app];
}


@end
