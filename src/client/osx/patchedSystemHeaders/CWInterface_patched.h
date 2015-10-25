/**
 * Patched CoreWLAN framework definitions for qaul.net. They were removed from the 
 * public headers by apple, but still available.
 * This file represents the SDK options available in OSX 10.8.
 */

#ifndef _CORE_WLAN_INTERFACE_H_
#define _CORE_WLAN_INTERFACE_H_

#import <Foundation/Foundation.h>
#import <CoreWLAN/CoreWLANTypes.h>

#pragma mark -
@class CWNetwork, CWChannel, CWConfiguration, SFAuthorization;

@interface CWInterface : NSObject {
@private
	void *_device;
	NSString *_interfaceName;
	NSArray *_capabilities;
	IONotificationPortRef _interfaceRemovedNotificationPort;
	io_iterator_t _interfaceRemovedNotification;
	BOOL _deviceAttached;
	id _eapolClient;
	id _ipMonitor;
	dispatch_queue_t _internalQueue;
	void *_serviceStore;
	void *_interfaceStore;
    BOOL _lastPowerState;
}

@property(readonly, assign) BOOL powerOn NS_AVAILABLE_MAC(10_7);
@property(readonly, copy) NSString *interfaceName NS_AVAILABLE_MAC(10_7);
@property(readonly) NSSet *supportedWLANChannels NS_AVAILABLE_MAC(10_7);
@property(readonly) CWChannel *wlanChannel NS_AVAILABLE_MAC(10_7);
@property(readonly) CWPHYMode activePHYMode NS_AVAILABLE_MAC(10_7);
@property(readonly) NSString *ssid NS_AVAILABLE_MAC(10_6);
@property(readonly) NSData *ssidData NS_AVAILABLE_MAC(10_7);
@property(readonly) NSString *bssid NS_AVAILABLE_MAC(10_6);
@property(readonly) NSInteger rssiValue NS_AVAILABLE_MAC(10_7);
@property(readonly) NSInteger noiseMeasurement NS_AVAILABLE_MAC(10_7);
@property(readonly) CWSecurity security NS_AVAILABLE_MAC(10_7);
@property(readonly) double transmitRate NS_AVAILABLE_MAC(10_7);
@property(readonly) NSString *countryCode NS_AVAILABLE_MAC(10_6);
@property(readonly) CWInterfaceMode interfaceMode NS_AVAILABLE_MAC(10_7);
@property(readonly) NSUInteger transmitPower NS_AVAILABLE_MAC(10_7);
@property(readonly) NSString *hardwareAddress NS_AVAILABLE_MAC(10_7);
@property(readonly, assign) BOOL deviceAttached NS_AVAILABLE_MAC(10_7);
@property(readonly) BOOL serviceActive NS_AVAILABLE_MAC(10_7);
@property(readonly) NSSet *cachedScanResults NS_AVAILABLE_MAC(10_7);
@property(readonly) CWConfiguration *configuration NS_AVAILABLE_MAC(10_6);
+ (NSSet *)interfaceNames NS_AVAILABLE_MAC(10_7);
+ (CWInterface *)interface NS_AVAILABLE_MAC(10_6);
+ (CWInterface *)interfaceWithName:(NSString *)name NS_AVAILABLE_MAC(10_6);
- (id)initWithInterfaceName:(NSString *)name NS_AVAILABLE_MAC(10_6);
- (BOOL)setPower:(BOOL)power error:(NSError **)error NS_AVAILABLE_MAC(10_6);
- (BOOL)setWLANChannel:(CWChannel *)channel error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (BOOL)setPairwiseMasterKey:(NSData *)key error:(NSError **)error NS_AVAILABLE_MAC(10_6);
- (BOOL)setWEPKey:(NSData *)key flags:(CWCipherKeyFlags)flags index:(NSUInteger)index error:(NSError **)error NS_AVAILABLE_MAC(10_6);
- (NSSet *)scanForNetworksWithSSID:(NSData *)ssid error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (NSSet *)scanForNetworksWithName:(NSString *)networkName error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (BOOL)associateToNetwork:(CWNetwork *)network password:(NSString *)password error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (BOOL)associateToEnterpriseNetwork:(CWNetwork *)network identity:(SecIdentityRef)identity username:(NSString *)username password:(NSString *)password error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (BOOL)startIBSSModeWithSSID:(NSData *)ssidData security:(CWIBSSModeSecurity)security channel:(NSUInteger)channel password:(NSString *)password error:(NSError **)error NS_AVAILABLE_MAC(10_7);
- (void)disassociate NS_AVAILABLE_MAC(10_6);
- (BOOL)commitConfiguration:(CWConfiguration *)configuration authorization:(SFAuthorization *)authorization error:(NSError **)error NS_AVAILABLE_MAC(10_7);
@end

#pragma mark -
@class SFAuthorization, CWConfiguration;
@interface CWInterface (Deprecated)
@property(readwrite, retain) SFAuthorization *authorization NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsWoW NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsWEP NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsAES_CCM NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsIBSS NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsTKIP NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsPMGT NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsHostAP NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsMonitorMode NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsWPA NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsWPA2 NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsWME NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsShortGI40MHz NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsShortGI20MHz NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL supportsTSN NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL power NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) BOOL powerSave NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSString *name NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSArray *supportedChannels NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSArray *supportedPHYModes NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *channel NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *phyMode NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *rssi NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *noise NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *txRate NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *securityMode NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *interfaceState NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *opMode NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSNumber *txPower NS_DEPRECATED_MAC(10_6, 10_7);
@property(readonly) NSData *bssidData NS_DEPRECATED_MAC(10_6, 10_7);
+ (NSArray *)supportedInterfaces NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)isEqualToInterface:(CWInterface *)interface NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)setChannel:(NSUInteger)channel error:(NSError **)error NS_DEPRECATED_MAC(10_6, 10_7);
- (NSArray *)scanForNetworksWithParameters:(NSDictionary *)parameters error:(NSError **)error NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)associateToNetwork:(CWNetwork *)network parameters:(NSDictionary *)parameters error:(NSError **)error NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)enableIBSSWithParameters:(NSDictionary *)parameters error:(NSError **)error NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)commitConfiguration:(CWConfiguration *)config error:(NSError **)error NS_DEPRECATED_MAC(10_6, 10_7);
@end

#endif /* _CORE_WLAN_INTERFACE_H_ */
