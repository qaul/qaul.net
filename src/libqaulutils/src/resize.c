/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaul/utils/resize.h"
#include "qaul/utils/defines.h"
#include <stdio.h>
#include <string.h>

#define CHECK(field, ret) { if((field) == NULL) return ret; }



int qlutils_resize_array(void **array, size_t type, size_t curr, size_t *max)
{
    CHECK(array, QL_ERROR)

    if(curr > *max) {
        return QL_ERROR;
    }


    /* Check the difference */
    size_t diff = *max - curr;
    size_t new_size = *max;

    /* If the array is (almost) completely full */
    if(diff <= 1) {

        if(*max < 32) {
            new_size = *max * 2;
        } else if(*max >= 32) {
            new_size = *max + 8;
        } else if(*max >= 64) {
            new_size = *max + (*max / 4);
        } else {
            return QL_ERROR;
        }

        void *new_array = calloc(new_size, type);
        if(new_array == NULL) return QL_ERROR;

        memcpy(new_array, array, curr * type);
        free(*array);
        *array = new_array;
        *max = new_size;
        return QL_SUCCESS;
    }

        /* Shrink the array slightly */
    else if(diff <= 16) {
        new_size = *max - 8;
    } else if(diff > 16) {
        new_size = *max - (diff / 2);
    } else if(diff > *max * 2) {
        new_size = *max - (*max / 2);
    }

    /* Check if resize necessary */
    if(new_size != *max) {

        void *new_array = calloc(new_size, type);
        if(new_array == NULL) return QL_ERROR;
        memcpy(new_array, array, curr * type);
        free(*array);
        *array = new_array;
        *max = new_size;
    }

    return QL_SUCCESS;
}