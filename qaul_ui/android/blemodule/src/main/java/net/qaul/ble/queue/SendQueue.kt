// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
 * This File contains the protocol logic of the qaul GATT Messaging send queue.
 * The classes are used in BleService.kt to manage the sending of messages:
 *
 * - chop the messages into chunks
 * - manage the sending queues
 *
 * There is a small 2 Byte message header for each chunk.
 * The 2 Bytes (16 bits) are packed as following:
 *
 * - 5 bits: message queue index
 * - 1 bit: resend indicator
 * - 10 bits: chunk index
 *
 * Each message is split into multiple chunks.
 * Large messages are split into multiple message parts.
 * Each message is sent in a separate queue.
 * After the queue was sent, the missing message chunks are requested
 * by the receiver and resent by the sender, setting the resend indicator bit to 1.
 */

package net.qaul.ble.test.ble.queue

import android.bluetooth.BluetoothDevice
import java.util.zip.CRC32
import java.util.LinkedList
import java.util.Queue
import kotlin.time.TimeSource
import kotlin.time.TimeSource.Monotonic.ValueTimeMark
import net.qaul.ble.test.ble.metrics.BleMetrics
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import net.qaul.ble.BleConstants


/**
 * A flush's chunks split across the three scheduler lanes:
 *  - flcChunks   → CONTROL: flow control (SEND_ID, ACK, ping, chunk requests), latency sensitive.
 *  - mediumChunks → MEDIUM: payload of short messages (routing updates, chat), kept ahead of files.
 *  - bulkChunks  → BULK: payload of large messages (images/files). Goes last.
 */
