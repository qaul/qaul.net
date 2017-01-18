/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include <mongoose/mongoose.h>
#include "mg_backend_apk.h"


static int android_read(void* cookie, char* buf, int size) {
    return AAsset_read((AAsset*)cookie, buf, size);
}

static int android_write(void* cookie, const char* buf, int size) {
    return EACCES; // can't provide write access to the apk
}

static fpos_t android_seek(void* cookie, fpos_t offset, int whence) {
    return AAsset_seek((AAsset*)cookie, offset, whence);
}

static int android_close(void* cookie) {
    AAsset_close((AAsset*)cookie);
    return 0;
}

// must be established by someone else...
AAssetManager* android_asset_manager;
time_t timestamp;

void android_set_asset_manager(AAssetManager* manager) {
    android_asset_manager = manager;
    struct timeval te;
    gettimeofday(&te, NULL); // get current time
    timestamp = te.tv_sec;
}

FILE* android_fopen(const char* fname, const char* mode) {
    if(mode[0] == 'w') return NULL;

    AAsset* asset = AAssetManager_open(android_asset_manager, fname, 0);
    if(!asset) return NULL;

    return funopen(asset, android_read, android_write, android_seek, android_close);
}

/*
 * Perform a 64-bit `stat()` call against given file.
 *
 * `path` should be UTF8 encoded.
 *
 * Return value is the same as for `stat()` syscall.
 */
int mg_stat(const char *path, cs_stat_t *st) {
    AAsset* asset = AAssetManager_open(android_asset_manager, path, 0);
    if(asset) {
        // set size
        st->st_size = AAsset_getLength(asset);
        // set file flag
        st->st_mode = S_IFREG | S_IRUSR;
        AAsset_close(asset);
    } else {
        AAssetDir *dir = AAssetManager_openDir(android_asset_manager, path);
        if (!dir) { // not found
            errno = ENOENT;
            return -1;
        }
        AAssetDir_close(dir);
        st->st_mode = S_IFDIR | S_IRUSR | S_IXUSR;
    }
    st->st_mtime = timestamp;
    st->st_atime = timestamp;
    st->st_ctime = timestamp;
    return 0;
}

/*
 * Open the given file and return a file stream.
 *
 * `path` and `mode` should be UTF8 encoded.
 *
 * Return value is the same as for the `fopen()` call.
 */
FILE *mg_fopen(const char *path, const char *mode) {
    return android_fopen(path, mode);
}

/*
 * Open the given file and return a file stream.
 *
 * `path` should be UTF8 encoded.
 *
 * Return value is the same as for the `open()` syscall.
 */
int mg_open(const char *path, int flag, int mode) { /* LCOV_EXCL_LINE */
    return open(path, flag, mode); /* LCOV_EXCL_LINE */
}
