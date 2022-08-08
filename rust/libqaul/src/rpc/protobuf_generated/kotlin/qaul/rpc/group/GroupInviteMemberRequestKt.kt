//Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/group/group_rpc.proto

package qaul.rpc.group;

@kotlin.jvm.JvmName("-initializegroupInviteMemberRequest")
inline fun groupInviteMemberRequest(block: qaul.rpc.group.GroupInviteMemberRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest =
  qaul.rpc.group.GroupInviteMemberRequestKt.Dsl._create(qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.newBuilder()).apply { block() }._build()
object GroupInviteMemberRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  class Dsl private constructor(
    private val _builder: qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.Builder
  ) {
    companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest = _builder.build()

    /**
     * <pre>
     * group id
     * </pre>
     *
     * <code>bytes group_id = 1;</code>
     */
    var groupId: com.google.protobuf.ByteString
      @JvmName("getGroupId")
      get() = _builder.getGroupId()
      @JvmName("setGroupId")
      set(value) {
        _builder.setGroupId(value)
      }
    /**
     * <pre>
     * group id
     * </pre>
     *
     * <code>bytes group_id = 1;</code>
     */
    fun clearGroupId() {
      _builder.clearGroupId()
    }

    /**
     * <pre>
     * user id
     * </pre>
     *
     * <code>bytes user_id = 2;</code>
     */
    var userId: com.google.protobuf.ByteString
      @JvmName("getUserId")
      get() = _builder.getUserId()
      @JvmName("setUserId")
      set(value) {
        _builder.setUserId(value)
      }
    /**
     * <pre>
     * user id
     * </pre>
     *
     * <code>bytes user_id = 2;</code>
     */
    fun clearUserId() {
      _builder.clearUserId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
inline fun qaul.rpc.group.GroupRpc.GroupInviteMemberRequest.copy(block: qaul.rpc.group.GroupInviteMemberRequestKt.Dsl.() -> kotlin.Unit): qaul.rpc.group.GroupRpc.GroupInviteMemberRequest =
  qaul.rpc.group.GroupInviteMemberRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()
