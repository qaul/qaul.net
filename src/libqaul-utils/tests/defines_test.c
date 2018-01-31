/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

/*
 * test basic defines because of 'What can possibly go wrong?'
 */

#include <qaul/utils/defines.h>
#include <qaul/utils/tests.h>

// ------------------------------------------------------------
int main(int argc, char *argv[])
{

    int fail = 0;

    /*
     * A process should return 0 at success, any non-zero value indicates an error.
     * Hail POSIX!!1!
     */
    if(QL_SUCCESS == QL_ERROR) {
	printf("QL_SUCCESS and QL_ERROR are defined to the same value.\n" \
	       "Sounds like a bed time paradox, doesn`t it?\n");
	fail = 1;
    }

    if(QL_SUCCESS != 0) {
	printf("QL_SUCCESS should be zero.\n");
	fail = 1;
    }

    if(QL_ERROR == 0) {
	printf("QL_ERROR should be a non-zero value.\n");
	fail = 1;
    }

    if(QL_TRUE == QL_FALSE) {
	printf("QL_TRUE and QL_FALSE are defined to the same value.\n" \
	       "Sounds like a bed time paradox, doesn`t it?\n");
	fail = 1;
    }

    if(QL_FALSE) {
	printf("QL_FALSE is wrong. 'if(QL_FALSE)' should not be executed.\n");
	fail = 1;
    }

    if(!QL_TRUE) {
	printf("QL_TRUE is wrong. 'if(!QL_TRUE)' should not be executed.\n");
	fail = 1;
    }

    if(fail == 1) {
	printf("Solve the problems above. Either some misconfiguration did happen or\n" \
	       "you are porting to a strange platform... Basic logic is not guaranteed\n" \
	       "to work properly so it make no sense to continiue anything past this point.");
	return 1;
    }

    /*
     * From here the defines and test (helper) macros should work
     * properly, like 'FAIL("error message");'
     */
    return QL_SUCCESS;
}
