// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/feed/feed_net.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.feed;

@kotlin.jvm.JvmName("-initializefeedMessageContent")
public inline fun feedMessageContent(block: qaul.net.feed.FeedMessageContentKt.Dsl.() -> kotlin.Unit): qaul.net.feed.FeedNet.FeedMessageContent =
  qaul.net.feed.FeedMessageContentKt.Dsl._create(qaul.net.feed.FeedNet.FeedMessageContent.newBuilder()).apply { block() }._build()
/**
 * ```
 * Feed Message Content
 * ```
 *
 * Protobuf type `qaul.net.feed.FeedMessageContent`
 */
public object FeedMessageContentKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.feed.FeedNet.FeedMessageContent.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.feed.FeedNet.FeedMessageContent.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.feed.FeedNet.FeedMessageContent = _builder.build()

    /**
     * ```
     * sender id
     * ```
     *
     * `bytes sender = 1;`
     */
    public var sender: com.google.protobuf.ByteString
      @JvmName("getSender")
      get() = _builder.getSender()
      @JvmName("setSender")
      set(value) {
        _builder.setSender(value)
      }
    /**
     * ```
     * sender id
     * ```
     *
     * `bytes sender = 1;`
     */
    public fun clearSender() {
      _builder.clearSender()
    }

    /**
     * ```
     * message content
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
     * message content
     * ```
     *
     * `string content = 2;`
     */
    public fun clearContent() {
      _builder.clearContent()
    }

    /**
     * ```
     * timestamp in milliseconds
     * ```
     *
     * `uint64 time = 3;`
     */
    public var time: kotlin.Long
      @JvmName("getTime")
      get() = _builder.getTime()
      @JvmName("setTime")
      set(value) {
        _builder.setTime(value)
      }
    /**
     * ```
     * timestamp in milliseconds
     * ```
     *
     * `uint64 time = 3;`
     */
    public fun clearTime() {
      _builder.clearTime()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.feed.FeedNet.FeedMessageContent.copy(block: qaul.net.feed.FeedMessageContentKt.Dsl.() -> kotlin.Unit): qaul.net.feed.FeedNet.FeedMessageContent =
  qaul.net.feed.FeedMessageContentKt.Dsl._create(this.toBuilder()).apply { block() }._build()

