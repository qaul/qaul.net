// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

/**
 * This File contains the logic for the interconnection of the
 * SendQueue and the ReceiveQueue that can use different MAC addresses
 * for the same qaul ID.
 *
 * On Android, the BLE MAC address can change on each connection,
 * depending on the device and Android version.
 * Incoming Connection can have a different MAC address
 * than outgoing connections, for the same qaul ID.
 */

package net.qaul.ble.service

import android.bluetooth.BluetoothDevice
import net.qaul.ble.AppLog
import net.qaul.ble.BLEUtils
import java.util.*

/**
 * Connection Manager to relate SendQueues and ReceiveQueues by qaul ID
 */
class ConnectionManager {
    val TAG: String = "ConnectionManager"

    // send queues mapped by qaul ID ( BLEUtils.byteArrayToLong(qaulId) )
    val sendQaulIdQueues = Collections.synchronizedMap(mutableMapOf<Long, SendQaulIdQueue>())
    
    /**
     * Add ReceiveQueueResult to the ConnectionManager
     */
    fun addReceiveQueueResult(receiveQueueResult: ReceiveQueueResult) {
        AppLog.d(TAG, "addReceiveQueueResult called")

        // check if qaul ID is set
        if (receiveQueueResult.qaulIdReceived == null) {
            AppLog.e(TAG, "Received queue result without qaul ID, ignoring.")
            return
        }

        // get or create SendQueue for qaul ID
        val qaulIdLong = BLEUtils.byteArrayToLong(receiveQueueResult.qaulIdReceived!!)
        var sendQaulIdQueue = sendQaulIdQueues[qaulIdLong]
        if (sendQaulIdQueue == null) {
            AppLog.e(TAG, "No sendQaulIdQueue found for qaulId: ${receiveQueueResult.qaulIdReceived} creating one.")
            sendQaulIdQueue = SendQaulIdQueue(receiveQueueResult.qaulIdReceived!!)
        }

        // check if we need to request chunks
        if (receiveQueueResult.flcRequestChunks.size > 0) {
            AppLog.e(TAG, "receiveQueueResult chunks requested")

            // add missing chunks to sendQaulIdQueue
            for (missingChunk in receiveQueueResult.flcRequestChunks) {
                sendQaulIdQueue.addMissingChunkIndexToRequest(missingChunk)
            }
        }

        // check if we need to send chunks
        if (receiveQueueResult.flcChunksRequested.size > 0) {
            AppLog.e(TAG, "receiveQueueResult chunks to send")
            // add missing chunks to sendQaulIdQueue
            for (missingChunk in receiveQueueResult.flcChunksRequested) {
                sendQaulIdQueue.addMissingChunkIndexToSend(missingChunk)
            }
        }

        // check if we received an ACK
        if (receiveQueueResult.flcAckReceived != null) {
            AppLog.d(TAG, "receiveQueueResult ACK received: Queue: ${receiveQueueResult.flcAckReceived!!.messageIndex}, Success: ${receiveQueueResult.flcAckReceived!!.success}, ErrorCode: ${receiveQueueResult.flcAckReceived!!.errorCode}")
            // add ACK to sendQaulIdQueue
            sendQaulIdQueue.addAckToSend(
                queueIndex = receiveQueueResult.flcAckReceived!!.messageIndex,
                success = receiveQueueResult.flcAckReceived!!.success,
                errorCode = receiveQueueResult.flcAckReceived!!.errorCode
            )
        }

        // check if we need to send an ACK 
        if (receiveQueueResult.flcSendAck != null) {
            AppLog.d(TAG, "receiveQueueResult ACK to send: Queue: ${receiveQueueResult.flcSendAck!!.messageIndex}, Success: ${receiveQueueResult.flcSendAck!!.success}, ErrorCode: ${receiveQueueResult.flcSendAck!!.errorCode}")
            // add ACK to sendQaulIdQueue
            sendQaulIdQueue.addFlcAck(
                queueIndex = receiveQueueResult.flcSendAck!!.messageIndex,
                success = receiveQueueResult.flcSendAck!!.success,
                errorCode = receiveQueueResult.flcSendAck!!.errorCode
            )
        }

        sendQaulIdQueues[qaulIdLong] = sendQaulIdQueue
    }

