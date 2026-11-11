import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/decorators/search_user_decorator.dart';
import 'package:qaul_ui/stores/stores.dart';

import '../test_utils/test_utils.dart';

class _TestUsersStore extends UsersStore {
  @override
  List<User> build() => const [];
}

void main() {
  final currentUser = User(
    name: 'Current User',
    id: Uint8List.fromList('current-user'.codeUnits),
  );

  testWidgets('clears the search after the decorator is disposed', (
    tester,
  ) async {
    final container = ProviderContainer(
      overrides: [
        defaultUserProvider.overrideWith((_) => currentUser),
        usersStoreProvider.overrideWith(_TestUsersStore.new),
      ],
    );
    addTearDown(container.dispose);

    await tester.pumpWidget(
      UncontrolledProviderScope(
        container: container,
        child: materialAppWithLocalizations(
          SearchUserDecorator(builder: (context, users) => const SizedBox()),
        ),
      ),
    );
    await tester.pump();

    container.read(usersSearchProvider.notifier).setQuery('alice');
    expect(container.read(usersSearchProvider).isActive, isTrue);

    await tester.pumpWidget(const SizedBox());
    await tester.pump();

    expect(tester.takeException(), isNull);
    expect(container.read(usersSearchProvider).isActive, isFalse);
  });
}
