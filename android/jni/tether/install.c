/*
 * Copyright (C) 2009 The Android Open Source Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#include <ctype.h>
#include <errno.h>
#include <stdarg.h>
#include <stdio.h>
#include <dirent.h>
#include <fcntl.h>
#include <signal.h>
#include <limits.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <sys/mount.h>
#include <sys/stat.h>
#include <sys/syscall.h>
#include <sys/types.h>
#include <sys/wait.h>
#include <unistd.h>

#include "cutils/misc.h"
#include "cutils/properties.h"
#include <sys/system_properties.h>
#include "edify/expr.h"
#include "tether.h"

//#include <hardware_legacy/wifi.h>
#include "hardware_legacy/hardware_legacy_stub.h"

#define init_module(mod, len, opts) syscall(__NR_init_module, mod, len, opts)
#define delete_module(mod, flags) syscall(__NR_delete_module, mod, flags)
const int READ_BUF_SIZE = 50;

static void *read_file(const char *filename, ssize_t *_size) {
  int ret, fd;
  struct stat sb;
  ssize_t size;
  void *buffer = NULL;

  /* open the file */
  fd = open(filename, O_RDONLY);
  if (fd < 0)
    return NULL;

  /* find out how big it is */
  if (fstat(fd, &sb) < 0)
    goto bail;
  size = sb.st_size;

  /* allocate memory for it to be read into */
  buffer = malloc(size);
  if (!buffer)
    goto bail;

  /* slurp it into our buffer */
  ret = read(fd, buffer, size);
  if (ret != size)
    goto bail;

  /* let the caller know how big it is */
  *_size = size;

bail:
  close(fd);
  return buffer;
}


char* InsModuleFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 2)
        return ErrorAbort(state, "%s() expects 2 args, got %d", name, argc);
    char *module_name;
    char *options;
    if (ReadArgs(state, argv, 2, &module_name, &options) < 0) {
      return NULL;
    }

    ssize_t len;
    void *image;
    int rc;

    if (!options)
      options = "";

    len = INT_MAX - 4095;
    errno = ENOMEM;
    image = read_file(module_name, &len);

    if (!image)
      return  strdup("");

    errno = 0;
    init_module(image, len, options);
    rc = errno;
    free(image);
    free(module_name);
    return (rc == 0 ? strdup("t") : strdup(""));
}


char* RmModuleFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *module_name;
    int retval;
    if (ReadArgs(state, argv, 1, &module_name) < 0)
        return NULL;

    retval = delete_module(module_name, O_NONBLOCK | O_EXCL);
    free(module_name);
    return (retval  == 0? strdup("t") : strdup(""));
}

char* ModuleLoadedFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *module_name;
    int retval;
    if (ReadArgs(state, argv, 1, &module_name) < 0)
        return NULL;

    int module_found = -1;
    FILE *modules;
    char buffer[READ_BUF_SIZE];
    char mname[READ_BUF_SIZE];

    if (! (modules = fopen("/proc/modules", "r")) ) {
      fprintf(stderr, "Can't open /proc/modules for read \n");
      return strdup("");
    }

    while(fgets(buffer, sizeof(buffer), modules)) {
      /* process the line */
        sscanf(buffer, "%s %*s", mname);
        if ((strstr(mname, module_name)) != NULL) {
          module_found = 0;
        }
    }
    fclose(modules);
    free(module_name);
    return (module_found == 0 ? strdup("t") : strdup(""));
}

int kill_processes_by_pidfile(int parameter, const char* pidfile) {
        FILE *pid = NULL;
        char buffer[READ_BUF_SIZE];

        if (! (pid = fopen(pidfile, "r")) ) {
                return -1;
        }
        if (fgets(buffer, READ_BUF_SIZE-1, pid) == NULL) {
                fclose(pid);
                return -1;
        }
        fclose(pid);


        // Trying to kill
        int signal = kill(strtol(buffer, NULL, 0), parameter);
        if (signal != 0) {
                fprintf(stderr, "Unable to kill process (%s)\n", buffer);
                return -1;
        }
        return 0;
}

