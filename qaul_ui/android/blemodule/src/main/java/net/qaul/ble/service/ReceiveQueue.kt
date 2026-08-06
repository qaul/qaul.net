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
class FlcAck(val messageIndex: Byte, val success: Boolean = false, val errorCode: Byte = 0)
data class ReceivedMessage(val qaulId: ByteArray, val message: ByteArray, val largeMessageIndicator: Byte)

/**
 * Return object of ReceiveQueue
 */
class ReceiveQueueResult {
    var bluetoothAddress: String = ""
    var qaulIdMissing: Boolean = false
    var qaulIdRequestReceived: Boolean = false
    var qaulIdReceived: ByteArray? = null
    var flcRequestChunks: MutableList<Int> = mutableListOf()
    var flcChunksRequested: MutableList<Int> = mutableListOf()
    var flcSendAck: FlcAck? = null
    var flcRequestAck: Byte? = null
    var flcAckReceived: FlcAck? = null
    var receivedMessage: ReceivedMessage? = null
}

/**
 * Message Structure with Analyzed Header
 */
class ReceiveMessageStructure (
    val queueIndex: Byte, // message queue index & message type
    val chunkIndex: Short,
    val resendIndicator: Boolean,
    val payload: ByteArray,
)

/**
 * Qaul GATT Messaging is a service class that handles the qaul messages
 * that are sent in chunks as GATT messages.
 */
class ReceiveQueue {
    private val TAG: String = "ReceiveQueue"

    var qaulId: ByteArray = ByteArray(8)
    var qaulIdKnown: Boolean = false
    // TO DELETE:
    //var incoming: MutableList<ByteArray> = mutableListOf()

    // Current Message Queue Index
    // there are 29 message receive queues, which are rotating.
    var currentMessageQueueIndex: Byte = 0

    // Last Update
    var lastUpdate: ValueTimeMark = TimeSource.Monotonic.markNow()

    // Receive Message Queues
    // each queue is identified by the message queue index
    var receiveQueues: MutableMap<Byte, ReceiveQueueMessage> = mutableMapOf()

    // Large Messages
    var largeMessageQueues: MutableMap<Int, ReceiveLargeMessage> = mutableMapOf()

    /**
     * Analyze an incoming message
     */
    fun incomingMessage(chunk: ByteArray, device: BluetoothDevice): ReceiveQueueResult {
        var binaryString = BLEUtils.toBinaryString(chunk)
        //AppLog.e(TAG, "GattMessaging incomingMessage: chunk: $binaryString")

        // analyze message header
        //val (type, index, payload) = messageHeader(chunk)
        val receiveMessageStructure = messageHeader(chunk)
        var receiveQueueResult: ReceiveQueueResult;

        // display message Type
        //AppLog.e(TAG, "GattMessaging incoming Message type / Queue Index: $receiveMessageStructure.queueIndex, Chunk index: ${receiveMessageStructure.chunkIndex}, resend indicator: ${receiveMessageStructure.resendIndicator}, payload size: ${receiveMessageStructure.payload.size}")

        // check if message is a flow control message
        if (receiveMessageStructure.queueIndex == 0x00.toByte()) {
            // Flow Control Message
            AppLog.e(TAG, "incomingMessage FLC message")
            val firstByte: Byte = chunk.get(0)
            receiveQueueResult = incomingFlowControlMessage(firstByte, receiveMessageStructure.payload)
        } else {
            // Chunk Content Message
            //AppLog.e(TAG, "GattMessaging incomingMessage chunk content message")
            receiveQueueResult = incomingMessageChunk(receiveMessageStructure)

            // check if we received a large message part
            if (receiveQueueResult.receivedMessage != null && 
                receiveQueueResult.receivedMessage!!.largeMessageIndicator != 0x00.toByte()) {

                val receivedMessage: ReceivedMessage? = addLargeMessagePart(receiveQueueResult.receivedMessage!!)
                receiveQueueResult.receivedMessage = receivedMessage
            }
        }

        // set bluetooth address
        val bleAddress: String? = device.getAddress()
        if (bleAddress != null) {
            receiveQueueResult.bluetoothAddress = bleAddress!!
        }

        // set qaul ID if known
        if (qaulIdKnown) {
            receiveQueueResult.qaulIdMissing = false
            receiveQueueResult.qaulIdReceived = qaulId
        } else {
            receiveQueueResult.qaulIdMissing = true
        }
    
        return receiveQueueResult
    }

