import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:typed_data';

import 'package:collection/collection.dart';
import 'package:fixnum/fixnum.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:permission_handler/permission_handler.dart';
import 'package:uuid/uuid.dart';

import '../qaul_rpc.dart';
import 'generated/connections/ble/ble_rpc.pb.dart';
import 'generated/connections/connections.pb.dart';
import 'generated/node/node.pb.dart';
import 'generated/node/user_accounts.pb.dart';
import 'generated/router/router.pb.dart';
import 'generated/router/users.pb.dart';
import 'generated/rpc/debug.pb.dart';
import 'generated/rpc/qaul_rpc.pb.dart';
import 'generated/services/chat/chat.pb.dart';
import 'generated/services/feed/feed.pb.dart';
import 'libqaul/libqaul.dart';
import 'rpc_translators/abstract_rpc_module_translator.dart';
import 'utils.dart';

class LibqaulWorker {
  LibqaulWorker(Reader reader) : _reader = reader {
    _init();
  }

  final Reader _reader;
  final _log = Logger('LibqaulWorker');

  Libqaul get _lib => _reader(libqaulProvider);

  Future<bool> get initialized => _initialized.future;
  final _initialized = Completer<bool>();

  final _heartbeats = Queue<bool>();

  Stream<bool> get onLibraryCrash => _streamController.stream;
  final _streamController = StreamController<bool>.broadcast();

  void _init() async {
    if (_initialized.isCompleted) return;
    // Throws when called for some reason
    // await _lib.load();
    await _lib.start();
    while (await _lib.initialized() != 1) {
      await Future.delayed(const Duration(milliseconds: 10));
    }
    // Request Log storage path
    _getLibqaulLogsStoragePath();

    Timer.periodic(const Duration(milliseconds: 100), (_) async {
      final n = await _lib.checkReceiveQueue();
      if (n > 0) _receiveResponse();
    });
    Timer.periodic(const Duration(seconds: 2), (_) async {
      if (_heartbeats.length > 5) {
        _log.warning('${_heartbeats.length} heartbeats unanswered by Libqaul');
        _streamController.add(true);
      }
      _heartbeats.addLast(true);
      _log.finest('requesting heartbeat to libqaul');
      final msg = Debug(heartbeatRequest: HeartbeatRequest());
      _encodeAndSendMessage(Modules.DEBUG, msg.writeToBuffer());
    });

    _initialized.complete(true);
  }

  // *******************************
  // Public rpc requests
  // *******************************
  Future<void> sendFeedMessage(String content) async {
    final msg = Feed(send: SendMessage(content: content));
    await _encodeAndSendMessage(Modules.FEED, msg.writeToBuffer());
  }

  Future<void> requestFeedMessages({int? lastIndex}) async {
    final msg = Feed(
      request: FeedMessageRequest(lastIndex: Int64(lastIndex ?? 0)),
    );
    _encodeAndSendMessage(Modules.FEED, msg.writeToBuffer());
  }

  Future<void> getUsers() async {
    await _encodeAndSendMessage(
        Modules.USERS, Users(userRequest: UserRequest()).writeToBuffer());

    _encodeAndSendMessage(Modules.ROUTER,
        Router(routingTableRequest: RoutingTableRequest()).writeToBuffer());
  }

  Future<void> verifyUser(User u) async {
    var entry = _baseUserEntryFrom(u);
    entry.verified = true;
    await _encodeAndSendMessage(
        Modules.USERS, Users(userUpdate: entry).writeToBuffer());
  }

  Future<void> unverifyUser(User u) async {
    var entry = _baseUserEntryFrom(u);
    entry.verified = false;
    await _encodeAndSendMessage(
        Modules.USERS, Users(userUpdate: entry).writeToBuffer());
  }

