/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#include "qaul/utils/arrays.h"
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


int qlutils_compact_array(void **array, size_t max)
{
    if(array == NULL) return 1;

    /* This array holds the item modifiers*/
    int shifts[max];
    memset(shifts, 0, sizeof(int) * max);

    /* First find out how we will move data */
    int acc = 0;
    for(int i = 0; i < max; i++) {
        void *data = array[i];
        if(data == NULL) {
            shifts[i + 1] = ++acc;
        }
    }

    /* Then move the data */
    for(int i = 0; i < max; i++) {
        if(shifts[i] != 0) {
            array[i - shifts[i]] = array[i];
            array[i] = NULL;
        }
    }

    return 0;
}