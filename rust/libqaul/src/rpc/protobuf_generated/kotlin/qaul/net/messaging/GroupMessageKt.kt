// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/messaging/messaging.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.messaging;

@kotlin.jvm.JvmName("-initializegroupMessage")
public inline fun groupMessage(block: qaul.net.messaging.GroupMessageKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.GroupMessage =
  qaul.net.messaging.GroupMessageKt.Dsl._create(qaul.net.messaging.MessagingOuterClass.GroupMessage.newBuilder()).apply { block() }._build()
/**
 * ```
 * group message
 * ```
 *
 * Protobuf type `qaul.net.messaging.GroupMessage`
 */
public object GroupMessageKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.messaging.MessagingOuterClass.GroupMessage.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.messaging.MessagingOuterClass.GroupMessage.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.messaging.MessagingOuterClass.GroupMessage = _builder.build()

    /**
     * `bytes content = 1;`
     */
    public var content: com.google.protobuf.ByteString
      @JvmName("getContent")
      get() = _builder.getContent()
      @JvmName("setContent")
      set(value) {
        _builder.setContent(value)
      }
    /**
     * `bytes content = 1;`
     */
    public fun clearContent() {
      _builder.clearContent()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.messaging.MessagingOuterClass.GroupMessage.copy(block: qaul.net.messaging.GroupMessageKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.GroupMessage =
  qaul.net.messaging.GroupMessageKt.Dsl._create(this.toBuilder()).apply { block() }._build()