    /**
     * Analyze message header
     *
     * @param chunk ByteArray
     * @return ReceiveMessageStructure
     */
    private fun messageHeader(chunk: ByteArray): ReceiveMessageStructure {
        var binaryString = BLEUtils.toBinaryString(chunk)

        // get message Type
        val b1: Byte = chunk.get(0)
        val type: Byte = (b1.toInt() and 0xFF shr 3).toByte()
        binaryString = BLEUtils.toBinaryString(type)

        // get more header information
        var chunkIndex: Short;
        var resendIndicator: Boolean = false;
        var payload: ByteArray = ByteArray(0)

        // check if message is a flow control message;
        if (type == 0x00.toByte()) {
            // Flow Control Message

            // get FLC message index
            chunkIndex = (b1.toInt() and 0x0F).toShort()

            // get message payload
            payload = chunk.sliceArray(1 until chunk.size)

            AppLog.e(TAG, "message Header: FLC message received: FLC header: $type, FLC type: $chunkIndex}")
        } else {
            // chunk content message

            // get resend indicator
            val resendIndicatorValue = ((b1.toInt() shr 2) and 0x01)
            if (resendIndicatorValue == 1) {
                resendIndicator = true
            }

            // get message index
            val b2: Byte = chunk.get(1)
            chunkIndex = ((b1.toInt() and 0x03 shl 8) + b2).toShort()

            // get message payload
            payload = chunk.sliceArray(2 until chunk.size)
        }

        return ReceiveMessageStructure(type, chunkIndex, resendIndicator, payload)
    }

