// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble

import android.text.TextUtils

object BLEUtils {
    private val HEX_ARRAY = "0123456789ABCDEF".toCharArray()
    const val TAG = "qaul-blemodule BLEUtils"
    fun byteToHex(byteArray: ByteArray?): String {
        val stringBuilder = StringBuffer()

        if (byteArray == null) {
            return stringBuilder.toString()
        }

        for (b in byteArray) {
            val st = String.format("%02X", b)
            stringBuilder.append(st)
        }
        return stringBuilder.toString()
    }

    fun byteToHex(data: Byte?): String {
        if (data == null) {
            return ""
        }
        return String.format("%02X", data)
    }

    fun hexToByteArray(data: String?): ByteArray? {
        var newData = ""
        if (TextUtils.isEmpty(data)) {
            return null
        }
        newData = data.toString()
        require(newData.length % 2 == 0) { "Must have an even length" }

        return newData.chunked(2)
            .map { it.toInt(16).toByte() }
            .toByteArray()
    }

    fun byteToInt(data: ByteArray?): Int {
        if (data == null) {
            return 0
        }
        when (data.size) {
            1 -> return data[0].toInt() and 0xFF
            2 -> return data[0].toInt() and 0xFF shl 8 or (data[1].toInt() and 0xFF)
            3 -> return data[0].toInt() and 0xFF shl 16 or (data[1].toInt() and 0xFF shl 8) or (data[2].toInt() and 0xFF)
            4 -> return data[0].toInt() and 0xFF shl 24 or (data[1].toInt() and 0xFF shl 16) or (data[2].toInt() and 0xFF shl 8) or (data[3].toInt() and 0xFF)
        }
        return 0
    }

    fun <T> List<T>.removeConcurrent(data: Any) {
        val itr: MutableIterator<T> = this.toMutableList().iterator()
        while (itr.hasNext()) {
            val t = itr.next()
            if (t == data) {
                itr.remove()
            }
        }
    }
    fun byteToInt(data: Byte?): Int {
        if (data == null) {
            return 0
        }
        return data.toInt() and 0xFF
    }
}