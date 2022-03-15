part of 'providers.dart';

final feedNotificationControllerProvider = Provider((ref) => FeedNotificationController(ref));

class FeedNotificationController {
  FeedNotificationController(Ref ref) : _ref = ref;
  final Ref _ref;
  late final SharedPreferences _prefs;
  late final User _user;

  int _lastIndex = -1;

  final _log = Logger('FeedNotificationController');

  static const _cacheKey = 'feedNotificationControllerLastPostIndexDataKey';

  Future<void> initialize() async {
    _prefs = await SharedPreferences.getInstance();
    if (_prefs.containsKey(_cacheKey)) _lastIndex = _prefs.getInt(_cacheKey)!;

    _user = _ref.read(defaultUserProvider)!;
    _ref.read(qaulWorkerProvider).requestFeedMessages();
    _ref.listen(feedMessagesProvider, _onFeedMessagesChanged);

    _log.config('Initialized:\n\t· Last Post Index: $_lastIndex');
  }

  void _updatePersistentCachedData() => _prefs.setInt(_cacheKey, _lastIndex);

  void _onFeedMessagesChanged(List<FeedPost>? _, List<FeedPost> posts) async {
    final queue = Queue<FeedPost>();
    posts.where((f) => (f.index ?? 1) > _lastIndex).forEach(queue.addLast);

    if (queue.isEmpty) return;

    _log.fine('Feed posts updated. New ones are: $queue');

    while (queue.isNotEmpty) {
      final post = queue.removeFirst();
      _lastIndex = post.index ?? 1;
      if (_lastMessageIsFromLocalUser(post)) continue;

      final message = LocalNotification(
        id: post.hashCode,
        title: 'New Feed Post:',
        body: post.content!,
        payload: 'qaul://feed',
      );
      LocalNotifications.instance.displayNotification(message);
      await Future.delayed(const Duration(milliseconds: 500));
    }

    _updatePersistentCachedData();
  }

  bool _lastMessageIsFromLocalUser(FeedPost post) =>
      post.senderId != null && post.senderId!.equals(_user.id);
}
