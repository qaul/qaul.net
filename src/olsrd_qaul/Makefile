# 
# written by contact@wachter-jud.net
# LICENCE GPL
#

OLSRD_PLUGIN =	true
PLUGIN_NAME =	olsrd_qaul
PLUGIN_VER =	0.1

TOPDIR = ../..
include $(TOPDIR)/Makefile.inc

ifeq ($(OS),android)
# On Android Google forgot to include regex engine code for Froyo version (but also there was
# no support in older versions for it) so we have here this missing code.
# http://groups.google.com/group/android-ndk/browse_thread/thread/5ea6f0650f0e3fc
SRCS +=     $(wildcard $(TOPDIR)/android/regex/reg*.c)
HDRS +=     $(wildcard $(TOPDIR)/android/regex/*.h)
CFLAGS += -D__POSIX_VISIBLE
endif

default_target: $(PLUGIN_FULLNAME)

$(PLUGIN_FULLNAME): $(OBJS) version-script.txt
		$(CC) $(LDFLAGS) -o $(PLUGIN_FULLNAME) $(OBJS) $(LIBS)

install:	$(PLUGIN_FULLNAME)
		$(STRIP) $(PLUGIN_FULLNAME)
		$(INSTALL_LIB)

clean:
		rm -f $(OBJS) $(SRCS:%.c=%.d) $(PLUGIN_FULLNAME)
