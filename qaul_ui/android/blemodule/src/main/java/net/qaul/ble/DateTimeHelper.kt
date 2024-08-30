// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble

import android.text.TextUtils
import android.util.Log
import java.text.ParseException
import java.text.SimpleDateFormat
import java.util.*

class DateTimeHelper {
    companion object {
        private val TAG:String = "qaul-blemodule DateTimeHelper"
        const val DATE_FORMAT = "yyyy-MM-dd"
        const val DATE_TIME_FORMAT = "yyyy-MM-dd HH:mm:ss"
        const val DISPLAY_TIME_DATE = "hh:mm a, dd MMM yyyy"
        const val DISPLAY_DATE = "MMMM dd, yyyy"

        fun convertFormat(strDate: String?, format: String?, convertFormat: String?): String? {
            if (TextUtils.isEmpty(strDate)) return ""
            val sdf = SimpleDateFormat(format, Locale.getDefault())
            val sdfNew = SimpleDateFormat(convertFormat, Locale.getDefault())
            return try {
                val date = sdf.parse(strDate)
                sdfNew.format(date)
            } catch (e: ParseException) {
                Log.e(TAG, e.toString())
                ""
            }
        }

        fun convertFormat(date: Date?, convertFormat: String?): String {
            return try {
                val sdfNew = SimpleDateFormat(convertFormat, Locale.getDefault())
                sdfNew.format(date)
            } catch (e: Exception) {
                e.printStackTrace()
                ""
            }
        }


        fun convertFormat(strDate: Long, convertFormat: String?): String? {
            if (strDate == 0L) return ""
            val sdfNew = SimpleDateFormat(convertFormat, Locale.getDefault())
            val date = Date(strDate)
            return sdfNew.format(date)
        }

        fun getDate(format: String?): String? {
            return try {
                val sdf = SimpleDateFormat(format, Locale.getDefault())
                val date = Date()
                sdf.format(date)
            } catch (e: IllegalArgumentException) {
                Log.e(TAG, e.toString())
                ""
            }
        }
    }
}