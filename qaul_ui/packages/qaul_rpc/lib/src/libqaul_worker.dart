import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/generated/node/node.pb.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
import 'package:qaul_rpc/src/generated/services/feed/feed.pb.dart' as pb_feed;
import 'package:qaul_rpc/src/rpc_translators/abstract_rpc_module_translator.dart';
import 'package:uuid/uuid.dart';

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
    final msg = pb_feed.Feed(send: pb_feed.SendMessage(content: content));
    await _encodeAndSendMessage(Modules.FEED, msg.writeToBuffer());
  }

  Future<void> requestFeedMessages({List<int>? lastReceived}) async {
    final msg = pb_feed.Feed(
      request: pb_feed.FeedMessageRequest(lastReceived: lastReceived),
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
      final message = QaulRpc.fromBuffer(response);

      if (message.module == Modules.FEED) {
        final resp = await FeedTranslator().decodeMessageBytes(message.data);
        if (resp != null) _processResponse(resp);
      } else if (message.module == Modules.NODE) {
        final resp = await NodeTranslator().decodeMessageBytes(message.data);
        debugPrint('RpcNode node id: ${resp?.data}');
      } else if (message.module == Modules.USERS) {
        final resp = await UsersTranslator().decodeMessageBytes(message.data);
        if (resp != null) _processResponse(resp);
      } else if (message.module == Modules.ROUTER) {
        final resp = await RouterTranslator().decodeMessageBytes(message.data);
        if (resp != null) _processResponse(resp);
      } else {
        throw UnhandledRpcMessageException.value(
            message.toString(), 'LibqaulWorker.receiveResponse');
      }
    }
  }

  void _processResponse(RpcTranslatorResponse resp) {
    // TODO: feed streams through here
    if (resp.data is List<User>) {
      final provider = _reader(usersProvider.notifier);

      for (final user in resp.data) {
        provider.contains(user) ? provider.update(user) : provider.add(user);
      }
      return;
    }
    if (resp.module == Modules.FEED) {
      if (resp.data != null && resp.data is List<FeedMessage>) {
        final provider = _reader(feedMessagesProvider.notifier);

        for (final msg in resp.data) {
          if (!provider.contains(msg)) provider.add(msg);
        }
        return;
      }
    }

    throw UnhandledRpcMessageException.value(
        resp.toString(), '_processResponse');
  }
}
