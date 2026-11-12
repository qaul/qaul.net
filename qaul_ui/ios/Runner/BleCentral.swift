//
//  BleCentral.swift
//  Runner
//
//
import CoreBluetooth

private class Link {
    var peripheral: CBPeripheral
    var msgChar: CBCharacteristic?
    var outbox = [Data]()
    init(_ peripheral: CBPeripheral) {self.peripheral = peripheral}
}

class BleCentral: NSObject {
    static let shared = BleCentral()
    
    private var centralManager: CBCentralManager!
    private var discoveredPeripherals = [UUID: CBPeripheral]()
    private var pendingConnect: UUID?   // connect in flight, until subscribed / failed / gone
    private let serviceUuid = CBUUID(string: "c7b64399-37d9-4c00-92a0-6d0a87346816")
    private let readCharUuid = CBUUID(string: "c7b64401-37d9-4c00-92a0-6d0a87346816")
    private let msgCharUuid  = CBUUID(string: "c7b64402-37d9-4c00-92a0-6d0a87346816")
    private let tracerCharUuid = CBUUID(string: "c7b64404-37d9-4c00-92a0-6d0a87346816")
    
    private var links = [UUID : Link]()

    private override init() {
            super.init()
            NSLog("QAULBLE init")
            centralManager = CBCentralManager(delegate: self, queue: nil)
            NSLog("QAULBLE manager created, state \(centralManager.state.rawValue)")
        
            qaul_ble_set_callback { action, addr, bytes, len in
                // Copy now: Rust frees bytes as soon as this returns.
                let data = (bytes != nil && len > 0) ? Data(bytes: bytes!, count: Int(len)) : Data()
                // This function calls on Rusts BLE thread, switch to swift main thread
                DispatchQueue.main.async {
                    BleCentral.shared.handle(action: action, addr: addr, data: data)
                }
            }
        }
    
    // What Rust asks for. Numbers match `Action` in ffi.rs
    func handle(action: UInt8, addr: QaulBleAddress, data: Data) {
        let peer = UUID(uuid: addr.bytes)
        NSLog("QAULBLE action \(action) peer \(peer) bytes \(data.count)")
        switch action {
        case 0: startScan()                 // Start
        case 1: centralManager.stopScan()   // Stop
        case 2:   // Connect
            guard pendingConnect == nil,
                  let p = discoveredPeripherals[peer] else {
                qaul_ble_connect_failed(addr)   // busy or never seen: the engine must hear back
                return
            }
            pendingConnect = peer
            connect(peripheral: p)
            // iOS never gives up on a connect by itself
            DispatchQueue.main.asyncAfter(deadline: .now() + 15) { [weak self] in
                guard let self, self.pendingConnect == peer else { return }   // already finished
                NSLog("QAULBLE connect to \(peer) timed out")
                self.pendingConnect = nil
                self.centralManager.cancelPeripheralConnection(p)
                qaul_ble_connect_failed(addr)
            }
        case 3: // disconnect
            if let p = discoveredPeripherals[peer] { centralManager.cancelPeripheralConnection(p) }
        case 4: // transmit
            if let _ = links[peer] { write(data, to: peer) }
            
        default: break
        }
    }
    
    func startScan() {
        centralManager.scanForPeripherals(withServices: [serviceUuid],
            options: [CBCentralManagerScanOptionAllowDuplicatesKey: true])
    }
    
    func connect(peripheral: CBPeripheral) {
        centralManager.connect(peripheral, options: nil)
     }
    
    // Queue a frame for MSG_CHAR. Frames leave in order, nothing is dropped while iOS is busy it just waits.
    func write(_ value: Data, to peripheral: UUID) {
        guard let link = links[peripheral] else { return }
        link.outbox.append(value)
        drain(link)
    }

    // Send queued frames while iOS has room. When it fills up, peripheralIsReady calls this again.
    private func drain(_ link: Link) {
        guard let msgChar = link.msgChar else { return }   // not discovered yet
           while !link.outbox.isEmpty && link.peripheral.canSendWriteWithoutResponse {
               let frame = link.outbox.removeFirst()
               link.peripheral.writeValue(frame, for: msgChar, type: .withoutResponse)
               qaul_ble_frame_sent(address(link.peripheral.identifier))
           }
    }
    
    private func address(_ id: UUID) -> QaulBleAddress {
        QaulBleAddress(bytes: id.uuid)
    }
}

extension BleCentral: CBCentralManagerDelegate {
    func centralManagerDidUpdateState(_ central: CBCentralManager) {
        NSLog("QAULBLE state \(central.state.rawValue)")
        print("Bluetooth state: \(central.state.rawValue)")
        switch central.state {
        case .poweredOn:
            qaul_ble_powered(true)
            print("Bluetooth on")
        case .poweredOff:
               // Alert user to turn on Bluetooth
            qaul_ble_powered(false)
        case .resetting:
            // Wait for next statew update and consider logging interruption of Bluetooth service
            qaul_ble_powered(false)
        case .unauthorized:
            // Alert user to enable Bluetooth permission in app Settings
            qaul_ble_powered(false)
        default:
            break
        }
         /*
            case .unsupported:
                // Alert user their device does not support Bluetooth and app will not work as expected
            case .unknown:
               // Wait for next state update */
    }
    
