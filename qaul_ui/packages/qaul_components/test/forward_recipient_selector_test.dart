import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  const recipients = [
    ForwardRecipient(
      id: 'public',
      displayName: 'Public',
      kind: ForwardRecipientKind.public,
    ),
    ForwardRecipient(
      id: 'ada',
      displayName: 'Ada',
      kind: ForwardRecipientKind.user,
    ),
    ForwardRecipient(
      id: 'group',
      displayName: 'Qaul group',
      kind: ForwardRecipientKind.group,
    ),
  ];

  testWidgets('renders sections and updates single selection', (tester) async {
    ForwardRecipient? selected;
    await tester.pumpWidget(
      MaterialApp(
        home: ForwardRecipientSelector(
          recipients: recipients,
          onRecipientSelected: (value) => selected = value,
          onSearchChanged: (_) {},
          onCancel: () {},
        ),
      ),
    );

    expect(find.text('Public'), findsNWidgets(2));
    expect(find.text('Users / Contacts'), findsOneWidget);
    expect(find.text('Groups'), findsOneWidget);

    await tester.tap(find.text('Ada'));
    await tester.pump();

    expect(selected?.id, 'ada');
    expect(find.byIcon(Icons.check_circle_outline), findsOneWidget);
  });

  testWidgets('opens search and reports query and close', (tester) async {
    final changes = <String>[];
    await tester.pumpWidget(
      MaterialApp(
        home: ForwardRecipientSelector(
          recipients: recipients,
          onRecipientSelected: (_) {},
          onSearchChanged: changes.add,
          onCancel: () {},
        ),
      ),
    );

    await tester.tap(find.byTooltip('Search Users / Contacts'));
    await tester.pump();
    expect(find.text('Search recipients'), findsOneWidget);

    await tester.enterText(find.byType(TextField), 'Ada');
    expect(changes, contains('Ada'));

    await tester.tap(find.byTooltip('Close search'));
    await tester.pump();
    expect(changes.last, '');
    expect(find.text('Search recipients'), findsNothing);
  });

  testWidgets('reflects updated initial state from its parent', (tester) async {
    late StateSetter rebuild;
    var selectedId = 'ada';
    var searchOpen = false;

    await tester.pumpWidget(
      MaterialApp(
        home: StatefulBuilder(
          builder: (context, setState) {
            rebuild = setState;
            return ForwardRecipientSelector(
              recipients: recipients,
              initialSelectedRecipientId: selectedId,
              initialSearchOpen: searchOpen,
              onRecipientSelected: (_) {},
              onSearchChanged: (_) {},
              onCancel: () {},
            );
          },
        ),
      ),
    );

    expect(find.byIcon(Icons.check_circle_outline), findsOneWidget);
    rebuild(() {
      selectedId = 'group';
      searchOpen = true;
    });
    await tester.pump();

    expect(find.text('Search recipients'), findsOneWidget);
    await tester.tap(find.byTooltip('Close search'));
    await tester.pump();
    expect(find.text('Search recipients'), findsNothing);
  });

  testWidgets('uses a safe avatar fallback for an empty user name', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: ForwardRecipientSelector(
          recipients: const [
            ForwardRecipient(
              id: 'empty',
              displayName: '',
              kind: ForwardRecipientKind.user,
            ),
          ],
          onRecipientSelected: (_) {},
          onSearchChanged: (_) {},
          onCancel: () {},
        ),
      ),
    );

    expect(find.text('?'), findsOneWidget);
    expect(tester.takeException(), isNull);
  });

  testWidgets('positions the online indicator over the avatar edge', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: ForwardRecipientSelector(
          recipients: const [
            ForwardRecipient(
              id: 'online',
              displayName: 'Online user',
              initials: 'OU',
              kind: ForwardRecipientKind.user,
              isOnline: true,
            ),
          ],
          onRecipientSelected: (_) {},
          onSearchChanged: (_) {},
          onCancel: () {},
        ),
      ),
    );

    final avatarRect = tester.getRect(find.byType(CircleAvatar));
    final indicatorRect = tester.getRect(
      find.byKey(const ValueKey('forward-recipient-online-indicator')),
    );

    expect(indicatorRect.center.dx, greaterThan(avatarRect.center.dx));
    expect(indicatorRect.center.dy, greaterThan(avatarRect.center.dy));
    expect(indicatorRect.left, lessThan(avatarRect.right));
    expect(indicatorRect.top, lessThan(avatarRect.bottom));
  });
}
