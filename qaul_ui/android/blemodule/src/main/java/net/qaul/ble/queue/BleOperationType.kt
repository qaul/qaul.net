package net.qaul.ble.test.ble.queue

import android.bluetooth.BluetoothDevice
import java.util.UUID

/**
 * Which scheduler lane a MSG_CHAR payload op rides. Non-MSG_CHAR ops (connect/discover/MTU/etc)
 * always ride CONTROL regardless of this. The scheduler drains lanes strictly in this order:
 * all CONTROL, then all MEDIUM, then BULK, so a lower lane can never delay a higher one.
 *   CONTROL — connection setup + flow-control (SEND_ID, ACK, ping, chunk requests). Latency critical.
 *   MEDIUM  — short message payloads (routing updates, chat). Kept ahead of large transfers so a file
 *             can't stall routing convergence.
 *   BULK    — large message payloads (images/files). Yields to everything else.
 */ //TODO: In the future we could potentially classify qaul routing messages as medium priority specifically instead of just gating by size, undecided whether this would improve
enum class OpLane { CONTROL, MEDIUM, BULK }

sealed class BleOperationType {
    abstract val device: BluetoothDevice
}
data class Connect(
    override val device: BluetoothDevice,
    val phy: Int = BluetoothDevice.PHY_LE_1M_MASK
) : BleOperationType()
data class Disconnect(override val device: BluetoothDevice) : BleOperationType()
data class ServiceDiscovery(override val device: BluetoothDevice) : BleOperationType()

data class CharacteristicRead(
    override val device: BluetoothDevice,
    val characteristicUuid: UUID
) : BleOperationType()

data class CharacteristicWrite(
    override val device: BluetoothDevice,
    val characteristicUuid: UUID,
    val writeType: Int,
    val payload: ByteArray,
    // true = route through the priority queue even though it's MSG_CHAR (flow-control notifies). Not
    // part of identity (equals/hashCode), only a routing hint.
    val lane: OpLane = OpLane.BULK
) : BleOperationType() {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as CharacteristicWrite
        return device == other.device &&
                characteristicUuid == other.characteristicUuid &&
                writeType == other.writeType &&
                payload.contentEquals(other.payload)
    }

    override fun hashCode(): Int {
        var result = device.hashCode()
        result = 31 * result + characteristicUuid.hashCode()
        result = 31 * result + writeType
        result = 31 * result + payload.contentHashCode()
        return result
    }
}

data class DescriptorRead(
    override val device: BluetoothDevice,
    val descriptorUuid: UUID
) : BleOperationType()

data class DescriptorWrite(
    override val device: BluetoothDevice,
    val descriptorUuid: UUID,
    val payload: ByteArray
) : BleOperationType() {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false

        other as DescriptorWrite

        if (device != other.device) return false
        if (descriptorUuid != other.descriptorUuid) return false
        if (!payload.contentEquals(other.payload)) return false

        return true
    }

    override fun hashCode(): Int {
        var result = device.hashCode()
        result = 31 * result + descriptorUuid.hashCode()
        result = 31 * result + payload.contentHashCode()
        return result
    }
}

data class MtuRequest(
    override val device: BluetoothDevice,
    val mtu: Int
) : BleOperationType()

data class EnableNotifications(
    override val device: BluetoothDevice,
    val characteristicUuid: UUID
) : BleOperationType()

data class DisableNotifications(
    override val device: BluetoothDevice,
    val characteristicUuid: UUID
) : BleOperationType()

data class ConnectionPriorityRequest(
    override val device: BluetoothDevice,
    val priority: Int
) : BleOperationType()

data class PhyRequest(
    override val device: BluetoothDevice,
    val txPhy: Int,
    val rxPhy: Int,
    val phyOptions: Int
) : BleOperationType()

data class NotifyCharacteristicChange(
    override val device: BluetoothDevice,
    val characteristicUuid: UUID,
    val confirmation: Boolean,
    val payload: ByteArray,
    // Which scheduler lane this MSG_CHAR notify rides (CONTROL for flow-control, MEDIUM for short
    // messages, BULK for large transfers). Not part of identity (equals/hashCode), only a routing hint.
    val lane: OpLane = OpLane.BULK
) : BleOperationType() {
    override fun equals(other: Any?): Boolean {
        if (this === other) return true
        if (javaClass != other?.javaClass) return false
        other as NotifyCharacteristicChange
        return device == other.device &&
                characteristicUuid == other.characteristicUuid &&
                confirmation == other.confirmation &&
                payload.contentEquals(other.payload)
    }

    override fun hashCode(): Int {
        var result = device.hashCode()
        result = 31 * result + characteristicUuid.hashCode()
        result = 31 * result + confirmation.hashCode()
        result = 31 * result + payload.contentHashCode()
        return result
    }
}
