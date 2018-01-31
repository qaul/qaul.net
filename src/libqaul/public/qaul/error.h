
#ifndef QAUL_ERROR_H
#define QAUL_ERROR_H

typedef enum ql_error_t {

    SUCCESS = 0,                    // Everything went okay
    ERROR = 1,                      // An unknown error occured
    FATAL = 2,                      // A fatal error occured
    MEMORY_ALLOCATION_FAILED,       // Not enough memory

    NOT_INITIALISED,                // The module wasn't initialised yet
    ALREADY_INITIALISED,            // The module has already been initialised

    INVALID_PARAMETERS,             // The provided parameters were invalid

    USER_ALREADY_EXISTS,
    USER_DOES_NOT_EXIST,
    INVALID_PASSPHRASE,

    ALREADY_AUTHENTICATED,
    INVALID_AUTHENTICATION,
    NOT_AUTHENTICATED,

    FILE_ALREADY_EXISTS,
    FILE_DOES_NOT_EXIST,

} ql_error_t;

#endif //QAUL_ERROR_H
