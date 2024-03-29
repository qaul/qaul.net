// Generated by the protocol buffer compiler. DO NOT EDIT!
// source: connections/ble/ble.proto

// Generated files should ignore deprecation warnings
@file:Suppress("DEPRECATION")
package qaul.sys.ble;

@kotlin.jvm.JvmName("-initializebleInfoResponse")
public inline fun bleInfoResponse(block: qaul.sys.ble.BleInfoResponseKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleInfoResponse =
  qaul.sys.ble.BleInfoResponseKt.Dsl._create(qaul.sys.ble.BleOuterClass.BleInfoResponse.newBuilder()).apply { block() }._build()
/**
 * ```
 * device information response message
 * ```
 *
 * Protobuf type `qaul.sys.ble.BleInfoResponse`
 */
public object BleInfoResponseKt {
  @kotlin.OptIn(com.google.protobuf.kotlin.OnlyForUseByGeneratedProtoCode::class)
  @com.google.protobuf.kotlin.ProtoDslMarker
  public class Dsl private constructor(
    private val _builder: qaul.sys.ble.BleOuterClass.BleInfoResponse.Builder
  ) {
    public companion object {
      @kotlin.jvm.JvmSynthetic
      @kotlin.PublishedApi
      internal fun _create(builder: qaul.sys.ble.BleOuterClass.BleInfoResponse.Builder): Dsl = Dsl(builder)
    }

    @kotlin.jvm.JvmSynthetic
    @kotlin.PublishedApi
    internal fun _build(): qaul.sys.ble.BleOuterClass.BleInfoResponse = _builder.build()

    /**
     * ```
     * fill in a device information of the BLE device
     * ```
     *
     * `.qaul.sys.ble.BleDeviceInfo device = 1;`
     */
    public var device: qaul.sys.ble.BleOuterClass.BleDeviceInfo
      @JvmName("getDevice")
      get() = _builder.getDevice()
      @JvmName("setDevice")
      set(value) {
        _builder.setDevice(value)
      }
    /**
     * ```
     * fill in a device information of the BLE device
     * ```
     *
     * `.qaul.sys.ble.BleDeviceInfo device = 1;`
     */
    public fun clearDevice() {
      _builder.clearDevice()
    }
    /**
     * ```
     * fill in a device information of the BLE device
     * ```
     *
     * `.qaul.sys.ble.BleDeviceInfo device = 1;`
     * @return Whether the device field is set.
     */
    public fun hasDevice(): kotlin.Boolean {
      return _builder.hasDevice()
    }
  }
}
@kotlin.jvm.JvmSynthetic
public inline fun qaul.sys.ble.BleOuterClass.BleInfoResponse.copy(block: qaul.sys.ble.BleInfoResponseKt.Dsl.() -> kotlin.Unit): qaul.sys.ble.BleOuterClass.BleInfoResponse =
  qaul.sys.ble.BleInfoResponseKt.Dsl._create(this.toBuilder()).apply { block() }._build()

public val qaul.sys.ble.BleOuterClass.BleInfoResponseOrBuilder.deviceOrNull: qaul.sys.ble.BleOuterClass.BleDeviceInfo?
  get() = if (hasDevice()) getDevice() else null

