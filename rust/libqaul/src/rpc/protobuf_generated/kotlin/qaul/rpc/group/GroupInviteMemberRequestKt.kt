// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/group/group_rpc.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.group;

@kotlin.jvm.JvmName("-initializegroupInviteMemberRequest")
public inline fun groupInviteMemberRequest(block: qaul.rpc.group.GroupInviteMemberRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest =
  qaul.rpc.group.GroupInviteMemberRequestKt.Dsl._create(qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * Invite member
 * ```
 *
 * Protobuf type `qaul.rpc.group.GroupInviteMemberRequest`
 */
public object GroupInviteMemberRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest = _builder.build()

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
     * user id
     * ```
     *
     * `bytes user_id = 2;`
     */
    public var userId: com.google.protobuf.ByteString
      @JvmName("getUserId")
      get() = _builder.getUserId()
      @JvmName("setUserId")
      set(value) {
        _builder.setUserId(value)
      }
    /**
     * ```
     * user id
     * ```
     *
     * `bytes user_id = 2;`
     */
    public fun clearUserId() {
      _builder.clearUserId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.copy(block: qaul.rpc.group.GroupInviteMemberRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest =
  qaul.rpc.group.GroupInviteMemberRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