data class ChunkBatch(
    val flcChunks: List<ByteArray>,
    val mediumChunks: List<ByteArray>,
    val bulkChunks: List<ByteArray>
)

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
    var largeMessageQueues: MutableMap<Int, SendLargeMessageQueue> = mutableMapOf()
    var largeMessageIndex: Int = 0

    // queue for Flow Control Messages (FLC) to send
    // they have the first priority
    var flcToSend: Queue<ByteArray> = LinkedList()
    // map of missing chunks to request
    // they have the second sending priority
    var missingChunksToRequest: MutableMap<Int, Int> = mutableMapOf()
    // map of missing chunks to send
    // they have the third sending priority
    var missingChunksToSend: MutableMap<Int, Int> = mutableMapOf()
    // Queues of whole messages waiting to be chunked, split by size. getChunks drains
    // shortMessagesToSend before messagesToSend so a short message (routing/chat) reaches the MEDIUM
    // scheduler lane ahead of a large multi-part transfer. This mirrors the scheduler's lane priority one level up.
    var shortMessagesToSend: Queue<Triple<ByteArray, String, Byte>> = LinkedList()
    var messagesToSend: Queue<Triple<ByteArray, String, Byte>> = LinkedList()

    /**
     * Maximum bytes per large-message part, derived from the negotiated MTU (chunkSize).
     *
     * the first-chunk header stores messageSize as only 2 bytes (unsigned, 0–65535).
     * Any part larger than 65535 bytes would have its size field overflow and the
     * receiver would always report "message size does not match".
     *
     * Cap at 65535:
     *   MTU=20  → 18397 B (cap not reached, 1023 chunks)
     *   MTU=512 → = 518644 B (capped, ~130 chunks)
     */
    val maxPartSize: Int
        get() = minOf(
            (chunkSize - FIRST_CHUNK_HEADER_SIZE) + 1022 * (chunkSize - CHUNK_HEADER_SIZE),
            65535   // messageSize is stored as 2 unsigned bytes in the first-chunk header
        )

    // A map of all send queues
    // There are 29 queues (indexes: 1-29).
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
    @Synchronized
    fun addMessage(message: ByteArray, messageId: String) {
        if (message.size <= maxPartSize) {
            // add a normal (single part) message to the short or large queue by size, so short
            // messages are pulled ahead of a big transfer's parts (see getChunks).
            val queue = if (message.size <= BleConstants.MEDIUM_MESSAGE_MAX_BYTES) shortMessagesToSend else messagesToSend
            queue.add(Triple(message, messageId, 0x00))
        } else {
            // add a large message, split into parts sized for the current MTU
            val maxTransmittable = maxPartSize * 4
            if (message.size > maxTransmittable) {
                AppLog.e(TAG, "addMessage: WARNING — message (${message.size} B) exceeds the 4-part protocol limit " +
                    "(${maxTransmittable} B at chunkSize=$chunkSize). Only the first ${maxTransmittable} B will be sent; " +
                    "the remaining ${message.size - maxTransmittable} B will be silently dropped. " +
                    "Qaul messages should be ≤65 KB, split larger payloads at the libqaul layer.")
            }
            val currentIndex = getNextLargeMessageIndex()

            // create Large Message Parts
            var largeMessageQueue = SendLargeMessageQueue(currentIndex, messageId, message, maxPartSize)

            // schedule messages out of the parts
            for (i in 0 until largeMessageQueue.partsTotal) {
                // get message part
                val messagePart = largeMessageQueue.parts[i]

                // put message into the queue
                messagesToSend.add(Triple(messagePart!!.messageData, messageId, messagePart!!.largeMessageIndicator))
            }

            // save Large Message Queue
            largeMessageQueues[currentIndex] = largeMessageQueue
        }
    }

    /**
     * Get next Large Message Index
     */
    @Synchronized
    fun getNextLargeMessageIndex(): Int {
        largeMessageIndex += 1
        if (largeMessageIndex > 15) {
            largeMessageIndex = 1
        }
        return largeMessageIndex
    }

    /**
     * Schedule a new message for sending and create the 
     * message chunks.
     * @param message the message to send
     * @return a queue of message chunks
     */
    @Synchronized
    fun getChunks(): ChunkBatch {
        // Flow control chunks, small, latency sensitive control traffic. Routed through the priority
        // queue so they aren't starved behind a large message transfer.
        val flcChunks: MutableList<ByteArray> = LinkedList()
        // send qaul ID as first message
        if (!qaulIdSent) {
            flcChunks.add(FlcCreate.createSendId(qaulId))
            qaulIdSent = true
        }
        // add all FLC messages (ACKs, pings, ...)
        flcChunks.addAll(flcToSend)
        flcToSend.clear()
        // add all missing chunk requests (control)
        flcChunks.addAll(getMissingChunksRequestFlc())

        val mediumChunks: MutableList<ByteArray> = LinkedList()
        val bulkChunks: MutableList<ByteArray> = LinkedList()
        // chunks we were asked to resend, classified by their originating message's size
        getMissingChunksToSend(mediumChunks, bulkChunks)

        // create a new message chunk, short messages first, so they reach the MEDIUM lane ahead of a
        // large transfer's remaining parts sitting in messagesToSend.
        val nextQueue = if (shortMessagesToSend.isNotEmpty()) shortMessagesToSend else messagesToSend
        if (nextQueue.isNotEmpty()) {
            // get index
            val messageIndex = getNextMessageIndex()
            if (messageIndex == null) {
                AppLog.e(TAG, "getChunks: No message index available. Cannot create message.")
                return ChunkBatch(flcChunks, mediumChunks, bulkChunks)
            }

            // get the first message from the chosen queue
            val (message, messageId, largeMessageIndicator) = nextQueue.remove()
            val sendQueueMessage = SendQueueMessage(qaulId, message, messageId, messageIndex, chunkSize, largeMessageIndicator)
            sendQueueMessage.state = SendQueueMessageState.SENDING
            sendQueues[messageIndex] = sendQueueMessage
            BleMetrics.onMessageSendStarted(messageId, message.size)

            // change large message part state
            trackLargeMessages(largeMessageIndicator, SendLargeMessageState.SENDING)

            // route this message's chunks to the medium or bulk lane by its size
            val dest = if (message.size <= BleConstants.MEDIUM_MESSAGE_MAX_BYTES) mediumChunks else bulkChunks
            dest.addAll(sendQueueMessage.getAllChunks())
        }

        return ChunkBatch(flcChunks, mediumChunks, bulkChunks)
    }

    /**
     * schedule FLC send qaul ID message
     */
    @Synchronized
    fun addFlcSendQaulId() {
        // create FLC send qaul ID message
        val flcSendId = FlcCreate.createSendId(qaulId)
        // add to the FLC queue
        flcToSend.add(flcSendId)
    }

    /**
     * schedule a FLC ACK message
     */
    @Synchronized
    fun addFlcAck(queueIndex: Byte, success: Boolean, errorCode: Byte) {
        // create FLC ACK message
        val flcAck = FlcCreate.createAck(queueIndex, success, errorCode)
        // add to the FLC queue
        flcToSend.add(flcAck)
    }

    /**
     * schedule a FLC PING message
     */
    @Synchronized
    fun addFlcPing() {
        // create FLC ping message
        val flcPing = FlcCreate.createPing()
        // add to the FLC queue
        flcToSend.add(flcPing)
    }

    /**
     * add missing chunk index to request
     */
    @Synchronized
    fun addMissingChunkIndexToRequest(missingChunkIndex: Int) {
        // add missing chunk to the map
        missingChunksToRequest[missingChunkIndex] = missingChunkIndex
    }

    /**
     * Get FLCs with Indexes of Missing Chunks to Request
     * 
     * Create FLC request for missing chunks messages
     * out of the missingChunksToRequest map.
     */
    private fun getMissingChunksRequestFlc(): Queue<ByteArray> {
        var flcRequestChunksQueue: Queue<ByteArray> = LinkedList()

        var chunkIndexes: MutableList<Int> = mutableListOf()
        missingChunksToRequest.forEach { (key, value) ->
            if (chunkIndexes.size >= 9) {
                // create FLC request for missing chunks message
                val flcRequestMissingChunks = FlcCreate.createRequestChunks(chunkIndexes)
                // add to the missing chunks queue
                flcRequestChunksQueue.add(flcRequestMissingChunks)

                // clear chunk indexes
                chunkIndexes.clear()
            }
            chunkIndexes.add(key)
        }
        missingChunksToRequest.clear()

        if (chunkIndexes.isNotEmpty()) {
            // create FLC request for missing chunks message
            val flcRequestMissingChunks = FlcCreate.createRequestChunks(chunkIndexes)
            // add to the missing chunks queue
            flcRequestChunksQueue.add(flcRequestMissingChunks)

            // clear chunk indexes
            chunkIndexes.clear()
        }

        return flcRequestChunksQueue
    }

    /**
     * add missing chunk index to send
     */
    @Synchronized
    fun addMissingChunkIndexToSend(missingChunkIndex: Int) {
        // add missing chunk to the map
        missingChunksToSend[missingChunkIndex] = missingChunkIndex
    }

    /**
     * Get Missing Chunks to Send
     *
     * Create missing chunk (resend) messages out of the missingChunksToSend map, routing each into
     * [mediumChunks] or [bulkChunks] by the size of the message it belongs to (so a large file's
     * resends stay in the bulk lane and a short message's resends stay ahead of files).
     */
    private fun getMissingChunksToSend(mediumChunks: MutableList<ByteArray>, bulkChunks: MutableList<ByteArray>) {
        missingChunksToSend.forEach { (key, value) ->
            // analyze request
            val queueIndex: Byte = (key shr 11).toByte()
            val chunkIndex: Int = key and 0x3FF
            // get missing chunk from the send queue
            val sendQueueMessage = sendQueues[queueIndex]
            if (sendQueueMessage == null) {
                AppLog.e(TAG, "getMissingChunksToSend: SendQueueMessage queue $queueIndex not found.")
                return@forEach
            } else if (sendQueueMessage.state == SendQueueMessageState.SENDING ||
                       sendQueueMessage.state == SendQueueMessageState.SENT    ||
                       sendQueueMessage.state == SendQueueMessageState.MISSING) {
                // Transition to MISSING so getHeader() sets resend indicator = 1 in the chunk
                // header. Without this the receiver sees resend=0 and rejects the chunk. TODO: Look back at states, specifically the SENT state
                sendQueueMessage.state = SendQueueMessageState.MISSING
                AppLog.e(TAG, "getMissingChunksToSend: resending chunk $chunkIndex for queue $queueIndex")
                val chunk = sendQueueMessage.getChunk(chunkIndex.toShort())
                val dest = if (sendQueueMessage.messageSize <= BleConstants.MEDIUM_MESSAGE_MAX_BYTES) mediumChunks else bulkChunks
                dest.add(chunk)
            } else {
                AppLog.e(TAG, "getMissingChunksToSend: queue $queueIndex in state ${sendQueueMessage.state}, cannot resend chunk $chunkIndex")
                return@forEach
            }
        }
        missingChunksToSend.clear()
    }

    /**
     * get the next message queue index
     *
     * This sets the new index to free index
     */
    private fun getNextMessageIndex(): Byte? {
        if (currentIndex == 0.toByte()) {
            currentIndex = 1.toByte() // start with index 1
            return currentIndex
        } else {
            var newIndex: Byte = (currentIndex + 1).toByte()
            if (newIndex > 29.toByte()) {
                // reset to 1 if index is greater than 29
                currentIndex = 1.toByte()
            } else {
                currentIndex = newIndex
            }

            // clear the send queue for this index
            if (sendQueues.containsKey(currentIndex)) {
                var sendQueueMessage = sendQueues[currentIndex]
                if (sendQueueMessage != null) {
                    if (sendQueueMessage.state != SendQueueMessageState.SUCCESS &&
                        sendQueueMessage.state != SendQueueMessageState.ERROR) {
                        AppLog.e(TAG, "getNextMessageIndex: clear queue $currentIndex with message ID ${sendQueueMessage.messageId} in state ${sendQueueMessage.state}")

                        // send message sending error to Libqaul


                        // set queue to empty
                        sendQueueMessage.state = SendQueueMessageState.EMPTY
                        sendQueueMessage.messageId = ""
                        sendQueueMessage.message = ByteArray(0)
                    }
                }
            }

            // check state of the send queue
            return currentIndex
        }
    }

    /**
     * FLC ACK received
     * This method is called when an ACK for a Flow Control Message (FLC) is received.
     * @param queueIndex the index of the queue (1-29)
     * @param success true if the ACK was successful, false if there was an error
     * @param errorCode the error code, if any
     * @return String indicating the message ID. Return empty string, if queue index is invalid or message is not found.
     */
    @Synchronized
    fun flcAckReceived(queueIndex: Byte, success: Boolean, errorCode: Byte): String {
        // check if queue index is valid
        if (queueIndex < 1 || queueIndex > 29) {
            AppLog.e(TAG, "GattMessaging FlcAckReceived: Invalid queue index $queueIndex. Must be between 1 and 29.")
            return ""
        }

        AppLog.e(TAG, "GattMessaging FlcAckReceived: queueIndex: $queueIndex, success: $success, errorCode: $errorCode")

        // check if message is in the send queue
        val sendQueueMessage = sendQueues[queueIndex]
        if (sendQueueMessage == null) {
            AppLog.e(TAG, "FlcAckReceived: Message with queue index $queueIndex not found in send queue.")
            return ""
        }

        var result = ""
        if (sendQueueMessage.largeMessageIndicator != 0x00.toByte()) {
            AppLog.e(TAG, "FlcAckReceived: Large Message ACK received for queue index $queueIndex.")
            if (success) {
                val largeMessageQueue = trackLargeMessages(sendQueueMessage.largeMessageIndicator, SendLargeMessageState.SUCCESS)
                if(largeMessageQueue != null && largeMessageQueue!!.partsSent >= largeMessageQueue!!.partsTotal) {
                    AppLog.e(TAG, "FlcAckReceived: Large Message with ID ${sendQueueMessage.messageId} fully sent successfully.")
                    result = largeMessageQueue!!.messageId
                    // delete large message queue
                    val largeMessageQueueIndex: Int = (sendQueueMessage.largeMessageIndicator.toInt() shr 4) and 0x0F
                    largeMessageQueues.remove(largeMessageQueueIndex)
                }
            } else {
                // reschedule message
                AppLog.e(TAG, "FlcAckReceived: Large Message with ID ${sendQueueMessage.messageId} sending error.")
                trackLargeMessages(sendQueueMessage.largeMessageIndicator, SendLargeMessageState.QUEUED)

                // put message into the queue
                messagesToSend.add(Triple(sendQueueMessage.message, sendQueueMessage.messageId, sendQueueMessage.largeMessageIndicator))
            }
            sendQueueMessage.ackReceived(success, errorCode)
        }
        else {
            result = sendQueueMessage.ackReceived(success, errorCode)
        }
        // remove message from send queue messages
        sendQueues.remove(queueIndex)

        if (result.isNotEmpty()) BleMetrics.onMessageAcknowledged(result)
        return result
    }

    /**
     * This Message chunk queue was sent
     *
     * TODO: would be better to use the queue index instead of the message ID
     * TODO: I believe this message ID is not really used in BleService, only on failure
     * TODO: large messages are misrepresented by a single message ID
     */
    @Synchronized
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
     * Connection lost. Mark every message handed to this queue but not yet  resolved
     *  as failed, and return their distinct ids so the caller can surface a real
     * failure result to libqaul. Covers messages mid-send (in [sendQueues], awaiting ACK) and messages still queued but not yet chunked ([messagesToSend]).
     *
     */
    @Synchronized
    fun failAllPending(): Set<String> {
        state = SendQueueState.CONNECTION_LOST
        val failed = LinkedHashSet<String>()
        // messages sent but not yet acked
        sendQueues.values.forEach { m ->
            if (m.state != SendQueueMessageState.SUCCESS &&
                m.state != SendQueueMessageState.ERROR &&
                m.state != SendQueueMessageState.EMPTY &&
                m.messageId.isNotEmpty()) {
                m.state = SendQueueMessageState.ERROR
                failed.add(m.messageId)
            }
        }
        // messages queued but not yet chunked (both lanes)
        shortMessagesToSend.forEach { if (it.second.isNotEmpty()) failed.add(it.second) }
        messagesToSend.forEach { if (it.second.isNotEmpty()) failed.add(it.second) }
        shortMessagesToSend.clear()
        messagesToSend.clear()
        return failed
    }

    /**
     * Track large Messages
     */
    @Synchronized
    fun trackLargeMessages(largeMessageIndicator: Byte, state: SendLargeMessageState): SendLargeMessageQueue? {
        // get queue index from largeMessageIndicator Byte
        val queueIndex: Int = (largeMessageIndicator.toInt() shr 4) and 0x0F
        if(queueIndex <= 0) {
            return null
        }

        // get part number from largeMessageIndicator Byte
        val largeMessagePart = (largeMessageIndicator.toInt() and 0x03)
        AppLog.e(TAG, "trackLargeMessages: Queue: $queueIndex, Part: $largeMessagePart, State: $state")

        // get large message queue
        val largeMessageQueue = largeMessageQueues[queueIndex]
        if (largeMessageQueue == null) {
            AppLog.e(TAG, "trackLargeMessages: Large Message Queue $queueIndex not found.")
            return null
        }

        // change state of the part
        largeMessageQueue.changeStateOfPart(largeMessagePart, state)

        // saved changed queue
        largeMessageQueues[queueIndex] = largeMessageQueue

        return largeMessageQueue
    }
}

