import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/generated/connections/connections.pb.dart';
import 'package:qaul_rpc/src/generated/node/node.pb.dart';
import 'package:qaul_rpc/src/generated/node/user_accounts.pb.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
import 'package:qaul_rpc/src/generated/services/feed/feed.pb.dart';
import 'package:qaul_rpc/src/rpc_translators/abstract_rpc_module_translator.dart';
import 'package:uuid/uuid.dart';

import 'libqaul/libqaul.dart';

class LibqaulWorker {
  LibqaulWorker(Reader reader) : _reader = reader {
    _init();
  }

  final Reader _reader;

  Libqaul get _lib => _reader(libqaulProvider);

  Future<bool> get initialized => _initialized.future;
  final _initialized = Completer<bool>();

  void _init() async {
    if (_initialized.isCompleted) return;
    // Throws when called for some reason
    // await _lib.load();
    await _lib.start();
    while (await _lib.initialized() != 1) {
      await Future.delayed(const Duration(milliseconds: 10));
    }
    Timer.periodic(const Duration(milliseconds: 100), (_) async {
      final n = await _lib.checkReceiveQueue();
      if (n > 0) _receiveResponse();
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

  Future<void> requestFeedMessages({List<int>? lastReceived}) async {
    final msg = Feed(
      request: FeedMessageRequest(lastReceived: lastReceived),
    );
    _encodeAndSendMessage(Modules.FEED, msg.writeToBuffer());
  }

  Future<void> getUsers() async {
    final id = await _encodeAndSendMessage(
        Modules.USERS, Users(userRequest: UserRequest()).writeToBuffer());

    _encodeAndSendMessage(Modules.ROUTER,
        Router(routingTableRequest: RoutingTableRequest()).writeToBuffer());

    debugPrint('*' * 80);
    debugPrint('ID: $id');
    debugPrint('*' * 80);
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

  // *******************************
  // Private (control) methods
  // *******************************
  Future<String> _encodeAndSendMessage(Modules module, Uint8List data) async {
    // create message
    QaulRpc message = QaulRpc();
    message.module = module;
    message.data = data;

    final user = _reader(defaultUserProvider).state;
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
        final resp =
            await ConnectionTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.FEED) {
        final resp = await FeedTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.NODE) {
        final resp = await NodeTranslator().decodeMessageBytes(m.data);
        debugPrint('RpcNode node id: ${resp?.data}');
      } else if (m.module == Modules.USERACCOUNTS) {
        final resp = await UserAccountsTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.USERS) {
        final resp = await UsersTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else if (m.module == Modules.ROUTER) {
        final resp = await RouterTranslator().decodeMessageBytes(m.data);
        if (resp != null) _processResponse(resp);
      } else {
        throw UnhandledRpcMessageException.value(
            m.toString(), 'LibqaulWorker.receiveResponse');
      }
    }
  }

  void _processResponse(RpcTranslatorResponse resp) {
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
        _reader(defaultUserProvider).state = resp.data;
      }
      return;
    }

    throw UnhandledRpcMessageException.value(
        resp.toString(), '_processResponse');
  }
}
