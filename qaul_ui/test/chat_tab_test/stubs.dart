part of 'chat_tab_test.dart';

class StubLibqaulWorker implements LibqaulWorker {
  @override
  void addDTNUser(Uint8List userId) {}

  @override
  Future<void> addNode(String address) {
    throw UnimplementedError();
  }

  @override
  Future<void> blockUser(User u) {
    throw UnimplementedError();
  }

  @override
  void createGroup(String name) {}

  @override
  Future<void> createUserAccount(String name) {
    throw UnimplementedError();
  }

  @override
  void deleteLogs() {}

  @override
  void getAllChatRooms() {}

  @override
  void getChatRoomMessages(Uint8List chatId, {int lastIndex = 0}) {}

  @override
  void getDTNConfiguration() {}

  @override
  Future<void> getDefaultUserAccount() {
    throw UnimplementedError();
  }

  @override
  void getFileHistory({int? offset, int? limit}) {}

  @override
  void getGroupInfo(Uint8List id) {}

  @override
  void getGroupInvitesReceived() {}

  @override
  Future<void> getNodeInfo() {
    throw UnimplementedError();
  }

  @override
  void getUserSecurityNumber(User u) {}

  @override
  Future<void> getUsers() {
    throw UnimplementedError();
  }

  @override
  Future<bool> get initialized => throw UnimplementedError();

  @override
  void inviteUserToGroup(User user, ChatRoom room) {}

  @override
  void removeDTNUser(Uint8List userId) {}

  @override
  Future<void> removeNode(String address) {
    throw UnimplementedError();
  }

  @override
  void removeUserFromGroup(User user, ChatRoom room) {}

  @override
  void renameGroup(ChatRoom room, String name) {}

  @override
  void replyToGroupInvite(Uint8List groupId, {required bool accepted}) {}

  @override
  Future<void> requestNodes() {
    throw UnimplementedError();
  }

  @override
  Future<void> requestPublicMessages({int? lastIndex}) async {
    throw UnimplementedError();
  }

  @override
  void sendBleInfoRequest() {}

  @override
  void sendFile(
      {required String pathName,
      required Uint8List conversationId,
      required String description}) {}

  @override
  void sendMessage(Uint8List chatId, String content) {}

  @override
  Future<void> sendPublicMessage(String content) {
    throw UnimplementedError();
  }

  @override
  void setLibqaulLogging(bool enabled) {}

  @override
  void setNodeState(String address, {bool active = true}) {}

  @override
  Future<void> unblockUser(User u) {
    throw UnimplementedError();
  }

  @override
  Future<void> unverifyUser(User u) {
    throw UnimplementedError();
  }

  @override
  Future<void> verifyUser(User u) {
    throw UnimplementedError();
  }
}

class NullChatNotificationController implements ChatNotificationController {
  @override
  late ValueNotifier<int?> newNotificationCount;

  @override
  String get cacheKey => throw UnimplementedError();

  @override
  void close() {}

  @override
  TabType get currentVisibleHomeTab => throw UnimplementedError();

  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) {
    throw UnimplementedError();
  }

  @override
  void execute(List<ChatRoom>? previous, List<ChatRoom> current) {}

  @override
  Future<void> initialize() async {}

  @override
  User get localUser => throw UnimplementedError();

  @override
  SharedPreferences get preferences => throw UnimplementedError();

  @override
  LocalNotification? process(ChatRoom value) {
    throw UnimplementedError();
  }

  @override
  Ref get ref => throw UnimplementedError();

  @override
  void removeNotifications() {}

  @override
  MapEntry<AlwaysAliveProviderListenable<List<ChatRoom>>,
          void Function(List<ChatRoom>? p1, List<ChatRoom> p2)>
      get strategy => throw UnimplementedError();

  @override
  void updatePersistentCachedData() {}
}
