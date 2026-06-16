package net.qaul.ble.test.ble.metrics

import android.util.Log
import kotlin.time.TimeSource
import kotlin.time.TimeSource.Monotonic.ValueTimeMark

/**
 * Test metrics collector for BLE message transfers.
 *
 * Usage:
 *   Send side  — SendQueue calls [onMessageSendStarted] when chunks begin transmitting,
 *                and [onMessageAcknowledged] when the remote ACKs the full message.
 *   Receive side — ReceiveQueueMessage calls [onMessageAssembled] after successful CRC,
 *                  passing the queue's [createdAt] mark as the start time.
 *
 * Results accumulate in [results] and can be printed with [getSummary].
 * Call [clear] between test runs.
 */
object BleMetrics {
    private const val TAG = "BleMetrics"

    enum class Direction { SENT, RECEIVED }

    data class TransferResult(
        val messageId: String,      // send messageId; empty string for received messages
        val sizeBytes: Int,
        val durationMs: Long,
        val direction: Direction
    ) {
        val throughputKbps: Double
            get() = if (durationMs > 0) sizeBytes * 8.0 / durationMs else 0.0
    }

    // In-flight sends: messageId → (sizeBytes, startTime)
    private val pendingSends = mutableMapOf<String, Pair<Int, ValueTimeMark>>()

    val results = mutableListOf<TransferResult>()

    // --------------------------------------------------------------------------------------------
    // Send side
    // --------------------------------------------------------------------------------------------

    /**
     * Called by SendQueue.getChunks() when a message's chunks are first enqueued for
     * transmission — i.e. the moment BLE writes start going out.
     *
     * For large messages this is called once per part (all parts share the same messageId).
     * Subsequent calls accumulate the size so the final metric covers the whole message,
     * while the start time is preserved from the very first call.
     */
    fun onMessageSendStarted(messageId: String, sizeBytes: Int) {
        val existing = pendingSends[messageId]
        if (existing != null) {
            // Large message: add this part's bytes, keep the original start time
            pendingSends[messageId] = Pair(existing.first + sizeBytes, existing.second)
            Log.d(TAG, "Send part added: $messageId (+$sizeBytes B, total ${existing.first + sizeBytes} B)")
        } else {
            pendingSends[messageId] = Pair(sizeBytes, TimeSource.Monotonic.markNow())
            Log.d(TAG, "Send started: $messageId ($sizeBytes B)")
        }
    }

    /**
     * Called by SendQueue.flcAckReceived() when the remote ACKs the complete message.
     * Records the round-trip time from first chunk out to ACK in.
     */
    fun onMessageAcknowledged(messageId: String) {
        val (sizeBytes, startedAt) = pendingSends.remove(messageId) ?: return
        val durationMs = startedAt.elapsedNow().inWholeMilliseconds
        val result = TransferResult(messageId, sizeBytes, durationMs, Direction.SENT)
        results.add(result)
        Log.i(TAG, "SENT: ${sizeBytes}B in ${durationMs}ms (${String.format("%.1f", result.throughputKbps)} kbps)")
    }

    // --------------------------------------------------------------------------------------------
    // Receive side
    // --------------------------------------------------------------------------------------------

    /**
     * Called by ReceiveQueueMessage.assembleMessage() on success.
     * [receivedAt] should be the queue's [createdAt] mark — i.e. when the first chunk arrived.
     */
    fun onMessageAssembled(sizeBytes: Int, receivedAt: ValueTimeMark) {
        val durationMs = receivedAt.elapsedNow().inWholeMilliseconds
        val result = TransferResult("", sizeBytes, durationMs, Direction.RECEIVED)
        results.add(result)
        Log.i(TAG, "RECEIVED: ${sizeBytes}B in ${durationMs}ms (${String.format("%.1f", result.throughputKbps)} kbps)")
    }

    // --------------------------------------------------------------------------------------------
    // Reporting
    // --------------------------------------------------------------------------------------------

    fun getSummary(): String {
        if (results.isEmpty()) return "No metrics recorded yet."
        val sb = StringBuilder("=== BLE Transfer Metrics ===\n")

        listOf(Direction.SENT, Direction.RECEIVED).forEach { dir ->
            val group = results.filter { it.direction == dir }
            if (group.isEmpty()) return@forEach
            val label = if (dir == Direction.SENT) "Sent" else "Received"
            val avgMs = group.map { it.durationMs }.average()
            val avgKbps = group.map { it.throughputKbps }.average()
            sb.append("$label (${group.size} messages):\n")
            sb.append("  avg time : ${avgMs.toLong()} ms\n")
            sb.append("  avg speed: ${String.format("%.1f", avgKbps)} kbps\n")
            group.forEach { r ->
                sb.append("  ${r.sizeBytes}B in ${r.durationMs}ms (${String.format("%.1f", r.throughputKbps)} kbps)\n")
            }
        }
        return sb.toString()
    }

    fun clear() {
        pendingSends.clear()
        results.clear()
        Log.i(TAG, "Metrics cleared")
    }
}
