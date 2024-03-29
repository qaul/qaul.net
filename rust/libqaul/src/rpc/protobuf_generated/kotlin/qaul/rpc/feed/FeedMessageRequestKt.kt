// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/feed/feed.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.feed;

@kotlin.jvm.JvmName("-initializefeedMessageRequest")
public inline fun feedMessageRequest(block: qaul.rpc.feed.FeedMessageRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.feed.FeedOuterClass.FeedMessageRequest =
  qaul.rpc.feed.FeedMessageRequestKt.Dsl._create(qaul.rpc.feed.FeedOuterClass.FeedMessageRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * request feed messages
 * ```
 *
 * Protobuf type `qaul.rpc.feed.FeedMessageRequest`
 */
public object FeedMessageRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.feed.FeedOuterClass.FeedMessageRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.feed.FeedOuterClass.FeedMessageRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.feed.FeedOuterClass.FeedMessageRequest = _builder.build()

    /**
     * ```
     * DEPRECATED
     * ```
     *
     * `bytes last_received = 1;`
     */
    public var lastReceived: com.google.protobuf.ByteString
      @JvmName("getLastReceived")
      get() = _builder.getLastReceived()
      @JvmName("setLastReceived")
      set(value) {
        _builder.setLastReceived(value)
      }
    /**
     * ```
     * DEPRECATED
     * ```
     *
     * `bytes last_received = 1;`
     */
    public fun clearLastReceived() {
      _builder.clearLastReceived()
    }

    /**
     * ```
     * Index of the last message received
     *
     * The message index is a continues numbering
     * of incoming messages in the database of the node.
     *
     * When this variable is set, only 
     * newer messages will be sent.
     * Default value is 0, when the value
     * is 0, all feed messages will be sent.
     * ```
     *
     * `uint64 last_index = 2;`
     */
    public var lastIndex: kotlin.Long
      @JvmName("getLastIndex")
      get() = _builder.getLastIndex()
      @JvmName("setLastIndex")
      set(value) {
        _builder.setLastIndex(value)
      }
    /**
     * ```
     * Index of the last message received
     *
     * The message index is a continues numbering
     * of incoming messages in the database of the node.
     *
     * When this variable is set, only 
     * newer messages will be sent.
     * Default value is 0, when the value
     * is 0, all feed messages will be sent.
     * ```
     *
     * `uint64 last_index = 2;`
     */
    public fun clearLastIndex() {
      _builder.clearLastIndex()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.feed.FeedOuterClass.FeedMessageRequest.copy(block: qaul.rpc.feed.FeedMessageRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.feed.FeedOuterClass.FeedMessageRequest =
  qaul.rpc.feed.FeedMessageRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

