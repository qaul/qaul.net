// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/messaging/messaging.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.messaging;

@kotlin.jvm.JvmName("-initializegroupInviteMessage")
public inline fun groupInviteMessage(block: qaul.net.messaging.GroupInviteMessageKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.GroupInviteMessage =
  qaul.net.messaging.GroupInviteMessageKt.Dsl._create(qaul.net.messaging.MessagingOuterClass.GroupInviteMessage.newBuilder()).apply { block() }._build()
/**
 * ```
 * group invite message
 * ```
 *
 * Protobuf type `qaul.net.messaging.GroupInviteMessage`
 */
public object GroupInviteMessageKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.messaging.MessagingOuterClass.GroupInviteMessage.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.messaging.MessagingOuterClass.GroupInviteMessage.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.messaging.MessagingOuterClass.GroupInviteMessage = _builder.build()

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
public inline fun qaul.net.messaging.MessagingOuterClass.GroupInviteMessage.copy(block: qaul.net.messaging.GroupInviteMessageKt.Dsl.() -> kotlin.Unit): qaul.net.messaging.MessagingOuterClass.GroupInviteMessage =
  qaul.net.messaging.GroupInviteMessageKt.Dsl._create(this.toBuilder()).apply { block() }._build()