    /**
     * Get SendQueueFlcs for qaul ID
     */
    fun getSendQueueFlcs(qaulId: ByteArray): SendQaulIdQueue? {
        val qaulIdKey = BLEUtils.byteArrayToLong(qaulId)
        return sendQaulIdQueues[qaulIdKey]
    }

    /**
     * Get or create SendQaulIdQueue for qaul ID
     */
    fun getAndRemoveSendQueue(qaulId: ByteArray): SendQaulIdQueue {
        val qaulIdKey = BLEUtils.byteArrayToLong(qaulId)
        var sendQaulIdQueue = sendQaulIdQueues[qaulIdKey]
        if (sendQaulIdQueue == null) {
            sendQaulIdQueue = SendQaulIdQueue(qaulId)
            AppLog.d(TAG, "Created SendQaulIdQueue for qaul ID: ${BLEUtils.byteToHex(qaulId)}")
        } else {
            sendQaulIdQueues.remove(qaulIdKey)
        }
        return sendQaulIdQueue
    }
}


/**
 * SendQueue for BLE messages
 * 
 * Each discovered receiving device has a SendQueue, 
 * which tracks the sending and creates the message chunks,
 * according to the qaul BLE GATT messaging protocol.
 */
class SendQaulIdQueue(qaulId: ByteArray) {
    val TAG: String = "SendQaulIdQueue"
    val qaulId = qaulId

    // send qaul ID
    var sendQaulId = false
    // TODO: ACKs to send
    var acksToSend: MutableMap<Byte, Pair<Boolean, Byte>> = mutableMapOf()
    // List of missing chunks to request
    // they have the second sending priority
    var missingChunksToRequest: MutableSet<Int> = mutableSetOf()
    // List of missing chunks to send
    // they have the third sending priority
    var missingChunksToSend: MutableSet<Int> = mutableSetOf()
    // ACK's received for sent messages
    var acksReceived: MutableMap<Byte, Pair<Boolean, Byte>> = mutableMapOf()

    /**
     * schedule FLC send qaul ID message
     */
    fun addFlcSendQaulId() {
        sendQaulId = true
    }


    /**
     * add ACK to send
     */
    fun addAckToSend(queueIndex: Byte, success: Boolean, errorCode: Byte) {
        // remove chunks to request with this queue index, if any
        val chunksIterator = missingChunksToRequest.iterator()
        while (chunksIterator.hasNext()) {
            val chunkIndex = chunksIterator.next()
            val chunkQueueIndex = chunkIndex shr 11
            if (chunkQueueIndex.toByte() == queueIndex) {
                chunksIterator.remove()
            }
        }

        // add ACK to the map
        acksToSend[queueIndex] = Pair(success, errorCode)
    }

    /**
     * add missing chunk index to request
     */
    fun addMissingChunkIndexToRequest(missingChunkIndex: Int) {
        // add missing chunk to the set
        missingChunksToRequest.add(missingChunkIndex)
    }

    /**
     * add a received ACK
     */
    fun addFlcAck(queueIndex: Byte, success: Boolean, errorCode: Byte) {
        // remove chunks to send with this queue index, if any
        val chunksIterator = missingChunksToSend.iterator()
        while (chunksIterator.hasNext()) {
            val chunkIndex = chunksIterator.next()
            val chunkQueueIndex = chunkIndex shr 11
            if (chunkQueueIndex.toByte() == queueIndex) {
                chunksIterator.remove()
            }
        }

        // add ACK to the map
        acksReceived[queueIndex] = Pair(success, errorCode)
    }

    /**
     * add missing chunk index to send
     */
    fun addMissingChunkIndexToSend(missingChunkIndex: Int) {
        // add missing chunk to the set
        missingChunksToSend.add(missingChunkIndex)
    }
}
