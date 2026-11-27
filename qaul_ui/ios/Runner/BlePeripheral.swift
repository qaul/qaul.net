//
//  BlePeripheral.swift
//  Runner
//
//  The peripheral role: our own qaul GATT server that other phones connect to.
//
import CoreBluetooth

/// One central connected to our server.
private class CentralLink {
    let central: CBCentral
    /// Holds frame to be sent when the queue has room. In practice it should only hold one frame as the engine holds the rest until frame sent, however a list so
    /// nothings lost if something changes down the line to send more than one at a time.
    var outbox = [Data]()
    init(_ central: CBCentral) { self.central = central }
}

class BlePeripheral: NSObject {
    static let shared = BlePeripheral()

    private var manager: CBPeripheralManager!
    /// Our qaul ID, set by start(). Kept so we can publish once Bluetooth is on.
    private var qaulId: Data?
    private var msgChar: CBMutableCharacteristic?

    private let serviceUuid  = CBUUID(string: "c7b64399-37d9-4c00-92a0-6d0a87346816")
    private let readCharUuid = CBUUID(string: "c7b64401-37d9-4c00-92a0-6d0a87346816")
    private let msgCharUuid  = CBUUID(string: "c7b64402-37d9-4c00-92a0-6d0a87346816")
    
    private var links = [UUID: CentralLink]()
    /// Centrals we dropped but that are still connected (we cant disconnect them). Their writes areignored until they
    /// unsubscribe, otherwise their pings would make the engine rebuild the link.
    private var dropped = Set<UUID>()
    private let role = UInt8(QAUL_BLE_PERIPHERAL)

    private override init() {
        super.init()
        manager = CBPeripheralManager(delegate: self, queue: nil)   // nil: means callbacks go on the main thread
    }

    /// Go on air with this qaul ID. If Bluetooth isn't on yet, then this happens in peripheralManagerDidUpdateState
    func start(qaulId: Data) {
        self.qaulId = qaulId
        if manager.state == .poweredOn { publish() }
    }

    /// Off air: stop advertising and remove the service.
    func stop() {
        qaulId = nil
        manager.stopAdvertising()
        manager.removeAllServices()
        msgChar = nil
        links.removeAll()
    }

    /// Build the qaul service and add it. Advertising starts in didAdd delegate, once iOS has accepted it.
    private func publish() {
        guard let qaulId else { return }
        manager.removeAllServices()   // in case of a restart, so we never publish it twice

        // READ_CHAR: our qaul ID.
        let readChar = CBMutableCharacteristic(
            type: readCharUuid,
            properties: [.read],
            value: qaulId,
            permissions: [.readable])

        // MSG_CHAR: frames both ways. Linux centrals write with response every 20 frames
        // (their tracer), Android without, so allow both. value nil: it changes, so we handle it
        let msg = CBMutableCharacteristic(
            type: msgCharUuid,
            properties: [.write, .writeWithoutResponse, .notify],
            value: nil,
            permissions: [.writeable])
        msgChar = msg

        let service = CBMutableService(type: serviceUuid, primary: true)
        service.characteristics = [readChar, msg]
        manager.add(service)
    }

    /// the ID's first 5 bytes as base64, to that it can be stored safely as the advert's name string
    private func advertName(_ id: Data) -> String {
        let b64 = id.prefix(5).base64EncodedString()        // standard: 8 characters, the last is "=" as padding
              .replacingOccurrences(of: "+", with: "-")
              .replacingOccurrences(of: "/", with: "_")
              .replacingOccurrences(of: "=", with: "")         // 7 characters remain + q makes 8 again, fitting into advert
        return "q" + b64
    }
    
    /// A peripheral cant drop a central, so forget it and send nothing more. This is the same on other platforms.
    /// For now, the central stays connected until its own liveness timeout ends the link. 
    func disconnect(_ peer: UUID) {
        // Only a subscribed central (has a link) will get the unsubscribe that clears it again.
        if links.removeValue(forKey: peer) != nil {
            dropped.insert(peer)
        }
    }

    /// Queue a frame for that central on MSG_CHAR.
    func transmit(_ value: Data, to peer: UUID) {
        guard let link = links[peer] else { return }
        link.outbox.append(value)
        _ = drain(link)
    }
    
