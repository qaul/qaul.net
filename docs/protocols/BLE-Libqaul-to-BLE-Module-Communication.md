# BLE libqaul ↔ ble_module System Communication

## Overview

The BLE subsystem consists of two main components that communicate via protobuf-encoded binary messages over in-process channels:

1. libqaul/src/connections/ble/ — The BLE connection manager inside libqaul. Manages state, node discovery, identification, and
routes application-layer messages (feed, messaging, routing) over BLE.
2. ble_module/ — The platform-specific BLE driver (currently Linux via bluer/BlueZ). Handles GATT server/client operations, BLE advertising, scanning, and raw message transport.

## Three Protobuf Protocols

| Proto file | Package | Purpose | Direction |
|---|---|---|---|
| ble.proto | qaul.sys.ble | System communication between libqaul and ble_module | libqaul ↔ ble_module |
| ble_net.proto | qaul.net.ble | Application payloads sent over BLE between peers | Node ↔ Node (wrapped inside BleDirectSend) |
| ble_rpc.proto | qaul.rpc.ble | UI control of the BLE module | UI ↔ libqaul |

## Channel Architecture

```TXT
  ┌──────────────────────────────────────────────────────────────┐
  │                        libqaul                               │
  │                                                              │
  │  Ble::module_start()  ──encode──►  Sys::send_message(buf)   │
  │  Ble::message_send()                     │                   │
  │  Ble::info_send_request()                │                   │
  │                                          │                   │
  │  Ble::sys_received(data) ◄── Sys::process_received_message() │
  │         ▲                                │                   │
  └─────────┼────────────────────────────────┼───────────────────┘
            │                                │
            │  Sys::send_to_libqaul()        │  #[cfg(linux)]
            │  (EXTERN_SEND channel)         │  ble_module::rpc::send_to_ble_module()
            │  crossbeam unbounded           │  (EXTERN_SEND channel, async_std bounded(32))
            │                                │
            │                                ▼
  ┌─────────┼────────────────────────────────────────────────────┐
  │         │           ble_module                                │
  │         │                                                    │
  │  BleResultSender ──► unbounded channel ──► sys_rpc_callback  │
  │    .send_device_found()                                      │
  │    .send_direct_received()           BleRpc.recv()           │
  │    .send_start_successful()            │                     │
  │    .send_stop_successful()             ▼                     │
  │                               listen_for_sys_msgs()          │
  │                               advertise_scan_listen()        │
  │                                                              │
  │                       BlueZ / bluer (Linux BLE stack)        │
  └──────────────────────────────────────────────────────────────┘
```

Key wiring (from sys.rs:100-108 and ble/mod.rs:121):

- libqaul → ble_module: Sys::send_message() calls ble_module::rpc::send_to_ble_module() on Linux (or Android::send_to_android() on Android), which pushes to an async_std::channel::bounded(32).
- ble_module → libqaul: On init, libqaul passes a callback Box::new(|sys_msg| Sys::send_to_libqaul(sys_msg)). The ble_module sends results through a BleResultSender → unbounded channel → callback → Sys::send_to_libqaul() → crossbeam channel → Sys::process_received_message() → Ble::sys_received().


## BLE Sys Protocol Messages (ble.proto)

All messages are wrapped in a Ble oneof envelope:

| Message | Direction | Purpose |
|---|---|---|
| BleInfoRequest | libqaul → module | Query device capabilities |
| BleInfoResponse | module → libqaul | Returns BleDeviceInfo (BT support, PHY, LE audio, etc.) |
| BleStartRequest | libqaul → module | Start BLE with qaul_id (8 bytes) + BlePowerSetting |
| BleStartResult | module → libqaul | Success/failure + BleError reason |
| BleStopRequest | libqaul → module | Stop BLE operations |
| BleStopResult | module → libqaul | Success/failure |
| BleDeviceDiscovered | module → libqaul | New peer found (qaul_id + rssi) |
| BleDeviceUnavailable | module → libqaul | Peer went out of range |
| BleDirectSend | libqaul → module | Send data to peer (message_id, receiver_id, sender_id, data) |
| BleDirectSendResult | module → libqaul | Send success/failure |
| BleDirectReceived | module → libqaul | Incoming data from peer (from, data) |


## Module Lifecycle State Machine

