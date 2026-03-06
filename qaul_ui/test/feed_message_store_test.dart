import 'dart:typed_data';

import 'package:fast_base58/fast_base58.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intl/date_symbol_data_local.dart';
import 'package:intl/intl.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'chat_tab/chat_tab_test.dart';

List<PublicPost> _testPublicMessages = [];
List<User> _testUsersForStore = [];
PaginationState? _testPagination;
Map<String, User?> _getUserByIdByBase58 = {};

class _TestPublicPostListNotifier extends PublicPostListNotifier {
  @override
  List<PublicPost> build() => _testPublicMessages;
}

class _TestPaginatedUsersNotifier extends PaginatedDataNotifier<User> {
  @override
  PaginatedData<User> build() =>
      PaginatedData(data: _testUsersForStore, pagination: _testPagination);
}

class _MockWorkerForGetByUserID extends StubLibqaulWorker {
  _MockWorkerForGetByUserID(super.ref);
  @override
  Future<User?> getUserById(Uint8List userId) =>
      Future.value(_getUserByIdByBase58[Base58Encode(userId)]);
}

ProviderContainer _container() => ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        publicMessagesProvider.overrideWith(() => _TestPublicPostListNotifier()),
        usersProvider.overrideWith(() => _TestPaginatedUsersNotifier()),
        qaulWorkerProvider.overrideWith((ref) => _MockWorkerForGetByUserID(ref)),
      ],
    );

Future<List<FeedMessage>> _readFeedAfterInit(ProviderContainer container) async {
  container.read(feedMessageStoreProvider);
  await Future.delayed(const Duration(milliseconds: 50));
  return container.read(feedMessageStoreProvider);
}

PublicPost _post({
  required String senderIdBase58,
  required String content,
  int index = 1,
}) =>
    PublicPost(
      senderId: Uint8List.fromList(senderIdBase58.codeUnits),
      index: index,
      senderIdBase58: senderIdBase58,
      messageId: Uint8List.fromList([index]),
      messageIdBase58: 'msg_$index',
      content: content,
      sendTime: DateTime(2025, 1, 1),
      receiveTime: DateTime(2025, 1, 1),
    );

User _user(String name, String id) => User(
      name: name,
      id: Uint8List.fromList(id.codeUnits),
    );

void main() {
  setUpAll(() async {
    await initializeDateFormatting('en');
  });

  setUp(() {
    Intl.defaultLocale = 'en';
    _testPublicMessages = [];
    _testUsersForStore = [];
    _testPagination = null;
    _getUserByIdByBase58 = {};
  });

  group('FeedMessage', () {
    test('forwards index and content for feed and refresh', () {
      final msg = PublicPost(
        senderId: Uint8List.fromList([1]),
        index: 42,
        senderIdBase58: 's58',
        messageId: Uint8List.fromList([2]),
        messageIdBase58: 'm58',
        content: 'Hi',
        sendTime: DateTime(2025, 1, 1),
        receiveTime: DateTime(2025, 1, 1),
      );
      final author = _user('A', 'id_a');
      final fm = FeedMessage(msg, author, '1 min ago');
      expect(fm.index, 42);
      expect(fm.content, 'Hi');
      expect(fm.author.idBase58, author.idBase58);
    });
  });

  group('FeedMessageStore', () {
    test('empty messages → empty feed', () async {
      _testPublicMessages = [];
      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);
      expect(state, isEmpty);
    });

    test('skips message when senderIdBase58 is null', () async {
      _testPublicMessages = [
        PublicPost(
          senderId: null,
          index: 1,
          senderIdBase58: null,
          messageId: Uint8List.fromList([1]),
          messageIdBase58: 'm1',
          content: 'No sender',
          sendTime: DateTime(2025, 1, 1),
          receiveTime: DateTime(2025, 1, 1),
        ),
      ];
      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);
      expect(state, isEmpty);
    });

    test('excludes message when getByUserID returns null', () async {
      _testPublicMessages = [_post(senderIdBase58: 'unknown', content: 'X')];
      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);
      expect(state, isEmpty);
    });

    test('all unknown senders → empty feed', () async {
      _testPublicMessages = [
        _post(senderIdBase58: 'u1', content: 'A'),
        _post(senderIdBase58: 'u2', content: 'B'),
      ];
      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);
      expect(state, isEmpty);
    });

    test('resolves author from store or getByUserID and excludes unknown', () async {
      final u1 = _user('U1', 'id1');
      final u2 = _user('U2', 'id2');
      _testPublicMessages = [
        _post(senderIdBase58: u1.idBase58, content: 'M1', index: 1),
        _post(senderIdBase58: u2.idBase58, content: 'M2', index: 2),
        _post(senderIdBase58: 'unknown', content: 'M3', index: 3),
      ];
      _testUsersForStore = [u1];
      _getUserByIdByBase58 = {u2.idBase58: u2};

      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);

      expect(state.length, 2);
      expect(state.map((e) => e.author.idBase58), containsAll([u1.idBase58, u2.idBase58]));
      expect(state.any((e) => e.content == 'M3'), isFalse);
    });

    test('single message with author from getByUserID', () async {
      final u = _user('Single', 'single_id');
      _testPublicMessages = [_post(senderIdBase58: u.idBase58, content: 'Only')];
      _getUserByIdByBase58 = {u.idBase58: u};

      final container = _container();
      addTearDown(container.dispose);
      final state = await _readFeedAfterInit(container);

      expect(state.length, 1);
      expect(state.single.author.idBase58, u.idBase58);
      expect(state.single.content, 'Only');
    });
  });
}