    /// Send queued frames until iOS's queue is full. Returns false when full, peripheralManagerIsReady
    /// can call this again. Theres no "can send?" check like the central, rely on updateValue returning false
    private func drain(_ link: CentralLink) -> Bool {
        guard let msgChar else { return false }
        while let frame = link.outbox.first {
            guard manager.updateValue(frame, for: msgChar, onSubscribedCentrals: [link.central]) else {
                return false   // full: frame stays first in line
            }
            link.outbox.removeFirst()
            qaul_ble_frame_sent(address(link.central.identifier), role)
        }
        return true
    }

    private func address(_ id: UUID) -> QaulBleAddress {
        QaulBleAddress(bytes: id.uuid)
    }
}

extension BlePeripheral: CBPeripheralManagerDelegate {
    func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        NSLog("QAULBLE peripheral state \(peripheral.state.rawValue)")
        if peripheral.state == .poweredOn {
            publish()   // does nothing until start() has given us its qaul ID
        }
    }

    func peripheralManager(_ peripheral: CBPeripheralManager, didAdd service: CBService, error: Error?) {
        if let error {
            NSLog("QAULBLE peripheral add service failed: \(error)")
            return
        }
        guard let qaulId else { return }
        // By using base64 to encode, and 5 bytes we fill the Ext ADV data to the max 31 byte length. Any more and it overflows and gets
        // placed in the second scan response instead, and the tx power replaces it in the ADV Data, which increases the
        // chance of failure if 2 packets are needed in order to identify a node.
        let name = advertName(qaulId)
        manager.startAdvertising([
            CBAdvertisementDataServiceUUIDsKey: [serviceUuid],
            CBAdvertisementDataLocalNameKey: name,
        ])
        NSLog("QAULBLE peripheral service added, advertising as \(name)")
    }

    func peripheralManagerDidStartAdvertising(_ peripheral: CBPeripheralManager, error: Error?) {
        if let error {
            NSLog("QAULBLE peripheral advertising failed: \(error)")
        } else {
            NSLog("QAULBLE peripheral advertising")
        }
    }

    /// A central subscribed to the MSG CHARACTERISTIC: the link now counts as up
    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didSubscribeTo characteristic: CBCharacteristic) {
        guard characteristic.uuid == msgCharUuid else { return }
        NSLog("QAULBLE peripheral \(central.identifier) subscribed, max update \(central.maximumUpdateValueLength)")
        links[central.identifier] = CentralLink(central)
        // iOS actually already takes off the 3 byte ATT header, but the engine does too, so add it back.
        qaul_ble_subscribed(address(central.identifier), role, UInt32(central.maximumUpdateValueLength + 3))
    }

    /// A central unsubscribed, usually because it disconnected.
    func peripheralManager(_ peripheral: CBPeripheralManager, central: CBCentral, didUnsubscribeFrom characteristic: CBCharacteristic) {
        guard characteristic.uuid == msgCharUuid else { return }
        NSLog("QAULBLE peripheral \(central.identifier) unsubscribed")
        links[central.identifier] = nil
        // The engine already forgot a dropped central, so no gone event. A later reconnect starts clean.
        if dropped.remove(central.identifier) != nil { return }
        qaul_ble_gone(address(central.identifier), role)
    }

    /// Writes to MSG CHARACTERISTIC: Can come before the subscribe: Android writes SEND_ID first.
    func peripheralManager(_ peripheral: CBPeripheralManager, didReceiveWrite requests: [CBATTRequest]) {
        for request in requests {
            guard request.characteristic.uuid == msgCharUuid, let value = request.value,
                  !dropped.contains(request.central.identifier) else { continue }
            value.withUnsafeBytes { raw in
                qaul_ble_frame_received(address(request.central.identifier), role,
                                        raw.bindMemory(to: UInt8.self).baseAddress, UInt32(raw.count))
            }
        }
        // ios rule: one respond per callback, with the first request. I believe needed to answer writes with response, otherwise does nothing.
        if let first = requests.first {
            peripheral.respond(to: first, withResult: .success)
        }
    }

    /// updateValue returned false earlier (queue full) but now theres room again. The queue is shared by
    /// all centrals and iOS doesnt say whose turn it is, so drain them all until it fills up again. In theory one could optimise here to have prio?
    func peripheralManagerIsReady(toUpdateSubscribers peripheral: CBPeripheralManager) {
        for link in links.values {
            if !drain(link) { break }
        }
    }
}
