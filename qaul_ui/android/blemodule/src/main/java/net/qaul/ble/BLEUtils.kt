// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble

import android.text.TextUtils
import android.util.Log

/**
 * Helper functions for BLE module for formatting and conversion
 * 
 * - Format ByteArray to hex String for logging
 * - Format ByteArray to binary String for logging
 * - Convert hex String to ByteArray
 * - Convert ByteArray to Int
 */
object BLEUtils {
    private val HEX_ARRAY = "0123456789ABCDEF".toCharArray()
    const val TAG = "qaul-blemodule BLEUtils"

    /**
     * Convert ByteArray to hex String
     * @param byteArray Byte array to format
     * @return Hex string representation of the byte array
     * If the byte array is null, return an empty string
     */
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

    /**
     * Convert Byte to hex String
     * @param data Byte to convert
     * @return Hex string representation of the byte
     * If the input Byte is null, return an empty string
     */
    fun byteToHex(data: Byte?): String {
        if (data == null) {
            return ""
        }
        return String.format("%02X", data)
    }

    /**
     * Convert ByteArray to binary String
     * @param byteArray ByteArray to convert
     * @return Binary string representation of the byte array
     * If the byte array is null, return an empty string
     */
    fun toBinaryString(byteArray: ByteArray?): String {
        if (byteArray == null) {
            return ""
        }
        val stringBuilder = StringBuilder()
        for (b in byteArray) {
            stringBuilder.append(String.format("%8s", Integer.toBinaryString(b.toInt() and 0xFF)).replace(' ', '0'))
            stringBuilder.append(" ")
        }
        return stringBuilder.toString()
    }

    /**
     * Convert Byte to binary String
     * @param byte Byte to convert
     * @return Binary String representation of the Byte
     * If the Byte is null, return an empty string
     */
    fun toBinaryString(byte: Byte?): String {
        if (byte == null) {
            return ""
        }
        
        return String.format("%8s", Integer.toBinaryString(byte.toInt() and 0xFF)).replace(' ', '0')
    }

    /**
     * Convert Short to binary String
     * @param short Short to convert
     * @return Binary string representation of the Short
     * If the Short is null, return an empty string
     */
    fun toBinaryString(short: Short?): String {
        if (short == null) {
            return ""
        }
        return String.format("%16s", Integer.toBinaryString(short.toInt() and 0xFFFF)).replace(' ', '0')
    }

    /**
     * Convert hex string to ByteArray
     * @param data Hex string to convert
     * @return ByteArray representation of the hex string
     * If the input string is null or empty, return null
     */
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

    /**
     * Convert Int to ByteBrray
     * @param value Int to convert
     * @return ByteArray representation of the Int
     */
    fun toByteArray(value: Int): ByteArray {
        return byteArrayOf(
            (value shr 24 and 0xFF).toByte(),
            (value shr 16 and 0xFF).toByte(),
            (value shr 8 and 0xFF).toByte(),
            (value and 0xFF).toByte()
        )
    }

    /**
     * Convert Short to ByteArray
     * @param value Short to convert
     * @return ByteArray representation of the Short
     */
    fun toByteArray(value: Short): ByteArray {
        return byteArrayOf(
            (value.toInt() shr 8 and 0xFF).toByte(),
            (value.toInt() and 0xFF).toByte()
        )
    }

    /**
     * Convert crc32 to ByteArray
     * @param value UInt to convert
     * @return ByteArray representation of the UInt
     * Note: Kotlin does not have a native UInt type, so this is a workaround
     */
    fun crc32ValueToByteArray(value: Long): ByteArray {
        return byteArrayOf(
            (value.toInt() shr 24 and 0xFF).toByte(),
            (value.toInt() shr 16 and 0xFF).toByte(),
            (value.toInt() shr 8 and 0xFF).toByte(),
            (value.toInt() and 0xFF).toByte()
        )
    }

    /**
     * Convert ByteArray to Int
     * @param data ByteArray to convert
     * @return Int representation of the ByteArray
     * If the input ByteArray is empty or null, return 0
     */
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
                val remove = itr.remove()
            }
        }
        Log.e(TAG, "removeConcurrent: ------------------- ${this.toMutableList().iterator()}")
    }

    /**
     * Convert Byte to Int
     * @param data Byte to convert
     * @return Int representation of the Byte
     * If the input Byte is null, return 0
     */
    fun byteToInt(data: Byte?): Int {
        if (data == null) {
            return 0
        }
        return data.toInt() and 0xFF
    }
}