/**
 * Large Message Constants
 */
// Largest part of a message in Bytes.
// Larger messages are split into multiple message parts.
// This gives a maximum of 4 parts.
const val MAX_MESSAGE_PART_SIZE = 18342

/**
 * Large Message State
 */
enum class SendLargeMessageState {
    QUEUED,
    SENDING,
    SUCCESS,
    ERROR
}

/**
 * Large Message Part
 */
class SendLargeMessagePart {
    val TAG: String = "SendLargeMessagePart"
    var partIndex: Int = 0
    var state: SendLargeMessageState = SendLargeMessageState.QUEUED
    var largeMessageIndicator: Byte = 0x00
    var messageData: ByteArray = ByteArray(0)

    /**
     * Create a new LargeMessagePart
     * @param partIndex the index of the part
     */
    constructor(partIndex: Int, messageData: ByteArray, largeMessageIndicator: Byte) {
        this.partIndex = partIndex
        this.messageData = messageData
        this.largeMessageIndicator = largeMessageIndicator
    }
}

/**
 * Large Message Queue
 * 
 * Large messages are messages that are larger than MAX_MESSAGE_PART_SIZE (18342 Bytes).
 * These messages are split into multiple parts.
 * The SendLargeMessageQueue class is used to keep track of the parts.
 * 
 * @param largeMessageQueueIndex the index of the large message queue
 * @param messageId the message ID
 * @param message the message to send
 */
