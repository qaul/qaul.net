// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/group/group_net.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.group;

@kotlin.jvm.JvmName("-initializeremovedMember")
public inline fun removedMember(block: qaul.net.group.RemovedMemberKt.Dsl.() -> kotlin.Unit): qaul.net.group.GroupNet.RemovedMember =
  qaul.net.group.RemovedMemberKt.Dsl._create(qaul.net.group.GroupNet.RemovedMember.newBuilder()).apply { block() }._build()
/**
 * ```
 * Removed member
 * ```
 *
 * Protobuf type `qaul.net.group.RemovedMember`
 */
public object RemovedMemberKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.group.GroupNet.RemovedMember.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.group.GroupNet.RemovedMember.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.group.GroupNet.RemovedMember = _builder.build()

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
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.group.GroupNet.RemovedMember.copy(block: qaul.net.group.RemovedMemberKt.Dsl.() -> kotlin.Unit): qaul.net.group.GroupNet.RemovedMember =
  qaul.net.group.RemovedMemberKt.Dsl._create(this.toBuilder()).apply { block() }._build()

