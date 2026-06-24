package net.qaul.ble

import java.util.UUID
import java.security.SecureRandom

object BleConstants {

    /**
     * This device's qaul ID (8 bytes). Injected from the qaul START_REQUEST (the real node ID)
     * via BleWrapperClass. Defaults to a random ID so the module still works standalone (the test
     * app) before any START_REQUEST arrives. Read by the advertiser, the role tiebreaker, and SendQueue.
     */
    @Volatile
    var LOCAL_QAUL_ID: ByteArray = ByteArray(8).also { SecureRandom().nextBytes(it) }


    // --------------------------------------------------------------------------------------------
    // Service & Characteristic UUIDs
    // These must match across all platforms (Android, iOS, Linux) for devices to discover
    // each other and communicate.
    // --------------------------------------------------------------------------------------------

    /** Main service UUID advertised by the GATT server. Scanner filters on this. */
    val SERVICE_UUID: UUID = UUID.fromString("4db14399-0bd0-4445-9906-47d9c4791cff")

    /** Characteristic used to read the remote device's qaul ID on initial connection. */
    val READ_CHAR: UUID = UUID.fromString("4db14401-0bd0-4445-9906-47d9c4791cff")

    /**
     * Characteristic used for message transport.
     * Properties: WRITE (client → server) + NOTIFY (server → client)
     * This single characteristic handles both directions of message traffic.
     */
    val MSG_CHAR: UUID = UUID.fromString("4db14402-0bd0-4445-9906-47d9c4791cff")

    /**
     * Characteristic the central reads to learn the peripheral's L2CAP CoC PSM (a 4-byte
     * big-endian int, dynamically assigned by the OS). The central uses it to open the
     * high-bandwidth L2CAP data channel for file transfer. -1 means L2CAP is unavailable.
     */
    val PSM_CHAR: UUID = UUID.fromString("4db14403-0bd0-4445-9906-47d9c4791cff")

    /**
     * Client Characteristic Configuration Descriptor UUID (standard BLE).
     * Writing 0x0001 to this descriptor on MSG_CHAR enables notifications,
     * allowing the server to push chunks to us without polling.
     */
    val CCCD_UUID: UUID = UUID.fromString("00002902-0000-1000-8000-00805f9b34fb")

    // --------------------------------------------------------------------------------------------
    // Connection settings
    // --------------------------------------------------------------------------------------------

    /** Maximum number of simultaneous peer connections. Android BLE is unreliable above 3. */
    const val MAX_CONNECTIONS = 3

    /** Company ID for the manufacturer-data block carrying the truncated qaul ID in advertisements.
     *  0xFFFF is the SIG value reserved for testing / internal use. */
    const val QAUL_MANUFACTURER_ID = 0xFFFF

    /** Number of leading qaul-ID bytes advertised — a non-authoritative pre-connection hint
     *  (the full ID is always verified post-connection). 5 bytes fits the 31-byte legacy advert
     *  budget and is collision-overkill for disambiguating local peers. */
    const val QAUL_ID_ADVERT_BYTES = 5

    /** Target MTU size to negotiate after connecting. Allows larger chunks than the 23-byte default. */
    const val TARGET_MTU = 517

    /** Default chunk size in bytes (Android default MTU 23 - 3 bytes GATT overhead = 20). */
    const val DEFAULT_CHUNK_SIZE = 20

    /** Timeout in milliseconds for a GATT operation before it is considered failed. */
    const val GATT_OPERATION_TIMEOUT_MS = 5_000L

    /** Timeout in milliseconds for initial connection before giving up. */
    const val CONNECTION_TIMEOUT_MS = 8_000L

    /** How long with no data before a connection is considered dead and force-disconnected. */
    const val LIVENESS_TIMEOUT_MS = 30_000L

    /** How often we check if all connections are still alive */
    const val LIVENESS_CHECK_INTERVAL_MS = 5_000L

    /** How long a connection may stay unresolved (qaulId never learned) before the unresolved
     *  reaper drops it as a stuck handshake / zombie. */
    const val UNRESOLVED_TIMEOUT_MS = 8_000L

    const val PING_INTERVAL_MS = 10_000L

    // --------------------------------------------------------------------------------------------
    // Reconnect / backoff settings

    /** Minimum delay between reconnect attempts in milliseconds. */
    const val RECONNECT_DELAY_MIN_MS = 5_000L

    /** Maximum delay between reconnect attempts in milliseconds. */
    const val RECONNECT_DELAY_MAX_MS = 60_000L

    /** Backoff multiplier applied after each failed reconnect attempt. */
    const val RECONNECT_BACKOFF_MULTIPLIER = 2.0

    /** Jitter factor applied to reconnect delay (±this fraction of the delay). */
    const val RECONNECT_JITTER_FACTOR = 0.25

    // --------------------------------------------------------------------------------------------
    // Scanner settings
    // --------------------------------------------------------------------------------------------

    /** How long a single scan window runs before stopping in milliseconds. */
    const val SCAN_DURATION_MS = 10_000L

    /** Delay between scan windows in milliseconds. */
    const val SCAN_INTERVAL_MS = 5_000L

    /** How long without seeing a device before it is considered out of range in milliseconds. */
    const val OUT_OF_RANGE_TIMEOUT_MS = 15_000L
}
