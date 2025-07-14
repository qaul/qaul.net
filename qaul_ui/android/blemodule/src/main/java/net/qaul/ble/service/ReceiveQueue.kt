// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.service

import android.bluetooth.BluetoothDevice
import kotlin.time.TimeSource
import kotlin.time.TimeSource.Monotonic.ValueTimeMark
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.model.FlowControlMessageType


/**
 * Helper Objects
 */
class FlcRequestChunks(val messageIndex: Byte, val chunkIndex: List<Short>)
class FlcAck(val messageIndex: Byte, val success: Boolean)
class ReceivedMessage(val qaulId: ByteArray, val message: ByteArray)

/**
 * Return object of ReceiveQueue
 */
class ReceiveQueueResult {
    var qaulIdMissing: Boolean = false
    var qaulIdRequestReceived: Boolean = false
    var qaulIdReceived: ByteArray? = null
    var flcRequestChunks: FlcRequestChunks? = null
    var flcRequestAck: FlcAck? = null
    var flcAckReceived: FlcAck? = null
    var receivedMessage: ReceivedMessage? = null
}

/**
 * Qaul GATT Messaging is a service class that handles the qaul messages
 * that are sent in chunks as GATT messages.
 */
class ReceiveQueue {
    val TAG: String = "ReceiveQueue"
    //var qaulId: ByteArray = ByteArray(16)
    var qaulIdKnown: Boolean = false
    var incoming: MutableList<ByteArray> = mutableListOf()
    var messageIndex: Byte = 0

    /**
     * Analyze an incoming message
     */
    fun incomingMessage(chunk: ByteArray, device: BluetoothDevice): ReceiveQueueResult {
        var binaryString = BLEUtils.toBinaryString(chunk)
        AppLog.e(TAG, "GattMessaging incomingMessage: chunk: $binaryString")

        // analyze message header
        val (type, index, payload) = messageHeader(chunk)

        // display message Type
        binaryString = BLEUtils.toBinaryString(type)
        AppLog.e(TAG, "GattMessaging incomingMessage type: $type, index: $index, payload size: ${payload.size}")

        // check if message is a flow control message
        if (type == 0x00.toByte()) {
            // Flow Control Message
            AppLog.e(TAG, "GattMessaging incomingMessage flow control message")
            return incomingFlowControlMessage(index, payload)
        } else {
            // Chunk Content Message
            AppLog.e(TAG, "GattMessaging incomingMessage chunk content message")
            return incomingMessageChunk(type, index, payload)
        }
    }

    /**
     * Analyze message header
     * @return message type, index, payload
     */
    fun messageHeader(chunk: ByteArray): Triple<Byte, Short, ByteArray> {
        var binaryString = BLEUtils.toBinaryString(chunk)

        // get message Type
        val b1: Byte = chunk.get(0)
        val type: Byte = (b1.toInt() and 0xFF shr 4).toByte()
        binaryString = BLEUtils.toBinaryString(type)

        // get more header information
        var chunkIndex: Short;
        var payload: ByteArray = ByteArray(0)

        // check if message is a flow control message;
        if (type == 0x00.toByte()) {
            // Flow Control Message

            // get FLC message index
            chunkIndex = (b1.toInt() and 0x0F).toShort()

            // get message payload
            payload = chunk.sliceArray(1 until chunk.size)

        } else {
            // chunk content message

            // get message index
            val b2: Byte = chunk.get(1)
            chunkIndex = (((b1.toInt() and 0xFF shl 8) + b2) and "0000111111111111".toInt(2)).toShort()

            // get message payload
            payload = chunk.sliceArray(2 until chunk.size)
        }

        return Triple(type, chunkIndex, payload)
    }