    func centralManager(_ central: CBCentralManager, didDiscover peripheral: CBPeripheral, advertisementData: [String : Any], rssi RSSI: NSNumber) {
        self.discoveredPeripherals[peripheral.identifier] = peripheral // only saves peripheral for now, needs wrapper to include rssi, adv data etc
        
        guard let data = advertisementData[CBAdvertisementDataManufacturerDataKey] as? Data,
              data.count >= 2, data[data.startIndex] == 0xFF, data[data.startIndex + 1] == 0xFF
        else { return }
        let prefix = Data(data.dropFirst(2).prefix(5))
        prefix.withUnsafeBytes { raw in
            qaul_ble_advert(address(peripheral.identifier),
                            raw.bindMemory(to: UInt8.self).baseAddress,
                            UInt32(raw.count),
                            Int16(truncating: RSSI))
        }
        
    }
    
    func centralManager(_ central: CBCentralManager, didConnect peripheral: CBPeripheral) {
        NSLog("QAULBLE connected to \(peripheral.identifier)")
        // Successfully connected. Store reference to peripheral if not already done.
        self.links[peripheral.identifier] = Link(peripheral)
        peripheral.delegate = self
        peripheral.discoverServices([serviceUuid])
    }
    
    // Failed to connect
    func centralManager(_ central: CBCentralManager, didFailToConnect peripheral: CBPeripheral, error: Error?) {
        if pendingConnect == peripheral.identifier { pendingConnect = nil }
        qaul_ble_connect_failed(address(peripheral.identifier))
        NSLog("QAULBLE connect failed \(String(describing: error))")
        // Handle error
    }
    
    // Disconnect response
    func centralManager(_ central: CBCentralManager, didDisconnectPeripheral peripheral: CBPeripheral, error: Error?) {
        if pendingConnect == peripheral.identifier { pendingConnect = nil }
        self.links[peripheral.identifier] = nil
        qaul_ble_gone(address(peripheral.identifier))
        // error is nil only when we cancelled the connection ourselves
        NSLog("QAULBLE disconnected \(peripheral.identifier), reason: \(String(describing: error))")
    }
}



extension BleCentral: CBPeripheralDelegate {
    
    //discovery succeeded
    func peripheral(_ peripheral: CBPeripheral, didDiscoverServices error: Error?) {
        if let error = error {
            NSLog("QAULBLE service discovery error: \(error)")
            return
        }
        guard let service = peripheral.services?.first(where: { $0.uuid == serviceUuid }) else { return }
        peripheral.discoverCharacteristics([readCharUuid, msgCharUuid, tracerCharUuid], for: service)
    }
    
    // char discovrered
    func peripheral(_ peripheral: CBPeripheral, didDiscoverCharacteristicsFor service: CBService, error: Error?) {
        if let error = error {
            NSLog("QAULBLE characteristic discovery error: \(error)")
            return
        }
        guard let characteristics = service.characteristics else {
            return
        }
        // setup relevant chars
        var tracerChar: CBCharacteristic?
        for char in characteristics {
            NSLog("QAULBLE char \(char.uuid)")
            if char.uuid == readCharUuid {
                peripheral.readValue(for: char) // read qaul id
            }
            else if char.uuid == msgCharUuid {
                links[peripheral.identifier]?.msgChar = char
            }
            else if char.uuid == tracerCharUuid {
                tracerChar = char
            }
        }
        
        if let tracerChar {
            peripheral.setNotifyValue(true, for: tracerChar)   // Linux peer: tracer HAS to be first, MSG then only in the callback
        } else if let msgChar = links[peripheral.identifier]?.msgChar {
            peripheral.setNotifyValue(true, for: msgChar)      // Android peer: no tracer exists so MSG now is fine
        }
        
    }
    
    // Successfully subscribed to or unsubscribed from notifications/indications on a characteristic
    func peripheral(_ peripheral: CBPeripheral, didUpdateNotificationStateFor characteristic: CBCharacteristic, error: Error?) {
        if characteristic.uuid == tracerCharUuid { // If tracer subscribe succeeded now we can do MSG
            if let error { NSLog("QAULBLE tracer subscribe failed, running untraced: \(error)") }
            if let msgChar = links[peripheral.identifier]?.msgChar {
                peripheral.setNotifyValue(true, for: msgChar)
            }
            return
        }
        
        if let error = error {
            NSLog("QAULBLE notify error \(characteristic.uuid): \(error)")
            return
        }
        
        guard characteristic.uuid == msgCharUuid, characteristic.isNotifying else { return }
        if pendingConnect == peripheral.identifier { pendingConnect = nil }
        
        let mtu = peripheral.maximumWriteValueLength(for: .withoutResponse) + 3 // this function alreayd provides mtu - 3 for att header, but so does the ble engine, so we add it back here
        qaul_ble_subscribed(address(peripheral.identifier), UInt32(mtu))
        
        NSLog("QAULBLE notifying \(characteristic.isNotifying) on \(characteristic.uuid)")
        NSLog("QAULBLE mtu: \(mtu)")
    }

    // READ_CHAR's read result and every MSG_CHAR notification arrive here
    func peripheral(_ peripheral: CBPeripheral, didUpdateValueFor characteristic: CBCharacteristic, error: Error?) {
        if let error = error {
            NSLog("QAULBLE value error \(characteristic.uuid): \(error)")
            return
        }
        guard let value = characteristic.value else {
            return
        }
        let addr = address(peripheral.identifier)
        value.withUnsafeBytes { raw in
            let ptr = raw.bindMemory(to: UInt8.self).baseAddress
            if characteristic.uuid == readCharUuid {
                qaul_ble_peer_id_read(addr, ptr, UInt32(raw.count))
            } else if characteristic.uuid == msgCharUuid {
                qaul_ble_frame_received(addr, ptr, UInt32(raw.count))
            }
        }
    }
    
    // iOS has room for more writes without response again
    func peripheralIsReady(toSendWriteWithoutResponse peripheral: CBPeripheral) {
        if let link = links[peripheral.identifier] { drain(link) }
    }
}
