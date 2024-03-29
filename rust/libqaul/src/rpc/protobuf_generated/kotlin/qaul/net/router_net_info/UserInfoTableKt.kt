// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: router/router_net_info.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.router_net_info;

@kotlin.jvm.JvmName("-initializeuserInfoTable")
public inline fun userInfoTable(block: qaul.net.router_net_info.UserInfoTableKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.UserInfoTable =
  qaul.net.router_net_info.UserInfoTableKt.Dsl._create(qaul.net.router_net_info.RouterNetInfo.UserInfoTable.newBuilder()).apply { block() }._build()
/**
 * ```
 * User information table
 * ```
 *
 * Protobuf type `qaul.net.router_net_info.UserInfoTable`
 */
public object UserInfoTableKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.router_net_info.RouterNetInfo.UserInfoTable.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.router_net_info.RouterNetInfo.UserInfoTable.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.router_net_info.RouterNetInfo.UserInfoTable = _builder.build()

    /**
     * An uninstantiable, behaviorless type to represent the field in
     * generics.
     */
    @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
    public class InfoProxy private constructor() : com.google.protobuf.kotlin.DslProxy()
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     */
     public val info: com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>
      @kotlin.jvm.JvmSynthetic
      get() = com.google.protobuf.kotlin.DslList(
        _builder.getInfoList()
      )
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     * @param value The info to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addInfo")
    public fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.add(value: qaul.net.router_net_info.RouterNetInfo.UserInfo) {
      _builder.addInfo(value)
    }
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     * @param value The info to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignInfo")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.plusAssign(value: qaul.net.router_net_info.RouterNetInfo.UserInfo) {
      add(value)
    }
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     * @param values The info to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("addAllInfo")
    public fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.addAll(values: kotlin.collections.Iterable<qaul.net.router_net_info.RouterNetInfo.UserInfo>) {
      _builder.addAllInfo(values)
    }
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     * @param values The info to add.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("plusAssignAllInfo")
    @Suppress("NOTHING_TO_INLINE")
    public inline operator fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.plusAssign(values: kotlin.collections.Iterable<qaul.net.router_net_info.RouterNetInfo.UserInfo>) {
      addAll(values)
    }
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     * @param index The index to set the value at.
     * @param value The info to set.
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("setInfo")
    public operator fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.set(index: kotlin.Int, value: qaul.net.router_net_info.RouterNetInfo.UserInfo) {
      _builder.setInfo(index, value)
    }
    /**
     * ```
     * user info
     * ```
     *
     * `repeated .qaul.net.router_net_info.UserInfo info = 1;`
     */
    @kotlin.jvm.JvmSynthetic
    @kotlin.jvm.JvmName("clearInfo")
    public fun com.google.protobuf.kotlin.DslList<qaul.net.router_net_info.RouterNetInfo.UserInfo, InfoProxy>.clear() {
      _builder.clearInfo()
    }

  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.router_net_info.RouterNetInfo.UserInfoTable.copy(block: qaul.net.router_net_info.UserInfoTableKt.Dsl.() -> kotlin.Unit): qaul.net.router_net_info.RouterNetInfo.UserInfoTable =
  qaul.net.router_net_info.UserInfoTableKt.Dsl._create(this.toBuilder()).apply { block() }._build()

