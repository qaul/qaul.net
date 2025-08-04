// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
 * This File contains the protocol logic of the qaul GATT Messaging receive queue.
 * The classes are used in BleService.kt to handle and store incoming chunks.
 */

package net.qaul.ble.service

import android.bluetooth.BluetoothDevice
import java.util.zip.CRC32
import kotlin.time.TimeSource
import kotlin.time.TimeSource.Monotonic.ValueTimeMark
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.model.FlowControlMessageType


/**
 * Helper Objects
 */
class FlcRequestChunks(val messageIndex: Byte, val chunkIndex: List<Short>)
class FlcAck(val messageIndex: Byte, val success: Boolean = false, val errorCode: Byte = 0)
class ReceivedMessage(val qaulId: ByteArray, val message: ByteArray)

/**
 * Return object of ReceiveQueue
 */
class ReceiveQueueResult {
    var qaulIdMissing: Boolean = false
    var qaulIdRequestReceived: Boolean = false
    var qaulIdReceived: ByteArray? = null
    var flcRequestChunks: FlcRequestChunks? = null
    var flcSendAck: FlcAck? = null
    var flcRequestAck: Byte? = null
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
        var receiveQueueResult: ReceiveQueueResult;

        // display message Type
        binaryString = BLEUtils.toBinaryString(type)
        AppLog.e(TAG, "GattMessaging incomingMessage type: $type, index: $index, payload size: ${payload.size}")

        // check if message is a flow control message
        if (type == 0x00.toByte()) {
            // Flow Control Message
            AppLog.e(TAG, "GattMessaging incomingMessage flow control message")
            receiveQueueResult = incomingFlowControlMessage(index.toByte(), payload)
        } else {
            // Chunk Content Message
            AppLog.e(TAG, "GattMessaging incomingMessage chunk content message")
            receiveQueueResult = incomingMessageChunk(type, index, payload)
        }

