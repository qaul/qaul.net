// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: node/user_accounts.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.rpc.user_accounts;

@kotlin.jvm.JvmName("-initializedefaultUserAccount")
public inline fun defaultUserAccount(block: qaul.rpc.user_accounts.DefaultUserAccountKt.Dsl.() -> kotlin.Unit): qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount =
  qaul.rpc.user_accounts.DefaultUserAccountKt.Dsl._create(qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount.newBuilder()).apply { block() }._build()
/**
 * ```
 * Session Information
 * ```
 *
 * Protobuf type `qaul.rpc.user_accounts.DefaultUserAccount`
 */
public object DefaultUserAccountKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount = _builder.build()

    /**
     * `bool user_account_exists = 1;`
     */
    public var userAccountExists: kotlin.Boolean
      @JvmName("getUserAccountExists")
      get() = _builder.getUserAccountExists()
      @JvmName("setUserAccountExists")
      set(value) {
        _builder.setUserAccountExists(value)
      }
    /**
     * `bool user_account_exists = 1;`
     */
    public fun clearUserAccountExists() {
      _builder.clearUserAccountExists()
    }

    /**
     * `.qaul.rpc.user_accounts.MyUserAccount my_user_account = 2;`
     */
    public var myUserAccount: qaul.rpc.user_accounts.UserAccountsOuterClass.MyUserAccount
      @JvmName("getMyUserAccount")
      get() = _builder.getMyUserAccount()
      @JvmName("setMyUserAccount")
      set(value) {
        _builder.setMyUserAccount(value)
      }
    /**
     * `.qaul.rpc.user_accounts.MyUserAccount my_user_account = 2;`
     */
    public fun clearMyUserAccount() {
      _builder.clearMyUserAccount()
    }
    /**
     * `.qaul.rpc.user_accounts.MyUserAccount my_user_account = 2;`
     * @return Whether the myUserAccount field is set.
     */
    public fun hasMyUserAccount(): kotlin.Boolean {
      return _builder.hasMyUserAccount()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount.copy(block: qaul.rpc.user_accounts.DefaultUserAccountKt.Dsl.() -> kotlin.Unit): qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccount =
  qaul.rpc.user_accounts.DefaultUserAccountKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val qaul.rpc.user_accounts.UserAccountsOuterClass.DefaultUserAccountOrBuilder.myUserAccountOrNull: qaul.rpc.user_accounts.UserAccountsOuterClass.MyUserAccount?
  get() = if (hasMyUserAccount()) getMyUserAccount() else null

