# Copyright 2009 The Android Open Source Project

LOCAL_PATH := $(call my-dir)

updater_src_files := \
	install.c \
	tether.c 

include $(CLEAR_VARS)

LOCAL_SRC_FILES := $(updater_src_files)

LOCAL_C_INCLUDES += $(dir $(inc))

LOCAL_STATIC_LIBRARIES := libedify

LOCAL_SHARED_LIBRARIES := libcutils \
			  libhardware_legacy

LOCAL_MODULE := tether

include $(BUILD_EXECUTABLE)



