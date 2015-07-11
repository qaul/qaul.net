/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#import "QaulConfigWifi.h"

// ----------------------------------------------
// definitions
// ----------------------------------------------
#ifndef NSAppKitVersionNumber10_4
#define NSAppKitVersionNumber10_4 824
#else
#define IS_OSX_10_5_OR_HIGHER 1
#endif

#ifndef NSAppKitVersionNumber10_5
#define NSAppKitVersionNumber10_5 949
#else
#define IS_OSX_10_6_OR_HIGHER 1
#endif

#ifndef NSAppKitVersionNumber10_6
#define NSAppKitVersionNumber10_6 1038
#else
#define IS_OSX_10_7_OR_HIGHER 1
#endif
// ----------------------------------------------

#ifdef IS_OSX_10_6_OR_HIGHER
#import <CoreWLAN/CoreWLAN.h>
#endif

@implementation QaulConfigWifi

- (id)init 
{ 
	NSLog(@"QaulConfigWifi init");
    
    if( self = [super init] ) 
	{ 
        [self setPaths];
    }
	return self; 
} 

- (void)setPaths
{
	NSLog(@"QaulConfigWifi setPaths");
    
    if(floor(NSAppKitVersionNumber) > NSAppKitVersionNumber10_4) 
		networksetupPath=[NSString stringWithFormat:@"/usr/sbin/networksetup"];
	else 
		networksetupPath=[NSString stringWithFormat:@"/System/Library/CoreServices/RemoteManagement/ARDAgent.app/Contents/Support/networksetup"];	
	
	airportPath = [NSString stringWithFormat:@"/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport"];
	qaulhelperPath = [NSString stringWithFormat:@"/Library/qaul.net/qaulhelper"];
}

- (BOOL)runTask:(NSString*)path arguments:(NSArray*)arguments
{
	NSTask *task;
	task = [[NSTask alloc] init];
	[task setLaunchPath:path];
	
	[task setArguments: arguments];
	
	NSPipe *pipe;
	pipe = [NSPipe pipe];
	[task setStandardOutput: pipe];
	
	NSFileHandle *file;
	file = [pipe fileHandleForReading];
	
	[task launch];
	
	NSData *data;
	data = [file readDataToEndOfFile];
	
	NSString *myString;
	myString = [[NSString alloc] initWithData: data encoding: NSUTF8StringEncoding];
	
	[task release];
	[myString release];
	
	return true;
}

- (BOOL)startAirport:(SCNetworkInterfaceRef)interface
{
	[self setPaths];
    NSLog(@"start airport\n");
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"enablewifi",[NSString stringWithFormat:@"%i",(int)floor(NSAppKitVersionNumber)],SCNetworkInterfaceGetBSDName(interface),nil]];
}

- (BOOL)stopAirport:(SCNetworkInterfaceRef)interface
{
	[self setPaths];
    NSLog(@"stop airport\n");
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"disablewifi",[NSString stringWithFormat:@"%i",(int)floor(NSAppKitVersionNumber)],SCNetworkInterfaceGetBSDName(interface),nil]];
}

- (BOOL)setAddress:(NSString*)address service:(SCNetworkServiceRef)service
{
	[self setPaths];
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"setip",SCNetworkServiceGetName(service),address,@"255.0.0.0",@"0.0.0.0",nil]];
}

- (BOOL)setDhcp:(SCNetworkServiceRef)service interface:(SCNetworkInterfaceRef)interface
{
	[self setPaths];
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"setdhcp",SCNetworkServiceGetName(service),SCNetworkInterfaceGetBSDName(interface),nil]];
}


- (BOOL)connect2network:(NSString*)name channel:(int)channel interface:(SCNetworkInterfaceRef)interface service:(SCNetworkServiceRef)service
{
	[self setPaths];
    NSLog(@"connect 2 network\n");
    BOOL created;
	
	if(floor(NSAppKitVersionNumber) < NSAppKitVersionNumber10_6)
	{
		NSLog(@"OSX 10.5 or lower");
        created = [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"createibss",[NSString stringWithFormat:@"%@",name],[NSString stringWithFormat:@"%i",channel],nil]];
	}
