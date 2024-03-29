// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: connections/ble/ble.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.sys.ble;

@kotlin.jvm.JvmName("-initializebleDeviceUnavailable")
public inline fun bleDeviceUnavailable(block: qaul.sys.ble.BleDeviceUnavailableKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleDeviceUnavailable =
  qaul.sys.ble.BleDeviceUnavailableKt.Dsl._create(qaul.sys.ble.BleOuterClass.BleDeviceUnavailable.newBuilder()).apply { block() }._build()
/**
 * ```
 * Device Unavailable
 *
 * A formerly discovered device has become 
 * unavailable. No messages can be sent to it.
 * ```
 *
 * Protobuf type `qaul.sys.ble.BleDeviceUnavailable`
 */
public object BleDeviceUnavailableKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.sys.ble.BleOuterClass.BleDeviceUnavailable.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.sys.ble.BleOuterClass.BleDeviceUnavailable.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.sys.ble.BleOuterClass.BleDeviceUnavailable = _builder.build()

    /**
     * ```
     * qaul id of the device that
     * became unavailable
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
     * qaul id of the device that
     * became unavailable
     * ```
     *
     * `bytes qaul_id = 1;`
     */
    public fun clearQaulId() {
      _builder.clearQaulId()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.sys.ble.BleOuterClass.BleDeviceUnavailable.copy(block: qaul.sys.ble.BleDeviceUnavailableKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleDeviceUnavailable =
  qaul.sys.ble.BleDeviceUnavailableKt.Dsl._create(this.toBuilder()).apply { block() }._build()

