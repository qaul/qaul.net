// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: router/router.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.router;

@kotlin.jvm.JvmName("-initializeroutingTableRequest")
public inline fun routingTableRequest(block: qaul.rpc.router.RoutingTableRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.router.RouterOuterClass.RoutingTableRequest =
  qaul.rpc.router.RoutingTableRequestKt.Dsl._create(qaul.rpc.router.RouterOuterClass.RoutingTableRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * UI request for routing table list
 * ```
 *
 * Protobuf type `qaul.rpc.router.RoutingTableRequest`
 */
public object RoutingTableRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.router.RouterOuterClass.RoutingTableRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.router.RouterOuterClass.RoutingTableRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.router.RouterOuterClass.RoutingTableRequest = _builder.build()
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.router.RouterOuterClass.RoutingTableRequest.copy(block: qaul.rpc.router.RoutingTableRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.router.RouterOuterClass.RoutingTableRequest =
  qaul.rpc.router.RoutingTableRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