int kill_processes_by_name(int parameter, const char* processName) {
        int returncode = 0;

        DIR *dir = NULL;
        struct dirent *next;

        // open /proc
        dir = opendir("/proc");
        if (!dir)
                fprintf(stderr, "Can't open /proc \n");

        while ((next = readdir(dir)) != NULL) {
                FILE *status = NULL;
                char filename[READ_BUF_SIZE];
                char buffer[READ_BUF_SIZE];
                char name[READ_BUF_SIZE];

                /* Must skip ".." since that is outside /proc */
                if (strcmp(next->d_name, "..") == 0)
                        continue;

                sprintf(filename, "/proc/%s/status", next->d_name);
                if (! (status = fopen(filename, "r")) ) {
                        continue;
                }
                if (fgets(buffer, READ_BUF_SIZE-1, status) == NULL) {
                        fclose(status);
                        continue;
                }
                fclose(status);

                /* Buffer should contain a string like "Name:   binary_name" */
                sscanf(buffer, "%*s %s", name);

                if ((strstr(name, processName)) != NULL) {
                        // Trying to kill
                        int signal = kill(strtol(next->d_name, NULL, 0), parameter);
                        if (signal != 0) {
                                fprintf(stderr, "Unable to kill process %s (%s)\n",name, next->d_name);
                                returncode = -1;
                        }
                }
        }
        closedir(dir);
        return returncode;
}


char* KillProcessFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *process_name;
    int retval;
    if (ReadArgs(state, argv, 1, &process_name) < 0)
        return NULL;

    kill_processes_by_name(2, process_name);
    kill_processes_by_name(9, process_name);
    return strdup("t");
}

char* KillProcessByPIDFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *pidfile;
    int retval;
    if (ReadArgs(state, argv, 1, &pidfile) < 0)
        return NULL;

    kill_processes_by_pidfile(2, pidfile);
    kill_processes_by_pidfile(9, pidfile);
    return strdup("t");
}

int file_exists(char *filename) {
    FILE *file = NULL;
    if (! (file = fopen(filename, "r")) )
      return -1;
    fclose(file);
    return 0;
}

char* FileExistsFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *filename;
    int retval;
    if (ReadArgs(state, argv, 1, &filename) < 0)
        return NULL;

    if (file_exists(filename) == -1) {
      free(filename);
      return strdup("");
    }
    free(filename);
    return strdup("t");
}


char* UnlinkFileFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *filename;
    int retval;
    if (ReadArgs(state, argv, 1, &filename) < 0)
        return NULL;

    if (unlink(filename) != 0) {
      free(filename);
      return strdup("");
    } else {
      free(filename);
      return strdup("t");
    }
}

char* WriteFileFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 2)
        return ErrorAbort(state, "%s() expects 2 args, got %d", name, argc);
    char *filename;
    char *line;
    int retval;
    if (ReadArgs(state, argv, 2, &filename, &line) < 0)
        return NULL;

    FILE *fd;
    if (! (fd = fopen(filename, "w")) ) {
      fprintf(stderr, "Can't open %s for write \n", filename);
      free(filename);
      free(line);
      return strdup("");
    }
    if (fwrite(line, strlen(line), 1, fd) == 1) {
      fclose(fd);
      free(filename);
      free(line);
      return strdup("t");
    } else {
      fclose(fd);
      free(filename);
      free(line);
      return strdup("");
    }
}

char* WhiteListMacsFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1)
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    char *filename;
    int retval;
    int returncode = 0;
    if (ReadArgs(state, argv, 1, &filename) < 0)
        return NULL;
    if (file_exists(filename) == 0) {
    FILE *macs;
    char buffer[20];
    char command[100];
    if (! (macs = fopen(filename, "r")) ) {
      fprintf(stderr, "Can't open %s for read \n", filename);
      free(filename);
      return strdup("");
    }
    while(fgets(buffer, sizeof(buffer), macs) && returncode == 0) {
        /* process the line */
      sscanf(buffer, "%s", buffer);
      sprintf(command,"/data/data/net.qaul.qaul/bin/iptables -t nat -I PREROUTING -m mac --mac-source %s -j ACCEPT", buffer);
      //fprintf(stdout, "Enabling whitelist for: %s \n", command);
      returncode = system(command);
    }
    fclose(macs);
  }
  free(filename);
  return (returncode == 0 ? strdup("t") : strdup(""));
}

char* ShowProgressFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 2) {
        return ErrorAbort(state, "%s() expects 2 args, got %d", name, argc);
    }
    char* frac_str;
    char* sec_str;
    if (ReadArgs(state, argv, 2, &frac_str, &sec_str) < 0) {
        return NULL;
    }

    double frac = strtod(frac_str, NULL);
    int sec = strtol(sec_str, NULL, 10);

    UpdaterInfo* ui = (UpdaterInfo*)(state->cookie);
    fprintf(ui->cmd_pipe, "progress %f %d\n", frac, sec);

    free(sec_str);
    return frac_str;
}

char* SetProgressFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1) {
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    }
    char* frac_str;
    if (ReadArgs(state, argv, 1, &frac_str) < 0) {
        return NULL;
    }

    double frac = strtod(frac_str, NULL);

    UpdaterInfo* ui = (UpdaterInfo*)(state->cookie);
    fprintf(ui->cmd_pipe, "set_progress %f\n", frac);

    return frac_str;
}

char* GetPropFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1) {
        return ErrorAbort(state, "%s() expects 1 arg, got %d", name, argc);
    }
    char* key;
    key = Evaluate(state, argv[0]);
    if (key == NULL) return NULL;

    char value[PROPERTY_VALUE_MAX];
    property_get(key, value, "");
    free(key);

    return strdup(value);
}

char* SetPropFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 2) {
        return ErrorAbort(state, "%s() expects 2 arg, got %d", name, argc);
    }
    char* key;
    char* value;
    key = Evaluate(state, argv[0]);
    value = Evaluate(state, argv[1]);
    if (key == NULL || value == NULL) return NULL;
    property_set(key, value);
    free(key);
	free(value);
    return strdup("");
}

// file_getprop(file, key)
//
//   interprets 'file' as a getprop-style file (key=value pairs, one
//   per line, # comment lines and blank lines okay), and returns the value
//   for 'key' (or "" if it isn't defined).
char* GetCfgFn(const char* name, State* state, int argc, Expr* argv[]) {
    char* result = NULL;
    char* buffer = NULL;
    char* key;
    char *filename = "/data/data/net.qaul.qaul/conf/tether.conf";
    if (ReadArgs(state, argv, 1, &key) < 0) {
        return NULL;
    }

    struct stat st;
    if (stat(filename, &st) < 0) {
        ErrorAbort(state, "%s: failed to stat \"%s\": %s",
                   name, filename, strerror(errno));
        goto done;
    }

#define MAX_FILE_GETPROP_SIZE    65536

    if (st.st_size > MAX_FILE_GETPROP_SIZE) {
        ErrorAbort(state, "%s too large for %s (max %d)",
                   filename, name, MAX_FILE_GETPROP_SIZE);
        goto done;
    }

    buffer = malloc(st.st_size+1);
    if (buffer == NULL) {
        ErrorAbort(state, "%s: failed to alloc %d bytes", name, st.st_size+1);
        goto done;
    }

    FILE* f = fopen(filename, "rb");
    if (f == NULL) {
        ErrorAbort(state, "%s: failed to open %s: %s",
                   name, filename, strerror(errno));
        goto done;
    }

    if (fread(buffer, 1, st.st_size, f) != st.st_size) {
        ErrorAbort(state, "%s: failed to read %d bytes from %s",
                   name, st.st_size+1, filename);
        fclose(f);
        goto done;
    }
    buffer[st.st_size] = '\0';

    fclose(f);

    char* line = strtok(buffer, "\n");
    do {
        // skip whitespace at start of line
        while (*line && isspace(*line)) ++line;

        // comment or blank line: skip to next line
        if (*line == '\0' || *line == '#') continue;

        char* equal = strchr(line, '=');
        if (equal == NULL) {
            ErrorAbort(state, "%s: malformed line \"%s\": %s not a prop file?",
                       name, line, filename);
            goto done;
        }

        // trim whitespace between key and '='
        char* key_end = equal-1;
        while (key_end > line && isspace(*key_end)) --key_end;
        key_end[1] = '\0';

        // not the key we're looking for
        if (strcmp(key, line) != 0) continue;

        // skip whitespace after the '=' to the start of the value
        char* val_start = equal+1;
        while(*val_start && isspace(*val_start)) ++val_start;

        // trim trailing whitespace
        char* val_end = val_start + strlen(val_start)-1;
        while (val_end > val_start && isspace(*val_end)) --val_end;
        val_end[1] = '\0';

        result = strdup(val_start);
        break;

    } while ((line = strtok(NULL, "\n")));

    if (result == NULL) result = strdup("");

  done:
    //free(filename);
    free(key);
    free(buffer);
    return result;
}


