
#include <qaul/qlutils.h>
#include <qaul/qlformat.h>
#include <qaul/qlerror.h>
#include <string.h>


int qlutils_resize_array(void **array, size_t type, size_t curr, size_t max)
{
    CHECK(array, QLSTATUS_INVALID_PARAMETERS)

    if(curr > max) {
        return QLSTATUS_INVALID_PARAMETERS;
    }


    /* Check the difference */
    size_t diff = max - curr;
    size_t new_size = max;

    /* If the array is (almost) completely full */
    if(diff <= 1) {

        if(max < 32) {
            new_size = max * 2;
        } else if(max >= 32) {
            new_size = max + 8;
        } else if(max >= 64) {
            new_size = max + (max / 4);
        } else {
            return QLSTATUS_ERROR;
        }

        void *new_array = calloc(new_size, type);
        if(new_array == NULL) return QLSTATUS_MALLOC_FAILED;

        memcpy(new_array, array, curr * type);
        free(*array);
        *array = new_array;
        return QLSTATUS_SUCCESS;
    }

    /* Shrink the array slightly */
    else if(diff <= 16) {
        new_size = max - 8;
    } else if(diff > 16) {
        new_size = max - (diff / 2);
    } else if(diff > max * 2) {
        new_size = max - (max / 2);
    }

    /* Check if resize necessary */
    if(new_size != max) {

        void *new_array = calloc(new_size, type);
        if(new_array == NULL) return QLSTATUS_MALLOC_FAILED;
        memcpy(new_array, array, curr * type);
        free(*array);
        *array = new_array;
    }

    return QLSTATUS_SUCCESS;
}