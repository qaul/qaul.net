/**
 * Patched CoreWLAN framework definitions for qaul.net. They were removed from the 
 * public headers by apple, but still available.
 */
 

#ifndef _CORE_WLAN_H_
#define _CORE_WLAN_H_

extern double CoreWLANFrameworkVersionNumber;
#define CoreWLANFrameworkVersionNumber2_0 200

#ifdef __OBJC__

#import <CoreWLAN/CoreWLANTypes.h>
#import <CoreWLAN/CoreWLANConstants.h>
#import <CoreWLAN/CoreWLANUtil.h>
//#import <CoreWLAN/CWInterface.h>
#import <CoreWLAN/CWNetwork.h>
#import <CoreWLAN/CWConfiguration.h>
#import <CoreWLAN/CWNetworkProfile.h>
#import <CoreWLAN/CWChannel.h>

// patched
#import "CoreWLANConstants_additions.h"
#import "CWInterface_patched.h"
#import "CWWirelessProfile.h"
#import "CW8021XProfile.h"

#endif
#endif /* _CORE_WLAN_H_ */
