// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/group/group_rpc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.group;

@kotlin.jvm.JvmName("-initializegroupReplyInviteResponse")
public inline fun groupReplyInviteResponse(block: qaul.rpc.group.GroupReplyInviteResponseKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupReplyInviteResponse =
  qaul.rpc.group.GroupReplyInviteResponseKt.Dsl._create(qaul.rpc.group.GroupRpc.GroupReplyInviteResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * Reply Invite Response
 * ```
 *
 * Protobuf type `qaul.rpc.group.GroupReplyInviteResponse`
 */
public object GroupReplyInviteResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.group.GroupRpc.GroupReplyInviteResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.group.GroupRpc.GroupReplyInviteResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.group.GroupRpc.GroupReplyInviteResponse = _builder.build()

    /**
     * ```
     * group id
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
     * group id
     * ```
     *
     * `bytes group_id = 1;`
     */
    public fun clearGroupId() {
      _builder.clearGroupId()
    }

    /**
     * ```
     * result
     * ```
     *
     * `.qaul.rpc.group.GroupResult result = 3;`
     */
    public var result: qaul.rpc.group.GroupRpc.GroupResult
      @JvmName("getResult")
      get() = _builder.getResult()
      @JvmName("setResult")
      set(value) {
        _builder.setResult(value)
      }
    /**
     * ```
     * result
     * ```
     *
     * `.qaul.rpc.group.GroupResult result = 3;`
     */
    public fun clearResult() {
      _builder.clearResult()
    }
    /**
     * ```
     * result
     * ```
     *
     * `.qaul.rpc.group.GroupResult result = 3;`
     * @return Whether the result field is set.
     */
    public fun hasResult(): kotlin.Boolean {
      return _builder.hasResult()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.group.GroupRpc.GroupReplyInviteResponse.copy(block: qaul.rpc.group.GroupReplyInviteResponseKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupReplyInviteResponse =
  qaul.rpc.group.GroupReplyInviteResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val qaul.rpc.group.GroupRpc.GroupReplyInviteResponseOrBuilder.resultOrNull: qaul.rpc.group.GroupRpc.GroupResult?
  get() = if (hasResult()) getResult() else null