class SendLargeMessageQueue {
    val TAG: String = "SendLargeMessageQueue"
    var index: Int = 0
    var partsTotal: Int = 0 // 0 = 1
    var partsSent: Int = 0 // 0 = none sent
    var parts: MutableMap<Int, SendLargeMessagePart> = mutableMapOf()
    var messageId: String = ""
    var messageSize: Int = 0

    /**
     * Create a new LargeMessageQueue
     * @param largeMessageQueueIndex the rotating large-message queue slot (1–15)
     * @param messageId the message ID
     * @param message the full message bytes to split into parts
     * @param maxPartSize maximum bytes per part, defaults to MAX_MESSAGE_PART_SIZE (for MTU=20)
     *                    but callers should pass SendQueue.maxPartSize to match the negotiated MTU.
     */
    constructor(largeMessageQueueIndex: Int, messageId: String, message: ByteArray, maxPartSize: Int = MAX_MESSAGE_PART_SIZE) {
        this.index = largeMessageQueueIndex
        this.messageId = messageId
        this.messageSize = message.size

        // calculate total parts
        this.partsTotal = Math.ceil(messageSize.toDouble() / maxPartSize).toInt().coerceAtMost(4)

        // fill in parts
        for (i in 0 until partsTotal) {
            // get message part
            val messagePart = message.sliceArray(i * maxPartSize until Math.min((i + 1) * maxPartSize, messageSize))
            val headerIndicator: Byte = createLargeMessageHeaderIndicator(index, partsTotal, i)

            // save part information
            val part = SendLargeMessagePart(i, messagePart, headerIndicator)
            parts[i] = part
        }
    }

