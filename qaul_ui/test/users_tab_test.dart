import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/screens/home/tabs/tab.dart';
import 'package:shared_preferences/shared_preferences.dart';

import 'chat_tab/chat_tab_test.dart';
import 'test_utils/test_utils.dart';

void main() {
  late Key usersKey;

  setUp(() {
    usersKey = UniqueKey();
    SharedPreferences.setMockInitialValues({});
  });

  testWidgets('Users tab loads first page and shows mock users', (tester) async {
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
    );
    addTearDown(container.dispose);

    final widget = UncontrolledProviderScope(
      container: container,
      child: materialAppWithLocalizations(BaseTab.users(key: usersKey)),
    );

    await tester.pumpWidget(widget);
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 150));
    await tester.pumpAndSettle();

    final users = container.read(usersProvider);
    expect(users.length, 50);
    expect(users.first.name, 'Mock User 1');
    expect(find.byType(ListView), findsOneWidget);
  });

  testWidgets('Users tab loads more users when scrolling near bottom',
      (tester) async {
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
    );
    addTearDown(container.dispose);

    final widget = UncontrolledProviderScope(
      container: container,
      child: materialAppWithLocalizations(BaseTab.users(key: usersKey)),
    );

    await tester.pumpWidget(widget);
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 150));
    await tester.pumpAndSettle();

    expect(container.read(usersProvider).length, 50);
    expect(container.read(usersProvider).first.name, 'Mock User 1');

    for (int i = 0; i < 6; i++) {
      await tester.drag(find.byType(ListView), const Offset(0, -500));
      await tester.pump(const Duration(milliseconds: 50));
    }
    await tester.pump(const Duration(milliseconds: 200));
    await tester.pumpAndSettle();

    final usersAfterScroll = container.read(usersProvider);
    expect(usersAfterScroll.length, greaterThan(50));
    expect(
      usersAfterScroll.any((user) => user.name == 'Mock User 51'),
      isTrue,
    );
  });

  testWidgets('Users tab shows pagination state after first load', (tester) async {
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => defaultUser),
        qaulWorkerProvider.overrideWith((ref) => StubLibqaulWorker(ref)),
      ],
    );
    addTearDown(container.dispose);

    final widget = UncontrolledProviderScope(
      container: container,
      child: materialAppWithLocalizations(BaseTab.users(key: usersKey)),
    );

    await tester.pumpWidget(widget);
    await tester.pump();
    await tester.pump(const Duration(milliseconds: 150));
    await tester.pumpAndSettle();

    final pagination = container.read(usersPaginationStateProvider);
    expect(pagination, isNotNull);
    expect(pagination!.hasMore, isTrue);
    expect(pagination.total, 125);
    expect(pagination.offset, 0);
    expect(pagination.limit, 50);

    final users = container.read(usersProvider);
    expect(users.length, 50);
  });
}
