/*
 * The olsr.org Optimized Link-State Routing daemon(olsrd)
 * Copyright (c) 2004, Andreas Tonnesen(andreto@olsr.org)
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 *
 * * Redistributions of source code must retain the above copyright
 *   notice, this list of conditions and the following disclaimer.
 * * Redistributions in binary form must reproduce the above copyright
 *   notice, this list of conditions and the following disclaimer in
 *   the documentation and/or other materials provided with the
 *   distribution.
 * * Neither the name of olsr.org, olsrd nor the names of its
 *   contributors may be used to endorse or promote products derived
 *   from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
 * "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
 * LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS
 * FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE
 * COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT,
 * INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
 * BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN
 * ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *
 * Visit http://www.olsr.org for more information.
 *
 * If you find this software useful feel free to make a donation
 * to the project. For more information see the website or contact
 * the copyright holders.
 *
 */

#include "../qaullib_private.h"
#include "mantissa.h"

/**
 *Function that converts a double to a mantissa/exponent
 *product as described in RFC3626:
 *
 * value = C*(1+a/16)*2^b [in seconds]
 *
 *  where a is the integer represented by the four highest bits of the
 *  field and b the integer represented by the four lowest bits of the
 *  field.
 *
 *@param interval the time interval to process
 *
 *@return a 8-bit mantissa/exponent product
 */
uint8_t
reltime_to_me(const olsr_reltime interval)
{
  uint8_t a, b;

  /* It is sufficent to compare the integer part since we test on >=.
   * So we have now only a floating point division and the rest of the loop
   * are only integer operations.
   *
   * const unsigned int unscaled_interval = interval / VTIME_SCALE_FACTOR;
   *
   * VTIME_SCALE_FACTOR = 1/16
   *
   * => unscaled_interval = interval(ms) / 1000 * 16
   *                      = interval(ms) / 125 * 2
   */
  const unsigned int unscaled_interval = interval / 125 * 2;
  b = 0;
  while (unscaled_interval >= (1U << b)) {
    b++;
  }

  if (b == 0) {
    a = 1;
    b = 0;
  } else {
    b--;
    if (b > 15) {
      a = 15;
      b = 15;
    } else {
      /* And again some maths simplification from the former version:
       *    a = 16 * ((double)interval / (VTIME_SCALE_FACTOR * (double)(1 << b)) - 1)
       * Since interval is already double:
       *    a = 16 * (interval / (VTIME_SCALE_FACTOR * (double)(1 << b)) - 1)
       * first, we can get rid of parentheses and change the * to a /
       *    a = 16 * (interval / VTIME_SCALE_FACTOR / (double)(1 << b) - 1)
       * then we make an integer addition from the floating point addition
       *    a = (int)(16.0 * interval / VTIME_SCALE_FACTOR / (double)(1 << b)) - 16
       * and we loose an unnecessary cast
       *    a = (int)(16.0 * interval / VTIME_SCALE_FACTOR / (1 << b)) - 16
       *
       * VTIME_SCALE_FACTOR = 1/16
       *
       * => a = (16 * interval(ms) / 1000 * 16 / (1 << b)) - 16
       *      = (interval(ms) * 256 / 1000 / (1 << b)) - 16
       *      = (interval(ms) * 32 / 125 / (1<<b)) - 16
       *      = (interval(ms) - 16/32*125*(1<<b)) * 32 / 125 / (1<<b)
       *      = (interval(ms) - 125*(1<<(b-1))) * 32 / 125 / (1<<b)
       *
       * 1. case: b >= 5
       *      = (interval(ms) - (125 << (b-1))) / 125 / (1 << (b-5))
       *      = (interval(ms) - (125 << (b-1))) / (125  << (b-5))
       *
       * 2. case: b <= 5
       *      = (interval(ms) - (125 << (b-1))) / 125 * (1 << (5-b))
       *      = (interval(ms) - (125 << (b-1))) * (1 << (5-b)) / 125
       */

      if (b >= 5) {
        a = (interval - (125 << (b - 1))) / (125 << (b - 5));
      } else {
        a = (interval - (125 << (b - 1))) * (1 << (5 - b)) / 125;
      }

      b += a >> 4;
      a &= 0x0f;
    }
  }

  return (a << 4) | (b & 0x0F);
}

/**
 * Function for converting a mantissa/exponent 8bit value back
 * to double as described in RFC3626:
 *
 * value = C*(1+a/16)*2^b [in seconds]
 *
 *  where a is the integer represented by the four highest bits of the
 *  field and b the integer represented by the four lowest bits of the
 *  field.
 *
 * me is the 8 bit mantissa/exponent value
 *
 * To avoid expensive floating maths, we transform the equation:
 *     value = C * (1 + a / 16) * 2^b
 * first, we make an int addition from the floating point addition:
 *     value = C * ((16 + a) / 16) * 2^b
 * then we get rid of a pair of parentheses
 *     value = C * (16 + a) / 16 * 2^b
 * and now we make an int multiplication from the floating point one
 *     value = C * (16 + a) * 2^b / 16
 * so that we can make a shift from the multiplication
 *     value = C * ((16 + a) << b) / 16
 * and sionce C and 16 are constants
 *     value = ((16 + a) << b) * C / 16
 *
 * VTIME_SCALE_FACTOR = 1/16
 *
 * =>  value(ms) = ((16 + a) << b) / 256 * 1000
 *
 * 1. case: b >= 8
 *           = ((16 + a) << (b-8)) * 1000
 *
 * 2. case: b <= 8
 *           = ((16 + a) * 1000) >> (8-b)
 */
olsr_reltime
me_to_reltime(const uint8_t me)
{
  const uint8_t a = me >> 4;
  const uint8_t b = me & 0x0F;

  if (b >= 8) {
    return ((16 + a) << (b - 8)) * 1000;
  }
  return ((16 + a) * 1000) >> (8 - b);
}

/*
 * Local Variables:
 * c-basic-offset: 2
 * indent-tabs-mode: nil
 * End:
 */
