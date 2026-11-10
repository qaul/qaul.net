import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  test('uses symmetrical shared chat surface shadows', () {
    for (final brightness in Brightness.values) {
      final header = chatSurfaceShadows(
        brightness,
        direction: ChatSurfaceShadowDirection.below,
      );
      final footer = chatSurfaceShadows(
        brightness,
        direction: ChatSurfaceShadowDirection.above,
      );

      expect(header, hasLength(2));
      expect(header.first, footer.first);
      expect(
        header.first,
        BoxShadow(
          offset: Offset.zero,
          blurRadius: 5,
          color: brightness == Brightness.dark
              ? const Color(0x66000000)
              : const Color(0x33000000),
        ),
      );
      expect(header.last.offset, const Offset(0, 10));
      expect(footer.last.offset, const Offset(0, -10));
      expect(header.last.blurRadius, 7);
      expect(
        header.last.color,
        brightness == Brightness.dark
            ? const Color(0x99000000)
            : const Color(0x66000000),
      );
    }
  });

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
