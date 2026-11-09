import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  test('uses shared chat surface shadows and strokes', () {
    for (final brightness in Brightness.values) {
      final header = chatSurfaceShadows(
        brightness,
        direction: ChatSurfaceShadowDirection.below,
      );
      final footer = chatSurfaceShadows(
        brightness,
        direction: ChatSurfaceShadowDirection.above,
      );

      final expectedShadow = brightness == Brightness.dark
          ? const BoxShadow(
              offset: Offset(0, 10),
              blurRadius: 7,
              color: Color(0x66000000),
            )
          : const BoxShadow(
              offset: Offset.zero,
              blurRadius: 5,
              color: Color(0x33000000),
            );
      final expectedFooterShadow = brightness == Brightness.dark
          ? const BoxShadow(
              offset: Offset(0, -10),
              blurRadius: 7,
              color: Color(0x66000000),
            )
          : expectedShadow;

      expect(header, [expectedShadow]);
      expect(footer, [expectedFooterShadow]);
      expect(
        chatSurfaceStroke(brightness),
        BorderSide(
          color: brightness == Brightness.dark
              ? const Color(0xFF9E9E9E)
              : const Color(0xFFD1D1D6),
          width: 0.5,
        ),
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

  testWidgets('uses the shared light chat surface stroke', (tester) async {
    final divider = await headerDivider(tester, ThemeData.light());

    expect(divider, chatSurfaceStroke(Brightness.light));
  });

  testWidgets('uses the shared dark chat surface stroke', (tester) async {
    final divider = await headerDivider(tester, ThemeData.dark());

    expect(divider, chatSurfaceStroke(Brightness.dark));
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
