import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  Future<BorderSide> headerDivider(WidgetTester tester, ThemeData theme) async {
    await tester.pumpWidget(
      MaterialApp(
        theme: theme,
        home: Scaffold(
          body: ChatHeader(
            applyTopSafeArea: false,
            onBackPressed: () {},
            avatar: const CircleAvatar(child: Text('M')),
            displayName: 'MaxX',
            isOnline: true,
            onlineLabel: 'online',
            lastSeenLabel: '',
          ),
        ),
      ),
    );

    final surface = tester.widget<DecoratedBox>(
      find.byKey(const ValueKey('chat-header-surface')),
    );
    final border = (surface.decoration as BoxDecoration).border! as Border;
    return border.bottom;
  }

  testWidgets('uses the shared light chat divider', (tester) async {
    final divider = await headerDivider(tester, ThemeData.light());

    expect(divider.width, 0.5);
    expect(divider.color, const Color(0xFFD1D1D6));
    expect(
      divider.color,
      const QaulColorSheet(Brightness.light).chatHeaderDivider,
    );
  });

  testWidgets('preserves the dark header divider', (tester) async {
    final divider = await headerDivider(tester, ThemeData.dark());

    expect(divider.width, 0.5);
    expect(
      divider.color,
      const QaulColorSheet(Brightness.dark).chatHeaderDivider,
    );
  });

  testWidgets('can hide back button and center title without subtitle', (
    tester,
  ) async {
    await tester.pumpWidget(
      MaterialApp(
        home: Scaffold(
          body: ChatHeader(
            applyTopSafeArea: false,
            showBackButton: false,
            onBackPressed: () {},
            avatar: const CircleAvatar(child: Text('M')),
            displayName: 'MaxX',
            isOnline: false,
            onlineLabel: '',
            lastSeenLabel: '',
          ),
        ),
      ),
    );

    expect(find.byIcon(Icons.arrow_back_rounded), findsNothing);
    expect(
      tester.getCenter(find.text('MaxX')).dy,
      closeTo(tester.getCenter(find.byType(CircleAvatar)).dy, 1),
    );
  });
}