```TXT
                      init()
                        │
                        ▼
                ┌──────────────┐
                │ Uninitalized │
                └──────┬───────┘
                       │ info_send_request()
                       ▼
             ┌──────────────────┐
             │ InfoRequestSent  │
             └────────┬─────────┘
                      │ InfoResponse received
                      ▼
              ┌───────────────┐
              │ InfoReceived  │──► auto-triggers module_start()
              └───────┬───────┘
                      │ BleStartRequest sent
                      ▼
            ┌────────────────────┐
            │ StartRequestSent   │
            └──────┬─────┬───────┘
                   │     │
          success  │     │ error_reason = RIGHTS_MISSING
                   ▼     ▼
         ┌──────────┐  ┌───────────────┐
         │StartSuccess│  │ RightsMissing │──► sends RightsRequest RPC to UI
         └──────────┘  └───────┬───────┘
                               │ RightsResult(granted=true)
                               │ triggers module_start() again
                               ▼
                      ┌──────────────┐
                      │ StartSuccess │
                      └──────┬───────┘
                             │ module_stop()
                             ▼
                   ┌──────────────────┐
                   │ StopRequestSent  │
                   └────────┬─────────┘
                            │ StopResult(success)
                            ▼
                      ┌──────────┐
                      │ Stopped  │
                      └──────────┘
```

## Node Discovery & Identification Flow

When the ble_module discovers a peer's BLE advertisement, it reads the peer's qaul_id (8-byte short ID) from the GATT read_char and sends BleDeviceDiscovered to libqaul. libqaul then does:

```TXT
  BleDeviceDiscovered(qaul_id, rssi)
           │
           ▼
  Neighbours::node_from_q8id(qaul_id)
      ┌────┴─────┐
      │          │
    Found    Not Found
      │          │
      ▼          ▼
  node_discovered()    node_to_confirm()
   │                      │
   ├─ NODES.insert()      ├─ TO_CONFIRM.insert()
   └─ Neighbours::        └─ identification_send(q8id, request=true)
      update_node()              │
                                 ▼
                      Sends BleMessage::Identification over BLE
                                 │
                                 ▼
                      Peer responds with their full PeerId
                                 │
                                 ▼
                      identification_received()
                       ├─ TO_CONFIRM.remove()
                       ├─ node_discovered(q8id, full_node_id)
                       └─ if was request: identification_send(q8id, false)
```

The NODES table maps q8id → BleNode { id: Vec<u8>, timestamp } and serves as a translation table between the 8-byte BLE ID and the full libp2p PeerId.


## Application Message Flow (Sending)

When libqaul services want to send data over BLE:

```TXT
    RouterInfo / Feed / Messaging service
           │
           ▼
    Ble::send_routing_info()     → BleMessage::Info(data)
    Ble::send_messaging_message()→ BleMessage::Messaging(data)
    Ble::send_feed_message()     → BleMessage::Feed(data)  (sent to BLE-only nodes)
           │
           ▼
    Ble::create_send_message(receiver_q8id, message)
           │
           ├─ Wraps in BleMessage protobuf (ble_net.proto)
           ├─ Encodes to bytes
           ▼
    Ble::message_send(receiver_id, sender_id, encoded_data)
           │
           ├─ Wraps in BleDirectSend protobuf (ble.proto)
           ├─ Encodes to bytes
           ▼
    Sys::send_message(buf)
           │
           ▼ (Linux)
    ble_module::rpc::send_to_ble_module(buf)
           │
           ▼
    ble_service: connect to peer, write chunks to msg_char()

  Application Message Flow (Receiving)

    Remote peer writes to our msg_char() GATT characteristic
           │
           ▼
    spawn_msg_listener() reassembles chunks (delimited by $$)
           │
           ▼
    BleResultSender::send_direct_received(from_q8id, data)
           │
           ▼ (callback → crossbeam channel)
    Sys::process_received_message() → Ble::sys_received()
           │
           ▼
    Ble::message_received(BleDirectReceived)
           │
           ├─ Looks up full PeerId from q8id via Neighbours
           ├─ Decodes BleMessage (ble_net.proto)
           │
      ┌────┴──────────┬──────────────┬────────────────┐
      ▼               ▼              ▼                ▼
    Info           Feed          Messaging      Identification
      │               │              │                │
  RouterInfo::   Feed::         Messaging::    identification_
   received()    received()     received()      received()
```

