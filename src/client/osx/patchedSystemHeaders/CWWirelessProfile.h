/**
 * Patched CoreWLAN framework definitions for qaul.net. They were removed from the 
 * public headers by apple, but still available.
 * This file represents the SDK options available in OSX 10.8.
 */

#ifndef _CORE_WLAN_WIRELESS_PROFILE_H_
#define _CORE_WLAN_WIRELESS_PROFILE_H_

#import <Foundation/Foundation.h>

@class CW8021XProfile, CWMutableNetworkProfile;

@interface CWWirelessProfile : NSObject <NSCopying, NSCoding> {
@private
    CWMutableNetworkProfile *_networkProfile;
}

@property(readwrite, copy) NSString *ssid NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, retain) NSNumber *securityMode NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, copy) NSString *passphrase NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, retain) CW8021XProfile *user8021XProfile NS_DEPRECATED_MAC(10_6, 10_7);
- (CWWirelessProfile*)init NS_DEPRECATED_MAC(10_6, 10_7);
+ (CWWirelessProfile*)profile NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)isEqualToProfile:(CWWirelessProfile*)profile NS_DEPRECATED_MAC(10_6, 10_7);
@end

#endif /* _CORE_WLAN_WIRELESS_PROFILE_H_ */
