// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: router/router_net_info.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.router_net_info;

@kotlin.jvm.JvmName("-initializerouterInfoMessage")
public inline fun routerInfoMessage(block: qaul.net.router_net_info.RouterInfoMessageKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage =
  qaul.net.router_net_info.RouterInfoMessageKt.Dsl._create(qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage.newBuilder()).apply { block() }._build()
/**
 * ```
 * Router information message
 * ```
 *
 * Protobuf type `qaul.net.router_net_info.RouterInfoMessage`
 */
public object RouterInfoMessageKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage = _builder.build()

    /**
     * ```
     * node id
     * ```
     *
     * `bytes node = 1;`
     */
    public var node: com.google.protobuf.ByteString
      @JvmName("getNode")
      get() = _builder.getNode()
      @JvmName("setNode")
      set(value) {
        _builder.setNode(value)
      }
    /**
     * ```
     * node id
     * ```
     *
     * `bytes node = 1;`
     */
    public fun clearNode() {
      _builder.clearNode()
    }

    /**
     * ```
     * Routing information table
     * ```
     *
     * `.qaul.net.router_net_info.RoutingInfoTable routes = 2;`
     */
    public var routes: qaul.net.router_net_info.RouterNetInfo.RoutingInfoTable
      @JvmName("getRoutes")
      get() = _builder.getRoutes()
      @JvmName("setRoutes")
      set(value) {
        _builder.setRoutes(value)
      }
    /**
     * ```
     * Routing information table
     * ```
     *
     * `.qaul.net.router_net_info.RoutingInfoTable routes = 2;`
     */
    public fun clearRoutes() {
      _builder.clearRoutes()
    }
    /**
     * ```
     * Routing information table
     * ```
     *
     * `.qaul.net.router_net_info.RoutingInfoTable routes = 2;`
     * @return Whether the routes field is set.
     */
    public fun hasRoutes(): kotlin.Boolean {
      return _builder.hasRoutes()
    }

    /**
     * ```
     * Latest Feed ids table
     * ```
     *
     * `.qaul.net.router_net_info.FeedIdsTable feeds = 4;`
     */
    public var feeds: qaul.net.router_net_info.RouterNetInfo.FeedIdsTable
      @JvmName("getFeeds")
      get() = _builder.getFeeds()
      @JvmName("setFeeds")
      set(value) {
        _builder.setFeeds(value)
      }
    /**
     * ```
     * Latest Feed ids table
     * ```
     *
     * `.qaul.net.router_net_info.FeedIdsTable feeds = 4;`
     */
    public fun clearFeeds() {
      _builder.clearFeeds()
    }
    /**
     * ```
     * Latest Feed ids table
     * ```
     *
     * `.qaul.net.router_net_info.FeedIdsTable feeds = 4;`
     * @return Whether the feeds field is set.
     */
    public fun hasFeeds(): kotlin.Boolean {
      return _builder.hasFeeds()
    }

    /**
     * ```
     * timestamp
     * ```
     *
     * `uint64 timestamp = 5;`
     */
    public var timestamp: kotlin.Long
      @JvmName("getTimestamp")
      get() = _builder.getTimestamp()
      @JvmName("setTimestamp")
      set(value) {
        _builder.setTimestamp(value)
      }
    /**
     * ```
     * timestamp
     * ```
     *
     * `uint64 timestamp = 5;`
     */
    public fun clearTimestamp() {
      _builder.clearTimestamp()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage.copy(block: qaul.net.router_net_info.RouterInfoMessageKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.RouterInfoMessage =
  qaul.net.router_net_info.RouterInfoMessageKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val qaul.net.router_net_info.RouterNetInfo.RouterInfoMessageOrBuilder.routesOrNull: qaul.net.router_net_info.RouterNetInfo.RoutingInfoTable?
  get() = if (hasRoutes()) getRoutes() else null

public val qaul.net.router_net_info.RouterNetInfo.RouterInfoMessageOrBuilder.feedsOrNull: qaul.net.router_net_info.RouterNetInfo.FeedIdsTable?
  get() = if (hasFeeds()) getFeeds() else null

