//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/feed/feed.proto

package qaul.rpc.feed;

@kotlin.jvm.JvmSynthetic
inline fun sendMessage(block: qaul.rpc.feed.SendMessageKt.Dsl.() -> Unit): qaul.rpc.feed.FeedOuterClass.SendMessage =
  qaul.rpc.feed.SendMessageKt.Dsl._create(qaul.rpc.feed.FeedOuterClass.SendMessage.newBuilder()).apply { block() }._build()
object SendMessageKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: qaul.rpc.feed.FeedOuterClass.SendMessage.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.feed.FeedOuterClass.SendMessage.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.feed.FeedOuterClass.SendMessage = _builder.build()

    /**
     * <code>string content = 1;</code>
     */
    var content: kotlin.String
      @JvmName("getContent")
      get() = _builder.getContent()
      @JvmName("setContent")
      set(value) {
        _builder.setContent(value)
      }
    /**
     * <code>string content = 1;</code>
     */
    fun clearContent() {
      _builder.clearContent()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun qaul.rpc.feed.FeedOuterClass.SendMessage.copy(block: qaul.rpc.feed.SendMessageKt.Dsl.() -> Unit): qaul.rpc.feed.FeedOuterClass.SendMessage =
  qaul.rpc.feed.SendMessageKt.Dsl._create(this.toBuilder()).apply { block() }._build()
