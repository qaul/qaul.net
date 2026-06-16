package net.qaul.ble.test.ble.connection

enum class BleRole {
    CENTRAL, // when we initiated the connection, we are the GATT client
    PERIPHERAL // when they connected to us, we are the GATT server
}