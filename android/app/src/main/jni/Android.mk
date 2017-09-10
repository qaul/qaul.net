LOCAL_PATH := $(call my-dir)

PJSIP_PATH ?= $(LOCAL_PATH)/../../../../../third_party/pjsip/src/pjsip
EXTRALIB_PATH ?= $(LOCAL_PATH)/../../../../../android_extra_lib

JNI_PATH := $(LOCAL_PATH)

LIBQAUL_SRC ?= $(LOCAL_PATH)/../../../../../src/libqaul
LIBQAUL_INCLUDE ?= ""

LIBQAULUTILS_SRC ?= $(LOCAL_PATH)/../../../../../src/libqaulutils/src
LIBQAULUTILS_INCLUDE ?= $(LOCAL_PATH)/../../../../../src/libqaulutils/include

QAULCONFIG_INCLUDE ?= $(LOCAL_PATH)/../../../../../include

MBEDTLS_PATH ?= $(LOCAL_PATH)/../../../../../external/mbedtls
MONGOOSE_PATH ?= $(LOCAL_PATH)/../../../../../third_party/mongoose

# wificonfig
# test from MeshApp, wlan_slovenia
# -> libwpa_client.so & libcutils.so pulled from phone and put into NDK
include $(CLEAR_VARS)
LOCAL_SRC_FILES := wificonfig/log.c \
	wificonfig/wifi_config.c
LOCAL_STATIC_LIBRARIES := libedify
LOCAL_SHARED_LIBRARIES := libhardware_legacy
LOCAL_LDLIBS += -L$(EXTRALIB_PATH) -lwpa_client -lcutils
LOCAL_MODULE := wificonfig
include $(BUILD_EXECUTABLE)

# libedify.a
include $(CLEAR_VARS)
LOCAL_SRC_FILES := edify/lex.yy.c \
	edify/parser.c \
	edify/expr.c
LOCAL_MODULE := libedify
include $(BUILD_STATIC_LIBRARY)

# hardware_legacy.so
include $(CLEAR_VARS)
LOCAL_SRC_FILES := hardware_legacy/hardware_legacy_stub.c
LOCAL_MODULE    := libhardware_legacy
include $(BUILD_SHARED_LIBRARY)

# tether
include $(CLEAR_VARS)
LOCAL_SRC_FILES := tether/install.c \
	tether/tether.c
LOCAL_STATIC_LIBRARIES := libedify
LOCAL_SHARED_LIBRARIES := libhardware_legacy
LOCAL_LDLIBS += -L$(EXTRALIB_PATH) -lcutils
LOCAL_MODULE := tether
include $(BUILD_EXECUTABLE)

# libqaul.so
include $(CLEAR_VARS)
LOCAL_MODULE := libqaul
LOCAL_SRC_FILES := net_qaul_qaul_NativeQaul.c \
	$(LIBQAULUTILS_SRC)/logging.c \
	$(LIBQAULUTILS_SRC)/validate.c \
	$(LIBQAUL_SRC)/qaullib.c \
	$(LIBQAUL_SRC)/qaullib_ipc.c \
	$(LIBQAUL_SRC)/qaullib_webserver.c \
	$(LIBQAUL_SRC)/qaullib_voip.c \
	$(LIBQAUL_SRC)/qaullib_webclient.c \
	$(LIBQAUL_SRC)/qaullib_threads.c \
	$(LIBQAUL_SRC)/qaullib_user.c \
	$(LIBQAUL_SRC)/qaullib_user_LL.c \
	$(LIBQAUL_SRC)/qaullib_filesharing.c \
	$(LIBQAUL_SRC)/qaullib_file_LL.c \
	$(LIBQAUL_SRC)/qaullib_exediscovery.c \
	$(LIBQAUL_SRC)/qaullib_udp_communication.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive_dhcp.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive_dns.c \
	$(LIBQAUL_SRC)/sqlite/sqlite3.c \
	$(LIBQAUL_SRC)/urlcode/urlcode.c \
	$(LIBQAUL_SRC)/olsrd/mantissa.c \
	$(LIBQAUL_SRC)/olsrd/hashing.c \
	$(LIBQAUL_SRC)/qaullib_appevent_LL.c \
	$(LIBQAUL_SRC)/qaullib_messaging.c \
	$(LIBQAUL_SRC)/qaullib_topo_LL.c \
	$(LIBQAUL_SRC)/qaullib_msg_LL.c \
	$(LIBQAUL_SRC)/mg_backend_apk.c \
	$(LIBQAUL_SRC)/qmongoose.c \
	$(LIBQAUL_SRC)/qaullib_crypto.c \
	$(LIBQAUL_SRC)/crypto/qcry_context.c \
	$(LIBQAUL_SRC)/crypto/qcry_arbiter.c \
	$(LIBQAUL_SRC)/crypto/qcry_hashing.c \
	$(LIBQAUL_SRC)/crypto/qcry_helper.c \
	$(LIBQAUL_SRC)/crypto/qcry_keys.c \
	$(LIBQAUL_SRC)/crypto/qcry_keystore.c

