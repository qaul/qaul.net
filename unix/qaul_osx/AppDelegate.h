/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#import <Cocoa/Cocoa.h>
#import <WebKit/WebKit.h>
#import "QaulConfigWifi.h"


@interface AppDelegate : NSObject <NSApplicationDelegate>
{
    NSWindow *qaulWindow;
    WebView *qaulWebView;
    
    IBOutlet id start_name;
	NSString *username;
	NSString *qaulResourcePath;
	int qaulStarted;
	
	// wifi config
	QaulConfigWifi *qaulConfigWifi;
    BOOL qaulInterfaceManual;
    NSString *qaulInterfaceName;
	SCNetworkInterfaceRef qaulWifiInterface;
	BOOL qaulWifiInterfaceSet;
    BOOL qaulWifiInterfaceConfigurable;
	SCNetworkServiceRef qaulServiceId;
	BOOL qaulServiceFound;
	BOOL qaulServiceConfigured;
	CFStringRef qaulServiceName;
	OSStatus status;
	
	// Timer
	NSTimer *qaullibTimer;
	NSTimer *qaullibTimer2;
	NSTimer *qaullibTimer3;
}

@property (assign) IBOutlet NSWindow *qaulWindow;
@property (assign) IBOutlet WebView *qaulWebView;

/**
 * copy files to the home directory the first time the application is started
 */
- (void)copyFilesAtFirstStartup;
- (void)createDirectory:(NSString *)directoryName atFilePath:(NSString *)filePath;

/** 
 * support methods
 */
- (void)endAlert:(id)sheet returnCode:(int)returnCode contextInfo:(void *)contextInfo;
- (void)init_app;

/**
 * create json of all interfaces for qaullib
 */
- (void)createInterfaceJson;

/**
 * timers
 */
- (void)startTimer;
- (void)stopTimer;
- (void)checkIpcMessage:(NSTimer *)theTimer;
- (void)checkIpcTopology:(NSTimer *)theTimer;
- (void)checkNames;

/**
 * wait timer
 */
- (void)startDelay:(NSTimeInterval)secs;
- (void)fireDelay;

@end
