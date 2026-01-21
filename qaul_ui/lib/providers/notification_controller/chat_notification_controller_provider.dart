part of '../providers.dart';

final chatNotificationControllerProvider =
    Provider((ref) => ChatNotificationController(ref));

class ChatNotificationController extends NotificationController<List<ChatRoom>>
    with DataProcessingStrategy<ChatRoom> {
  // ignore: use_super_parameters
  ChatNotificationController(Ref ref) : super(ref);

  final _chats = <_ChatData>[];

  final _log = Logger('ChatNotificationController');

  @override
  String get cacheKey => 'chatNotificationControllerChatDataKey';

  @override
  MapEntry<dynamic, void Function(List<ChatRoom>?, List<ChatRoom>)>
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
    _log.config(
        'Initialized:\n\t· User: ${localUser.name}\n\t· Cached chats: $_chats');
  }

  @override
  void updatePersistentCachedData() =>
      preferences.setStringList(cacheKey, _chats.map(jsonEncode).toList());

  // ***************************************************************************
  // DataProcessingStrategy<ChatRoom> Mixin
  // ***************************************************************************
  @override
  Iterable<ChatRoom> entriesToBeProcessed(List<ChatRoom> values) {
    var newMessages =
        List<ChatRoom>.from(values.where((room) => !_localCacheContains(room)))
          ..addAll(values.where(_localCacheContains).where(_hasNewMessage));
    if (UserPrefsHelper.instance.notifyOnlyForVerifiedUsers) {
      final verifiedIds = ref
          .read(usersProvider)
          .where((u) => u.isVerified ?? false)
          .map((e) => e.id);
      newMessages = newMessages
          .where((room) => verifiedIds
              .where((id) => id.equals(room.conversationId))
              .isNotEmpty)
          .toList();
    }
    _log.fine('Chat Rooms updated. New ones are: $newMessages');
    return newMessages;
  }

  bool _localCacheContains(ChatRoom r1) =>
      !_chats.indexWhere((r2) => r2.roomIdBase58 == r1.idBase58).isNegative;

  bool _hasNewMessage(ChatRoom r1) {
    final r2 = _chats.firstWhereOrNull((r2) => r2.roomIdBase58 == r1.idBase58);
    if (r2 == null) return false;
    return r2.unreadCount < r1.unreadCount;
  }

  @override
  LocalNotification? process(ChatRoom value) {
    _updateLocalCachedChatWith(value);
    if (currentVisibleHomeTab == TabType.chat) return null;
    if (!UserPrefsHelper.instance.chatNotificationsEnabled ||
        _lastMessageIsFromLocalUser(value)) {
      return null;
    }
    if (value.lastMessagePreview is! TextMessageContent) {
      _log.info(
          'message of type ${value.lastMessagePreview.runtimeType} received, but no notification was sent');
      return null;
    }

    return LocalNotification(
      id: value.hashCode,
      title: value.name == null || value.name!.isEmpty ? 'New Message' : value.name!,
      body: (value.lastMessagePreview as TextMessageContent).content,
      payload: 'qaul://chat/${value.idBase58}',
    );
  }

  bool _lastMessageIsFromLocalUser(ChatRoom r1) =>
      localUser.id.equals(r1.lastMessageSenderId ?? []);

  void _updateLocalCachedChatWith(ChatRoom r) {
    var newChatData = _ChatData(r.idBase58, r.unreadCount);
    _log.fine('updating cached chat (${r.name}) data: $newChatData');
    final res = _chats.indexWhere((r2) => r2.roomIdBase58 == r.idBase58);
    res.isNegative ? _chats.add(newChatData) : _chats[res] = newChatData;
  }

  @override
  void close() => updatePersistentCachedData();
}

@immutable
class _ChatData {
  const _ChatData(this.roomIdBase58, this.unreadCount);

  _ChatData.fromJson(Map<String, dynamic> json)
      : roomIdBase58 = json['roomIdBase58'],
        unreadCount = json['unreadCount'];

  final String roomIdBase58;
  final int unreadCount;

  Map<String, dynamic> toJson() =>
      {'roomIdBase58': roomIdBase58, 'unreadCount': unreadCount};

  @override
  String toString() =>
      'ChatData(unreadCount: $unreadCount, roomIdBase58: $roomIdBase58)';
}
