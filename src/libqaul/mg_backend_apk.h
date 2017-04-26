/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * qaullib web server using mongoose web server.
 *
 * The web server is reachable via port 8081. If all the binary installers are present
 * port 80 is forwarded to 8081. The web server delivers the static web pages from
 * the globally installed www directory and the shared files from the files folder
 * in the users .qaul folder in the users home directory.
 *
 * In this file are the functions that create the dynamic pages.
 *
 * link to some static pages:
 * captive portal installer download page: http://localhost:8081/
 * qaul.net GUI: http://localhost:8081/qaul.html
 * qaul.net web client: http://localhost:8081/qaul_web.html
 */

#ifndef _QAULLIB_MG_BACKEND_APK
#define _QAULLIB_MG_BACKEND_APK

#include <android/asset_manager.h>

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void android_set_asset_manager(AAssetManager* manager);

#ifdef __cplusplus
}
#endif // __cplusplus

#endif // _QAULLIB_MG_BACKEND_APK
