package net.qaul.ble.test.ble.util

import android.bluetooth.BluetoothGatt
import android.bluetooth.BluetoothGattCharacteristic
import android.bluetooth.BluetoothGattDescriptor
import android.util.Log
import java.util.UUID

// Standard Bluetooth SIG base UUID suffix
private const val BT_SIG_BASE_UUID_SUFFIX = "-0000-1000-8000-00805f9b34fb"

// Known standard service UUIDs (16-bit short UUIDs)
private val STANDARD_SERVICES = mapOf(
    "1800" to "Generic Access",
    "1801" to "Generic Attribute",
    "1802" to "Immediate Alert",
    "1803" to "Link Loss",
    "1804" to "Tx Power",
    "1805" to "Current Time",
    "180a" to "Device Information",
    "180d" to "Heart Rate",
    "180f" to "Battery",
    "1810" to "Blood Pressure",
    "1811" to "Alert Notification",
    "1812" to "Human Interface Device",
    "1816" to "Cycling Speed and Cadence",
    "1818" to "Cycling Power",
    "1819" to "Location and Navigation",
    "181a" to "Environmental Sensing",
    "181c" to "User Data",
    "181d" to "Weight Scale",
    "181e" to "Bond Management",
    "181f" to "Continuous Glucose Monitoring",
    "1820" to "Internet Protocol Support",
    "1821" to "Indoor Positioning",
    "1822" to "Pulse Oximeter",
    "1823" to "HTTP Proxy",
    "1824" to "Transport Discovery",
    "1825" to "Object Transfer",
    "1826" to "Fitness Machine",
    "1827" to "Mesh Provisioning",
    "1828" to "Mesh Proxy",
    "1829" to "Reconnection Configuration"
)

// Known standard characteristic UUIDs (16-bit short UUIDs)
private val STANDARD_CHARACTERISTICS = mapOf(
    "2a00" to "Device Name",
    "2a01" to "Appearance",
    "2a02" to "Peripheral Privacy Flag",
    "2a03" to "Reconnection Address",
    "2a04" to "Peripheral Preferred Connection Parameters",
    "2a05" to "Service Changed",
    "2a06" to "Alert Level",
    "2a07" to "Tx Power Level",
    "2a08" to "Date Time",
    "2a19" to "Battery Level",
    "2a1c" to "Temperature Measurement",
    "2a1d" to "Temperature Type",
    "2a24" to "Model Number String",
    "2a25" to "Serial Number String",
    "2a26" to "Firmware Revision String",
    "2a27" to "Hardware Revision String",
    "2a28" to "Software Revision String",
    "2a29" to "Manufacturer Name String",
    "2a2a" to "IEEE 11073-20601 Regulatory Cert",
    "2a37" to "Heart Rate Measurement",
    "2a38" to "Body Sensor Location",
    "2a39" to "Heart Rate Control Point",
    "2a3f" to "Alert Status",
    "2a46" to "New Alert",
    "2a4d" to "Report",
    "2a50" to "PnP ID",
    "2a5a" to "Aggregate",
    "2a63" to "Cycling Power Measurement",
    "2a6d" to "Pressure",
    "2a6e" to "Temperature",
    "2a6f" to "Humidity",
    "2a76" to "UV Index",
    "2a77" to "Irradiance",
    "2a7e" to "Aerobic Heart Rate Lower Limit",
    "2a9b" to "Body Composition Feature",
    "2a9c" to "Body Composition Measurement",
    "2a9d" to "Weight Measurement",
    "2a9e" to "Weight Scale Feature",
    "2acc" to "Fitness Machine Feature",
    "2acd" to "Treadmill Data",
    "2ad2" to "Rower Data",
    "2ad6" to "Supported Resistance Level Range",
    "2ad9" to "Fitness Machine Control Point",
    "2ada" to "Fitness Machine Status"
)

/**
 * Returns a human-readable name for a UUID if it's a standard Bluetooth SIG UUID,
 * otherwise returns the full UUID string.
 */
fun UUID.toReadableName(): String {
    val uuidStr = toString().lowercase()
    // Check if it matches the BT SIG base UUID pattern: 0000XXXX-0000-1000-8000-00805f9b34fb
    if (uuidStr.endsWith(BT_SIG_BASE_UUID_SUFFIX) && uuidStr.startsWith("0000")) {
        val shortCode = uuidStr.substring(4, 8) // extract the XXXX part
        STANDARD_SERVICES[shortCode]?.let { return "$it ($shortCode)" }
        STANDARD_CHARACTERISTICS[shortCode]?.let { return "$it ($shortCode)" }
        return "Unknown SIG UUID ($shortCode)"
    }
    return uuidStr // custom/proprietary UUID, return as-is
}

/**
 * Prints all services and characteristics discovered on a GATT connection,
 * with human-readable names for standard Bluetooth SIG UUIDs.
 */
fun BluetoothGatt.printGattTable() {
    if (services.isEmpty()) {
        Log.i("printGattTable", "No services found for ${device.address}")
        return
    }
    services.forEach { service ->
        val serviceLabel = service.uuid.toReadableName()
        val characteristics = service.characteristics.joinToString(separator = "\n  |--") { char ->
            char.uuid.toReadableName()
        }
        Log.i("printGattTable", "\nService: $serviceLabel\n  |--$characteristics")
    }
}

// GATT Characteristic Property Checks
fun BluetoothGattCharacteristic.isReadable(): Boolean =
    containsProperty(BluetoothGattCharacteristic.PROPERTY_READ)

fun BluetoothGattCharacteristic.isWritable(): Boolean =
    containsProperty(BluetoothGattCharacteristic.PROPERTY_WRITE)

fun BluetoothGattCharacteristic.isWritableWithoutResponse(): Boolean =
    containsProperty(BluetoothGattCharacteristic.PROPERTY_WRITE_NO_RESPONSE)

fun BluetoothGattCharacteristic.containsProperty(property: Int): Boolean {
    return properties and property != 0
}

// GATT Descriptor Property Checks

fun BluetoothGattDescriptor.isReadable(): Boolean =
    containsPermission(BluetoothGattDescriptor.PERMISSION_READ) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_READ_ENCRYPTED) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_READ_ENCRYPTED_MITM)

fun BluetoothGattDescriptor.isWritable(): Boolean =
    containsPermission(BluetoothGattDescriptor.PERMISSION_WRITE) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_WRITE_ENCRYPTED) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_WRITE_ENCRYPTED_MITM) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_WRITE_SIGNED) ||
            containsPermission(BluetoothGattDescriptor.PERMISSION_WRITE_SIGNED_MITM)

fun BluetoothGattDescriptor.containsPermission(permission: Int): Boolean =
    permissions and permission != 0

// Notification checks

fun BluetoothGattCharacteristic.isIndicatable(): Boolean =
    containsProperty(BluetoothGattCharacteristic.PROPERTY_INDICATE)

fun BluetoothGattCharacteristic.isNotifiable(): Boolean =
    containsProperty(BluetoothGattCharacteristic.PROPERTY_NOTIFY)





fun ByteArray.toHexString(): String =
    joinToString(separator = " ", prefix = "0x") { String.format("%02X", it) }
