import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:typed_data';

import 'package:fixnum/fixnum.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';
import 'package:logging/logging.dart';
import 'package:protobuf/protobuf.dart' as pb;
import 'package:utils/utils.dart';
import 'package:uuid/uuid.dart';

import '../qaul_rpc.dart';
import 'generated/connections/ble/ble_rpc.pb.dart';
import 'generated/connections/connections.pb.dart';
import 'generated/node/node.pb.dart';
import 'generated/node/user_accounts.pb.dart';
import 'generated/router/users.pb.dart';
import 'generated/rpc/debug.pb.dart';
import 'generated/rpc/qaul_rpc.pb.dart';
import 'generated/services/chat/chat.pb.dart';
import 'generated/services/chat/chatfile_rpc.pb.dart';
import 'generated/services/dtn/dtn_rpc.pb.dart';
import 'generated/services/feed/feed.pb.dart';
import 'generated/services/group/group_rpc.pb.dart';
import 'internal/file_history.dart';
import 'libqaul/libqaul.dart';
import 'rpc_translators/abstract_rpc_module_translator.dart';
import 'utils.dart';

final qaulWorkerProvider = Provider<LibqaulWorker>((ref) => LibqaulWorker(ref));

final libqaulLogsStoragePath = StateProvider<String?>((ref) => null);

class LibqaulWorker {
  LibqaulWorker(Ref ref) : _ref = ref {
    _init();
  }

  final Ref _ref;
  final _log = Logger('LibqaulWorker');

  Libqaul get _lib => _ref.read(libqaulProvider);

  Future<bool> get initialized => _initialized.future;
  final _initialized = Completer<bool>();