    /**
     * Handle incoming flow control messages
     */
    private fun incomingFlowControlMessage(flcType: Byte, payload: ByteArray): ReceiveQueueResult {
        var receiveQueueResult = ReceiveQueueResult()
        if (!qaulIdKnown) {
            receiveQueueResult.qaulIdMissing = true
        }

        AppLog.e(TAG, "incomingFlowControlMessage type: $flcType, payload size: ${payload.size}")
        var binaryString = BLEUtils.toBinaryString(payload)
        AppLog.e(TAG, "incomingFlowControlMessage payload: $binaryString")

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
                    qaulId = payload
                    receiveQueueResult.qaulIdReceived = payload
                    receiveQueueResult.qaulIdMissing = false
                    AppLog.e(TAG, "incomingFlowControlMessage:received qaul_id: ${BLEUtils.toBinaryString(payload)}")
                }
            }
            // Missing chunks
            FlowControlMessageType.MISSING_CHUNKS.value -> {
                AppLog.e(TAG, "incomingFlowControlMessage: MISSING_CHUNKS")
                for (i in payload.indices step 2) {
                    val highByte: Byte = payload.get(i)
                    val lowByte: Byte = payload.get(i + 1)
                    val chunkIndex: Int = ((highByte.toInt() and 0xFF shl 8) + (lowByte.toInt() and 0xFF))
                    AppLog.e(TAG, "incomingFlowControlMessage: missing chunk index: $chunkIndex}")

                    // add missing chunk to send queue
                    receiveQueueResult.flcChunksRequested.add(chunkIndex)
                }
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
    private fun incomingMessageChunk(receiveMessageStructure: ReceiveMessageStructure): ReceiveQueueResult {
        AppLog.e(TAG, "incomingMessageChunk: queue index: ${receiveMessageStructure.queueIndex}, chunk index: ${receiveMessageStructure.chunkIndex}, resend indicator: ${receiveMessageStructure.resendIndicator}, payload size: ${receiveMessageStructure.payload.size}")
        var binaryString = BLEUtils.toBinaryString(receiveMessageStructure.payload)
        //AppLog.e(TAG, "payload: $binaryString")

        // get ReceiveQueueMessage
        var receiveQueueMessage: ReceiveQueueMessage? = receiveQueues.get(receiveMessageStructure.queueIndex)

        // check if ReceiveQueueMessage exists
        if (receiveQueueMessage == null) {
            // create new ReceiveQueueMessage
            receiveQueueMessage = ReceiveQueueMessage()
            receiveQueueMessage.messageQueueIndex = receiveMessageStructure.queueIndex
        } else if (receiveMessageStructure.resendIndicator == false) {
            // check if it is first chunk, or
            // check if the chunk index is lower than highest chunk index, or
            // check if the total chunks are smaller than current chunk index.
            if (receiveMessageStructure.chunkIndex == 0.toShort() ||
                receiveMessageStructure.chunkIndex <= receiveQueueMessage.highestChunkIndex ||
                (receiveQueueMessage.totalChunks != null &&
                 receiveQueueMessage.totalChunks!! <= receiveMessageStructure.chunkIndex)) {
                AppLog.e(TAG, "incomingMessageChunk: invalid queue, highest index ${receiveQueueMessage.highestChunkIndex} > current index ${receiveMessageStructure.chunkIndex}, we create a new queue")
                // Then we have a new queue
                receiveQueueMessage = ReceiveQueueMessage()
                receiveQueueMessage.messageQueueIndex = receiveMessageStructure.queueIndex
            }       
        }

        // add chunk to queue
        val receiveQueueResult = receiveQueueMessage.addReceivedChunk(receiveMessageStructure, qaulIdKnown)
        receiveQueues.put(receiveMessageStructure.queueIndex, receiveQueueMessage)

        return receiveQueueResult
    }

    /**
     * Add a part of a large message
     */
    private fun addLargeMessagePart(receivedMessage: ReceivedMessage): ReceivedMessage? {
        // get large message indicator
        val largeMessageIndicator = receivedMessage.largeMessageIndicator
        val qaulId = receivedMessage.qaulId

        // analyze large message indicator
        val largeMessageIndex = (largeMessageIndicator.toInt() shr 4) and 0x0F
        val partsTotal = (largeMessageIndicator.toInt() shr 2) and 0x03
        val partReceived = (largeMessageIndicator.toInt() and 0x03)

        AppLog.e(TAG, "addLargeMessagePart: large message index: $largeMessageIndex, parts total: $partsTotal, part received: $partReceived")

        // check if large message queue exists
        var receiveLargeMessage: ReceiveLargeMessage? = largeMessageQueues.get(largeMessageIndex)

        // check if parts total is equal
        if (receiveLargeMessage != null && receiveLargeMessage.partsTotal != partsTotal) {
            // error, parts total does not match
            AppLog.e(TAG, "addLargeMessagePart: parts total does not match existing large message queue")
            // remove existing large message queue
            largeMessageQueues.remove(largeMessageIndex)
            receiveLargeMessage = null
        }

        // create large message queue if it does not exist
        if (receiveLargeMessage == null) {
            // create new large message queue
            receiveLargeMessage = ReceiveLargeMessage(largeMessageIndex, partsTotal)
        }

        // add part to large message queue
        val allPartsReceived = receiveLargeMessage.addPart(partReceived, receivedMessage.message)
        largeMessageQueues.put(largeMessageIndex, receiveLargeMessage)

        // check if all parts are received
        var receivedMessage: ReceivedMessage? = null;
        if (allPartsReceived) {
            // assemble final large message
            var finalMessage = ByteArray(0)
            for (i in 0 until partsTotal) {
                finalMessage += receiveLargeMessage.messageParts[i]!!
            }

            // remove large message queue
            largeMessageQueues.remove(largeMessageIndex)

            // add final message to receiveQueueResult
            receivedMessage = ReceivedMessage(
                qaulId,
                finalMessage,
                0x00.toByte()
            )
        }

        return receivedMessage
    }
}

/**
 * message receiving state
 */
enum class ReceiveQueueMessageState {
    RECEIVING,              // receiving messages
    WAITING_ON_MISSING,     // last index received and waiting for missing chunks
    RECEIVED,               // message was received successfully, SUCCESS ACK sent
    ERROR                   // message was not received successfully, ERROR ACK sent
}

/**
 * protocol constants
 */
