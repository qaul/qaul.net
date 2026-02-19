import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/widgets/widgets.dart';

import '../test_utils/test_utils.dart';

void main() {
  TestWidgetsFlutterBinding.ensureInitialized();

  final testUser = User(
    name: 'Test User',
    id: Uint8List.fromList('testUserId'.codeUnits),
  );

  Widget wrapWithProviders(Widget child) {
    return ProviderScope(
      overrides: [
        defaultUserProvider.overrideWith((_) => testUser),
      ],
      child: materialAppWithLocalizations(child),
    );
  }

  group('QaulAvatar', () {
    testWidgets('shows user initials when user is provided', (tester) async {
      await tester.pumpWidget(
        wrapWithProviders(QaulAvatar.tiny(user: testUser)),
      );

      expect(find.text('TU'), findsOneWidget);
      expect(find.byType(CircleAvatar), findsOneWidget);
    });

    testWidgets('small and large variants show same initials', (tester) async {
      await tester.pumpWidget(
        wrapWithProviders(QaulAvatar.small(user: testUser)),
      );
      expect(find.text('TU'), findsOneWidget);

      await tester.pumpWidget(
        wrapWithProviders(QaulAvatar.large(user: testUser)),
      );
      expect(find.text('TU'), findsOneWidget);
    });

    testWidgets('shows default user initials when user is null', (tester) async {
      await tester.pumpWidget(wrapWithProviders(QaulAvatar.small()));

      expect(find.text('TU'), findsOneWidget);
    });

    testWidgets('small without badge still shows initials', (tester) async {
      await tester.pumpWidget(
        wrapWithProviders(
          QaulAvatar.small(user: testUser, badgeEnabled: false),
        ),
      );

      expect(find.text('TU'), findsOneWidget);
      expect(find.byType(CircleAvatar), findsOneWidget);
    });

    testWidgets('groupSmall builds without throwing', (tester) async {
      await tester.pumpWidget(wrapWithProviders(QaulAvatar.groupSmall()));

      expect(tester.takeException(), isNull);
    });

    testWidgets('groupLarge builds without throwing', (tester) async {
      await tester.pumpWidget(wrapWithProviders(QaulAvatar.groupLarge()));

      expect(tester.takeException(), isNull);
    });
  });
}
