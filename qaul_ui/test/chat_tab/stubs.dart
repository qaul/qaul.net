part of 'chat_tab_test.dart';

class StubLibqaulWorker implements LibqaulWorker {
  StubLibqaulWorker(this.ref);

  final Ref ref;
  final _logger = Logger('StubLibqaulWorker');

  @override
  Future<void> sendMessage(Uint8List chatId, String content) async {
    _logger.info('sending message "$content" to chat id: "$chatId"');
    final room = ref.read(currentOpenChatRoom);

    final index = (room?.messages?.length ?? 0) + 1;
    final message = Message(
      senderId: defaultUser.id,
      messageId: Uint8List.fromList(content.codeUnits),
      content: TextMessageContent(content),
      index: index,
      sentAt: DateTime(2000),
      receivedAt: DateTime(2000),
    );

    final msgs = List<Message>.from(
      room == null || room.messages == null ? [] : room.messages!,
    );
    msgs.add(message);
    ref.read(currentOpenChatRoom.notifier).state =
        buildGroupChat(messages: msgs);
  }

  @override
  Future<ChatConversationList?> getChatRoomMessages(Uint8List chatId,
      {int lastIndex = 0}) async {
    _logger.info('requested messages fetch; ignoring...');
    return null;
  }

  @override
  Future<List<ChatRoom>> getAllChatRooms() async {
    _logger.info('requested all Chat rooms fetch; ignoring...');
    return [];
  }

  @override
  Future<List<GroupInvite>> getGroupInvitesReceived() async {
    _logger.info('requested group invites fetch; ignoring...');
    return [];
  }

  static const int _mockTotalUsers = 125;
  static const int _defaultPageSize = 50;

  @override
  Future<PaginatedUsers?> getUsers({int? offset, int? limit}) async {
    final requestedOffset = offset ?? 0;
    final requestedLimit = limit ?? _defaultPageSize;
    _logger.info(
      'getUsers (mock): offset=$requestedOffset limit=$requestedLimit '
      'total=$_mockTotalUsers',
    );
    await Future.delayed(const Duration(milliseconds: 50));
    final start = requestedOffset.clamp(0, _mockTotalUsers);
    final end = (requestedOffset + requestedLimit).clamp(0, _mockTotalUsers);
    final count = end - start;
    final mockUsers = List<User>.generate(
      count,
      (index) {
        final globalIndex = start + index;
        return User(
          name: 'Mock User ${globalIndex + 1}',
          id: Uint8List.fromList('mock_user_$globalIndex'.codeUnits),
        );
      },
    );
    final hasMore = end < _mockTotalUsers;
    final pagination = PaginationState(
      hasMore: hasMore,
      total: _mockTotalUsers,
      offset: requestedOffset,
      limit: requestedLimit,
    );
    return PaginatedUsers(users: mockUsers, pagination: pagination);
  }

  // -------------------------------------------
  // Unimplemented methods
  // -------------------------------------------
  @override
  Future<bool> addDTNUser(Uint8List userId) => throw UnimplementedError();

  @override
  Future<void> addNode(String address, [String? name]) =>
      throw UnimplementedError();

  @override
  Future<void> blockUser(User u) => throw UnimplementedError();

  @override
  Future<bool> createGroup(String name) => throw UnimplementedError();

  @override
  Future<void> createUserAccount(String name) => throw UnimplementedError();

  @override
  Future<void> deleteLogs() => throw UnimplementedError();

  @override
  Future<DTNConfiguration?> getDTNConfiguration() =>
      throw UnimplementedError();

  @override
  Future<User?> getDefaultUserAccount() => throw UnimplementedError();

  @override
  Future<List<FileHistoryEntity>> getFileHistory(
          {int? page, int? itemsPerPage}) =>
      throw UnimplementedError();

  @override
  Future<ChatRoom?> getGroupInfo(Uint8List id) => throw UnimplementedError();

  @override
  Future<PaginatedUsers?> getOnlineUsers({int? offset, int? limit}) =>
      throw UnimplementedError();

  @override
  Future<User?> getUserById(Uint8List userId) => Future.value(null);

  @override
  Future<NodeInfo?> getNodeInfo() => throw UnimplementedError();

  @override
  Future<SecurityNumber?> getUserSecurityNumber(User u) =>
      throw UnimplementedError();

  @override
  Future<bool> get initialized => Future.value(true);

  @override
  Future<bool> inviteUserToGroup(User user, ChatRoom room) =>
      throw UnimplementedError();

  @override
  Future<bool> removeDTNUser(Uint8List userId) => throw UnimplementedError();

  @override
  Future<void> removeNode(String address) => throw UnimplementedError();

  @override
  Future<bool> removeUserFromGroup(User user, ChatRoom room) =>
      throw UnimplementedError();

  @override
  Future<bool> renameGroup(ChatRoom room, String name) =>
      throw UnimplementedError();

  @override
  Future<bool> replyToGroupInvite(Uint8List groupId,
          {required bool accepted}) =>
      throw UnimplementedError();

  @override
  Future<List<InternetNode>> requestNodes() => throw UnimplementedError();

  @override
  Future<void> requestPublicMessages({
    int? lastIndex,
    int? offset,
    int? limit,
  }) =>
      throw UnimplementedError();

  @override
  Future<void> sendBleInfoRequest() => throw UnimplementedError();

  @override
  Future<void> sendFile(
          {required String pathName,
          required Uint8List conversationId,
          required String description}) =>
      throw UnimplementedError();

  @override
  Future<void> sendPublicMessage(String content) => throw UnimplementedError();

  @override
  Future<void> setLibqaulLogging(bool enabled) => throw UnimplementedError();

  @override
  Future<void> setNodeState(String address, {bool active = true}) =>
      throw UnimplementedError();

  @override
  Future<void> unblockUser(User u) => throw UnimplementedError();

  @override
  Future<void> unverifyUser(User u) => throw UnimplementedError();

  @override
  Future<void> verifyUser(User u) => throw UnimplementedError();

  @override
  Future<void> renameNode(String address, {required String name}) async {}
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
  MapEntry<dynamic, void Function(List<ChatRoom>? p1, List<ChatRoom> p2)>
      get strategy => throw UnimplementedError();

  @override
  void updatePersistentCachedData() {}
}
