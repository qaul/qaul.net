//
//  BleBridge.swift
//  Runner
//

import Foundation

/// The one Swift entry point for ble_module: receives every Rust action and hands it to the right role.
enum BleBridge {
    static func register() {
        _ = BleCentral.shared   // starts CoreBluetooth, so qaul_ble_powered is reported before libqaul's BLE init checks it
        _ = BlePeripheral.shared
        
        // Set the callback so Rust can call into Swift
        qaul_ble_set_callback { action, addr, role, bytes, len in
            // Copy now: Rust frees bytes as soon as this returns.
            let data = (bytes != nil && len > 0) ? Data(bytes: bytes!, count: Int(len)) : Data()
            // This function calls on Rusts BLE thread, switch to swift main thread
            DispatchQueue.main.async { BleBridge.handle(action: action, addr: addr, role: role, data: data) }
        }
        
        
    }

    private static func handle(action: UInt8, addr: QaulBleAddress, role: UInt8, data: Data) {
        let peer = UUID(uuid: addr.bytes)
        let peripheralRole = role == UInt8(QAUL_BLE_PERIPHERAL)
        switch action {
        case 0:  // Start:
            BleCentral.shared.start()
            BlePeripheral.shared.start(qaulId: data)
        case 1:  // Stop
            BleCentral.shared.stop()
            BlePeripheral.shared.stop()
        case 2:  // Connect: only a central connects
            BleCentral.shared.connect(peer)
        case 3:  // Disconnect
            if peripheralRole { BlePeripheral.shared.disconnect(peer) } else { BleCentral.shared.disconnect(peer) }
        case 4:  // Transmit
            if peripheralRole { BlePeripheral.shared.transmit(data, to: peer) } else { BleCentral.shared.transmit(data, to: peer) }
        default: break
        }
    }
}