  Future<void> blockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = true;
    await _encodeAndSendMessage(
        Modules.USERS, Users(userUpdate: entry).writeToBuffer());
  }

  Future<void> unblockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = false;
    await _encodeAndSendMessage(
        Modules.USERS, Users(userUpdate: entry).writeToBuffer());
  }

  Future<void> getNodeInfo() async => await _encodeAndSendMessage(
      Modules.NODE, Node(getNodeInfo: true).writeToBuffer());

  Future<void> requestNodes() async => await _encodeAndSendMessage(
      Modules.CONNECTIONS,
      Connections(internetNodesRequest: InternetNodesRequest())
          .writeToBuffer());

  Future<void> addNode(String address) async => await _encodeAndSendMessage(
      Modules.CONNECTIONS,
      Connections(internetNodesAdd: InternetNodesEntry(address: address))
          .writeToBuffer());

  Future<void> removeNode(String address) async => await _encodeAndSendMessage(
      Modules.CONNECTIONS,
      Connections(internetNodesRemove: InternetNodesEntry(address: address))
          .writeToBuffer());

  Future<void> getDefaultUserAccount() async {
    final message = UserAccounts(getDefaultUserAccount: true);
    await _encodeAndSendMessage(Modules.USERACCOUNTS, message.writeToBuffer());
  }

  Future<void> createUserAccount(String name) async {
    final msg = UserAccounts(createUserAccount: CreateUserAccount(name: name));
    await _encodeAndSendMessage(Modules.USERACCOUNTS, msg.writeToBuffer());
  }

  void sendDebugPanicMessage() async {
    await _encodeAndSendMessage(Modules.DEBUG, Uint8List(0));
  }

  void getAllChatRooms() async {
    final msg = Chat(overviewRequest: ChatOverviewRequest());
    await _encodeAndSendMessage(Modules.CHAT, msg.writeToBuffer());
  }

  void getChatRoomMessages(Uint8List chatId, {int lastIndex = 0}) async {
    final msg = Chat(
      conversationRequest: ChatConversationRequest(
        conversationId: chatId,
        lastIndex: Int64(lastIndex),
      ),
    );
    await _encodeAndSendMessage(Modules.CHAT, msg.writeToBuffer());
  }

  void sendMessage(Uint8List chatId, String content) async {
    final msg = Chat(
      send: ChatMessageSend(conversationId: chatId, content: content),
    );
    await _encodeAndSendMessage(Modules.CHAT, msg.writeToBuffer());
  }

  void setLibqaulLogging(bool enabled) async {
    final msg = Debug(logToFile: LogToFile(enable: enabled));
    await _encodeAndSendMessage(Modules.DEBUG, msg.writeToBuffer());
  }

  void deleteLogs() async {
    final msg = Debug(deleteLibqaulLogsRequest: DeleteLibqaulLogsRequest());
    await _encodeAndSendMessage(Modules.DEBUG, msg.writeToBuffer());
  }

  void sendBleInfoRequest() async {
    for (final message in [
      Ble(infoRequest: InfoRequest()).writeToBuffer(),
      Ble(discoveredRequest: DiscoveredRequest()).writeToBuffer(),
    ]) {
      await _encodeAndSendMessage(Modules.BLE, message);
    }
  }

  // *******************************
  // Private (helper) methods
  // *******************************
  UserEntry _baseUserEntryFrom(User u) => UserEntry(
        name: u.name,
        idBase58: u.idBase58,
        id: u.id,
        key: u.key,
        keyType: u.keyType,
        keyBase58: u.keyBase58,
      );

  void _getLibqaulLogsStoragePath() async {
    final msg = Debug(storagePathRequest: StoragePathRequest());
    await _encodeAndSendMessage(Modules.DEBUG, msg.writeToBuffer());
  }

  // *******************************
  // Private (control) methods
  // *******************************
  Future<String> _encodeAndSendMessage(Modules module, Uint8List data) async {
    // create message
    QaulRpc message = QaulRpc();
    message.module = module;
    message.data = data;

    final user = _reader(defaultUserProvider);
    if (user != null) message.userId = user.id;

    final id = const Uuid().v4();
    message.requestId = id;

    // encode it
    Uint8List messageEncoded = message.writeToBuffer();

    // send it
    final libqaul = _reader(libqaulProvider);
    await libqaul.sendRpc(messageEncoded);

    return id;
  }

  Future<void> _receiveResponse() async {
    final response = await _lib.receiveRpc();

    if (response != null) {
      final m = QaulRpc.fromBuffer(response);

      if (m.module == Modules.CONNECTIONS) {
        final resp = await ConnectionTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.FEED) {
        final resp = await FeedTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.NODE) {
        final resp = await NodeTranslator().decodeMessageBytes(m.data);
        if (resp != null && resp.data is NodeInfo) {
          _reader(nodeInfoProvider.state).state = resp.data;
        }
      } else if (m.module == Modules.USERACCOUNTS) {
        final resp = await UserAccountsTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.USERS) {
        final resp = await UsersTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.ROUTER) {
        final resp = await RouterTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.CHAT) {
        final resp = await ChatTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.DEBUG) {
        final resp = await DebugTranslator().decodeMessageBytes(m.data);
        if (resp?.data is bool) {
          _log.finest('libqaul answered a heartbeat request');
          _heartbeats.removeFirst();
        }
        if (resp?.data is String) {
          final path = await findFolderWithFilesOfExtension(
              Directory(resp!.data), '.log');
          _log.info('libqaul log storage path: $path');
          _reader(libqaulLogsStoragePath.state).state = path;
        }
      } else if (m.module == Modules.BLE) {
        final resp = await BleTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else {
        throw UnhandledRpcMessageException.value(
            m.toString(), 'LibqaulWorker.receiveResponse');
      }
    }
  }

  void _processResponse(RpcTranslatorResponse resp) async {
    if (resp.data is List<User>) {
      final provider = _reader(usersProvider.notifier);

      for (final user in resp.data) {
        provider.contains(user) ? provider.update(user) : provider.add(user);
      }
      return;
    }
    if (resp.module == Modules.FEED) {
      if (resp.data != null && resp.data is List<FeedPost>) {
        final provider = _reader(feedMessagesProvider.notifier);

        for (final msg in resp.data) {
          if (!provider.contains(msg)) provider.add(msg);
        }
        return;
      }
    }
    if (resp.module == Modules.CONNECTIONS) {
      if (resp.data != null && resp.data is List<InternetNode>) {
        _reader(connectedNodesProvider.notifier).state = resp.data;
      }
      return;
    }
    if (resp.module == Modules.USERACCOUNTS) {
      if (resp.data != null && resp.data is User) {
        _reader(defaultUserProvider.state).state = resp.data;
      }
      return;
    }
    if (resp.module == Modules.CHAT) {
      if (resp.data != null) {
        if (resp.data is List<ChatRoom>) {
          final state = _reader(chatRoomsProvider.notifier);
          for (final room in resp.data) {
            if (!state.contains(room)) {
              state.add(room);
            } else {
              state.update(room);
            }
          }
          return;
        }
        if (resp.data is ChatRoom) {
          final currentRoom = _reader(currentOpenChatRoom);

          if (currentRoom != null &&
              currentRoom.conversationId.equals(resp.data.conversationId)) {
            _reader(currentOpenChatRoom.notifier).state = resp.data;
          }
          return;
        }
      }
    }
    if (resp.module == Modules.BLE) {
      if (resp.data is BleConnectionStatus) {
        var newStatus = resp.data as BleConnectionStatus;
        _log.finer('BLE Module: received new status $newStatus');
        final currentStatus = _reader(bleStatusProvider);
        if (currentStatus != null) {
          newStatus = currentStatus.copyWith(
            status: newStatus.status,
            deviceInfo: newStatus.deviceInfo,
            discoveredNodes: newStatus.discoveredNodes,
            nodesPendingConfirmation: newStatus.discoveredNodes,
          );
          _log.finest(
              'BLE Module: merged status with current status. New Status: $newStatus');
        }
        _reader(bleStatusProvider.state).state = newStatus;
        return;
      } else if (resp.data is BleRightsRequest) {
        if (Platform.isAndroid) {
          final permissions = await [
            Permission.bluetooth,
            Permission.bluetoothScan,
            Permission.bluetoothConnect,
            Permission.bluetoothAdvertise
          ].request();

          final stats = <String>[];
          for (final p in permissions.entries) {
            stats.add('\n\t· ${p.key}: ${p.value}');
          }
          Future.delayed(const Duration(seconds: 5)).then((value) => _log.config(
              '[Android] Required BLE Permission Statuses: ${stats.join()}'));

          final msg = Ble(
            rightsResult: RightsResult(
                rightsGranted: permissions.values
                    .where((p) => p != PermissionStatus.granted)
                    .isEmpty),
          );
          await _encodeAndSendMessage(Modules.FEED, msg.writeToBuffer());
          return;
        }
      }
    }

    _log.severe('_processResponse: UnhandledRpcMessageException($resp)');
    throw UnhandledRpcMessageException.value(
        resp.toString(), '_processResponse');
  }
}
