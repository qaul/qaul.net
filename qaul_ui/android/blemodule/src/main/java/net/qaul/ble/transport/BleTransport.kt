package net.qaul.ble.test.ble.transport

interface BleTransport {
    suspend fun send(data: ByteArray): Result<Unit>
    fun onReceive (callback: (ByteArray) -> Unit)
    suspend fun close()
    val isConnected: Boolean
}