char* UIPrintFn(const char* name, State* state, int argc, Expr* argv[]) {
    char** args = ReadVarArgs(state, argc, argv);
    if (args == NULL) {
        return NULL;
    }

    int size = 0;
    int i;
    for (i = 0; i < argc; ++i) {
        size += strlen(args[i]);
    }
    char* buffer = malloc(size+1);
    size = 0;
    for (i = 0; i < argc; ++i) {
        strcpy(buffer+size, args[i]);
        size += strlen(args[i]);
        free(args[i]);
    }
    free(args);
    buffer[size] = '\0';

    char* line = strtok(buffer, "\n");
    while (line) {
        fprintf(((UpdaterInfo*)(state->cookie))->cmd_pipe,
                "ui_print %s\n", line);
        line = strtok(NULL, "\n");
    }
    fprintf(((UpdaterInfo*)(state->cookie))->cmd_pipe, "ui_print\n");

    return buffer;
}

char* GetActionFn(const char* name, State* state, int argc, Expr* argv[]) {
  return strdup(((UpdaterInfo*)(state->cookie))->action);
}

// log("some message");
// log("t", "some message");
char* LogFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1 && argc != 2) {
        return ErrorAbort(state, "%s() expects 1 or 2 args", name);
    }
    char *status;
    char *message;
    time_t time_now;
    time(&time_now);

    if (argc == 1) {
      if (ReadArgs(state, argv, 1, &message) < 0)
        return NULL;
      status = strdup("t");
    } else {
      if (ReadArgs(state, argv, 2, &status, &message) < 0)
        return NULL;
    }
    if (strcmp(status,"t") == 0) {
      fprintf(((UpdaterInfo*)(state->cookie))->log_fd,
        "<div class=\"date\">%s</div><div class=\"action\">%s...</div><div class=\"output\"></div><div class=\"done\">done</div><hr>",asctime(localtime(&time_now)),message);
    }
    else {
      property_set("tether.status","failed");
      fprintf(((UpdaterInfo*)(state->cookie))->log_fd,
        "<div class=\"date\">%s</div><div class=\"action\">%s...</div><div class=\"output\"></div><div class=\"failed\">failed</div><hr>",asctime(localtime(&time_now)),message);
    }
    return strdup("");
}

char* RunProgramFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 1) {
        return ErrorAbort(state, "%s() expects 1 arg", name);
    }
    char *cmd;
    if (ReadArgs(state, argv, 1, &cmd) < 0)
        return NULL;

    fprintf(stderr, "about to run: [%s]\n", cmd);

    int status = system(cmd);
    if (-1 != status)
		status = WEXITSTATUS(status);

    free(cmd);

    if (status != 0) {
        // fprintf(stderr, "run_program failed: %s\n", strerror(status));
        return strdup("");
    }
	return strdup("t");
}

char* LoadWifiFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 0)
        return ErrorAbort(state, "%s() expects 0 arg, got %d", name, argc);

    if (wifi_load_driver() != 0) {
      return ErrorAbort(state, "Unable to load wifi-driver: %s", strerror(errno));
    }
    return strdup("t");
}

char* UnloadWifiFn(const char* name, State* state, int argc, Expr* argv[]) {
    if (argc != 0)
        return ErrorAbort(state, "%s() expects 0 arg, got %d", name, argc);

    if (wifi_unload_driver() != 0) {
      return ErrorAbort(state, "Unable to unload wifi-driver: %s", strerror(errno));
    }
    return strdup("t");
}

void RegisterInstallFunctions() {
    RegisterFunction("insmod", InsModuleFn);
    RegisterFunction("rmmod", RmModuleFn);
    RegisterFunction("module_loaded", ModuleLoadedFn);
    RegisterFunction("kill_process", KillProcessFn);
    RegisterFunction("kill_pidfile", KillProcessByPIDFn);
    RegisterFunction("file_exists", FileExistsFn);
    RegisterFunction("file_write", WriteFileFn);
    RegisterFunction("file_unlink", UnlinkFileFn);
    RegisterFunction("load_wifi", LoadWifiFn);
    RegisterFunction("unload_wifi", UnloadWifiFn);
    RegisterFunction("log", LogFn);
    RegisterFunction("whitelist_macs", WhiteListMacsFn);
    RegisterFunction("show_progress", ShowProgressFn);
    RegisterFunction("set_progress", SetProgressFn);
    RegisterFunction("getprop", GetPropFn);
    RegisterFunction("setprop", SetPropFn);
    RegisterFunction("getcfg", GetCfgFn);
    RegisterFunction("action", GetActionFn);
    RegisterFunction("ui_print", UIPrintFn);
    RegisterFunction("run_program", RunProgramFn);
}
