// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/crypto/crypto_net.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.crypto;

@kotlin.jvm.JvmName("-initializesecondHandshake")
public inline fun secondHandshake(block: qaul.net.crypto.SecondHandshakeKt.Dsl.() -> kotlin.Unit): qaul.net.crypto.CryptoNet.SecondHandshake =
  qaul.net.crypto.SecondHandshakeKt.Dsl._create(qaul.net.crypto.CryptoNet.SecondHandshake.newBuilder()).apply { block() }._build()
/**
 * ```
 * Second Handshake Message
 * ```
 *
 * Protobuf type `qaul.net.crypto.SecondHandshake`
 */
public object SecondHandshakeKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.crypto.CryptoNet.SecondHandshake.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.crypto.CryptoNet.SecondHandshake.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.crypto.CryptoNet.SecondHandshake = _builder.build()

    /**
     * ```
     * confirm message ID of first handshake message
     * ```
     *
     * `bytes signature = 1;`
     */
    public var signature: com.google.protobuf.ByteString
      @JvmName("getSignature")
      get() = _builder.getSignature()
      @JvmName("setSignature")
      set(value) {
        _builder.setSignature(value)
      }
    /**
     * ```
     * confirm message ID of first handshake message
     * ```
     *
     * `bytes signature = 1;`
     */
    public fun clearSignature() {
      _builder.clearSignature()
    }

    /**
     * ```
     * received at timestamp
     * ```
     *
     * `uint64 received_at = 2;`
     */
    public var receivedAt: kotlin.Long
      @JvmName("getReceivedAt")
      get() = _builder.getReceivedAt()
      @JvmName("setReceivedAt")
      set(value) {
        _builder.setReceivedAt(value)
      }
    /**
     * ```
     * received at timestamp
     * ```
     *
     * `uint64 received_at = 2;`
     */
    public fun clearReceivedAt() {
      _builder.clearReceivedAt()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.crypto.CryptoNet.SecondHandshake.copy(block: qaul.net.crypto.SecondHandshakeKt.Dsl.() -> kotlin.Unit): qaul.net.crypto.CryptoNet.SecondHandshake =
  qaul.net.crypto.SecondHandshakeKt.Dsl._create(this.toBuilder()).apply { block() }._build()

