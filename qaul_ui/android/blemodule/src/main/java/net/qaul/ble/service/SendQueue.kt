// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
 * This File contains the protocol logic of the qaul GATT Messaging send queue.
 * The classes are used in BleService.kt to manage the sending of messages:
 *
 * - chop the messages into chunks
 * - manage the sending queues
 */

package net.qaul.ble.service

import android.bluetooth.BluetoothDevice
import java.util.zip.CRC32
import java.util.LinkedList
import java.util.Queue
import kotlin.time.TimeSource
import kotlin.time.TimeSource.Monotonic.ValueTimeMark
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils


/**
 * send queue state
 */
enum class SendQueueState {
    NO_QAUL_ID,
    NEW,
    OK,
    CONNECTION_LOST
}

/**
 * SendQueue for BLE messages
 * 
 * Each discovered receiving device has a SendQueue, 
 * which tracks the sending and creates the message chunks,
 * according to the qaul BLE GATT messaging protocol.
 */
class SendQueue(qaulId: ByteArray) {
    val TAG: String = "SendQueue"
    val qaulId = qaulId
    var qaulIdSent: Boolean = false
    var qaulIdKnown: Boolean = false
    var sendId: Boolean = true
    var chunkSize: Int = 20
    var currentIndex: Byte = 0
    var currentMessageId: String = ""
    var state: SendQueueState = SendQueueState.NEW

    // queue for Flow Control Messages (FLC) to send
    // they have the first priority
    var flcToSend: Queue<ByteArray> = LinkedList()
    // queue of requested missing chunks
    // they have the second sending priority
    var missingChunksToSend: Queue<ByteArray> = LinkedList()
    // queue of messages to send
    // they have the third sending priority
    var messagesToSend: Queue<Pair<ByteArray, String>> = LinkedList()

    // A map of all send queues. 
    // There are 14 queues (indexes: 1-14).
    // The first messages starts with index 1, once the messages is sent
    // the index is incremented by 1.
    var sendQueues: MutableMap<Byte, SendQueueMessage> = mutableMapOf()

    // A list of all missing chunks that are requested
    // by the receiving device.
    var missing: MutableList<ByteArray> = mutableListOf()

    /**
     * adds a new Message to the sending queue
     * @param message the message to add
     */
    fun addMessage(message: ByteArray, messageId: String) {
        messagesToSend.add(Pair(message, messageId))
    }

    /**
     * Schedule a new message for sending and create the 
     * message chunks.
     * @param message the message to send
     * @return a queue of message chunks
     */
    fun getChunks(): Triple<Queue<ByteArray>, Byte?, String> {
        var chunks: Queue<ByteArray> = LinkedList()
        // send qaul ID as first message
        if (!qaulIdSent) {
            chunks.add(FlcCreate.createSendId(qaulId))
            qaulIdSent = true
        }
        // add all FLC messages to the queue
        chunks.addAll(flcToSend)
        // add all missing chunks to the queue
        chunks.addAll(missingChunksToSend)
        // TODO: check if queues are emptied

        // create a new message chunk
        if (messagesToSend.isNotEmpty()) {
            // get index
            val messageIndex = getNextMessageIndex()
            if (messageIndex == null) {
                AppLog.e(TAG, "getChunks: No message index available. Cannot create message.")
                return Triple(chunks, null, "")
            }

            // get the first message from the queue
            val (message, messageId) = messagesToSend.remove()
            val sendQueueMessage = SendQueueMessage(qaulId, message, messageId, messageIndex, 20)
            sendQueueMessage.state = SendQueueMessageState.SENDING
            sendQueues.put(messageIndex, sendQueueMessage)

            // add the message chunks to the queue
            chunks.addAll(sendQueueMessage.getAllChunks())

            // DEBUG
            AppLog.e(TAG, "getChunks: Total chunks in queue: ${chunks.size}")


            return Triple(chunks, messageIndex, messageId)
        }

        return Triple(chunks, null, "")
    }

    /**
     * schedule a FLC ACK message
     */
    fun addFlcAck(queueIndex: Byte, success: Boolean, errorCode: Byte) {
        // create FLC ACK message
        val flcAck = FlcCreate.createAck(queueIndex, success, errorCode)
        // add to the FLC queue
        flcToSend.add(flcAck)
    }

