// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/dtn/dtn_rpc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.dtn;

@kotlin.jvm.JvmName("-initializedtnStateResponse")
public inline fun dtnStateResponse(block: qaul.rpc.dtn.DtnStateResponseKt.Dsl.() -> kotlin.Unit): qaul.rpc.dtn.DtnRpc.DtnStateResponse =
  qaul.rpc.dtn.DtnStateResponseKt.Dsl._create(qaul.rpc.dtn.DtnRpc.DtnStateResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * Dtn State Response
 * ```
 *
 * Protobuf type `qaul.rpc.dtn.DtnStateResponse`
 */
public object DtnStateResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.dtn.DtnRpc.DtnStateResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.dtn.DtnRpc.DtnStateResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.dtn.DtnRpc.DtnStateResponse = _builder.build()

    /**
     * ```
     * used size
     * ```
     *
     * `uint64 used_size = 1;`
     */
    public var usedSize: kotlin.Long
      @JvmName("getUsedSize")
      get() = _builder.getUsedSize()
      @JvmName("setUsedSize")
      set(value) {
        _builder.setUsedSize(value)
      }
    /**
     * ```
     * used size
     * ```
     *
     * `uint64 used_size = 1;`
     */
    public fun clearUsedSize() {
      _builder.clearUsedSize()
    }

    /**
     * ```
     * dtn message count
     * ```
     *
     * `uint32 dtn_message_count = 2;`
     */
    public var dtnMessageCount: kotlin.Int
      @JvmName("getDtnMessageCount")
      get() = _builder.getDtnMessageCount()
      @JvmName("setDtnMessageCount")
      set(value) {
        _builder.setDtnMessageCount(value)
      }
    /**
     * ```
     * dtn message count
     * ```
     *
     * `uint32 dtn_message_count = 2;`
     */
    public fun clearDtnMessageCount() {
      _builder.clearDtnMessageCount()
    }

    /**
     * ```
     * unconfirmed count
     * ```
     *
     * `uint32 unconfirmed_count = 3;`
     */
    public var unconfirmedCount: kotlin.Int
      @JvmName("getUnconfirmedCount")
      get() = _builder.getUnconfirmedCount()
      @JvmName("setUnconfirmedCount")
      set(value) {
        _builder.setUnconfirmedCount(value)
      }
    /**
     * ```
     * unconfirmed count
     * ```
     *
     * `uint32 unconfirmed_count = 3;`
     */
    public fun clearUnconfirmedCount() {
      _builder.clearUnconfirmedCount()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.dtn.DtnRpc.DtnStateResponse.copy(block: qaul.rpc.dtn.DtnStateResponseKt.Dsl.() -> kotlin.Unit): qaul.rpc.dtn.DtnRpc.DtnStateResponse =
  qaul.rpc.dtn.DtnStateResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

