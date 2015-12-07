/**
 * Patched CoreWLAN framework definitions for qaul.net. They were removed from the 
 * public headers by apple, but still available.
 * This file represents the SDK options available in OSX 10.8.
 */

#ifndef _CORE_WLAN_8021X_PROFILE_H_
#define _CORE_WLAN_8021X_PROFILE_H_

#import <Foundation/Foundation.h>

@interface CW8021XProfile : NSObject <NSCopying, NSCoding> {
@private
    NSString *_userDefinedName;
    NSString *_ssid;
    NSString *_username;
    NSString *_password;
    BOOL _alwaysPromptForPassword;
}

@property(readwrite, copy) NSString *userDefinedName NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, copy) NSString *ssid NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, copy) NSString *username NS_DEPRECATED_MAC(10_6, 10_7);
@property(readwrite, copy) NSString *password NS_DEPRECATED_MAC(10_6, 10_7);
@property BOOL alwaysPromptForPassword NS_DEPRECATED_MAC(10_6, 10_7);
- (CW8021XProfile*)init NS_DEPRECATED_MAC(10_6, 10_7);
+ (CW8021XProfile*)profile NS_DEPRECATED_MAC(10_6, 10_7);
- (BOOL)isEqualToProfile:(CW8021XProfile *)profile NS_DEPRECATED_MAC(10_6, 10_7);
+ (NSArray*)allUser8021XProfiles NS_DEPRECATED_MAC(10_6, 10_7);
@end

#endif /* _CORE_WLAN_8021X_PROFILE_H_ */
