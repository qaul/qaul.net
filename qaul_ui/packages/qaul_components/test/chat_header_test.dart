import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  Future<void> pumpHeader(
    WidgetTester tester, {
    required bool showBackButton,
    bool includeMenu = false,
    ThemeData? theme,
  }) {
    return tester.pumpWidget(
      MaterialApp(
        theme: theme ?? ThemeData.dark(),
        home: Scaffold(
          body: SizedBox(
            width: 320,
            child: ChatHeader(
              applyTopSafeArea: false,
              showBackButton: showBackButton,
              onBackPressed: () {},
              avatar: const CircleAvatar(child: Text('M')),
              displayName: 'MaxX',
              isOnline: true,
              onlineLabel: 'online',
              lastSeenLabel: '',
              menuEntries: includeMenu
                  ? const [ChatHeaderMenuEntry(id: 'info', label: 'Info')]
                  : const [],
              onMenuSelected: includeMenu ? (_) {} : null,
            ),
          ),
        ),
      ),
    );
  }

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

  testWidgets('matches the phone header layout measurements', (tester) async {
    for (final theme in [ThemeData.dark(), ThemeData.light()]) {
      await pumpHeader(
        tester,
        showBackButton: true,
        includeMenu: true,
        theme: theme,
      );

      final surface = find.byKey(const ValueKey('chat-header-surface'));
      final avatar = find.byKey(const ValueKey('chat-header-avatar'));
      final backButton = find.byKey(const ValueKey('chat-header-back-button'));
      final backIcon = find.byIcon(Icons.arrow_back_rounded);
      final overflowIcon = find.byIcon(Icons.more_vert);
      final surfaceRect = tester.getRect(surface);
      final avatarRect = tester.getRect(avatar);
      final backButtonRect = tester.getRect(backButton);
      final backIconRect = tester.getRect(backIcon);
      final overflowIconRect = tester.getRect(overflowIcon);

      expect(backIconRect.left, closeTo(20, 1));
      expect(backButtonRect.width, greaterThanOrEqualTo(40));
      expect(backButtonRect.height, greaterThanOrEqualTo(40));
      expect(avatarRect.left, closeTo(50, 1));
      expect(tester.getRect(find.text('MaxX')).left, closeTo(99, 1));
      expect(surfaceRect.bottom - avatarRect.bottom, closeTo(9.3, 1));
      expect(surfaceRect.right - overflowIconRect.right, closeTo(18, 1));
      expect(tester.widget<Text>(find.text('MaxX')).style!.letterSpacing, 0.5);
      expect(
        tester.widget<Text>(find.text('online')).style!.letterSpacing,
        0.5,
      );
    }
  });

  testWidgets('matches the dark phone header golden', (tester) async {
    tester.view.physicalSize = const Size(320, 100);
    tester.view.devicePixelRatio = 1;
    addTearDown(tester.view.resetPhysicalSize);
    addTearDown(tester.view.resetDevicePixelRatio);

    await pumpHeader(tester, showBackButton: true, includeMenu: true);

    await expectLater(
      find.byKey(const ValueKey('chat-header-surface')),
      matchesGoldenFile('goldens/chat_header_phone_dark.png'),
    );
  });

  testWidgets('uses a 20px avatar inset without the back button', (
    tester,
  ) async {
    await pumpHeader(tester, showBackButton: false);

    expect(
      tester.getRect(find.byKey(const ValueKey('chat-header-avatar'))).left,
      closeTo(20, 1),
    );
  });
}
