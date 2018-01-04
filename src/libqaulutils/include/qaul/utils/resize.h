/*
 * qaul.net is free software
 * licensed under GPL (version 3)
 */

#ifndef QAUL_RESIZE_H
#define QAUL_RESIZE_H

#include <stdlib.h>

/**
 * A simple utility function which resizes an array, if it is required
 *
 * If the array is full, add space according to a simple strategy
 *   - If array < 32  then double size
 *   - If array >= 32 then add + 8
 *   - If array is >= 64 then add + (size / 4)
 *
 * If the array is almost empty, remove space according to the strategy
 *   - If diff <= 32, then remove 8
 *   - If diff >= 32, then remove (diff  /2)
 *   - If diff > size * 2, then remove (size / 2)
 *
 * @param array Pointer to an array
 * @param curr The currently used size in the array
 * @param max The maximum size of the array
 * @return
 */
int qlutils_resize_array(void **array, size_t type, size_t curr, size_t *max);


#endif
