Code Style Guide
================

The coding style conventions were changed April-May 2017 

Naming Conventions
------------------

* All functions and variable names are written lower case and concatinated with `_`
* Functions: start with `ql` which stands for QauL or QaulLib.
  * Each module has it's own namespace identifier. For example crypto functions start with `qlcry`
  * Try to use short (but easily readable) namespace handles. For example `qluser` instead of `qlusr`
  * Try to avoid overly long handles such as `qlmessaging` and instead use `qlmesg`
* Variable names should be precise in their function or value. No single letter variables
  * Exceptions are index variables for arrays or loops
  * Local variables have no guidelines beyond these common sense rules
  * Global variables (fields, static state, etc.) should begin with the same namespace handle as the rest of the module
    * Normal variables are all lowercase such as `qlcry_arbiter` or `qlmesg_local_buffer`
    * Constant values should be defined via `#define` and use ALL-CAPS names such as `QLCRY_THREAD_LIMIT`
* Structs and enums should use the same namespace handles as the rest of the module
  * Structs that implement a context for something should end in `_ctx` (such as `qlcry_arbit_ctx`)
  * Enums that make type-destinctions should end in `_t` (such as `qlcry_cipher_t`)

Code Style
----------

Use **4** spaces to indent code.
Code according to the example:

```
/**
 * A function should ALWAYS have a standardised block comment above it that
 * explains what it does, roughly what it's side 
 *
 */
int qluser_something_rather(int param, int otherparam)
{
    int local_var;

    if(local_var == value)
        do_something;

    /** Use block comments for blocks of code */
    if(local_var == value) {
        do_this;
        do_that;

    } else {

        /* Or for single (complicated) lines */
        do_another_thing;
    }

    // Or use line-comments. Just be consistent within the same file
    return 1;
}
```


Clustering
----------

Files that belong to a single module should be kept in a folder. All files in this
folder then share the same namespace prefix (such as `qlcry`) as described above.
Modules are complex and sometimes it's better to move functionality into seperate files
which is why they are folders and can contain multiple files.

The crypto submodule for example consists of

 * `qlcry_arbit`
 * `qlcry_keygen`
 * `qlcry_keystore`
 * `qlcry_context`
 * etc.

Other modules should make use of a similar name-spacing scheme to make sure that code files
don't get too long but can still be associated with the rest of their module correctly.


Comments & Documentation
------------------------

The documentation shall be as much in the code as possible. The comments shall
be done in a [doxygen](https://en.wikipedia.org/wiki/Doxygen) compatible manner.

Every function shall be documented before it's declaration (in the header file).
Every header file shall start with an explanation of what it does.

Example template for a header file:

```
/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/*********************************************************************************
 *
 * Something that explains the core functionality of this file. For example
 * I could be writing here that these functions are used to generate user-id's
 * or something.
 *
 * But I shouldn't mention specific functions here, just general usecases
 *
 *********************************************************************************
 */

/**
 * Example function needing @a param and @a otherparam for example reason
 *
 * @param Description of method's or function's input parameter
 * @param ...
 *
 * @return Description of the return value
 */
int qlstuff_submodule_function(int param, int otherparam);
```


Licensing
---------

All code pushed to qaul.net must be under GPLv3 license.
Every file shall start with the qaul.net GPLv3 license declaration.

example:

```
/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */
```
