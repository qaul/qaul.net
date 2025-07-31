// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
 * This File contains the protocol logic of the qaul GATT Messaging receive queue.
 * The classes are used in BleService.kt to handle and store incoming chunks.
 */

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
 * Receive Chunk State
 */
enum class ReceiveChunkState {
    CURRENT_QUEUE,
    NEW_QUEUE,
    MISSING_CHUNK,
    INVALID_CHUNK
}

/**
 * Qaul GATT Messaging is a service class that handles the qaul messages
 * that are sent in chunks as GATT messages.
 */
class ReceiveQueue {
    private val TAG: String = "ReceiveQueue"
    //var qaulId: ByteArray = ByteArray(8)
    var qaulIdKnown: Boolean = false
    // TO DELETE:
    //var incoming: MutableList<ByteArray> = mutableListOf()

    // Current Message Queue Index
    // there are 14 message receive queues, which are rotating.
    var currentMessageQueueIndex: Byte = 0

    // Last Update
    var lastUpdate: ValueTimeMark = TimeSource.Monotonic.markNow()

    // Receive Message Queues
    // each queue is identified by the message queue index
    var receiveQueues: MutableMap<Byte, ReceiveQueueMessage> = mutableMapOf()

    /**
     * Analyze an incoming message
     */
    fun incomingMessage(chunk: ByteArray, device: BluetoothDevice): ReceiveQueueResult {
        var binaryString = BLEUtils.toBinaryString(chunk)
        AppLog.e(TAG, "GattMessaging incomingMessage: chunk: $binaryString")

        // analyze message header
        val (type, index, payload) = messageHeader(chunk)
        var result: ReceiveQueueResult;

        // display message Type
        binaryString = BLEUtils.toBinaryString(type)
        AppLog.e(TAG, "GattMessaging incomingMessage type: $type, index: $index, payload size: ${payload.size}")

        // check if message is a flow control message
        if (type == 0x00.toByte()) {
            // Flow Control Message
            AppLog.e(TAG, "GattMessaging incomingMessage flow control message")
            result = incomingFlowControlMessage(index.toByte(), payload)
        } else {
            // Chunk Content Message
            AppLog.e(TAG, "GattMessaging incomingMessage chunk content message")
            result = incomingMessageChunk(type, index, payload)
        }

        return result
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
    fun incomingFlowControlMessage(flcType: Byte, payload: ByteArray): ReceiveQueueResult {
        var result = ReceiveQueueResult()

        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage type: $flcType, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage payload: $binaryString")

        when (flcType) {
            FlowControlMessageType.REQUEST_QAUL_ID.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: REQUEST_QAUL_ID")
                // fill in ReceiveQueueResult
                result.qaulIdRequestReceived = true
            }
            FlowControlMessageType.SEND_QAUL_ID.value -> {
                // check payload size
                if (payload.size != 8) {
                    AppLog.e(TAG, "GattMessaging incomingFlowControlMessage payload size is not 8")
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
            FlowControlMessageType.MISSING_CHUNKS.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: MISSING_CHUNKS")
            }
            FlowControlMessageType.ACK_SUCCESS.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: ACK_SUCCESS")
            }
            FlowControlMessageType.ACK_ERROR.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: ACK_ERROR")
            }
            FlowControlMessageType.MISSING_ACK_MESSAGES.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: MISSING_ACK_MESSAGES")

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
                AppLog.e(TAG, "incomingFlowControlMessage: unknown")
            }
        }
        return result
    }

    /**
     * Handle incoming message chunks
     */
    fun incomingMessageChunk(queue: Byte, index: Short, payload: ByteArray): ReceiveQueueResult {
        AppLog.e(TAG, "GattMessaging incomingMessageChunk")
        AppLog.e(TAG, "queue: $queue, index: $index, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "payload: $binaryString")

        // check Chunk State
        val chunkState = checkChunkState(queue)

        when (chunkState) {
            ReceiveChunkState.CURRENT_QUEUE -> {
                AppLog.e(TAG, "GattMessaging incomingMessageChunk current queue")

                // get ReceiveQueueMessage
                var receiveQueueMessage: ReceiveQueueMessage? = receiveQueues.get(queue)

                // check if index exists
                if (receiveQueueMessage == null) {
                    // this is considered to be an error
                    AppLog.e(TAG, "ERROR: Message queue does not exist in receiveQueues")
                    // TODO: return an error FLC message
                    return ReceiveQueueResult()
                }

                // add chunk to queue
                val result = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                return result
            }
            ReceiveChunkState.NEW_QUEUE -> {
                AppLog.e(TAG, "GattMessaging incomingMessageChunk new queue")
                currentMessageQueueIndex = queue

                // create new ReceiveQueueMessage
                var receiveQueueMessage = ReceiveQueueMessage()
                receiveQueueMessage.messageQueueIndex = queue

                // add chunk to queue
                var result = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                // check if qaulId is known
                if (!qaulIdKnown) {
                    result.qaulIdMissing = true
                }
                return result
            }
            ReceiveChunkState.MISSING_CHUNK -> {
                AppLog.e(TAG, "GattMessaging incomingMessageChunk missing chunk")

                // get ReceiveQueueMessage
                var receiveQueueMessage: ReceiveQueueMessage? = receiveQueues.get(queue)

                // check if index exists
                if (receiveQueueMessage == null && qaulIdKnown == false) {
                    receiveQueueMessage = ReceiveQueueMessage()
                } else if (receiveQueueMessage == null) {
                    return ReceiveQueueResult()
                }

                // add chunk to queue
                val result = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                return result
            }
            ReceiveChunkState.INVALID_CHUNK -> {
                AppLog.e(TAG, "GattMessaging incomingMessageChunk invalid chunk")
                return ReceiveQueueResult()
            }
        }
    }

    /**
     * Check Chunk State
     */
    fun checkChunkState(queue: Byte): ReceiveChunkState {
        // check if current queue
        if (queue == currentMessageQueueIndex) {
            return ReceiveChunkState.CURRENT_QUEUE
        }

        // check if new queue
        if (currentMessageQueueIndex == 0.toByte() ||
            (queue > currentMessageQueueIndex && queue <= currentMessageQueueIndex + 3) ||
            (currentMessageQueueIndex > 11 && queue <= 3 - (14 - currentMessageQueueIndex))) {
            return ReceiveChunkState.NEW_QUEUE
        }

        // chunk is missing chunk update
        return ReceiveChunkState.MISSING_CHUNK
    }
}

