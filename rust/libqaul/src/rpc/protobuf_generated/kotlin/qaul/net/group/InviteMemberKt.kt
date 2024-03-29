// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: services/group/group_net.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.net.group;

@kotlin.jvm.JvmName("-initializeinviteMember")
public inline fun inviteMember(block: qaul.net.group.InviteMemberKt.Dsl.() -> kotlin.Unit): qaul.net.group.GroupNet.InviteMember =
  qaul.net.group.InviteMemberKt.Dsl._create(qaul.net.group.GroupNet.InviteMember.newBuilder()).apply { block() }._build()
/**
 * ```
 * Invite member
 * ```
 *
 * Protobuf type `qaul.net.group.InviteMember`
 */
public object InviteMemberKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.net.group.GroupNet.InviteMember.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.net.group.GroupNet.InviteMember.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.net.group.GroupNet.InviteMember = _builder.build()

    /**
     * ```
     * Group Info
     * ```
     *
     * `.qaul.net.group.GroupInfo group = 1;`
     */
    public var group: qaul.net.group.GroupNet.GroupInfo
      @JvmName("getGroup")
      get() = _builder.getGroup()
      @JvmName("setGroup")
      set(value) {
        _builder.setGroup(value)
      }
    /**
     * ```
     * Group Info
     * ```
     *
     * `.qaul.net.group.GroupInfo group = 1;`
     */
    public fun clearGroup() {
      _builder.clearGroup()
    }
    /**
     * ```
     * Group Info
     * ```
     *
     * `.qaul.net.group.GroupInfo group = 1;`
     * @return Whether the group field is set.
     */
    public fun hasGroup(): kotlin.Boolean {
      return _builder.hasGroup()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.net.group.GroupNet.InviteMember.copy(block: qaul.net.group.InviteMemberKt.Dsl.() -> kotlin.Unit): qaul.net.group.GroupNet.InviteMember =
  qaul.net.group.InviteMemberKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val qaul.net.group.GroupNet.InviteMemberOrBuilder.groupOrNull: qaul.net.group.GroupNet.GroupInfo?
  get() = if (hasGroup()) getGroup() else null

