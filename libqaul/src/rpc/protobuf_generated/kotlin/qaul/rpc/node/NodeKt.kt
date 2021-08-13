//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: node/node.proto

package qaul.rpc.node;

@kotlin.jvm.JvmSynthetic
inline fun node(block: qaul.rpc.node.NodeKt.Dsl.() -> Unit): qaul.rpc.node.NodeOuterClass.Node =
  qaul.rpc.node.NodeKt.Dsl._create(qaul.rpc.node.NodeOuterClass.Node.newBuilder()).apply { block() }._build()
object NodeKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    @kotlin.jvm.JvmField private val _builder: qaul.rpc.node.NodeOuterClass.Node.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.node.NodeOuterClass.Node.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.node.NodeOuterClass.Node = _builder.build()

    /**
     * <code>bool get_node_info = 1;</code>
     */
    var getNodeInfo: kotlin.Boolean
      @JvmName("getGetNodeInfo")
      get() = _builder.getGetNodeInfo()
      @JvmName("setGetNodeInfo")
      set(value) {
        _builder.setGetNodeInfo(value)
      }
    /**
     * <code>bool get_node_info = 1;</code>
     */
    fun clearGetNodeInfo() {
      _builder.clearGetNodeInfo()
    }
    /**
     * <code>bool get_node_info = 1;</code>
     * @return Whether the getNodeInfo field is set.
     */
    fun hasGetNodeInfo(): kotlin.Boolean {
      return _builder.hasGetNodeInfo()
    }

    /**
     * <code>.qaul.rpc.node.NodeInformation info = 2;</code>
     */
    var info: qaul.rpc.node.NodeOuterClass.NodeInformation
      @JvmName("getInfo")
      get() = _builder.getInfo()
      @JvmName("setInfo")
      set(value) {
        _builder.setInfo(value)
      }
    /**
     * <code>.qaul.rpc.node.NodeInformation info = 2;</code>
     */
    fun clearInfo() {
      _builder.clearInfo()
    }
    /**
     * <code>.qaul.rpc.node.NodeInformation info = 2;</code>
     * @return Whether the info field is set.
     */
    fun hasInfo(): kotlin.Boolean {
      return _builder.hasInfo()
    }
    val messageCase: qaul.rpc.node.NodeOuterClass.Node.MessageCase
      @JvmName("getMessageCase")
      get() = _builder.getMessageCase()

    fun clearMessage() {
      _builder.clearMessage()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun qaul.rpc.node.NodeOuterClass.Node.copy(block: qaul.rpc.node.NodeKt.Dsl.() -> Unit): qaul.rpc.node.NodeOuterClass.Node =
  qaul.rpc.node.NodeKt.Dsl._create(this.toBuilder()).apply { block() }._build()