#ifdef IS_OSX_10_6_OR_HIGHER
	else 
	{
		NSLog(@"OSX 10.6 or higher");
		
		// use CoreWLAN framework for OSX >= 10.6
		// ----------------------------------------------
		// create ibss network
		NSMutableDictionary *ibssParams = [NSMutableDictionary dictionaryWithCapacity:0];
		[ibssParams setValue:name forKey:kCWIBSSKeySSID];
		[ibssParams setValue:[NSNumber numberWithInt:channel] forKey:kCWIBSSKeyChannel];
		//[ibssParams setValue:@"" forKey:kCWIBSSKeyPassphrase];
		NSError *error = nil;
		CWInterface* wifiInterface = [CWInterface interfaceWithName:[NSString stringWithFormat:@"%@",SCNetworkInterfaceGetBSDName(interface)]];
		//if (wifiInterface) NSLog(@"CWInterface wifi interface created: %@", wifiInterface);
		//else NSLog([NSString stringWithFormat:@"%@",SCNetworkInterfaceGetBSDName(interface)]);
		created = [wifiInterface enableIBSSWithParameters:[NSDictionary dictionaryWithDictionary:ibssParams] error:&error];
		
		// if creation failed try to join the existing qaul.net network
		if(!created)
		{
			//NSLog(@"Error: %@", error);
			NSLog(@"join network");
			
			CW8021XProfile *user8021XProfile = [CW8021XProfile profile];
			user8021XProfile.ssid = name;
			user8021XProfile.userDefinedName = name;			
			user8021XProfile.username = nil;
			user8021XProfile.password = nil;
			
			NSMutableDictionary *params = [NSMutableDictionary dictionaryWithCapacity:0];
			[params setValue:user8021XProfile forKey:kCWAssocKey8021XProfile];
			// scan for network:
			error = nil;
			NSMutableArray* scan = [NSMutableArray arrayWithArray:[wifiInterface scanForNetworksWithParameters:params error:&error]];
			if(error) 
				NSLog(@"scanning error: %@", error);
			else 
				NSLog(@"objects in array: %i",[scan count]);
			// loop through networks and search for qaul.net
			CWNetwork *selectedNetwork;
			for(selectedNetwork in scan)
			{
				if ([name isEqualToString:selectedNetwork.ssid]) 
					break;
			}
			NSLog(@"network Name: %@", selectedNetwork.ssid);
			if(selectedNetwork)
			{
				error = nil;
				params = nil;
				[params setValue:nil forKey:kCWAssocKeyPassphrase];
				created = [wifiInterface associateToNetwork:selectedNetwork parameters:[NSDictionary dictionaryWithDictionary:params] error:&error];
				if(created) 
				{
					NSLog(@"qaul.net joined");
				}
				else 
					NSLog(@"joining qaul.net failed: %@", error);				
			}
			else 
			{
				NSLog(@"Network qaul.net not found!");
				created = false;
			}
            
		}
	}
#endif
    
    // set dns servers for internet gateway
    // TODO: WLAN adapter names with spaces
    if([self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"setdns",SCNetworkServiceGetName(service),nil]])
        NSLog(@"DNS servers set");
    
	return created;
}

- (BOOL)startOlsrd:(SCNetworkInterfaceRef)interface
{
	[self setPaths];
    NSLog(@"qaulhelper startolsrd %@ %@", @"no", SCNetworkInterfaceGetBSDName(interface));
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"startolsrd", @"no", SCNetworkInterfaceGetBSDName(interface), nil]];
}

- (BOOL)stopOlsrd
{
	[self setPaths];
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"stopolsrd",nil]];
}

- (BOOL)startPortForwarding:(SCNetworkInterfaceRef)interface
{
	[self setPaths];
    NSLog(@"start port forwarding");
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"startportforwarding",SCNetworkInterfaceGetBSDName(interface),nil]];
}

- (BOOL)stopPortForwarding
{
	[self setPaths];
    return [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"stopportforwarding",nil]];
}

- (BOOL)createNetworkProfile
{
	[self setPaths];
    NSLog(@"createNetworkProfile");
	
	NSTask *task;
	task = [[NSTask alloc] init];
	[task setLaunchPath: @"/usr/sbin/networksetup"];
	
	NSArray *arguments;
	arguments = [NSArray arrayWithObjects: @"-getcurrentlocation", nil];
	[task setArguments: arguments];
	
	NSPipe *pipe;
	pipe = [NSPipe pipe];
	[task setStandardOutput: pipe];
	
	NSFileHandle *file;
	file = [pipe fileHandleForReading];
	
	[task launch];
	
	NSData *data;
	data = [file readDataToEndOfFile];
	
	NSString *allFile;	
	allFile = [[NSString alloc] initWithData: data encoding: NSUTF8StringEncoding];
	arguments = [allFile componentsSeparatedByString:@"\n"];
	networkProfile = [[arguments objectAtIndex:0] description];
	
	NSLog (@"current networkProfile: '%@'", networkProfile);
    [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"createnetworkprofile",@"new",nil]];
    
	[task release];
	
	// ----------------------------------------------------------------
    [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"createnetworkprofile",@"qaul.net",nil]];
    [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"switchnetworkprofile",@"qaul.net",nil]];
    
	NSLog(@"createNetworkProfile created");
	return true;	
}

- (BOOL)deleteNetworkProfile
{
	[self setPaths];
    NSLog(@"deleteNetworkProfile");
    [self runTask:qaulhelperPath arguments:[NSArray arrayWithObjects:@"switchnetworkprofile",@"new",nil]];
	NSLog(@"deleteNetworkProfile deleted");
	
	return true;		
}

@end
