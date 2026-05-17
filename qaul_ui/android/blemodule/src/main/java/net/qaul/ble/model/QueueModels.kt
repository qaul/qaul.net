// Copyright (c) 2025 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

package net.qaul.ble.model

import android.bluetooth.BluetoothDevice
import net.qaul.ble.AppLog

/**
 * Flow Control Message Type
 */
enum class FlowControlMessageType(val value: Byte) {
    REQUEST_QAUL_ID(0x00),
    SEND_QAUL_ID(0x01),
    MISSING_CHUNKS(0x02),
    ACK_SUCCESS(0x03),
    ACK_ERROR(0x04),
    MISSING_ACK_MESSAGES(0x05),
}

/**
 * Queue instance for Flow Control Message
 */
class FlowControlQueueMessage(
	val qaulId: ByteArray,
	val type: FlowControlMessageType,
	val payload: ByteArray = byteArrayOf()
	) {}

/**
 * Queue instance for missing chunk request
 */
class MissingChunkQueueMessage(
	val qaulId: ByteArray,
	val index: Byte,
	val chunkIndex: Short,
	val payload: ByteArray
	) {}
