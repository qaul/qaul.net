part of 'providers.dart';

final chatNotificationControllerProvider = Provider((ref) => ChatNotificationController(ref));

class ChatNotificationController {
  ChatNotificationController(Ref ref) : _ref = ref;
  final Ref _ref;
  late final SharedPreferences _prefs;
  late final User _user;

  ChatRoom? _currentOpenRoom;
  final _chats = <_ChatData>[];

  final _log = Logger('ChatNotificationController');

  static const _cacheKey = 'chatNotificationControllerChatDataKey';

  Future<void> initialize() async {
    _prefs = await SharedPreferences.getInstance();
    if (_prefs.containsKey(_cacheKey)) {
      // _prefs.remove(_cacheKey);
      _chats.addAll(_prefs.getStringList(_cacheKey)!.map((e) {
        return _ChatData.fromJson(jsonDecode(e));
      }));
    }

    _user = _ref.read(defaultUserProvider)!;
    _ref.read(qaulWorkerProvider).getAllChatRooms();
    _ref.listen(chatRoomsProvider, _onChatRoomsChanged);
    _ref.listen(currentOpenChatRoom.state, _updateCurrentOpenRoom);

    _log.config('Initialized:\n· User: ${_user.name}\n· Cached chats: $_chats');
  }

  void _updateCurrentOpenRoom(_, StateController<ChatRoom?> notifier) {
    _currentOpenRoom = notifier.state;
  }

  void _updatedPersistentCachedChatData() =>
      _prefs.setStringList(_cacheKey, _chats.map(jsonEncode).toList());

  void _onChatRoomsChanged(List<ChatRoom>? _, List<ChatRoom> rooms) async {
    final queue = Queue<ChatRoom>();
    rooms.where((room) => !_localCacheContains(room)).forEach(queue.addLast);
    rooms.where(_localCacheContains).where(_hasNewMessage).forEach(queue.addLast);

    if (queue.isEmpty) return;

    _log.fine('Chat Rooms updated. New ones are: $queue');

    while (queue.isNotEmpty) {
      final room = queue.removeFirst();
      _updateLocalCachedChatWith(room);
      if (_lastMessageIsFromLocalUser(room)) continue;

      final message = LocalNotification(
        id: room.hashCode,
        title: room.name ?? 'New Message',
        body: room.lastMessagePreview!,
        payload: room.idBase58,
      );
      LocalNotifications.instance.displayNotification(message);
      await Future.delayed(const Duration(milliseconds: 500));
    }

    _updatedPersistentCachedChatData();
  }

  bool _localCacheContains(ChatRoom r1) =>
      !_chats.indexWhere((r2) => r2.roomIdBase58 == r1.idBase58).isNegative;

  bool _hasNewMessage(ChatRoom r1) {
    final r2 = _chats.firstWhereOrNull((r2) => r2.roomIdBase58 == r1.idBase58);
    if (r2 == null) return false;
    return r2.lastMessageIndex < (r1.lastMessageIndex ?? 1);
  }

  bool _lastMessageIsFromLocalUser(ChatRoom r1) {
    if (_currentOpenRoom == null || _currentOpenRoom!.messages == null) {
      return false;
    }
    if (!_currentOpenRoom!.conversationId.equals(r1.conversationId)) {
      return false;
    }
    return _currentOpenRoom!.messages!.last.senderId.equals(_user.id);
  }

  void _updateLocalCachedChatWith(ChatRoom r) {
    var newChatData = _ChatData(r.idBase58, r.lastMessageIndex ?? 1);
    final res = _chats.indexWhere((r2) => r2.roomIdBase58 == r.idBase58);
    res.isNegative ? _chats.add(newChatData) : _chats[res] = newChatData;
  }
}

@immutable
class _ChatData {
  const _ChatData(this.roomIdBase58, this.lastMessageIndex);

  _ChatData.fromJson(Map<String, dynamic> json)
      : roomIdBase58 = json['roomIdBase58'],
        lastMessageIndex = json['lastMessageIndex'];

  final String roomIdBase58;
  final int lastMessageIndex;

  Map<String, dynamic> toJson() =>
      {'roomIdBase58': roomIdBase58, 'lastMessageIndex': lastMessageIndex};

  @override
  String toString() => 'ChatData(lastMessageIndex: $lastMessageIndex, roomIdBase58: $roomIdBase58)';
}
