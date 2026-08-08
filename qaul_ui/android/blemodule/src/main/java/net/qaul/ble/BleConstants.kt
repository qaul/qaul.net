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

    /** Maximum number of simultaneous peer connections. **/
    @Volatile
    var MAX_CONNECTIONS = 4

    /** Values the debug overlay cycles through.
     *   3-4 is what we have has validated so far. */
    val MAX_CONNECTION_OPTIONS = intArrayOf(2, 3, 4, 5, 6, 7, 8)

    /** anti-islanding: enables gossiped link-state (SEND_NEIGHBOURS flc) feeds the Stage 1 fill-gate
     *  (ConnectionPool.shouldAcceptEdge) and Stage 2 proactive drop  */
    const val ANTI_ISLANDING = true

    /**Enable topology mechanisms. Both require [ANTI_ISLANDING] (they read
     *  the gossiped link-state). */
    const val STAGE1_FILL_GATE = true
    const val STAGE2_PROACTIVE_DROP = true

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

    /** False until the BLE engine has finished staging (advertiser up) when Qaul is launched. The GATT server registers
     *  first, so between registration and staging a peer can connect and will cause problems */
    @Volatile
    var ENGINE_READY = false

    /** Push every short-range link to 2M as soon as it connects.
     *
 */
    const val PREFER_2M = false

    /** Allow an established Coded link to be upgraded when the peer is seen close up.
     *  TODO: a single RSSI spike promotes a long-range link to a short-range PHY, which then drops as soon as the peer moves away again. */
    const val ALLOW_PHY_UPGRADE = true

    /** How long a continous run of 1M sightings must last before a coded link is upgraded to
     *  1M (expensive to get wrong, if 1M then fails, re-establishing at range is the
     *  fragile part). Also we need to be wary of coded/1M flipping if on the border of 1Ms reach. */
    const val PHY_UPGRADE_CONFIRM_MS = 10_000L

    /** Connection interval requested for an idle link.
     *  Every open link consumes a connection event this often, and the controller must interleave
     *  them all with scanning and advertising. At high, several concurrent links can exhaust the
     *  radio schedule, missed events accumulate into continous supervision timeouts / churn when above a
     * conneciton cap of 3*/
    const val IDLE_CONNECTION_PRIORITY = android.bluetooth.BluetoothGatt.CONNECTION_PRIORITY_BALANCED

    const val HIGH_LOAD_CONNECTION_PRIORITY = android.bluetooth.BluetoothGatt.CONNECTION_PRIORITY_HIGH

    /** How long a link stays escalated (high priority + 2M) after its last bulk lane activity
     *  before dropping back to idle. Prevents flipping quickly between consecutive bulk sends */
    const val BULK_HOLD_DOWN_MS = 10_000L

    /** Most links allowed to hold the bulk escalation (High priority + 2M) at once. High costs ~4x the connection events
     * so if 4 links are all high, the radio can be overwhelmed and links timeout.
     *  TODO: setting to 2 hasnt been truly validated */
    const val MAX_ESCALATED_LINKS = 2


    /** Theoretical advert interval at low latency mode, combined with [CODED_ONLY_CONFIRM_INTERVALS] to give
     * a margin of waiting for a close range advert before choosing coded.
   */
    const val NOMINAL_ADVERT_INTERVAL_MS = 100L

    /** How many advertising intervals of evidence before concluding a peer is Coded only reachable. */
    const val CODED_ONLY_CONFIRM_INTERVALS = 10

    /** How long a peer must be seen ONLY on Coded before we accept Coded as its only route and
     *  connect long-range.
     *
     (see the TODO on BleAdvertiser's ADVERTISE_MODE) — a hardcoded value would silently
     *  become too short and start forcing Coded links on peers that are  advertising slower. */
    // TODO: This would have to change if advert modes are ever changed for battery optimization etc*/
    const val CODED_ONLY_CONFIRM_MS = NOMINAL_ADVERT_INTERVAL_MS * CODED_ONLY_CONFIRM_INTERVALS

    /** Minimum RSSI before a bulk transfer may raise a link from 1M to 2M.  */
    const val BULK_2M_MIN_RSSI = -80

    /** Connection admission control: max outbound CENTRAL connects we'll have in flight at once (connected but not yet
     *  qaul id resolved). Auto connect is gated on this. Prevents the scanner from piling on
     *  connects faster than the serial GATT queue can drain, which jams the queue with hung connectGatts,
     *  reaps connections before their READ_CHAR runs etc, and can wedge the whole BLE stack. */
    const val MAX_CONCURRENT_CONNECTING = 1

    /** Wrong role connect defer. When we discover a peer we should be PERIPHERAL to (their
     *  advertised qaul ID < ours), wait this long for them to connect to us before we connect
     *  outbound ourselves, after the window we connect anyway as a fallback.
     *
     *  Useful as it makes exactly one side of every pair the
     *  initiator, so a cluster forming at once produces  about half the connect attempts it otherwise
     *  would, with less load on the GATT queue and fewer 133s.
     *
    * If "Defer window lapsed" appears a lot in logs, the mechanism isn't earning its
     *  keep and should be removed rather than tuned. TODO: Check again if this seems to be correct, it might be different if scanner mode was lowered */
    const val WRONG_ROLE_DEFER_MS = 8_000L

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

    /** Dual-PHY advertising: on capable hardware (extended advertising + Coded PHY), advertise both a
     *  legacy 1M set (short range, discoverable by every device) and a Coded extended set (long range,
     *  discoverable only by BLE 5+ scanners). Non-capable devices fall back to legacy automatically. */
    const val DUAL_PHY_ADVERTISING = true

    /** Target MTU size to negotiate after connecting. Allows larger chunks than the 23-byte default. */
    const val TARGET_MTU = 517

    /** Default chunk size in bytes (Android default MTU 23 - 3 bytes GATT overhead = 20). */
    const val DEFAULT_CHUNK_SIZE = 20

    /** Message-size threshold (bytes)
     *  A message whose total size is at or below this rides the MEDIUM lane (routing updates, chat) so
     *  it stays ahead of large transfers, anything larger (images/files) rides the BULK lane.
     *  TODO: tune against real qaul routing-message sizes once measured */
    const val MEDIUM_MESSAGE_MAX_BYTES = 16000

    // Op timeouts. The base values are the short-range budgets validated in close range 4 device
    // runs. Coded links multiply them by [CODED_TIMEOUT_MULTIPLIER]
    //
    // The problem: a timeout that is too long stalls the  scheduler for
    // every peer until it expires, one that is too short force advances a still live op

    /** Watchdog timeout for fast GATT ops (reads, writes, notifies, MTU, descriptor, PHY).
     *  usually under 300ms on a close idle 1M link, but the budget must cover the worst case, and MTU
     *  requests were repeatedly observed exceeding 4s under real app load */
    const val FAST_OP_TIMEOUT_MS = 4_000L

    /** Budget for MTU and PHY negotiation. */
    const val NEGOTIATION_OP_TIMEOUT_MS = 10_000L

    /** Watchdog timeout for service discovery, the slowest non connect op and the most fragile at
     *  range, being many sequential round trips each of which can be lost. */
    const val SERVICE_DISCOVERY_TIMEOUT_MS = 5_000L

    /** Multiplier applied to op/handshake timeouts on a coded  link. Coded spends 8x
     *  the airtime per bit and needs more retransmissions at range */
    const val CODED_TIMEOUT_MULTIPLIER = 3

    /** Timeout in milliseconds for initial connection before giving up. */
    const val CONNECTION_TIMEOUT_MS = 8_000L

    /** How long with no data before a connection is considered dead and force-disconnected. */
    const val LIVENESS_TIMEOUT_MS = 16_000L

    /** How often we check if all connections are still alive */
    const val LIVENESS_CHECK_INTERVAL_MS = 5_000L

    /** How long a connection may stay unresolved (qaulId never learned) before the unresolved
     *  reaper drops it as a stuck handshake / zombie. */
    const val UNRESOLVED_TIMEOUT_MS = 6_000L

    /** How often an unresolved connection re-sends its SEND_ID. this handles a SEND_ID simply lost over the air, which is the dominant
     *  failure at range. Short enough for several attempts inside [UNRESOLVED_TIMEOUT_MS]. */
    const val IDENTITY_RETRY_MS = 1_500L

    const val PING_INTERVAL_MS = 5_000L

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

    // TODO: this needs reviewed
    const val CODED_RECONNECT_FREE_RETRIES = 5

    /** Minimum delay between reconnect attempts in milliseconds. */
    const val RECONNECT_DELAY_MIN_MS = 5_000L

    /** Maximum delay between reconnect attempts in milliseconds. */
    const val RECONNECT_DELAY_MAX_MS = 60_000L
    // TODO: this needs reviewed
    const val CODED_RECONNECT_DELAY_MAX_MS = 30_000L

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

    /** Neighbour link state settings **/
    private val neighbourSeq = java.util.concurrent.atomic.AtomicInteger(
        SecureRandom().nextInt(0x10000)
    )

    /**  Neighbour FLC time to live (max relay hops for link-state gossip) **/
    const val TTL = 3

    /** Re-broadcast our own neighbour list this often as a soft state keepalive, even when it hasn't
     *  changed. Needed so peers' link-state entries don't time out on a static mesh. */
    const val NEIGHBOUR_KEEPALIVE_MS = 15_000L

    /** Discard a gossiped link-state entry if no fresher update has arrived in this window
     */
    const val LINK_STATE_TIMEOUT_MS = 45_000L

    /** Call once per emit cycle when you originate a fresh neighbour list. Returns the new seq. */
    fun nextNeighbourSeq(): Int = neighbourSeq.incrementAndGet() and 0xFFFF  // 16 bit

    /** How long the neigbourhood has to be stable (no node has joined or left) before proactively dropping a link is considered. */
    const val T_STABLE_MS = 35_000L
}
