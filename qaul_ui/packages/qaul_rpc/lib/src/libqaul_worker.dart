import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_rpc/src/rpc/rpc_module.dart';
import 'package:qaul_rpc/src/generated/rpc/qaul_rpc.pb.dart';
import 'package:qaul_rpc/src/generated/router/users.pb.dart';
import 'package:qaul_rpc/src/generated/router/router.pb.dart';
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
    Timer.periodic(const Duration(milliseconds: 1500), (_) async {
      await _initialized.future;
      await getUsers();
    });

    _initialized.complete(true);
  }

  // *******************************
  // Public rpc requests
  // *******************************
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

      if (message.module == Modules.USERS) {
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

    throw UnhandledRpcMessageException.value(
        resp.toString(), '_processResponse');
  }
}
