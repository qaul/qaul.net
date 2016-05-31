Code Style Guide
================

Naming Conventions
------------------

* Functions: start with an upper case character
  * qaullib prefix:     Ql_ (TODO: rename existing functions accordingly )
* Variables: start with an lower case character
  * qaullib prefix for public:  ql_ (TODO: rename existing variables accordingly)


Code Style
----------

Use tabs to indent code.
Code according to the example:

´´´
int Ql_FunctionName(int param, int otherparam)
{
  int variable_name

  if (variable_name == value)
    do something;

  if (variable_name == value)
  {
    do this;
    do that;
  }
  else
  {
    do another thing;
  }

  return 1;
}
´´´

Editor styles:
* Tab width: 4


Clustering
----------

The functions of a functional module shall be clustered in one file.


Comments & Documentation
------------------------

The documentation shall be as much in the code as possible. The comments shall
be done in a [doxygen](https://en.wikipedia.org/wiki/Doxygen) compatible manner.

Every function shall be documented before it's declaration (in the header file).
Every header file shall start with an explanation of what this module does.
Try to write self explanatory code with meaningful function and variable names.

example template for a header file:

´´´
/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/**
 * example template that explains the formatting and use of comments in qaul.net
 *
 * functions in the public API
 *   void Ql_FunctionInThePublicAPI(void);
 *   void Ql_OtherFunctionInThePublicAPI(int commandId);
 * @see include/qaullib.h
 */

/**
 * Example function needing @a param and @a otherparam for example reason
 *
 * @retval 1 example was successful
 * @retval 0 example failed
 */
int Ql_FunctionName(int param, int otherparam);
´´´


Licensing
---------

All code pushed to qaul.net must be under GPLv3 license.
Every file shall start with the qaul.net GPLv3 license declaration.

example:

´´´
/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */
´´´
