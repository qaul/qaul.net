// Copyright (c) 2023 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.service

import android.util.Log
import java.nio.ByteBuffer
import java.nio.ByteOrder
import java.math.BigInteger
import net.qaul.ble.model.FlowControlMessageType

/**
 * Helper object to create Flow Control Messages (FLC)
 */
object FlcCreate {
    /**
     * Create an ID request message
     * @return FLC message ByteArray
     */
    fun createIdRequest(): ByteArray {
        val message = byteArrayOf(FlowControlMessageType.REQUEST_QAUL_ID.value.toByte())
        return message
    }

    /**
     * Create a send ID message
     * @param qaulId The 16 Byte qaul ID to send
     * @return FLC message ByteArray
     */
    fun createSendId(qaulId: ByteArray): ByteArray {
        val header = byteArrayOf(FlowControlMessageType.SEND_QAUL_ID.value.toByte())
        val message = header + qaulId
        return message
    }

    /**
     * Create a request for chunks
     */
    fun createRequestChunks(messageIndex: Byte, chunkIndex: List<Short>): ByteArray {
        val message = ByteArray(2 + chunkIndex.size * 2)
        message[0] = FlowControlMessageType.MISSING_CHUNKS.value.toByte()
        message[1] = messageIndex.toByte()
        for (i in chunkIndex.indices) {
            val index = 3 + i * 2

            val value = chunkIndex[i].toInt()
            message[index] = (value shr 0).toByte() // high byte
            message[index + 1] = (value shr 8).toByte() // low byte
        }
        return message
    }

    /**
     * Create an ACK message
     * @param queueIndex Index of the missing ACK
     */
    fun createAck(queueIndex: Byte, success: Boolean, errorCode: Byte): ByteArray {
        if (success) {
            val message = ByteArray(2)
            message[0] = FlowControlMessageType.ACK_SUCCESS.value.toByte()
            message[1] = queueIndex
            return message
        } else {
            val message = ByteArray(3)
            message[0] = FlowControlMessageType.ACK_ERROR.value.toByte()
            message[1] = queueIndex.toByte()
            message[2] = errorCode.toByte() // reason for failure
            return message
        }
    }

    /**
     * Create ACK request message
     * @param queueIndex Index of the missing ACK
     */
    fun createAckRequest(queueIndex: Byte): ByteArray {
        val message = ByteArray(2)
        message[0] = FlowControlMessageType.MISSING_ACK_MESSAGES.value.toByte()
        message[1] = queueIndex.toByte()
        return message
    }
}