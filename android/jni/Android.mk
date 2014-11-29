LOCAL_PATH := $(call my-dir)

PJSIP_PATH ?= $(LOCAL_PATH)
EXTRALIB_PATH ?= $(SYSROOT)/usr/lib/

JNI_PATH := $(LOCAL_PATH)

LIBQAUL_SRC ?= $(../../libqaul)
LIBQAUL_INCLUDE ?= ""

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
	$(LIBQAUL_SRC)/qaullib_crypto.c \
	$(LIBQAUL_SRC)/qaullib_udp_communication.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive_dhcp.c \
	$(LIBQAUL_SRC)/captive/qaullib_captive_dns.c \
	$(LIBQAUL_SRC)/mongoose/mongoose.c \
	$(LIBQAUL_SRC)/sqlite/sqlite3.c \
	$(LIBQAUL_SRC)/urlcode/urlcode.c \
	$(LIBQAUL_SRC)/polarssl/sha1.c \
	$(LIBQAUL_SRC)/olsrd/mantissa.c \
	$(LIBQAUL_SRC)/olsrd/hashing.c \
	$(LIBQAUL_SRC)/qaullib_appevent_LL.c \
	$(LIBQAUL_SRC)/qaullib_messaging.c \
	$(LIBQAUL_SRC)/qaullib_topo_LL.c \
	$(LIBQAUL_SRC)/qaullib_msg_LL.c \
	$(LIBQAUL_SRC)/qaullib_validate.c
#LOCAL_EXPORT_C_INCLUDES := $(LOCAL_PATH)/../../libqaul
LOCAL_EXPORT_C_INCLUDES := $(LIBQAUL_INCLUDE)
LOCAL_CFLAGS := \
	-I$(PJSIP_PATH)/pjsip/include \
	-I$(PJSIP_PATH)/pjlib/include \
	-I$(PJSIP_PATH)/pjlib-util/include \
	-I$(PJSIP_PATH)/pjmedia/include \
	-I$(PJSIP_PATH)/pjnath/include \
	-I$(LIBQAUL_INCLUDE)
LOCAL_STATIC_LIBRARIES := \
    pjsua \
    pjsip-ua \
    pjsip-simple \
    pjsip \
    pjsdp \
    pjmedia-audiodev \
    pjmedia-codec \
    pjmedia \
    pjmedia-videdev \
    pjnath \
    pjlib-util \
    resample \
    milenage \
    srtp \
    gsmcodec \
    speex \
    ilbccodec \
    g7221codec \
    pj

# add logging for debugging
LOCAL_LDLIBS := \
    -L$(SYSROOT)/usr/lib -llog \
    -lOpenSLES

include $(BUILD_SHARED_LIBRARY)

# include externally built libraries
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip-simple
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-simple-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsip-ua
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsip-ua-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsua
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjsip/lib/libpjsua-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjsip/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-videodev
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-videodev-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-codec 
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-codec-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjmedia-audiodev
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjmedia-audiodev-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjlib-util
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjlib-util/lib/libpjlib-util-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjlib-util/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := speex
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libspeex-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := ilbccodec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libilbccodec-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := resample
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libresample-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := milenage
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libmilenage-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := gsmcodec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libgsmcodec-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := g7221codec
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libg7221codec-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := srtp
LOCAL_SRC_FILES         := $(PJSIP_PATH)/third_party/lib/libsrtp-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/third_party/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjnath
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjnath/lib/libpjnath-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjnath/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pj
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjlib/lib/libpj-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjlib/include
include $(PREBUILT_STATIC_LIBRARY)
include $(CLEAR_VARS)
LOCAL_MODULE            := pjsdp
LOCAL_SRC_FILES         := $(PJSIP_PATH)/pjmedia/lib/libpjsdp-arm-unknown-linux-androideabi.a
LOCAL_EXPORT_C_INCLUDES := ../../pjproject_android/pjmedia/include
include $(PREBUILT_STATIC_LIBRARY)

# libnativetask.so
include $(CLEAR_VARS)
LOCAL_MODULE    := libnativetask
LOCAL_SRC_FILES := android_tether_system_NativeTask.c 
include $(BUILD_SHARED_LIBRARY)