    /**
     * Handle incoming flow control messages
     */
    fun incomingFlowControlMessage(flcType: Short, payload: ByteArray): ReceiveQueueResult {
        var result = ReceiveQueueResult()

        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage type: $flcType, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage payload: $binaryString")

        when (flcType) {
            FlowControlMessageType.REQUEST_QAUL_ID.value.toShort() -> {
                // fill in ReceiveQueueResult
                result.qaulIdRequestReceived = true
            }
            FlowControlMessageType.SEND_QAUL_ID.value.toShort() -> {
                // check payload size
                if (payload.size != 16) {
                    AppLog.e(TAG, "GattMessaging incomingFlowControlMessage payload size is not 16")
                    if(!qaulIdKnown) {
                        result.qaulIdMissing = true
                    }
                } else {
                    // set qaul_id
                    qaulIdKnown = true
                    result.qaulIdReceived = payload
                }
            }
            // Missing chunks
            FlowControlMessageType.MISSING_CHUNKS.value.toShort() -> {
                

            }
            FlowControlMessageType.ACK_SUCCESS.value.toShort() -> {
                
                
            }
            FlowControlMessageType.ACK_ERROR.value.toShort() -> {
                result

            }
            FlowControlMessageType.MISSING_ACK_MESSAGES.value.toShort() -> {
                // check payload size
                if (payload.size >= 1) {
                    // get message index
                    val b1: Byte = payload.get(0)

                    // check if message was received
                    

                    // send ACK message if it was received
                    // send Error if not
                }
            }
            else -> {
                AppLog.e(TAG, "GattMessaging unknown incomingFlowControlMessage")
            }
        }
        return result
    }

    /**
     * Handle incoming message chunks
     */
    fun incomingMessageChunk(queue: Byte, index: Short, chunk: ByteArray): ReceiveQueueResult {
        AppLog.e(TAG, "GattMessaging incomingMessageChunk")
        AppLog.e(TAG, "queue: $queue, index: $index, chunk size: ${chunk.size}")
        var binaryString = BLEUtils.toBinaryString(chunk)
        AppLog.e(TAG, "chunk: $binaryString")

        // get ReceiveQueueMessage from index

        // check if index exists
    

        // check if first message
/*
        if (index == 0) {
            AppLog.e(TAG, "GattMessaging incomingMessageChunk first message")


        }

        // create a new ReceiveQueueMessage
        val receiveQueueMessage = ReceiveQueueMessage()
        receiveQueueMessage.messageIndex = index.toByte()
        receiveQueueMessage.messageSize = message.size
        receiveQueueMessage.totalChunks = (message.size / receiveQueueMessage.chunkSize).toShort()

        // add the first chunk
        if (receiveQueueMessage.addReceivedChunk(message)) {
            AppLog.e(TAG, "GattMessaging incomingMessageChunk all chunks received")

            return receiveQueueMessage
        }
*/
        val receiveQueueResult = ReceiveQueueResult()
        return receiveQueueResult        
    }
}

/**
 * message receiving state
 */
enum class ReceiveQueueMessageState {
    RECEIVING,
    WAITING_ON_MISSING,
    RECEIVED_MISSING_ID,
    RECEIVED,
    ERROR
}

/**
 * protocol constants
 */
const val HEADER_SIZE = 2
const val FIRST_MESSAGE_HEADER = 10

/**
 * ReceiveQueueMessage is a data class that holds the information of a receiving message
 * until all chunks have been received successfully.
 */
class ReceiveQueueMessage {
    val TAG: String = "ReceiveQueueMessage"
    var firstMessageReceived: Boolean = false
    var state: ReceiveQueueMessageState = ReceiveQueueMessageState.RECEIVING
    var createdAt: ValueTimeMark = TimeSource.Monotonic.markNow()
    var receivedAt: ValueTimeMark = TimeSource.Monotonic.markNow()
    var sentAt: ValueTimeMark? = null

    var messageIndex: Byte = 0
    var messageSize: Int? = null
    var totalChunks: Short? = null
    var chunkSize: Int = 20
    var currentChunkIndex: Short = 0

    var missingChunks: List<Short> = listOf()
    var receivedChunks: Map<Short, ByteArray> = mapOf()

    /**
     * Add a newly received chunk message
     */
    fun addReceivedChunk(index: Short, payload: ByteArray): ReceiveQueueResult {
        val receivedQueueResult = ReceiveQueueResult()

        // check if chunk is already received
        if (receivedChunks.containsKey(index)) {
            AppLog.e(TAG, "GattMessaging addChunk chunk already received")
            return receivedQueueResult
        }

        // add chunk to received chunks
        //receivedChunks[index] = message


        // check if all chunks are received
        if (totalChunks != null && receivedChunks.size == totalChunks!!.toInt()) {
            state = ReceiveQueueMessageState.RECEIVED
            AppLog.e(TAG, "GattMessaging addChunk all chunks received")

            // TODO: create final message from received chunks
            //receivedQueueResult.receivedMessage = ReceivedMessage(qaulId, receivedChunks.values.reduce { acc, bytes -> acc + bytes })
            return receivedQueueResult
        }

        // check if this is the index with the last message
        // TODO: request missing chunks if not all chunks are received

        return receivedQueueResult
    }

}
