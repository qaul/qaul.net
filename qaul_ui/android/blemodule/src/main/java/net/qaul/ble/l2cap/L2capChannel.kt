package net.qaul.ble.test.ble.l2cap

import android.bluetooth.BluetoothSocket
import android.util.Log
import java.io.DataInputStream
import java.io.DataOutputStream
import java.io.IOException
import java.util.concurrent.Executors
import kotlin.time.TimeSource

/**
 * Wraps a connected L2CAP CoC [BluetoothSocket] and provides framed message send/receive
 * with throughput measurement.
 *
 * Phase 1 PoC: this validates the L2CAP channel works on real hardware and measures its
 * throughput. The win over GATT is that
 * writes go to an OS-buffered stream, no per-write callback, no single-outstanding-write
 * rule, no notify/onNotificationSent turnaround, less per packet overhead.
 *

 * Threading:
 *  - one reader thread runs a blocking read loop
 *  - one single-thread executor serializes writes
 *  close() shuts both down. Closing the socket is what unblocks the reader's blocking read().
 */
class L2capChannel(
    private val socket: BluetoothSocket,
    private val label: String,
    private val onMessageReceived: (ByteArray) -> Unit = {},
    private val onClosed: () -> Unit = {}
) {
    private val TAG = "L2capChannel"
    private val input = DataInputStream(socket.inputStream)
    private val output = DataOutputStream(socket.outputStream)
    private val writer = Executors.newSingleThreadExecutor()
    @Volatile private var running = true

    init {
        Thread({ readLoop() }, "l2cap-reader-$label").apply {
            isDaemon = true
            start()
        }
    }

    private fun readLoop() {
        try {
            while (running) {
                // 4-byte length prefix, then the full payload
                val length = input.readInt()            // throws EOFException when the socket closes
                if (length <= 0 || length > MAX_MESSAGE) {
                    Log.e(TAG, "[$label] invalid frame length $length — closing")
                    break
                }
                val buf = ByteArray(length)
                val start = TimeSource.Monotonic.markNow()
                input.readFully(buf)                    // blocks until the whole message arrives
                val ms = start.elapsedNow().inWholeMilliseconds
                val kbps = if (ms > 0) length * 8.0 / ms else 0.0
                Log.i(TAG, "[$label] L2CAP RECEIVED: $length B in $ms ms (${"%.1f".format(kbps)} kbps)")
                onMessageReceived(buf)
            }
        } catch (e: IOException) {
            if (running) Log.i(TAG, "[$label] read loop ended: ${e.message}")
        } finally {
            close()
        }
    }

    /**
     * Send a framed message. Serialized on the writer thread; returns immediately.
     * NB: write() returns when the OS buffers the data, so for small payloads the "SENT"
     * time is near-zero. For large payloads write() blocks once the OS buffer fills, so the
     * time becomes meaningful, but the RECEIVED metric on the other end is authoritative.
     */
    fun send(data: ByteArray) {
        writer.execute {
            try {
                val start = TimeSource.Monotonic.markNow()
                output.writeInt(data.size)
                output.write(data)
                output.flush()
                val ms = start.elapsedNow().inWholeMilliseconds
                val kbps = if (ms > 0) data.size * 8.0 / ms else 0.0
                Log.i(TAG, "[$label] L2CAP SENT: ${data.size} B in $ms ms (${"%.1f".format(kbps)} kbps)")
            } catch (e: IOException) {
                Log.e(TAG, "[$label] write failed: ${e.message}")
                close()
            }
        }
    }

    @Synchronized
    fun close() {
        if (!running) return
        running = false
        try { socket.close() } catch (_: IOException) {}   // unblocks the reader's blocking read()
        writer.shutdownNow()
        onClosed()
        Log.i(TAG, "[$label] channel closed")
    }

    companion object {
        private const val MAX_MESSAGE = 16 * 1024 * 1024   // 16 MB sanity cap on a single frame
    }
}
