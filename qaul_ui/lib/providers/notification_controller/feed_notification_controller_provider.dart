part of '../providers.dart';

final feedNotificationControllerProvider = Provider((ref) => FeedNotificationController(ref));

class FeedNotificationController extends NotificationController<List<FeedPost>>
    with DataProcessingStrategy<FeedPost> {
  FeedNotificationController(Ref ref) : super(ref);

  int _lastIndex = -1;

  final _log = Logger('FeedNotificationController');

  @override
  String get cacheKey => 'feedNotificationControllerLastPostIndexDataKey';

  @override
  MapEntry<AlwaysAliveProviderListenable<List<FeedPost>>,
          void Function(List<FeedPost>?, List<FeedPost>)>
      get strategy => MapEntry(feedMessagesProvider, execute);

  @override
  Future<void> initialize() async {
    await super.initialize();
    if (preferences.containsKey(cacheKey)) {
      _lastIndex = preferences.getInt(cacheKey)!;
    }
    ref.read(qaulWorkerProvider).requestFeedMessages();
    _log.config('Initialized:\n\t· Last Post Index: $_lastIndex');
  }

  @override
  void updatePersistentCachedData() => preferences.setInt(cacheKey, _lastIndex);

  // ***************************************************************************
  // DataProcessingStrategy<FeedPost> Mixin
  // ***************************************************************************
  @override
  Iterable<FeedPost> entriesToBeProcessed(List<FeedPost> values) {
    final newPosts = values.where((f) => (f.index ?? 1) > _lastIndex);
    _log.fine('Feed posts updated. New ones are: $newPosts');
    return newPosts;
  }

  @override
  LocalNotification? process(FeedPost value) {
    if (currentVisibleHomeTab == TabType.feed) return null;
    if (!UserPrefsHelper().feedNotificationsEnabled) return null;
    _lastIndex = value.index ?? 1;
    if (_lastMessageIsFromLocalUser(value)) return null;
    return LocalNotification(
      id: value.hashCode,
      title: 'New Feed Post:',
      body: value.content!,
      payload: 'qaul://feed',
    );
  }

  bool _lastMessageIsFromLocalUser(FeedPost post) =>
      post.senderId != null && post.senderId!.equals(localUser.id);

  @override
  void close() => updatePersistentCachedData();
}