    /**
     * Create Large Message Header Indicator
     */
    fun createLargeMessageHeaderIndicator(queue: Int, partsTotal: Int, part: Int): Byte {
        val queueIndex: Int = queue shl 4
        val partsTotalIndex: Int = partsTotal shl 2
        val indicator: Int = queueIndex or partsTotalIndex or (part and 0x03)
        return (indicator and 0xFF).toByte()
    }

    /**
     * Change state of a part
     */
    fun changeStateOfPart(partIndex: Int, state: SendLargeMessageState) {
        val part = parts[partIndex]
        if (part == null) {
            AppLog.e(TAG, "changeStateOfPart: Part $partIndex not found in Large Message Queue $index.")
        }
        
        // update state
        part!!.state = state
        // save state
        parts[partIndex] = part

        // update parts sent count
        if (state == SendLargeMessageState.SUCCESS) {
            partsSent += 1
        }

        // DEBUG: this is only for debugging and can be removed
        if (partsSent >= partsTotal) {
            AppLog.e(TAG, "Large Message Queue $index: All parts sent successfully.")
        }
    }
}

/**
 * message sending state
 */
enum class SendQueueMessageState {
    EMPTY,   // queue is waiting to receive new messages
    QUEUED,  // message is queued for sending
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
    var largeMessageIndicator: Byte = 0x00
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
    constructor(qaulId: ByteArray, message: ByteArray, messageId: String, messageIndex: Byte, chunkSize: Int, largeMessage: Byte) {
        this.qaulId = qaulId
        this.message = message
        this.messageId = messageId
        this.chunkSize = chunkSize
        this.messageSize = message.size
        this.messageIndex = messageIndex
        this.largeMessageIndicator = largeMessage
        this.totalChunks = getChunkCount()
    }

