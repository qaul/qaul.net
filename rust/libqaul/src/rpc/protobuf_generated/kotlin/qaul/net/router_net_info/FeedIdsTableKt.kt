// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: router/router_net_info.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.router_net_info;

@kotlin.jvm.JvmName("-initializefeedIdsTable")
public inline fun feedIdsTable(block: qaul.net.router_net_info.FeedIdsTableKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.FeedIdsTable =
  qaul.net.router_net_info.FeedIdsTableKt.Dsl._create(qaul.net.router_net_info.RouterNetInfo.FeedIdsTable.newBuilder()).apply { block() }._build()
/**
 * ```
 * List of feed ID's
 * ```
 *
 * Protobuf type `qaul.net.router_net_info.FeedIdsTable`
 */
public object FeedIdsTableKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.router_net_info.RouterNetInfo.FeedIdsTable.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.router_net_info.RouterNetInfo.FeedIdsTable.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.router_net_info.RouterNetInfo.FeedIdsTable = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class IdsProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     */
     public val ids: com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getIdsList()
      )
    /**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     * @param value The ids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addIds")
    public fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.add(value: com.google.protobuf.ByteString) {
      _builder.addIds(value)
    }/**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     * @param value The ids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignIds")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.plusAssign(value: com.google.protobuf.ByteString) {
      add(value)
    }/**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     * @param values The ids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllIds")
    public fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.addAll(values: kotlin.collections.Iterable<com.google.protobuf.ByteString>) {
      _builder.addAllIds(values)
    }/**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     * @param values The ids to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllIds")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.plusAssign(values: kotlin.collections.Iterable<com.google.protobuf.ByteString>) {
      addAll(values)
    }/**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     * @param index The index to set the value at.
     * @param value The ids to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setIds")
    public operator fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.set(index: kotlin.Int, value: com.google.protobuf.ByteString) {
      _builder.setIds(index, value)
    }/**
     * ```
     * feed id
     * ```
     *
     * `repeated bytes ids = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearIds")
    public fun com.google.protobuf.kotlin.DslList<com.google.protobuf.ByteString, IdsProxy>.clear() {
      _builder.clearIds()
    }}
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.router_net_info.RouterNetInfo.FeedIdsTable.copy(block: qaul.net.router_net_info.FeedIdsTableKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.FeedIdsTable =
  qaul.net.router_net_info.FeedIdsTableKt.Dsl._create(this.toBuilder()).apply { block() }._build()

