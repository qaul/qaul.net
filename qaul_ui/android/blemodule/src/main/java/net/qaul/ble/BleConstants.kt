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

    /** anti-islanding (ADD-ONLY). Combats cold start fracture into islands by (a) holding the
     *  last free connection slot briefly for a "bridge" rather than filling it with a peer we can
     *  already reach, and (b) exchanging 2 hop neighbour lists (a small FLC message) so we can tell a
     *  bridge from a redundant triangle topology. In this 1st phase, it never drops a healthy link, so it cannot cause
     *  connection flapping. can be disabled if it misbehaves. */
    const val ANTI_ISLANDING = true

    /** How long to hold the last free slot for a bridge (a peer not already reachable within 2 hops)
     *  before filling it with whatever is available, so the slot is never wasted. */
    const val RESERVE_SLOT_HOLD_MS = 10_000L

    /** TEST ONLY — force a fixed topology (e.g. a line for multi-hop testing) even when every device is
     *  in RF range. If non-empty, this device only forms/keeps connections with peers whose qaul ID
     *  begins with one of these hex prefixes (lowercase, no separators).. Empty = normal operation. */
    val TEST_NEIGHBOUR_ALLOWLIST: Set<String> = emptySet()

    /** True if [idBytes] (an advertised prefix or a full q8id) is a permitted neighbour under the test
     *  allowlist. Always true when the allowlist is empty. Matches by hex prefix, so a few leading bytes is enough to identify a peer. */
    fun isAllowedNeighbour(idBytes: ByteArray): Boolean {
        if (TEST_NEIGHBOUR_ALLOWLIST.isEmpty()) return true
        val hex = idBytes.joinToString("") { "%02x".format(it) }
        return TEST_NEIGHBOUR_ALLOWLIST.any { hex.startsWith(it.lowercase()) }
    }

    /** Connection admission control: max outbound CENTRAL connects we'll have in flight at once (connected but not yet
     *  qaul id resolved). Auto connect is gated on this. Prevents the scanner from piling on
     *  connects faster than the serial GATT queue can drain, which jams the queue with hung connectGatts,
     *  reaps connections before their READ_CHAR runs etc, and can wedge the whole BLE stack. */
    const val MAX_CONCURRENT_CONNECTING = 1

    /** Wrong role connect defer. When we discover a peer we should be PERIPHERAL to (their
     *  advertised qaul ID < ours), wait this long for them to connect to us before we connect outbound
     *  ourselves. Stops our outbound connect from racing their inbound one and forming a dual link that
     *  collapses with 133 or wastes time doing tiebreakers. After the window we connect anyway as a fallback. */
     // TODO: More testing of this, does waiting waste opportunities to form connections quicker even if they are in the wrong direction
    const val WRONG_ROLE_DEFER_MS = 9_000L

    /** Company ID for the manufacturer-data block carrying the truncated qaul ID in advertisements.
     *  0xFFFF is the SIG value reserved for testing / internal use. */
    const val QAUL_MANUFACTURER_ID = 0xFFFF

    /** Number of leading qaul-ID bytes advertised, a non-authoritative pre-connection hint
     *  (the full ID is always verified post connection anyway, this helps with pre connect decision-making). 5 bytes fits the 31-byte legacy advert
     *  budget and is collision overkill for disambiguating local peers. */
    const val QAUL_ID_ADVERT_BYTES = 5

    /** use LE Coded PHY (long range, S=8) for advertising and the connection link.
    * Only takes effect on hardware that supports Coded PHY + extended
     *  advertising (see the BLE CAPS startup log). non-capable devices fall back to legacy/2M so they
     *  still work at normal range. Currently, both ends of a link must support Coded for the long range link to form.*/
    @Volatile
    var USE_CODED_PHY = false

    /** Target MTU size to negotiate after connecting. Allows larger chunks than the 23-byte default. */
    const val TARGET_MTU = 517

    /** Default chunk size in bytes (Android default MTU 23 - 3 bytes GATT overhead = 20). */
    const val DEFAULT_CHUNK_SIZE = 20

    /** Message-size threshold (bytes)
     *  A message whose total size is at or below this rides the MEDIUM lane (routing updates, chat) so
     *  it stays ahead of large transfers, anything larger (images/files) rides the BULK lane.
     *  TODO: tune against real qaul routing-message sizes once measured */
    const val MEDIUM_MESSAGE_MAX_BYTES = 16000

    /** Watchdog timeout for fast GATT ops (reads, writes, notifies, MTU, descriptor, PHY). These
     *  complete in well under 300ms, so a hung one is caught quickly. Kept short because a hang holds the single scheduler slot, blocking all ops. */
    const val FAST_OP_TIMEOUT_MS = 2_500L

    /** Watchdog timeout for service discovery, the one legitimately-slow non-connect op (up to ~2s, usually ~1s
     *  more on devices with many services / slow links), so it gets a more generous window. */
    const val SERVICE_DISCOVERY_TIMEOUT_MS = 5_000L

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

    /** Show the on-device floating BLE stats overlay (BleDebugOverlay) while BLE is running. For debugging purposes,
     *  set false to disable. Needs the "Draw over other apps" permission, requested on first show. */
    const val DEBUG_OVERLAY = true

    /** Pause the scan during each connect attempt. DISABLED: confirmed in field logs to restart the
     *  scan often enough (during connect/tiebreaker churn) to trip Android's ~5-startScan/30s limit,
     *  which silently kills the scanner (scanResults freeze at 0 while peers keep advertising).
     *  TODO: Review whether its still worth it aslong as we enforce a limit to stay under 5 starts per second
     *  */
    const val SCAN_PAUSE_DURING_CONNECT = false

    // --------------------------------------------------------------------------------------------
    // Startup staging
    // The engine comes up in stages so we don't fire connects into a half-initialised local stack.
    // More testing needed to see whether this matters, will really only occur in testing when we start multiple phones in range at the same time.
    // Order: GATT server (immediate) then advertiser then scanner. The scanner goes last because it drives
    // the active connects, by the time it starts, our GATT server is registered and we're advertising.
    // --------------------------------------------------------------------------------------------

    /** Delay after engine start before the advertiser comes up, so the GATT server has finished
     *  registering its service and we're discoverable with a complete service. */
    const val STARTUP_ADVERTISE_DELAY_MS = 750L

    /** Delay after engine start before the scanner (and thus auto-connect) begins. Jittered within
     *  [MIN,MAX]: the floor lets the local stack settle (fixes the single-device-restart storm); the
     *  random spread keeps several devices cold-starting together from all connecting on the same tick. */
    const val STARTUP_SCAN_DELAY_MIN_MS = 2_000L
    const val STARTUP_SCAN_DELAY_MAX_MS = 3_500L

    // --------------------------------------------------------------------------------------------
    // Radio health watchdog
    // Android can silently kill a long scan/advert with no onScanFailed callback (screen-off,
    // stack hiccup) the isScanning/isAdvertising flags stay true while the radio is dark.
    // The watchdog watches scan result silence (ground truth) and force restarts both systems.
    // --------------------------------------------------------------------------------------------

    /** How often the radio health watchdog checks for a dark scanner. */
    const val RADIO_HEALTH_INTERVAL_MS = 15_000L

    /** No scan result for this long, assume the scan died silently and force restart the radio.
     *  Must stay well above the restart rate so the restarts themselves never trip the scan limit. */
    const val SCAN_SILENCE_RESTART_MS = 20_000L

    // --------------------------------------------------------------------------------------------
    // Reconnect / backoff settings

    /** Number of consecutive connect failures to a peer that retry immediately (no backoff) before
     *  the exponential backoff kicks in. Transient 133s are normal in a dense mesh and usually
     *  succeed on the next try, we only want to back off a peer that keeps failing, not silence a
     *  node for seconds over one blip. */
    const val RECONNECT_FREE_RETRIES = 2

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
