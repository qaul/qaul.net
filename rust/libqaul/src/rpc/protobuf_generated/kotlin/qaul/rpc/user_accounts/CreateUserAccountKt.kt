// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: node/user_accounts.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.user_accounts;

@kotlin.jvm.JvmName("-initializecreateUserAccount")
public inline fun createUserAccount(block: qaul.rpc.user_accounts.CreateUserAccountKt.Dsl.() -> kotlin.Unit): qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount =
  qaul.rpc.user_accounts.CreateUserAccountKt.Dsl._create(qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount.newBuilder()).apply { block() }._build()
/**
 * ```
 * create a new user on this node
 * ```
 *
 * Protobuf type `qaul.rpc.user_accounts.CreateUserAccount`
 */
public object CreateUserAccountKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount = _builder.build()

    /**
     * `string name = 1;`
     */
    public var name: kotlin.String
      @JvmName("getName")
      get() = _builder.getName()
      @JvmName("setName")
      set(value) {
        _builder.setName(value)
      }
    /**
     * `string name = 1;`
     */
    public fun clearName() {
      _builder.clearName()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount.copy(block: qaul.rpc.user_accounts.CreateUserAccountKt.Dsl.() -> kotlin.Unit): qaul.rpc.user_accounts.UserAccountsOuterClass.CreateUserAccount =
  qaul.rpc.user_accounts.CreateUserAccountKt.Dsl._create(this.toBuilder()).apply { block() }._build()

