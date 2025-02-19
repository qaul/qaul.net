part of '../providers.dart';

final publicNotificationControllerProvider =
    Provider((ref) => PublicNotificationController(ref));

class PublicNotificationController
    extends NotificationController<List<PublicPost>>
    with DataProcessingStrategy<PublicPost> {
  // ignore: use_super_parameters
  PublicNotificationController(Ref ref) : super(ref);

  int _lastIndex = -1;

  final _log = Logger('PublicNotificationController');

  @override
  String get cacheKey => 'publicNotificationControllerLastPostIndexDataKey';

  @override
  MapEntry<StateNotifierProvider<PublicPostListNotifier, List<PublicPost>>,
          void Function(List<PublicPost>?, List<PublicPost>)>
      get strategy => MapEntry(publicMessagesProvider, execute);

  @override
  Future<void> initialize() async {
    await super.initialize();
    if (preferences.containsKey(cacheKey)) {
      _lastIndex = preferences.getInt(cacheKey)!;
    }
    ref.read(qaulWorkerProvider).requestPublicMessages();
    _log.config('Initialized:\n\t· Last Post Index: $_lastIndex');
  }

  @override
  void updatePersistentCachedData() => preferences.setInt(cacheKey, _lastIndex);

  // ***************************************************************************
  // DataProcessingStrategy<PublicPost> Mixin
  // ***************************************************************************
  @override
  Iterable<PublicPost> entriesToBeProcessed(List<PublicPost> values) {
    var newPosts = values.where((f) => (f.index ?? 1) > _lastIndex).toList();
    if (UserPrefsHelper().notifyOnlyForVerifiedUsers) {
      final verifiedIds = ref
          .read(usersProvider)
          .where((u) => u.isVerified ?? false)
          .map((e) => e.id);
      newPosts = newPosts
          .where((post) =>
              post.senderId != null &&
              verifiedIds.where((id) => id.equals(post.senderId!)).isNotEmpty)
          .toList();
    }
    if (newPosts.isEmpty) return [];
    _log.fine('Public posts updated. New ones are: $newPosts');
    _updateCachedIndex([...newPosts]);
    return newPosts;
  }

  void _updateCachedIndex(List<PublicPost> newPosts) {
    var maxIndex = newPosts.map((e) => e.index ?? 0).reduce(max);
    if (maxIndex > _lastIndex) {
      _log.finer('updating last public post index to $maxIndex');
      _lastIndex = maxIndex;
      updatePersistentCachedData();
    }
  }

  @override
  LocalNotification? process(PublicPost value) {
    if (currentVisibleHomeTab == TabType.public) {
      _log.finer('currently in Public tab, filtering notifications');
      return null;
    }
    if (!UserPrefsHelper().publicTabNotificationsEnabled) {
      _log.finer('public notifications disabled, filtering notifications');
      return null;
    }
    if (_lastMessageIsFromLocalUser(value)) {
      _log.finer('message $value is from local user, filtering notification');
      return null;
    }
    return LocalNotification(
      id: value.hashCode,
      title: 'New Public Post:',
      body: value.content!,
      payload: 'qaul://public',
    );
  }

  bool _lastMessageIsFromLocalUser(PublicPost post) =>
      post.senderId != null && post.senderId!.equals(localUser.id);

  @override
  void close() => updatePersistentCachedData();
}