// Chunk Header Size in Bytes
const val CHUNK_HEADER_SIZE = 2
// First Chunk Header Size in Bytes
const val FIRST_CHUNK_HEADER_SIZE = 19
// Maximum number of missing chunks in one gap.
// If the gap is larger, an error is issued.
const val MAX_MISSING_GAP = 12      

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
    var largeMessageIndicator: Byte = 0
    var messageQueueIndex: Byte = 0
    var messageSize: Int? = null
    var totalChunks: Short? = null
    var crc32Checksum: Long? = null
    var chunkSize: Int = 20

    // tracking received chunks
    var receivedChunks: MutableMap<Short, ByteArray> = mutableMapOf()
    // current chunk index tracks the highest received chunk index
    var highestChunkIndex: Short = 0

    // Track Missing Chunks
    var chunksMissing: MutableList<Short> = mutableListOf()
    // Request Round
    // first round = 0
    var requestRound: Int = 0
    // Receive Index
    var receiveIndex: Short = 0

    /**
     * Add a newly received chunk message
     */
    fun addReceivedChunk(receiveMessageStructure: ReceiveMessageStructure, qaulIdKnown: Boolean): ReceiveQueueResult {
        var receiveQueueResult = ReceiveQueueResult()
        val index: Short = receiveMessageStructure.chunkIndex

        // check if chunk has already been received
        if (receivedChunks.containsKey(index)) {
            AppLog.e(TAG, "addReceivedChunk chunk already received")
            return receiveQueueResult
        }

        // check if this is the first message
        if (index == 0.toShort()) {
            AppLog.e(TAG, "GattMessaging addChunk first message received")
            firstChunkReceived = true

            // get payload & and set message information
            val firstChunkPayload = analyzeFirstChunk(receiveMessageStructure.payload)

            // add payload to receivedChunks
            receivedChunks.put(index, firstChunkPayload)
        } else {
            // add payload to received chunks
            receivedChunks.put(index, receiveMessageStructure.payload)
        }

        // ---------------------------
        // update current chunk index
        // ---------------------------
        
        if (qaulId.size == 0 && !receiveMessageStructure.resendIndicator && receivedChunks.size == 1 ) {
            // we do not have a qaulId yet and we therefore don't request
            // missing chunks for the first message
            
        } else if (!receiveMessageStructure.resendIndicator) {
            // new chunks received
            // check if it is the first message
            if (receiveIndex +1 < index) {
                for (i in (receiveIndex + 1)until index) {
                    // add missing chunk to chunksMissing
                    chunksMissing.add(i.toShort())
                    // add missing chunk to ReceiveQueueResult
                    val missingChunkIndex = (messageQueueIndex.toInt() shl 11) or i.toInt()
                    receiveQueueResult.flcRequestChunks.add(missingChunkIndex)
                }
            }
        } else {
            // missing chunk received
            chunksMissing.forEach { it ->
                if (it >  index && it < receiveIndex) {
                    // re-request missing chunk
                    val missingChunkIndex = (messageQueueIndex.toInt() shl 11) or it.toInt()
                    receiveQueueResult.flcRequestChunks.add(missingChunkIndex)
                }
            }

            // remove index from missing chunks
            chunksMissing.remove(index)
        }

        // update current chunk index
        receiveIndex = index
        if (index > highestChunkIndex) {
            highestChunkIndex = index
        }

        // check if all chunks are received
        if (totalChunks != null && 
            receivedChunks.size == totalChunks!!.toInt()) {
            // Create final message from received chunks
            receiveQueueResult = assembleMessage(receiveQueueResult)
        }

        return receiveQueueResult
    }

    /**
     * Analyze First Message chunk without the 2 Byte Header
     * @param chunk ByteArray
     * @return ByteArray with First Message Payload
     */
    private fun analyzeFirstChunk(chunk: ByteArray): ByteArray {
        AppLog.e(TAG, "analyzeFirstChunk: chunk size: ${chunk.size}")

        // check if chunk is large enough
        if (chunk.size < FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE) {
            AppLog.e(TAG, "analyzeFirstChunk: chunk size is too small: ${chunk.size}")
            return ByteArray(0)
        }

        // get large message information
        largeMessageIndicator = chunk[0]

        // get message size
        messageSize = BLEUtils.byteArrayToInt(chunk.sliceArray(1..2))
        AppLog.e(TAG, "analyzeFirstChunk: message size: $messageSize")

        // get total chunks
        totalChunks = BLEUtils.byteArrayToInt(chunk.sliceArray(3..4)).toShort()
        AppLog.e(TAG, "analyzeFirstChunk: total chunks: $totalChunks")

        // get crc32 checksum
        crc32Checksum = BLEUtils.byteArrayToCrc32Value(chunk.sliceArray(5..8))

        // get qaulId
        val receivedQaulId = chunk.sliceArray(9..16)
        if (!qaulId.size.equals(8)) {
            qaulId = receivedQaulId
            AppLog.e(TAG, "analyzeFirstChunk: qaulId: ${BLEUtils.byteToHex(receivedQaulId)}")
        } else {
            AppLog.e(TAG, "analyzeFirstChunk: qaulId already known")
        }

        // get payload
        val payload: ByteArray
        if (chunk.size > FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE) {
            payload = chunk.sliceArray(FIRST_CHUNK_HEADER_SIZE - CHUNK_HEADER_SIZE until chunk.size)
        } else {
            payload = ByteArray(0)
        }
        //AppLog.e(TAG, "analyzeFirstChunk: payload: ${BLEUtils.toBinaryString(payload)}")
        return payload
    }
    
    /**
     * Assemble the final message from received chunks
     * @param result ReceiveQueueResult
     * @return ReceivedMessage
     */
    private fun assembleMessage(receiveQueueResult: ReceiveQueueResult): ReceiveQueueResult {
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
                    
                    // DEBUG
                    //AppLog.e(TAG, "assembleMessage chunk $i, size: ${receivedChunks[i.toShort()]!!.size}")
                    //AppLog.e(TAG, "assembleMessage message size: ${message.size}")
                }
            }

            // check if message size matches
            //AppLog.e(TAG, "assembleMessage: total message size: ${message.size}, expected size: $messageSize")
            if (message.size != messageSize) {
                AppLog.e(TAG, "assembleMessage message size does not match")
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
                receiveQueueResult.receivedMessage = ReceivedMessage(
                    qaulId,
                    message,
                    largeMessageIndicator
                )
            }
        }

        // Set queue state & create response
        if (error) {
            AppLog.e(TAG, "assembleMessage schedule FLC ACK error message")

            // on error
            // set queue state
            state = ReceiveQueueMessageState.ERROR
            // create FLC message with error
            receiveQueueResult.flcSendAck = FlcAck(
                messageQueueIndex,
                false,
                0
            )
        } else {
            AppLog.e(TAG, "assembleMessage schedule FLC ACK success message")

            // on success
            // set queue state
            state = ReceiveQueueMessageState.RECEIVED
            // create FLC message with success
            receiveQueueResult.flcSendAck = FlcAck(
                messageQueueIndex,
                true
            )
        }

        return receiveQueueResult
    }
}