        return receiveQueueResult
    }

    /**
     * Analyze message header
     * @return message type, index, payload
     */
    private fun messageHeader(chunk: ByteArray): Triple<Byte, Short, ByteArray> {
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
    private fun incomingFlowControlMessage(flcType: Byte, payload: ByteArray): ReceiveQueueResult {
        var receiveQueueResult = ReceiveQueueResult()
        if (!qaulIdKnown) {
            receiveQueueResult.qaulIdMissing = true
        }

        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage type: $flcType, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "GattMessaging incomingFlowControlMessage payload: $binaryString")

        when (flcType) {
            FlowControlMessageType.REQUEST_QAUL_ID.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: REQUEST_QAUL_ID")
                // fill in ReceiveQueueResult
                receiveQueueResult.qaulIdRequestReceived = true
            }
            FlowControlMessageType.SEND_QAUL_ID.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: SEND_QAUL_ID")
                // check payload size
                if (payload.size != 8) {
                    AppLog.e(TAG, "SEND_QAUL_ID payload size is not 8")
                } else {
                    // set qaul_id
                    qaulIdKnown = true
                    receiveQueueResult.qaulIdReceived = payload
                    receiveQueueResult.qaulIdMissing = false
                    AppLog.e(TAG, "incomingFlowControlMessage:received qaul_id: ${BLEUtils.toBinaryString(payload)}")
                }
            }
            // Missing chunks
            FlowControlMessageType.MISSING_CHUNKS.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: MISSING_CHUNKS")
            }
            FlowControlMessageType.ACK_SUCCESS.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: ACK_SUCCESS")
                receiveQueueResult.flcAckReceived = FlcAck(
                    payload[0],
                    true
                )
            }
            FlowControlMessageType.ACK_ERROR.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: ACK_ERROR")
                receiveQueueResult.flcAckReceived = FlcAck(
                    payload[0],
                    false,
                    payload[1]
                )
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
        return receiveQueueResult
    }

    /**
     * Handle incoming message chunks
     */
    private fun incomingMessageChunk(queue: Byte, index: Short, payload: ByteArray): ReceiveQueueResult {
        AppLog.e(TAG, "incomingMessageChunk: queue: $queue, index: $index, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "payload: $binaryString")

        // check Chunk State
        val chunkState = checkChunkState(queue)

        when (chunkState) {
            ReceiveChunkState.CURRENT_QUEUE -> {
                AppLog.e(TAG, "incomingMessageChunk: current queue")

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
                val receiveQueueResult = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                return receiveQueueResult
            }
            ReceiveChunkState.NEW_QUEUE -> {
                AppLog.e(TAG, "incomingMessageChunk: new queue")
                currentMessageQueueIndex = queue

                // create new ReceiveQueueMessage
                var receiveQueueMessage = ReceiveQueueMessage()
                receiveQueueMessage.messageQueueIndex = queue

                // add chunk to queue
                var receiveQueueResult = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                // check if qaulId is known
                if (!qaulIdKnown) {
                    receiveQueueResult.qaulIdMissing = true
                }
                return receiveQueueResult
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
                val receiveQueueResult = receiveQueueMessage.addReceivedChunk(index, payload, qaulIdKnown)
                receiveQueues.put(queue, receiveQueueMessage)

                return receiveQueueResult
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
    private fun checkChunkState(queue: Byte): ReceiveChunkState {
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
const val FIRST_CHUNK_HEADER_SIZE = 18
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
    var firstChunkReceived: Boolean = false
    var state: ReceiveQueueMessageState = ReceiveQueueMessageState.RECEIVING
    var createdAt: ValueTimeMark = TimeSource.Monotonic.markNow()
    var receivedAt: ValueTimeMark = TimeSource.Monotonic.markNow()
    var sentAt: ValueTimeMark? = null

    var qaulId: ByteArray = ByteArray(0)
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
            firstChunkReceived = true

            // get payload & and set message information
            val firstChunkPayload = analyzeFirstChunk(payload)

            // add payload to receivedChunks
            receivedChunks.put(index, firstChunkPayload)
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
    private fun analyzeFirstChunk(chunk: ByteArray): ByteArray {
        // check if chunk is large enough
        if (chunk.size < FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE) {
            AppLog.e(TAG, "analyzeFirstChunk: chunk size is too small: ${chunk.size}")
            return ByteArray(0)
        }

        // get message size
        messageSize = BLEUtils.byteArrayToInt(chunk.sliceArray(0..1))

        // get total chunks
        totalChunks = BLEUtils.byteArrayToInt(chunk.sliceArray(2..3)).toShort()

        // get crc32 checksum
        crc32Checksum = BLEUtils.byteArrayToCrc32Value(chunk.sliceArray(4..7))

        // get qaulId
        val receivedQaulId = chunk.sliceArray(8..15)
        if (!qaulId.size.equals(8)) {
            qaulId = receivedQaulId
        }

        // get payload
        val payload: ByteArray
        if (chunk.size > FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE) {
            payload = chunk.sliceArray(FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE until chunk.size)
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
        var receivedQueueResult = ReceiveQueueResult()
        if (!qaulIdKnown) {
            receivedQueueResult.qaulIdMissing = true
        }
        
        // update index and check if we have missing chunks
        if (index == 0.toShort()) {
            // handle first chunk
            // don't update current chunk index
            if (!qaulIdKnown) {
                receivedQueueResult.qaulIdReceived = qaulId
                receivedQueueResult.qaulIdMissing = false
            }
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
            // Create final message from received chunks
            receivedQueueResult = assembleMessage(receivedQueueResult)
        }

        return receivedQueueResult
    }

    /**
     * Assemble the final message from received chunks
     * @param result ReceiveQueueResult
     * @return ReceivedMessage
     */
    private fun assembleMessage(receivedQueueResult: ReceiveQueueResult): ReceiveQueueResult {
        var error = false
        var message = ByteArray(0)

        // check if all chunks are received
        if (totalChunks == null || receivedChunks.size != totalChunks!!.toInt()) {
            AppLog.e(TAG, "assembleMessage not all chunks received")
            error = true
        }

        // put all received chunks together
        if (!error) {
            for (i in 0 until totalChunks!!) {
                if (!receivedChunks.containsKey(i.toShort())) {
                    AppLog.e(TAG, "assembleMessage missing chunk: $i")
                    error = true
                    break
                } else {
                    // append chunk to message
                    message += receivedChunks[i.toShort()]!!
                    AppLog.e(TAG, "assembleMessage chunk $i, size: ${receivedChunks[i.toShort()]!!.size}")
                    AppLog.e(TAG, "assembleMessage message size: ${message.size}")
                }
            }

            // check if message size matches
            AppLog.e(TAG, "assembleMessage: total message size: ${message.size}, expected size: $messageSize")
            if (message.size != messageSize) {
                AppLog.e(TAG, "GattMessaging assembleMessage message size does not match")
                error = true
            }
        }

        // check CRC32 checksum
        if (!error) {
            // calculate CRC
            val crc32 = CRC32()
            crc32.update(message)
            val crc32Value = crc32.value

            if (crc32Value != crc32Checksum) {
                AppLog.e(TAG, "GattMessaging createFinalMessage CRC32 checksum does not match")
                error = true
            } else {
                receivedQueueResult.receivedMessage = ReceivedMessage(
                    qaulId,
                    message
                )
            }
        }

        // Set queue state & create response
        if (error) {
            // on error
            // set queue state
            state = ReceiveQueueMessageState.ERROR
            // create FLC message with error
            receivedQueueResult.flcSendAck = FlcAck(
                messageQueueIndex,
                false,
                0
            )
        } else {
            // on success
            // set queue state
            state = ReceiveQueueMessageState.RECEIVED
            // create FLC message with success
            receivedQueueResult.flcSendAck = FlcAck(
                messageQueueIndex,
                true
            )
        }

        return receivedQueueResult
    }
}