    /**
     * get the next message queue index
     *
     * TODO: implement the new queue solution:
     * - set new index to free index
     * - set next two index also to free index
     */
    private fun getNextMessageIndex(): Byte? {
        if (currentIndex == 0.toByte()) {
            currentIndex = 1.toByte() // start with index 1
            return currentIndex
        } else {
            var newIndex: Byte = (currentIndex + 1).toByte()
            if (newIndex > 14.toByte()) {
                // reset to 1 if index is greater than 14
                currentIndex = 1.toByte()
            } else {
                currentIndex = newIndex
            }

            // check state of the send queue
            if (sendQueues.containsKey(currentIndex)) {
                var sendQueueMessage = sendQueues[currentIndex]
                if (sendQueueMessage != null) {
                    if (sendQueueMessage.state == SendQueueMessageState.SUCCESS || 
                        sendQueueMessage.state == SendQueueMessageState.ERROR) {

                    } else {
                        // TODO: find better solution
                        AppLog.e(TAG, "GattMessaging getNextMessageIndex: Message with index $currentIndex is still in state ${sendQueueMessage.state}. Cannot increment index.")
                        return null // error
                    }
                }
            }
            return currentIndex
        }
    }

    /**
     * FLC ACK received
     * This method is called when an ACK for a Flow Control Message (FLC) is received.
     * @param queueIndex the index of the queue (1-14)
     * @param success true if the ACK was successful, false if there was an error
     * @param errorCode the error code, if any
     * @return String indicating the message ID. Return empty string, if queue index is invalid or message is not found.
     */
    fun flcAckReceived(queueIndex: Byte, success: Boolean, errorCode: Byte): String {
        // check if queue index is valid
        if (queueIndex < 1 || queueIndex > 14) {
            AppLog.e(TAG, "GattMessaging FlcAckReceived: Invalid queue index $queueIndex. Must be between 1 and 14.")
            return ""
        }

        AppLog.e(TAG, "GattMessaging FlcAckReceived: queueIndex: $queueIndex, success: $success, errorCode: $errorCode")

        // check if message is in the send queue
        val sendQueueMessage = sendQueues[queueIndex]
        if (sendQueueMessage == null) {
            AppLog.e(TAG, "FlcAckReceived: Message with queue index $queueIndex not found in send queue.")
            return ""
        }

        val result = sendQueueMessage.ackReceived(success, errorCode)
        sendQueues[queueIndex] = sendQueueMessage

        return result
    }

    /**
     * This Message chunk queue was sent
     *
     * TODO: would be better to use the queue index instead of the message ID
     */
    fun messageSent(messageId: String) {
        // check if message is current message
        if (currentMessageId == messageId) {
            currentMessageId = ""
        }

        // check if message is in the send queue
        for ((index, sendQueueMessage) in sendQueues) {
            if (sendQueueMessage.messageId == messageId) {
                // set state to SENT
                sendQueueMessage.setSent()
                return
            }
        }

        AppLog.e(TAG, "GattMessaging messageSent: Message with ID $messageId not found in send queue.")
    }

    /**
     * set connection lost
     * This method is called when the connection to the device is lost.
     * @return message ID that could not be sent
     */
    fun setConnectionLost(): String? {
        state = SendQueueState.CONNECTION_LOST
        // check state of the send queues
        var sendQueueMessage = sendQueues.get(currentIndex)
        if (sendQueueMessage != null &&
            (sendQueueMessage.state == SendQueueMessageState.SENDING || 
             sendQueueMessage.state == SendQueueMessageState.QUEUED)) {
                // set state to ERROR
                sendQueueMessage.state = SendQueueMessageState.ERROR
                return sendQueueMessage.messageId
        }
        else if (messagesToSend.isNotEmpty()) {
            // get the first message ID that could not be sent
            val message = messagesToSend.poll()
            val messageId = message.second
            return messageId
        } else {
            return null
        }
        //flcToSend.clear()
        //missingChunksToSend.clear()
        //messagesToSend.clear()
        //sendQueues.clear()
    }
}

/**
 * message sending state
 */
enum class SendQueueMessageState {
    QUEUED,
    SENDING, // the message is currently being sent
    SENT,    // the message was sent
    MISSING, // some chunks are missing
    SUCCESS, // message was successfully received
    ERROR    // an error occurred while sending the message
}

/**
 * SendQueueMessage represents a single message in the send queue.
 * It contains the message data and the messages sending state.
 */
class SendQueueMessage {
    val TAG: String = "SendQueueMessage"
    var qaulId: ByteArray = ByteArray(0)
    var messageId: String = ""
    var messageIndex: Byte = 0
    var state: SendQueueMessageState = SendQueueMessageState.QUEUED

    var message: ByteArray = ByteArray(0)
    var messageSize: Int = 0

    var totalChunks: Short = 0
    var chunkSize: Int = 20 // depends on GATT MTU size

    var createdAt: ValueTimeMark = TimeSource.Monotonic.markNow()
    var updatedAt: ValueTimeMark = TimeSource.Monotonic.markNow()

    /**
     * Create a new SendQueueMessage
     * @param message the message to send
     * @param chunkSize the size of each chunk
     */
    constructor(qaulId: ByteArray, message: ByteArray, messageId: String, messageIndex: Byte, chunkSize: Int) {
        this.qaulId = qaulId
        this.message = message
        this.messageId = messageId
        this.chunkSize = chunkSize
        this.messageSize = message.size
        this.messageIndex = messageIndex
        this.totalChunks = getChunkCount()
    }

