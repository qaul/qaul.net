/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#import <AppKit/NSApplication.h>
#import <Foundation/Foundation.h>
#import <SystemConfiguration/SystemConfiguration.h>

@interface QaulConfigWifi : NSObject 
{
	NSString* networksetupPath; 
	NSString* airportPath;
    NSString* qaulhelperPath;
	NSString* networkProfile;
}

- (void)setPaths;
- (BOOL)runTask:(NSString*)path arguments:(NSArray*)arguments;
- (BOOL)startAirport:(SCNetworkInterfaceRef)interface;
- (BOOL)stopAirport:(SCNetworkInterfaceRef)interface;
- (BOOL)setAddress:(NSString*)address service:(SCNetworkServiceRef)service mask:(NSString*)mask gateway:(NSString*)gateway;
- (BOOL)setDhcp:(SCNetworkServiceRef)service interface:(SCNetworkInterfaceRef)interface;
- (BOOL)connect2network:(NSString*)name channel:(int)channel interface:(SCNetworkInterfaceRef)interface service:(SCNetworkServiceRef)service;
- (BOOL)startOlsrd:(int)isGateway interface:(NSString*)interface;
- (BOOL)stopOlsrd;
- (BOOL)startPortForwarding:(NSString*)interface;
- (BOOL)stopPortForwarding;
- (BOOL)startGateway:(NSString*)gateway;
- (BOOL)stopGateway;
- (BOOL)createNetworkProfile;
- (BOOL)deleteNetworkProfile;

@end