/**
 * message receiving state
 */
enum class ReceiveQueueMessageState {
    RECEIVING,              // receiving messages
    WAITING_ON_MISSING,     // last index received and waiting for missing chunks
    RECEIVED_MISSING_ID,    // ??? what does this mean?
    RECEIVED,               // message was received successfully, SUCCESS ACK sent
    ERROR                   // message was not received successfully, ERROR ACK sent
}

/**
 * protocol constants
 */
// Chunk Header Size in Bytes
const val CHUNK_HEADER_SIZE = 2
// First Chunk Header Size in Bytes
const val FIRST_CHUNK_HEADER_SIZE = 12
// Maximum number of missing chunks in one gap.
// If the gap is larger, an error is issued.
const val MAX_MISSING_GAP = 12      

/**
 * Missing Chunk State
 */
enum class MissingChunkState {
    UNREQUESTED,    // this chunk was not requested yet
    REQUEST_SENT    // request for this chunk was sent
}

/**
 * Missing Chunk
 */
data class MissingChunk(val index: Short) {
    var state: MissingChunkState = MissingChunkState.UNREQUESTED
    var updated_at: ValueTimeMark = TimeSource.Monotonic.markNow()
}

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

    // TO DELETE:
    var messageQueueIndex: Byte = 0
    var messageSize: Int? = null
    var totalChunks: Short? = null
    var crc32Checksum: Long? = null
    var chunkSize: Int = 20
    var currentChunkIndex: Short = 0

    var missingChunks: MutableMap<Short, MissingChunk> = mutableMapOf()
    var receivedChunks: MutableMap<Short, ByteArray> = mutableMapOf()

    /**
     * Add a newly received chunk message
     */
    fun addReceivedChunk(index: Short, payload: ByteArray, qaulIdKnown: Boolean): ReceiveQueueResult {
        var receivedQueueResult = ReceiveQueueResult()

        // check if chunk has already been received
        if (receivedChunks.containsKey(index)) {
            AppLog.e(TAG, "GattMessaging addChunk chunk already received")
            return receivedQueueResult
        }

        // check if this is the first message
        if (index == 0.toShort()) {
            AppLog.e(TAG, "GattMessaging addChunk first message received")
            firstMessageReceived = true

            // get payload & and set message information
            val firstMessagePayload = analyzeFirstMessage(payload)

            // add payload to receivedChunks
            receivedChunks.put(index, firstMessagePayload)
        } else {
            // add payload to received chunks
            receivedChunks.put(index, payload)
        }

        // update current chunk index
        receivedQueueResult = updateMessageIndex(index, qaulIdKnown)

        return receivedQueueResult
    }

    /**
     * Analyze First Message chunk without the 2 Byte Header
     * @param chunk ByteArray
     * @return ByteArray with First Message Payload
     */
    private fun analyzeFirstMessage(chunk: ByteArray): ByteArray {
        // check if chunk is large enough
        if (chunk.size < FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE) {
            AppLog.e(TAG, "GattMessaging analyzeFirstMessagePayload chunk size is too small")
            return ByteArray(0)
        }

        // get message size
        messageSize = BLEUtils.byteArrayToInt(chunk.sliceArray(0 until 1))

        // get total chunks
        totalChunks = BLEUtils.byteArrayToInt(chunk.sliceArray(2 until 3)).toShort()

        // get crc32 checksum
        crc32Checksum = BLEUtils.byteArrayToCrc32Value(chunk.sliceArray(4 until 7))

        // get payload
        val payload: ByteArray
        if (chunk.size > 8) {
            payload = chunk.sliceArray(8 until chunk.size)
        } else {
            payload = ByteArray(0)
        }
        return payload
    }


    // TODO: request missing chunks!!!
    /**
     * Update the current message index
     * @param index Short
     * @return ReceiveQueueResult
     */
    fun updateMessageIndex(index: Short, qaulIdKnown: Boolean): ReceiveQueueResult {
        var result = ReceiveQueueResult()

        // update index and check if we have missing chunks
        if (index == 0.toShort()) {
            // handle first chunk
            // don't update current chunk index
        } else if (index < currentChunkIndex) {
            // this is a missing chunk update
            // don't update current message index
        } else if (currentChunkIndex + 1 == index.toInt()) {
            // update current chunk index
            currentChunkIndex = index
        } else {
            if (qaulIdKnown) {
                // we have missing chunks
                for (i in (currentChunkIndex + 1)..(index - 1)) {
                    // add missing chunk to missingChunks
                    if (!missingChunks.containsKey(i.toShort())) {
                        missingChunks.put(i.toShort(), MissingChunk(i.toShort()))
                    }
                }
            }
            else {
                // we don't know what is going on, so we don't act on missing chunks
            }

            // update current chunk index
            currentChunkIndex = index
        }

        // remove from missing chunks if it exists
        if (missingChunks.contains(index)) {
            missingChunks.remove(index)
        }

        // check if all chunks are received
        if (totalChunks != null && 
            receivedChunks.size == totalChunks!!.toInt()) {
            state = ReceiveQueueMessageState.RECEIVED
            AppLog.e(TAG, "GattMessaging addChunk all chunks received")

            // TODO: create final message from received chunks
            //receivedQueueResult.receivedMessage = ReceivedMessage(qaulId, receivedChunks.values.reduce { acc, bytes -> acc + bytes })
        }

        return result
    }
}
