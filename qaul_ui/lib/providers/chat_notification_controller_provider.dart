part of 'providers.dart';

final chatNotificationControllerProvider = Provider((ref) => ChatNotificationController(ref));

class ChatNotificationController extends NotificationController<List<ChatRoom>>
    with DataProcessingStrategy<ChatRoom> {
  ChatNotificationController(Ref ref) : super(ref);

  ChatRoom? _currentOpenRoom;
  final _chats = <_ChatData>[];

  final _log = Logger('ChatNotificationController');

  @override
  String get cacheKey => 'chatNotificationControllerChatDataKey';

  @override
  MapEntry<AlwaysAliveProviderListenable<List<ChatRoom>>,
          void Function(List<ChatRoom>?, List<ChatRoom>)>
      get strategy => MapEntry(chatRoomsProvider, execute);

  @override
  Future<void> initialize() async {
    await super.initialize();
    if (preferences.containsKey(cacheKey)) {
      _chats.addAll(preferences.getStringList(cacheKey)!.map((e) {
        return _ChatData.fromJson(jsonDecode(e));
      }));
    }
    ref.read(qaulWorkerProvider).getAllChatRooms();
    ref.listen(currentOpenChatRoom.state, _updateCurrentOpenRoom);
    _log.config('Initialized:\n\t· User: ${localUser.name}\n\t· Cached chats: $_chats');
  }

  void _updateCurrentOpenRoom(_, StateController<ChatRoom?> notifier) {
    _currentOpenRoom = notifier.state;
  }

  @override
  void updatePersistentCachedData() =>
      preferences.setStringList(cacheKey, _chats.map(jsonEncode).toList());

  // ***************************************************************************
  // DataProcessingStrategy<ChatRoom> Mixin
  // ***************************************************************************
  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) {
    final newMessages = List<ChatRoom>.from(values.where((room) => !_localCacheContains(room)))
      ..addAll(values.where(_localCacheContains).where(_hasNewMessage));
    _log.fine('Chat Rooms updated. New ones are: $newMessages');
    return newMessages;
  }

  bool _localCacheContains(ChatRoom r1) =>
      !_chats.indexWhere((r2) => r2.roomIdBase58 == r1.idBase58).isNegative;

  bool _hasNewMessage(ChatRoom r1) {
    final r2 = _chats.firstWhereOrNull((r2) => r2.roomIdBase58 == r1.idBase58);
    if (r2 == null) return false;
    return r2.lastMessageIndex < (r1.lastMessageIndex ?? 1);
  }

  @override
  LocalNotification? process(ChatRoom value) {
    if (!UserPrefsHelper().chatNotificationsEnabled) return null;
    _updateLocalCachedChatWith(value);
    if (_lastMessageIsFromLocalUser(value)) return null;
    return LocalNotification(
      id: value.hashCode,
      title: value.name ?? 'New Message',
      body: value.lastMessagePreview!,
      payload: value.idBase58,
    );
  }

  bool _lastMessageIsFromLocalUser(ChatRoom r1) {
    if (_currentOpenRoom == null || _currentOpenRoom!.messages == null) {
      return false;
    }
    if (!_currentOpenRoom!.conversationId.equals(r1.conversationId)) {
      return false;
    }
    return _currentOpenRoom!.messages!.last.senderId.equals(localUser.id);
  }

  void _updateLocalCachedChatWith(ChatRoom r) {
    var newChatData = _ChatData(r.idBase58, r.lastMessageIndex ?? 1);
    final res = _chats.indexWhere((r2) => r2.roomIdBase58 == r.idBase58);
    res.isNegative ? _chats.add(newChatData) : _chats[res] = newChatData;
  }

  @override
  void close() => updatePersistentCachedData();
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