  final _heartbeats = Queue<bool>();
  final _pendingRequests = <String, _PendingRequest>{};

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
      while ((await _lib.checkReceiveQueue()) > 0) {
        await _receiveResponse();
      }
    });
    Timer.periodic(const Duration(seconds: 2), (_) async {
      if (_heartbeats.length == 5) {
        _log.warning('libqaul stopped responding to heartbeats.');
      }
      if (_heartbeats.length < 5) _heartbeats.addLast(true);
      _log.finer('requesting heartbeat to libqaul');
      final msg = Debug(heartbeatRequest: HeartbeatRequest());
      _sendMessage(Modules.DEBUG, msg);
    });

    _initialized.complete(true);
  }

  // *******************************
  // Public rpc requests
  // *******************************
  Future<void> sendPublicMessage(String content) async {
    final msg = Feed(send: SendMessage(content: content));
    await _sendMessage(Modules.FEED, msg);
  }

  Future<void> requestPublicMessages({
    int? lastIndex,
    int? offset,
    int? limit,
  }) async {
    final request = FeedMessageRequest(lastIndex: Int64(lastIndex ?? 0));
    if (offset != null) request.offset = offset;
    if (limit != null) request.limit = limit;
    final msg = Feed(request: request);
    await _sendMessage(Modules.FEED, msg);
  }

  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    final request = UserRequest();
    if (offset != null) request.offset = offset;
    if (limit != null) request.limit = limit;
    final result = await _sendRequest<PaginatedUsers>(
      module: Modules.USERS,
      data: Users(userRequest: request),
      adapter: (res) {
        if (res.data is PaginatedUsers) return res.data as PaginatedUsers;
        return null;
      },
    );
    return result;
  }

  Future<PaginatedUsers?> getOnlineUsers({int? offset, int? limit}) async {
    final request = UserOnlineRequest(offset: offset, limit: limit);
    final result = await _sendRequest<PaginatedUsers>(
      module: Modules.USERS,
      data: Users(userOnlineRequest: request),
      adapter: (res) {
        if (res.data is PaginatedUsers) return res.data as PaginatedUsers;
        return null;
      },
    );
    return result;
  }

  Future<User?> getUserById(Uint8List userId) async {
    final result = await _sendRequest<User>(
      module: Modules.USERS,
      data: Users(getUserByIdRequest: GetUserByIDRequest(userId: userId)),
      adapter: (res) {
        if (res.data is GetUserByIdResult) {
          return (res.data as GetUserByIdResult).user;
        }
        return null;
      },
    );
    return result;
  }

  Future<SecurityNumber?> getUserSecurityNumber(User u) async {
    final msg = Users(
      securityNumberRequest: SecurityNumberRequest(userId: u.id.toList()),
    );
    final result = await _sendRequest<SecurityNumber>(
      module: Modules.USERS,
      data: msg,
      adapter: (res) {
        if (res.data is SecurityNumber) return res.data as SecurityNumber;
        return null;
      },
    );
    return result;
  }

  Future<void> verifyUser(User u) async {
    var entry = _baseUserEntryFrom(u);
    entry.verified = true;
    await _sendMessage(Modules.USERS, Users(userUpdate: entry));
  }

  Future<void> unverifyUser(User u) async {
    var entry = _baseUserEntryFrom(u);
    entry.verified = false;
    await _sendMessage(Modules.USERS, Users(userUpdate: entry));
  }

  Future<void> blockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = true;
    await _sendMessage(Modules.USERS, Users(userUpdate: entry));
  }

  Future<void> unblockUser(User u) async {
    final entry = _baseUserEntryFrom(u);
    entry.blocked = false;
    await _sendMessage(Modules.USERS, Users(userUpdate: entry));
  }

  Future<NodeInfo?> getNodeInfo() async {
    final result = await _sendRequest<NodeInfo>(
      module: Modules.NODE,
      data: Node(getNodeInfo: true),
      adapter: (res) {
        if (res.data is NodeInfo) return res.data as NodeInfo;
        return null;
      },
    );
    return result;
  }

  // -------------------
  // CONNECTIONS Requests
  // -------------------
  Future<List<InternetNode>> requestNodes() async {
    final result = await _sendRequest<List<InternetNode>>(
      module: Modules.CONNECTIONS,
      data: Connections(internetNodesRequest: InternetNodesRequest()),
      adapter: (res) {
        if (res.data is List<InternetNode>) {
          return res.data as List<InternetNode>;
        }
        return null;
      },
    );
    return result ?? [];
  }

  Future<void> addNode(String address, [String? name]) async =>
      await _sendMessage(
          Modules.CONNECTIONS,
          Connections(
            internetNodesAdd: InternetNodesEntry(
              address: address,
              enabled: true,
              name: name,
            ),
          ));

  Future<void> removeNode(String address) async => await _sendMessage(
      Modules.CONNECTIONS,
      Connections(internetNodesRemove: InternetNodesEntry(address: address)));

  Future<void> setNodeState(String address, {bool active = true}) async {
    var msg = Connections(
      internetNodesState: InternetNodesEntry(address: address, enabled: active),
    );
    await _sendMessage(Modules.CONNECTIONS, msg);
  }

  Future<void> renameNode(String address, {required String name}) async {
    var msg = Connections(
      internetNodesRename: InternetNodesEntry(address: address, name: name),
    );
    await _sendMessage(Modules.CONNECTIONS, msg);
  }

  // -------------------
  Future<User?> getDefaultUserAccount() async {
    final message = UserAccounts(getDefaultUserAccount: true);
    final result = await _sendRequest<User>(
      module: Modules.USERACCOUNTS,
      data: message,
      adapter: (res) {
        if (res.data is User) return res.data as User;
        return null;
      },
    );
    return result;
  }

  Future<void> createUserAccount(String name) async {
    final msg = UserAccounts(createUserAccount: CreateUserAccount(name: name));
    await _sendMessage(Modules.USERACCOUNTS, msg);
  }

  Future<List<ChatRoom>> getAllChatRooms() async {
    final msg = Group(groupListRequest: GroupListRequest());
    final result = await _sendRequest<List<ChatRoom>>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is List<ChatRoom>) return res.data as List<ChatRoom>;
        return null;
      },
    );
    return result ?? [];
  }

  Future<ChatConversationList?> getChatRoomMessages(Uint8List chatId,
      {int lastIndex = 0}) async {
    final msg = Chat(
      conversationRequest: ChatConversationRequest(
        groupId: chatId,
        lastIndex: Int64(lastIndex),
      ),
    );
    final result = await _sendRequest<ChatConversationList>(
      module: Modules.CHAT,
      data: msg,
      adapter: (res) {
        if (res.data is ChatConversationList) {
          return res.data as ChatConversationList;
        }
        return null;
      },
    );
    return result;
  }

  Future<void> sendMessage(Uint8List chatId, String content) async {
    final msg = Chat(
      send: ChatMessageSend(groupId: chatId, content: content),
    );
    await _sendMessage(Modules.CHAT, msg);
  }

  // -------------------
  // GROUP Requests
  // -------------------
  Future<ChatRoom?> getGroupInfo(Uint8List id) async {
    final msg = Group(groupInfoRequest: GroupInfoRequest(groupId: id.toList()));
    final result = await _sendRequest<ChatRoom>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is ChatRoom) return res.data as ChatRoom;
        return null;
      },
    );
    return result;
  }

  Future<List<GroupInvite>> getGroupInvitesReceived() async {
    final msg = Group(groupInvitedRequest: GroupInvitedRequest());
    final result = await _sendRequest<List<GroupInvite>>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is List<GroupInvite>) {
          return res.data as List<GroupInvite>;
        }
        return null;
      },
    );
    return result ?? [];
  }

  Future<bool> createGroup(String name) async {
    assert(name.isNotEmpty);
    final msg = Group(groupCreateRequest: GroupCreateRequest(groupName: name));
    final result = await _sendRequest<bool>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  Future<bool> renameGroup(ChatRoom room, String name) async {
    assert(name.isNotEmpty);
    final msg = Group(
      groupRenameRequest: GroupRenameRequest(
          groupId: room.conversationId.toList(), groupName: name),
    );
    final result = await _sendRequest<bool>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  Future<bool> inviteUserToGroup(User user, ChatRoom room) async {
    final msg = Group(
      groupInviteMemberRequest: GroupInviteMemberRequest(
        groupId: room.conversationId.toList(),
        userId: user.id.toList(),
      ),
    );
    final result = await _sendRequest<bool>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  Future<bool> removeUserFromGroup(User user, ChatRoom room) async {
    final msg = Group(
      groupRemoveMemberRequest: GroupRemoveMemberRequest(
        groupId: room.conversationId.toList(),
        userId: user.id.toList(),
      ),
    );
    final result = await _sendRequest<bool>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  Future<bool> replyToGroupInvite(Uint8List groupId,
      {required bool accepted}) async {
    final msg = Group(
      groupReplyInviteRequest: GroupReplyInviteRequest(
        groupId: groupId.toList(),
        accept: accepted,
      ),
    );
    final result = await _sendRequest<bool>(
      module: Modules.GROUP,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  // -------------------
  // CHATFILE Requests
  // -------------------
  Future<void> sendFile({
    required String pathName,
    required Uint8List conversationId,
    required String description,
  }) async {
    var file = File(pathName);
    final maxUncompressedSizeKB = 150 * 1000;
    if (isImage(file.path) && file.statSync().size >= maxUncompressedSizeKB) {
      final compressed = await processImage(file);
      if (compressed != null) file = compressed;
    }
    final msg = ChatFile(
        sendFileRequest: SendFileRequest(
      pathName: file.path,
      groupId: conversationId.toList(),
      description: description,
    ));
    await _sendMessage(Modules.CHATFILE, msg);
  }

  Future<List<FileHistoryEntity>> getFileHistory(
      {int page = 0, int itemsPerPage = 20}) async {
    final msg = ChatFile(
        fileHistory: FileHistoryRequest(
      offset: page * itemsPerPage,
      limit: itemsPerPage,
    ));

    final result = await _sendRequest<List<FileHistoryEntity>>(
      module: Modules.CHATFILE,
      data: msg,
      adapter: (res) {
        if (res.data is List<FileHistoryEntity>) {
          return res.data as List<FileHistoryEntity>;
        }
        return null;
      },
    );

    // Backwards compat: clear the provider as the old implementation did
    _ref.read(fileHistoryEntitiesProvider.notifier).clear();
    return result ?? [];
  }

  // -------------------
  // DTN Requests
  // -------------------
  Future<DTNConfiguration?> getDTNConfiguration() async {
    final msg = DTN(dtnConfigRequest: DtnConfigRequest());
    final result = await _sendRequest<DTNConfiguration>(
      module: Modules.DTN,
      data: msg,
      adapter: (res) {
        if (res.data is DTNConfiguration) {
          return res.data as DTNConfiguration;
        }
        return null;
      },
    );
    return result;
  }

  Future<bool> addDTNUser(Uint8List userId) async {
    final msg =
        DTN(dtnAddUserRequest: DtnAddUserRequest(userId: userId.toList()));
    final result = await _sendRequest<bool>(
      module: Modules.DTN,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  Future<bool> removeDTNUser(Uint8List userId) async {
    final msg = DTN(dtnRemoveUserRequest: DtnRemoveUserRequest(userId: userId));
    final result = await _sendRequest<bool>(
      module: Modules.DTN,
      data: msg,
      adapter: (res) {
        if (res.data is bool) return res.data as bool;
        return null;
      },
    );
    return result ?? false;
  }

  // -------------------
  Future<void> setLibqaulLogging(bool enabled) async {
    final msg = Debug(logToFile: LogToFile(enable: enabled));
    await _sendMessage(Modules.DEBUG, msg);
  }

  Future<void> deleteLogs() async {
    final msg = Debug(deleteLibqaulLogsRequest: DeleteLibqaulLogsRequest());
    await _sendMessage(Modules.DEBUG, msg);
  }

  Future<void> sendBleInfoRequest() async {
    for (final message in [
      Ble(infoRequest: InfoRequest()),
      Ble(discoveredRequest: DiscoveredRequest()),
    ]) {
      await _sendMessage(Modules.BLE, message);
    }
  }

  // *******************************
  // Private (helper) methods
  // *******************************
  UserEntry _baseUserEntryFrom(User u) => UserEntry(
        name: u.name,
        id: u.id,
        keyBase58: u.keyBase58,
      );

  void _getLibqaulLogsStoragePath() async {
    final msg = Debug(storagePathRequest: StoragePathRequest());
    await _sendMessage(Modules.DEBUG, msg);
  }

  // *******************************
  // Private (control) methods
  // *******************************
  Future<void> _sendMessage(Modules module, pb.GeneratedMessage data,
      {String? requestId}) async {
    requestId ??= const Uuid().v4();
    QaulRpc message = QaulRpc()
      ..module = module
      ..data = data.writeToBuffer()
      ..requestId = requestId;

    final user = _ref.read(defaultUserProvider);
    if (user != null) message.userId = user.id;

    await _ref.read(libqaulProvider).sendRpc(message.writeToBuffer());
  }

  Future<T?> _sendRequest<T>({
    required Modules module,
    required pb.GeneratedMessage data,
    required T? Function(RpcTranslatorResponse) adapter,
    Duration timeout = const Duration(seconds: 10),
  }) async {
    final requestId = const Uuid().v4();
    final completer = Completer<T?>();

    final timer = Timer(timeout, () {
      if (!completer.isCompleted) {
        _log.warning('RPC request $requestId timed out after $timeout');
        completer.complete(null);
        _pendingRequests.remove(requestId);
      }
    });

    _pendingRequests[requestId] = _PendingRequest<T>(
      completer: completer,
      adapter: adapter,
      timer: timer,
    );

    await _sendMessage(module, data, requestId: requestId);

    return completer.future;
  }

  Future<void> _receiveResponse() async {
    final response = await _lib.receiveRpc();
    if (response == null) return;

    final m = QaulRpc.fromBuffer(response);
    final translator = RpcModuleTranslator.translatorFactory(m.module);

    RpcTranslatorResponse? res;
    try {
      res = await translator.decodeMessageBytes(m.data, _ref);
    } catch (e, st) {
      // Translators (e.g. GroupTranslator, DTNTranslator) throw on error
      // responses. If there's a pending request, propagate the error
      // immediately.
      if (m.requestId.isNotEmpty) {
        final pending = _pendingRequests.remove(m.requestId);
        if (pending != null && !pending.completer.isCompleted) {
          pending.timer.cancel();
          _log.warning('decodeMessageBytes failed for ${m.requestId}', e, st);
          pending.completer.completeError(e, st);
        }
      }
      return;
    }

    // Resolve pending Future-based requests if a matching requestId exists
    if (m.requestId.isNotEmpty) {
      final pending = _pendingRequests.remove(m.requestId);
      if (pending != null && !pending.completer.isCompleted) {
        pending.timer.cancel();
        try {
          final value = res != null ? pending.adapter(res) : null;
          pending.completer.complete(value);
        } catch (e, st) {
          _log.warning('Error in RPC adapter for ${m.requestId}', e, st);
          pending.completer.completeError(e, st);
        }
      }
    }

    if (res == null) return;

    if (res.module == Modules.BLE && res.data is BleRightsRequest) {
      _log.fine('BleRightsRequest received, must be handled by native code');
      return;
    }
    if (res.module != Modules.DEBUG) {
      translator.processResponse(res, _ref);
      return;
    }

    if (res.data is bool) {
      _log.finer('libqaul answered a heartbeat request');
      if (_heartbeats.isNotEmpty) _heartbeats.clear();
    }
    if (res.data is String) {
      final path =
          await findFolderWithFilesOfExtension(Directory(res.data), '.log');
      _log.info('libqaul log storage path: $path');
      _ref.read(libqaulLogsStoragePath.notifier).state = path;
    }
  }
}

class _PendingRequest<T> {
  _PendingRequest({
    required this.completer,
    required this.adapter,
    required this.timer,
  });

  final Completer<T?> completer;
  final T? Function(RpcTranslatorResponse) adapter;
  final Timer timer;
}
