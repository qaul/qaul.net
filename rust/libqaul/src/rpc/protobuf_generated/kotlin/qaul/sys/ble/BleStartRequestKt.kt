// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: connections/ble/ble.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.sys.ble;

@kotlin.jvm.JvmName("-initializebleStartRequest")
public inline fun bleStartRequest(block: qaul.sys.ble.BleStartRequestKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleStartRequest =
  qaul.sys.ble.BleStartRequestKt.Dsl._create(qaul.sys.ble.BleOuterClass.BleStartRequest.newBuilder()).apply { block() }._build()
/**
 * ```
 * Start Device
 *
 * the module will try to start the device, power it up,
 * get all rights, configure it for qaul, and
 * send & receive advertising messages
 * ```
 *
 * Protobuf type `qaul.sys.ble.BleStartRequest`
 */
public object BleStartRequestKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.sys.ble.BleOuterClass.BleStartRequest.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.sys.ble.BleOuterClass.BleStartRequest.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.sys.ble.BleOuterClass.BleStartRequest = _builder.build()

    /**
     * ```
     * qaul ID
     *
     * The small 16 byte qaul id
     * to be used to identify this node
     * ```
     *
     * `bytes qaul_id = 1;`
     */
    public var qaulId: com.google.protobuf.ByteString
      @JvmName("getQaulId")
      get() = _builder.getQaulId()
      @JvmName("setQaulId")
      set(value) {
        _builder.setQaulId(value)
      }
    /**
     * ```
     * qaul ID
     *
     * The small 16 byte qaul id
     * to be used to identify this node
     * ```
     *
     * `bytes qaul_id = 1;`
     */
    public fun clearQaulId() {
      _builder.clearQaulId()
    }

    /**
     * ```
     * power settings 
     * ```
     *
     * `.qaul.sys.ble.BlePowerSetting power_setting = 2;`
     */
    public var powerSetting: qaul.sys.ble.BleOuterClass.BlePowerSetting
      @JvmName("getPowerSetting")
      get() = _builder.getPowerSetting()
      @JvmName("setPowerSetting")
      set(value) {
        _builder.setPowerSetting(value)
      }
    public var powerSettingValue: kotlin.Int
      @JvmName("getPowerSettingValue")
      get() = _builder.getPowerSettingValue()
      @JvmName("setPowerSettingValue")
      set(value) {
        _builder.setPowerSettingValue(value)
      }
    /**
     * ```
     * power settings 
     * ```
     *
     * `.qaul.sys.ble.BlePowerSetting power_setting = 2;`
     */
    public fun clearPowerSetting() {
      _builder.clearPowerSetting()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.sys.ble.BleOuterClass.BleStartRequest.copy(block: qaul.sys.ble.BleStartRequestKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleStartRequest =
  qaul.sys.ble.BleStartRequestKt.Dsl._create(this.toBuilder()).apply { block() }._build()

