/*
 * Copyright (C) 2009 Ben Buxton
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

#include <errno.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

#include "edify/expr.h"
#include "tether.h"
#include "install.h"

// Where in the package we expect to find the edify script to execute.
#define SCRIPT_NAME "/data/data/net.qaul.qaul/conf/tether.edify"

int main(int argc, char** argv) {
	FILE *f;

	// Set up the pipe for sending commands back to the parent process.
    int fd = atoi(argv[2]);
    FILE* cmd_pipe = fdopen(fd, "wb");
    setlinebuf(cmd_pipe);

    struct stat st;
    if (stat(SCRIPT_NAME, &st) < 0) {
      fprintf(stderr, "Could not stat %s: %s", SCRIPT_NAME, strerror(errno));
      return 1;
    }
    if (st.st_size > 128000) {
      fprintf(stderr, "%s too large (max 128k)", SCRIPT_NAME);
      return 1;
    }
    char *script = malloc(st.st_size+1);

    f = fopen(SCRIPT_NAME, "rb");
    if (f == NULL) {
      fprintf(stderr, "Cannot read %s\n", SCRIPT_NAME);
      return 1;
    }
    if (fread(script, 1, st.st_size, f) != st.st_size) {
      fprintf(stderr, "Failed to read %d bytes from %s", st.st_size+1, SCRIPT_NAME);
      return 1;
    }
    script[st.st_size] = '\0';
    fclose(f);

    // Configure edify's functions.

    RegisterBuiltins();
    RegisterInstallFunctions();
    FinishRegistration();

    // Parse the script.

    Expr* root;
    int error_count = 0;
    yy_scan_string(script);
    int error = yyparse(&root, &error_count);
    if (error != 0 || error_count > 0) {
        fprintf(stderr, "%d parse errors\n", error_count);
        return 6;
    }

    // Evaluate the parsed script.

    UpdaterInfo updater_info;
    updater_info.cmd_pipe = cmd_pipe;
    updater_info.log_fd = fopen ("/data/data/net.qaul.qaul/var/tether.log","w");

    updater_info.action = strdup(argv[1]);

    State state;
    state.cookie = &updater_info;
    state.script = script;
    state.errmsg = NULL;

    char* result = Evaluate(&state, root);
    if (result == NULL) {
        if (state.errmsg == NULL) {
            fprintf(stderr, "script aborted (no error message)\n");
            fprintf(cmd_pipe, "ui_print script aborted (no error message)\n");
        } else {
            fprintf(stderr, "script aborted: %s\n", state.errmsg);
            char* line = strtok(state.errmsg, "\n");
            while (line) {
                fprintf(cmd_pipe, "ui_print %s\n", line);
                line = strtok(NULL, "\n");
            }
            fprintf(cmd_pipe, "ui_print\n");
        }
        free(state.errmsg);
        return 7;
    } else {
        fprintf(stderr, "script result was [%s]\n", result);
        free(result);
    }

    free(script);

    return 0;
}
