import 'dart:async';
import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:qaul_ui/screens/home/user_details_screen.dart';
import 'package:qaul_ui/stores/stores.dart';

import 'test_utils/test_utils.dart';

class _PublicLoadingController extends PublicNotificationController {
  _PublicLoadingController(super.ref, this.firstPage);

  final Completer<PaginatedPosts?> firstPage;

  @override
  Future<PaginatedPosts?> initializeWithFirstPage() => firstPage.future;
}

List<FeedMessage> _feedMessages = [];

class _StaticFeedMessageStore extends FeedMessageStore {
  @override
  List<FeedMessage> build() => _feedMessages;

  @override
  Future<void> refreshPublic() async {}

  @override
  Future<PaginatedPosts?> loadMore(int offset, {int limit = 50}) async {
    return _emptyFirstPage();
  }
}

PaginatedPosts _emptyFirstPage() => PaginatedPosts(
      posts: [],
      pagination: PaginationState(
        hasMore: false,
        total: 0,
        offset: 0,
        limit: 50,
      ),
    );

User _fullUser(String name, List<int> q8id) => User(
      name: name,
      id: Uint8List.fromList([0, 1, 2, 3, 4, 5, ...q8id, 18, 19]),
    );

FeedMessage _feedMessage(User author, String content) => FeedMessage(
      PublicPost(
        senderId: author.id,
        senderIdBase58: author.idBase58,
        messageId: Uint8List.fromList(content.codeUnits),
        messageIdBase58: content,
        content: content,
        sendTime: DateTime(2026, 1, 1),
        receiveTime: DateTime(2026, 1, 1),
      ),
      author,
      'now',
    );

void main() {
  testWidgets('shows loading instead of empty public text during first load',
      (tester) async {
    final firstPage = Completer<PaginatedPosts?>();

    await tester.pumpWidget(
      ProviderScope(
        overrides: [
          publicNotificationControllerProvider.overrideWith(
            (ref) => _PublicLoadingController(ref, firstPage),
          ),
        ],
        child: materialAppWithLocalizations(
          BaseTab.public(disablePageViewScroll: ValueNotifier(false)),
        ),
      ),
    );
    await tester.pump();

    expect(find.byType(QaulLoadingIndicator), findsOneWidget);
    expect(find.text('No public messages yet'), findsNothing);

    firstPage.complete(_emptyFirstPage());
    await tester.pump();
    await tester.pump();

    expect(find.byType(QaulLoadingIndicator), findsNothing);
    expect(find.text('No public messages yet'), findsOneWidget);
  });

  testWidgets('tapping own feed author opens the account tab', (tester) async {
    final self = _fullUser('Self Author', [10, 11, 12, 13, 14, 15, 16, 17]);
    final selfFromFeed = User(
      name: self.name,
      id: Uint8List.fromList(self.id.sublist(6, 14)),
    );
    _feedMessages = [_feedMessage(selfFromFeed, 'my post')];

    final firstPage = Completer<PaginatedPosts?>()..complete(_emptyFirstPage());
    final container = ProviderContainer.test(
      overrides: [
        defaultUserProvider.overrideWith((_) => self),
        feedMessageStoreProvider.overrideWith(_StaticFeedMessageStore.new),
        publicNotificationControllerProvider.overrideWith(
          (ref) => _PublicLoadingController(ref, firstPage),
        ),
      ],
    );
    addTearDown(container.dispose);
    addTearDown(() => _feedMessages = []);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: materialAppWithLocalizations(
          BaseTab.public(disablePageViewScroll: ValueNotifier(false)),
        ),
      ),
    );
    await tester.pump();
    await tester.pump();

    expect(container.read(homeScreenControllerProvider), TabType.public);

    await tester.tap(find.text('Self Author'));
    await tester.pumpAndSettle();

    expect(container.read(homeScreenControllerProvider), TabType.account);
    expect(find.byType(UserDetailsScreen), findsNothing);
  });

  testWidgets('tapping another feed author opens user details', (tester) async {
    final self = _fullUser('Self Author', [10, 11, 12, 13, 14, 15, 16, 17]);
    final other = _fullUser('Other Author', [20, 21, 22, 23, 24, 25, 26, 27]);
    _feedMessages = [_feedMessage(other, 'their post')];

    final firstPage = Completer<PaginatedPosts?>()..complete(_emptyFirstPage());
    final container = ProviderContainer.test(
      overrides: [
        defaultUserProvider.overrideWith((_) => self),
        feedMessageStoreProvider.overrideWith(_StaticFeedMessageStore.new),
        publicNotificationControllerProvider.overrideWith(
          (ref) => _PublicLoadingController(ref, firstPage),
        ),
      ],
    );
    addTearDown(container.dispose);
    addTearDown(() => _feedMessages = []);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: materialAppWithLocalizations(
          BaseTab.public(disablePageViewScroll: ValueNotifier(false)),
        ),
      ),
    );
    await tester.pump();
    await tester.pump();

    await tester.tap(find.text('Other Author'));
    await tester.pumpAndSettle();

    expect(container.read(homeScreenControllerProvider), TabType.public);
    expect(find.byType(UserDetailsScreen), findsOneWidget);
  });
}