    /**
     * create all chunks for this message
     * @return a queue of message chunks
     */
    fun getAllChunks(): Queue<ByteArray> {
        var chunks: Queue<ByteArray> = LinkedList()

        // DEBUG
        AppLog.e(TAG, "getAllChunks: message: ${BLEUtils.toBinaryString(message)}")
        AppLog.e(TAG, "getAllChunks: messageSize: $messageSize")
        AppLog.e(TAG, "getAllChunks: $totalChunks chunks:")

        // create all chunks
        for (i in 0..(totalChunks -1)) {
            val chunk = getChunk(i.toShort())

            // DEBUG
            AppLog.e(TAG, "$i: ${BLEUtils.toBinaryString(chunk)}")

            chunks.add(chunk)
        }

        return chunks
    }

    /**
     * Get specific chunk of the message
     * @param chunkIndex the index of the chunk (0-based)
     * @return the chunk as ByteArray
     */
    fun getChunk(chunkIndex: Short): ByteArray {
        var header = getHeader(chunkIndex)
        if (chunkIndex == 0.toShort()) {
            header = getFirstHeader()
        }

        val payload = getPayload(chunkIndex)
        val chunk = header + payload
        return chunk
    }

    /**
     * Message was sent, set the state to SENT
     */
    fun setSent() {
        state = SendQueueMessageState.SENT
        updatedAt = TimeSource.Monotonic.markNow()
    }

    /**
     * ACK received for this message chunk
     * @param success true if the ACK was successful, false if there was an error
     * @param errorCode the error code, if any
     * @return String indicating the message ID. Return empty string, if state is invalid.
     */
    fun ackReceived(success: Boolean, errorCode: Byte): String {
        if (state == SendQueueMessageState.SENDING || state == SendQueueMessageState.QUEUED) {
            AppLog.e(TAG, "ACK received in invalid state: $state")
            return ""
        }

        if (success) {
            state = SendQueueMessageState.SUCCESS
        } else {
            state = SendQueueMessageState.ERROR
            AppLog.e(TAG, "ACK received with error code: $errorCode")
        }
        updatedAt = TimeSource.Monotonic.markNow()

        return messageId
    }

    /**
     * Get the number of message chunks for this message
     * @return the number of chunks
     */
    private fun getChunkCount(): Short {
        var count = Math.ceil((messageSize - (chunkSize - FIRST_CHUNK_HEADER_SIZE)).toDouble() / (chunkSize - CHUNK_HEADER_SIZE)) +1
        return count.toInt().toShort()
    }

    /**
     * Get the message header
     * @param queueIndex the index of the queue (1-14)
     * @param chunkIndex the index of the chunk (0-based)
     * @return the header
     */
    private fun getHeader(chunkIndex: Short): ByteArray {   
        val headerInt: Int = (messageIndex.toInt() and 0xFF shl 12) or (chunkIndex.toInt() and 0xFFF)
        val header: ByteArray = ByteArray(2)
        header[0] = (headerInt shr 8).toByte() // high byte
        header[1] = (headerInt and 0xFF).toByte() // low byte
        return header
    }

    /**
     * Get the first chunk header
     * Header size is 18 Bytes and contains:
     *
     * - 2 Bytes Chunk Header
     * - 2 Bytes message size
     * - 2 Bytes total chunks
     * - 4 Bytes CRC32 
     * - 8 Bytes qaulId.
     *
     * @return the first message header
     */
    private fun getFirstHeader(): ByteArray {
        // calculate CRC
        val crc32 = CRC32()
        crc32.update(message)
        val crc32Value = crc32.value

        val header = getHeader(0)
        val headerMessageSize = BLEUtils.toByteArray(messageSize).sliceArray(2..3)
        val headerTotalChunks = BLEUtils.toByteArray(totalChunks)
        val crc32Bytes = BLEUtils.crc32ValueToByteArray(crc32Value)
        val firstHeader: ByteArray = header + headerMessageSize + headerTotalChunks + crc32Bytes + qaulId

        return firstHeader
    }

    /**
     * Get the message chunk payload
     * @param index the index of the chunk
     * @return the message chunk
     */
    fun getPayload(index: Short): ByteArray {
        var start: Int = 0
        var end: Int = 0
        if (index == 0.toShort()) {
            end = Math.min(chunkSize - FIRST_CHUNK_HEADER_SIZE, messageSize)
        } else {
            val payloadSize = chunkSize - CHUNK_HEADER_SIZE
            start = (chunkSize - FIRST_CHUNK_HEADER_SIZE) + (index - 1) * payloadSize
            end = Math.min(start + payloadSize, messageSize)
        }

        return message.sliceArray(start until end)
    }
}

