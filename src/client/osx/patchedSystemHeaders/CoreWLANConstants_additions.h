/**
 * Patched CoreWLAN framework definitions for qaul.net. They were removed from the 
 * public headers by apple, but still available.
 */

/**
 * Missing definitions in CoreWLAN/CoreWLANConstants.h
 */
FOUNDATION_EXTERN NSString * const kCWAssocKeyPassphrase NS_AVAILABLE_MAC(10_6);
FOUNDATION_EXTERN NSString * const kCWAssocKey8021XProfile NS_AVAILABLE_MAC(10_6);
FOUNDATION_EXTERN NSString * const kCWIBSSKeySSID NS_AVAILABLE_MAC(10_6);
FOUNDATION_EXTERN NSString * const kCWIBSSKeyChannel NS_AVAILABLE_MAC(10_6);
FOUNDATION_EXTERN NSString * const kCWIBSSKeyPassphrase NS_AVAILABLE_MAC(10_6);
