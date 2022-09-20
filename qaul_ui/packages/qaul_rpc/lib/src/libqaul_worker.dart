import 'dart:async';
import 'dart:collection';
import 'dart:io';
import 'dart:typed_data';

import 'package:fixnum/fixnum.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:permission_handler/permission_handler.dart';
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

final qaulWorkerProvider =
    Provider<LibqaulWorker>((ref) => LibqaulWorker(ref.read));

final libqaulLogsStoragePath = StateProvider<String?>((ref) => null);

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

  Future<void> requestPublicMessages({int? lastIndex}) async {
    final msg = Feed(
      request: FeedMessageRequest(lastIndex: Int64(lastIndex ?? 0)),
    );
    _sendMessage(Modules.FEED, msg);
  }

  Future<void> getUsers() async =>
      await _sendMessage(Modules.USERS, Users(userRequest: UserRequest()));

  void getUserSecurityNumber(User u) async {
    final msg = Users(
      securityNumberRequest: SecurityNumberRequest(userId: u.id.toList()),
    );
    _sendMessage(Modules.USERS, msg);
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

  Future<void> getNodeInfo() async =>
      await _sendMessage(Modules.NODE, Node(getNodeInfo: true));

  // -------------------
  // CONNECTIONS Requests
  // -------------------
  Future<void> requestNodes() async => await _sendMessage(Modules.CONNECTIONS,
      Connections(internetNodesRequest: InternetNodesRequest()));

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

  void setNodeState(String address, {bool active = true}) async {
    var msg = Connections(
      internetNodesState: InternetNodesEntry(address: address, enabled: active),
    );
    _sendMessage(Modules.CONNECTIONS, msg);
  }

  void renameNode(String address, {required String name}) {
    var msg = Connections(
      internetNodesRename: InternetNodesEntry(address: address, name: name),
    );
    _sendMessage(Modules.CONNECTIONS, msg);
  }

  // -------------------
  Future<void> getDefaultUserAccount() async {
    final message = UserAccounts(getDefaultUserAccount: true);
    await _sendMessage(Modules.USERACCOUNTS, message);
  }

  Future<void> createUserAccount(String name) async {
    final msg = UserAccounts(createUserAccount: CreateUserAccount(name: name));
    await _sendMessage(Modules.USERACCOUNTS, msg);
  }

  void getAllChatRooms() async {
    final msg = Group(groupListRequest: GroupListRequest());
    await _sendMessage(Modules.GROUP, msg);
  }

  void getChatRoomMessages(Uint8List chatId, {int lastIndex = 0}) async {
    final msg = Chat(
      conversationRequest: ChatConversationRequest(
        groupId: chatId,
        lastIndex: Int64(lastIndex),
      ),
    );
    await _sendMessage(Modules.CHAT, msg);
  }

  void sendMessage(Uint8List chatId, String content) async {
    final msg = Chat(
      send: ChatMessageSend(groupId: chatId, content: content),
    );
    await _sendMessage(Modules.CHAT, msg);
  }

  // -------------------
  // GROUP Requests
  // -------------------
  void getGroupInfo(Uint8List id) async {
    final msg = Group(groupInfoRequest: GroupInfoRequest(groupId: id.toList()));
    await _sendMessage(Modules.GROUP, msg);
  }

  void getGroupInvitesReceived() async {
    final msg = Group(groupInvitedRequest: GroupInvitedRequest());
    await _sendMessage(Modules.GROUP, msg);
  }

  void createGroup(String name) async {
    assert(name.isNotEmpty);
    final msg = Group(groupCreateRequest: GroupCreateRequest(groupName: name));
    await _sendMessage(Modules.GROUP, msg);
  }

  void renameGroup(ChatRoom room, String name) async {
    assert(name.isNotEmpty);
    final msg = Group(
      groupRenameRequest: GroupRenameRequest(
          groupId: room.conversationId.toList(), groupName: name),
    );
    await _sendMessage(Modules.GROUP, msg);
  }

  void inviteUserToGroup(User user, ChatRoom room) async {
    final msg = Group(
      groupInviteMemberRequest: GroupInviteMemberRequest(
        groupId: room.conversationId.toList(),
        userId: user.id.toList(),
      ),
    );
    await _sendMessage(Modules.GROUP, msg);
  }

  void removeUserFromGroup(User user, ChatRoom room) async {
    final msg = Group(
      groupRemoveMemberRequest: GroupRemoveMemberRequest(
        groupId: room.conversationId.toList(),
        userId: user.id.toList(),
      ),
    );
    await _sendMessage(Modules.GROUP, msg);
  }

  void replyToGroupInvite(Uint8List groupId, {required bool accepted}) async {
    final msg = Group(
      groupReplyInviteRequest: GroupReplyInviteRequest(
        groupId: groupId.toList(),
        accept: accepted,
      ),
    );
    await _sendMessage(Modules.GROUP, msg);
  }

  // -------------------
  // CHATFILE Requests
  // -------------------
  void sendFile({
    required String pathName,
    required Uint8List conversationId,
    required String description,
  }) async {
    var file = File(pathName);
    if (isImage(file.path) && file.statSync().size >= 150.kb) {
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
    Future<void> sendFileHistoryRequest() async {
      final msg = ChatFile(
          fileHistory: FileHistoryRequest(
        offset: page * itemsPerPage,
        limit: itemsPerPage,
      ));
      await _sendMessage(Modules.CHATFILE, msg);
    }

    List<FileHistoryEntity> newItems = [];
    try {
      await sendFileHistoryRequest();

      for (var i = 0; i < 5; i++) {
        await Future.delayed(Duration(milliseconds: (i + 1) * 500));
        newItems = _reader(fileHistoryEntitiesProvider);
        if (newItems.isNotEmpty) break;
      }
    } catch (error) {
      _log.warning('error fetching file history', error, StackTrace.current);
    } finally {
      _reader(fileHistoryEntitiesProvider.notifier).clear();
    }
    return newItems;
  }

  // -------------------
  // DTN Requests
  // -------------------
  void getDTNConfiguration() async {
    final msg = DTN(dtnConfigRequest: DtnConfigRequest());
    await _sendMessage(Modules.DTN, msg);
  }

  void addDTNUser(Uint8List userId) async {
    final msg =
        DTN(dtnAddUserRequest: DtnAddUserRequest(userId: userId.toList()));
    await _sendMessage(Modules.DTN, msg);
  }

  void removeDTNUser(Uint8List userId) async {
    final msg = DTN(dtnRemoveUserRequest: DtnRemoveUserRequest(userId: userId));
    await _sendMessage(Modules.DTN, msg);
  }

  // -------------------
  void setLibqaulLogging(bool enabled) async {
    final msg = Debug(logToFile: LogToFile(enable: enabled));
    await _sendMessage(Modules.DEBUG, msg);
  }

  void deleteLogs() async {
    final msg = Debug(deleteLibqaulLogsRequest: DeleteLibqaulLogsRequest());
    await _sendMessage(Modules.DEBUG, msg);
  }

  void sendBleInfoRequest() async {
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
  Future<void> _sendMessage(Modules module, pb.GeneratedMessage data) async {
    QaulRpc message = QaulRpc()
      ..module = module
      ..data = data.writeToBuffer()
      ..requestId = const Uuid().v4();

    final user = _reader(defaultUserProvider);
    if (user != null) message.userId = user.id;

    await _reader(libqaulProvider).sendRpc(message.writeToBuffer());
  }

  Future<void> _receiveResponse() async {
    final response = await _lib.receiveRpc();
    if (response == null) return;

    final m = QaulRpc.fromBuffer(response);
    final translator = RpcModuleTranslator.translatorFactory(m.module);
    final res = await translator.decodeMessageBytes(m.data, _reader);
    if (res == null) return;

    if (res.module == Modules.BLE && res.data is BleRightsRequest) {
      // TODO mode to ble_translator
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
        await _sendMessage(Modules.BLE, msg);
      }
      return;
    }
    if (res.module != Modules.DEBUG) {
      translator.processResponse(res, _reader);
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
      _reader(libqaulLogsStoragePath.state).state = path;
    }
  }
}
