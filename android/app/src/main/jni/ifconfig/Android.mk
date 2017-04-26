ifneq ($(TARGET_SIMULATOR),true)

LOCAL_PATH:= $(call my-dir)

include $(CLEAR_VARS)
LOCAL_SRC_FILES:= ifconfig.c
LOCAL_MODULE := ifconfig-tether

#LOCAL_SHARED_LIBRARIES := libcutils

include $(BUILD_EXECUTABLE)

endif  # TARGET_SIMULATOR != true
