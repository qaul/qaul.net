// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: router/router.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.router;

@kotlin.jvm.JvmName("-initializeroutingTableConnection")
public inline fun routingTableConnection(block: qaul.rpc.router.RoutingTableConnectionKt.Dsl.() -> kotlin.Unit): qaul.rpc.router.RouterOuterClass.RoutingTableConnection =
  qaul.rpc.router.RoutingTableConnectionKt.Dsl._create(qaul.rpc.router.RouterOuterClass.RoutingTableConnection.newBuilder()).apply { block() }._build()
/**
 * ```
 * Routing table connection entry.
 * This message contains a connection to a specific user.
 * ```
 *
 * Protobuf type `qaul.rpc.router.RoutingTableConnection`
 */
public object RoutingTableConnectionKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.router.RouterOuterClass.RoutingTableConnection.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.router.RouterOuterClass.RoutingTableConnection.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.router.RouterOuterClass.RoutingTableConnection = _builder.build()

    /**
     * ```
     * the connection module (LAN, Internet, BLE, etc.)
     * ```
     *
     * `.qaul.rpc.router.ConnectionModule module = 2;`
     */
    public var module: qaul.rpc.router.RouterOuterClass.ConnectionModule
      @JvmName("getModule")
      get() = _builder.getModule()
      @JvmName("setModule")
      set(value) {
        _builder.setModule(value)
      }
    public var moduleValue: kotlin.Int
      @JvmName("getModuleValue")
      get() = _builder.getModuleValue()
      @JvmName("setModuleValue")
      set(value) {
        _builder.setModuleValue(value)
      }
    /**
     * ```
     * the connection module (LAN, Internet, BLE, etc.)
     * ```
     *
     * `.qaul.rpc.router.ConnectionModule module = 2;`
     */
    public fun clearModule() {
      _builder.clearModule()
    }

    /**
     * ```
     * the round trip time for this connection
     * ```
     *
     * `uint32 rtt = 3;`
     */
    public var rtt: kotlin.Int
      @JvmName("getRtt")
      get() = _builder.getRtt()
      @JvmName("setRtt")
      set(value) {
        _builder.setRtt(value)
      }
    /**
     * ```
     * the round trip time for this connection
     * ```
     *
     * `uint32 rtt = 3;`
     */
    public fun clearRtt() {
      _builder.clearRtt()
    }

    /**
     * ```
     * hop count
     * ```
     *
     * `uint32 hop_count = 5;`
     */
    public var hopCount: kotlin.Int
      @JvmName("getHopCount")
      get() = _builder.getHopCount()
      @JvmName("setHopCount")
      set(value) {
        _builder.setHopCount(value)
      }
    /**
     * ```
     * hop count
     * ```
     *
     * `uint32 hop_count = 5;`
     */
    public fun clearHopCount() {
      _builder.clearHopCount()
    }

    /**
     * ```
     * node id via which this connection is routed
     * ```
     *
     * `bytes via = 4;`
     */
    public var via: com.google.protobuf.ByteString
      @JvmName("getVia")
      get() = _builder.getVia()
      @JvmName("setVia")
      set(value) {
        _builder.setVia(value)
      }
    /**
     * ```
     * node id via which this connection is routed
     * ```
     *
     * `bytes via = 4;`
     */
    public fun clearVia() {
      _builder.clearVia()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.router.RouterOuterClass.RoutingTableConnection.copy(block: qaul.rpc.router.RoutingTableConnectionKt.Dsl.() -> kotlin.Unit): qaul.rpc.router.RouterOuterClass.RoutingTableConnection =
  qaul.rpc.router.RoutingTableConnectionKt.Dsl._create(this.toBuilder()).apply { block() }._build()

