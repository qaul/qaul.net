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
    var newPosts = values.where((f) => (f.index ?? 1) > _lastIndex).toList();
    if (UserPrefsHelper().notifyOnlyForVerifiedUsers) {
      final verifiedIds =
          ref.read(usersProvider).where((u) => u.isVerified ?? false).map((e) => e.id);
      newPosts = newPosts.where((post) =>
          post.senderId != null && verifiedIds.where((id) => id.equals(post.senderId!)).isNotEmpty).toList();
    }
    if (newPosts.isEmpty) return [];
    _log.fine('Feed posts updated. New ones are: $newPosts');
    _updateCachedIndex([...newPosts]);
    return newPosts;
  }

  void _updateCachedIndex(List<FeedPost> newPosts) {
    var maxIndex = newPosts.map((e) => e.index ?? 0).reduce(max);
    if (maxIndex > _lastIndex) {
      _log.finer('updating last feed post index to $maxIndex');
      _lastIndex = maxIndex;
      updatePersistentCachedData();
    }
  }

  @override
  LocalNotification? process(FeedPost value) {
    if (currentVisibleHomeTab == TabType.feed) {
      _log.finer('currently in Feed tab, filtering notifications');
      return null;
    }
    if (!UserPrefsHelper().feedNotificationsEnabled) {
      _log.finer('feed notifications disabled, filtering notifications');
      return null;
    }
    if (_lastMessageIsFromLocalUser(value)) {
      _log.finer('message $value is from local user, filtering notification');
      return null;
    }
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
