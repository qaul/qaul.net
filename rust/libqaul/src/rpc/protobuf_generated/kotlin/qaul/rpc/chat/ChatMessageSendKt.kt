// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/chat/chat.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.chat;

@kotlin.jvm.JvmName("-initializechatMessageSend")
public inline fun chatMessageSend(block: qaul.rpc.chat.ChatMessageSendKt.Dsl.() -> kotlin.Unit): qaul.rpc.chat.ChatOuterClass.ChatMessageSend =
  qaul.rpc.chat.ChatMessageSendKt.Dsl._create(qaul.rpc.chat.ChatOuterClass.ChatMessageSend.newBuilder()).apply { block() }._build()
/**
 * ```
 * send chat message
 * ```
 *
 * Protobuf type `qaul.rpc.chat.ChatMessageSend`
 */
public object ChatMessageSendKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.chat.ChatOuterClass.ChatMessageSend.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.chat.ChatOuterClass.ChatMessageSend.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.chat.ChatOuterClass.ChatMessageSend = _builder.build()

    /**
     * ```
     * group id to which this message is sent
     * ```
     *
     * `bytes group_id = 1;`
     */
    public var groupId: com.google.protobuf.ByteString
      @JvmName("getGroupId")
      get() = _builder.getGroupId()
      @JvmName("setGroupId")
      set(value) {
        _builder.setGroupId(value)
      }
    /**
     * ```
     * group id to which this message is sent
     * ```
     *
     * `bytes group_id = 1;`
     */
    public fun clearGroupId() {
      _builder.clearGroupId()
    }

    /**
     * ```
     * content of the message
     * ```
     *
     * `string content = 2;`
     */
    public var content: kotlin.String
      @JvmName("getContent")
      get() = _builder.getContent()
      @JvmName("setContent")
      set(value) {
        _builder.setContent(value)
      }
    /**
     * ```
     * content of the message
     * ```
     *
     * `string content = 2;`
     */
    public fun clearContent() {
      _builder.clearContent()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.chat.ChatOuterClass.ChatMessageSend.copy(block: qaul.rpc.chat.ChatMessageSendKt.Dsl.() -> kotlin.Unit): qaul.rpc.chat.ChatOuterClass.ChatMessageSend =
  qaul.rpc.chat.ChatMessageSendKt.Dsl._create(this.toBuilder()).apply { block() }._build()