/**
 * Receive Large Message Tracker
 *
 * This class contains all the logic and the saves the parts of a large message being received.
 */
class ReceiveLargeMessage {
    val TAG: String = "ReceiveLargeMessage"
    var index: Int = 0
    var partsTotal: Int = 0 // 0 = 1
    var partsReceived: Int = 0 // 0 = none received
    var messageParts: Array<ByteArray?> = arrayOfNulls(4)

    /**
     * Create a new LargeMessagePart
     * @param partIndex the index of the part
     */
    constructor(largeMessageIndex: Int, partsTotal: Int) {
        index = largeMessageIndex
        this.partsTotal = partsTotal
    }

    /**
     * Add a new message part
     *
     * If all parts of a messages were successfully received, the function returns true.
     *
     * @param largeMessageIndicator Byte
     * @param messagePart ByteArray
     * @return Boolean - true if all parts were received, false otherwise
     */
    fun addPart(partReceived: Int, messagePart: ByteArray): Boolean {
        if (messageParts[partReceived] != null) {
            AppLog.e(TAG, "addPart: Part $partReceived already exists in Large Message Queue $index.")
        } else {
            partsReceived += 1
        }

        // add part
        messageParts[partReceived] = messagePart

        // check if all parts are received
        if (partsReceived >= partsTotal) {
            return true
        }
        return false
    }
}