LOCAL_EXPORT_C_INCLUDES := $(LIBQAUL_INCLUDE)
LOCAL_C_INCLUDES := \
	$(LIBQAUL_SRC)/include \
	$(MONGOOSE_PATH) \
	$(QAULCONFIG_INCLUDE) \
	$(LIBQAULUTILS_INCLUDE)
LOCAL_CFLAGS := \
	-I$(LIBQAUL_SRC) \
	-I$(LIBQAUL_SRC)/include \
	-I$(QAULCONFIG_INCLUDE)\
	-I$(PJSIP_PATH)/pjsip/include \
	-I$(PJSIP_PATH)/pjlib/include \
	-I$(PJSIP_PATH)/pjlib-util/include \
	-I$(PJSIP_PATH)/pjmedia/include \
	-I$(PJSIP_PATH)/pjnath/include
LOCAL_STATIC_LIBRARIES := \
    mbedtls \
    pjsua \
    pjsip-ua \
    pjsip-simple \
    pjsip \
    pjsdp \
    pjmedia-audiodev \
    pjmedia-codec \
    pjmedia \
    pjmedia-videodev \
    pjnath \
    pjlib-util \
    resample \
    milenage \
    srtp \
    gsmcodec \
    speex \
    ilbccodec \
    g7221codec \
    webrtc \
    pj

# add logging for debugging
LOCAL_LDLIBS := \
    -L$(SYSROOT)/usr/lib -llog \
    -lOpenSLES \
    -landroid

include $(BUILD_SHARED_LIBRARY)

# include externally built libraries
include $(CLEAR_VARS)
LOCAL_MODULE            := mbedtls
LOCAL_SRC_FILES         := $(MBEDTLS_PATH)/lib/libmbedcrypto.a
LOCAL_EXPORT_C_INCLUDES := $(MBEDTLS_PATH)/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip-simple
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-simple-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip-ua
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-ua-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsua
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsua-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-videodev
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-videodev-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-codec 
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-codec-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-audiodev
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-audiodev-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjlib-util
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjlib-util/lib/libpjlib-util-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjlib-util/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := speex
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libspeex-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := ilbccodec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libilbccodec-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := resample
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libresample-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := milenage
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libmilenage-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := gsmcodec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libgsmcodec-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := g7221codec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libg7221codec-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := srtp
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libsrtp-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := webrtc
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libwebrtc-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjnath
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjnath/lib/libpjnath-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjnath/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pj
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjlib/lib/libpj-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjlib/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsdp
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjsdp-qaul.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)

# libnativetask.so
include $(CLEAR_VARS)
LOCAL_MODULE    := libnativetask
LOCAL_SRC_FILES := android_tether_system_NativeTask.c 
include $(BUILD_SHARED_LIBRARY)