    /**
     * create all chunks for this message
     * @return a queue of message chunks
     */
    fun getAllChunks(): Queue<ByteArray> {
        var chunks: Queue<ByteArray> = LinkedList()

        // DEBUG
        //AppLog.e(TAG, "getAllChunks: message: ${BLEUtils.toBinaryString(message)}")
        //AppLog.e(TAG, "getAllChunks: messageSize: $messageSize")
        //AppLog.e(TAG, "getAllChunks: $totalChunks chunks:")

        // create all chunks
        for (i in 0..(totalChunks -1)) {
            //if (i == 5) continue  // DEBUG: simulate a lost chunk
            val chunk = getChunk(i.toShort())

            // DEBUG
            //AppLog.e(TAG, "$i: ${BLEUtils.toBinaryString(chunk)}")

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
        // Valid states for receiving an ACK:
        //   SENDING  — most common, chunks were enqueued but physical-send confirmation hasn't arrived
        //              (messageSent() is not called in the current BleConnection implementation)
        //   SENT     — all chunks confirmed written at the link layer
        //   MISSING  — some chunks were re-requested; still waiting for the full round-trip
        if (state != SendQueueMessageState.SENDING &&
            state != SendQueueMessageState.SENT &&
            state != SendQueueMessageState.MISSING) {
            AppLog.e(TAG, "ACK received in unexpected state: $state — ignoring")
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
     * Get the message chunk header
     *
     * The header is 2 Bytes long.
     *
     * @param chunkIndex the index of the chunk (0-based)
     * @return the header
     */
    private fun getHeader(chunkIndex: Short): ByteArray {
        val resendIndicator: Int = if (state == SendQueueMessageState.SENDING || state == SendQueueMessageState.QUEUED) 0 else 1
        val headerInt: Int = (messageIndex.toInt() and 0xFF shl 11) or (resendIndicator and 0xF shl 10) or (chunkIndex.toInt() and 0xFFF)
        val header: ByteArray = ByteArray(2)
        header[0] = (headerInt shr 8).toByte() // high byte
        header[1] = (headerInt and 0xFF).toByte() // low byte
        return header
    }

    /**
     * Get the first chunk header
     * Header size is 19 Bytes and contains:
     *
     * - 2 Bytes Chunk Header
     * - 1 Byte Large Message Part
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
        val firstHeader1: ByteArray = header + largeMessageIndicator
        val firstHeader2: ByteArray = firstHeader1 + headerMessageSize
        val firstHeader3: ByteArray = firstHeader2 + headerTotalChunks
        val firstHeader4: ByteArray = firstHeader3 + crc32Bytes
        val firstHeader: ByteArray = firstHeader4 + qaulId

        //AppLog.e(TAG, "getFirstHeader: ${firstHeader.size} Bytes: ${BLEUtils.toBinaryString(firstHeader)}")

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

