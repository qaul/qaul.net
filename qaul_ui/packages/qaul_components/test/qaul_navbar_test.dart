import 'package:badges/badges.dart' as badges;
import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  group('navBarIconPath', () {
    test('returns path for menu icon', () {
      expect(navBarIconPath('menu'), 'assets/icons/menu.svg');
    });

    test('returns path for any name', () {
      expect(navBarIconPath('foo'), 'assets/icons/foo.svg');
    });
  });

  group('navBarTabIconPath', () {
    test('returns outlined path when not selected', () {
      expect(
        navBarTabIconPath(TabType.chat, false),
        'assets/icons/chat-outlined.svg',
      );
    });

    test('returns filled path when selected', () {
      expect(
        navBarTabIconPath(TabType.chat, true),
        'assets/icons/chat-filled.svg',
      );
    });

    test('returns empty string for account tab', () {
      expect(navBarTabIconPath(TabType.account, false), '');
      expect(navBarTabIconPath(TabType.account, true), '');
    });

    test('returns correct paths for all tab types with icons', () {
      expect(
        navBarTabIconPath(TabType.public, false),
        'assets/icons/public-outlined.svg',
      );
      expect(
        navBarTabIconPath(TabType.users, false),
        'assets/icons/people-outlined.svg',
      );
      expect(
        navBarTabIconPath(TabType.network, false),
        'assets/icons/network-outlined.svg',
      );
    });
  });

  group('navBarColors', () {
    test('returns dark theme colors for Brightness.dark', () {
      final theme = ThemeData.dark();
      final (selected, icon, active) = navBarColors(theme);
      expect(selected, const Color(0xFF333333));
      expect(icon, theme.iconTheme.color ?? Colors.white);
      expect(
        active,
        theme.navigationBarTheme.surfaceTintColor ??
            theme.iconTheme.color ??
            Colors.white,
      );
    });

    test('returns light theme colors for Brightness.light', () {
      final theme = ThemeData.light();
      final (selected, icon, active) = navBarColors(theme);
      expect(selected, const Color(0xFFE5E5E5));
      expect(icon, kNavBarIconColorLight);
      expect(active, kNavBarIconColorLight);
    });
  });

  group('navBarTabIconSize', () {
    test('returns correct size for each tab type', () {
      expect(navBarTabIconSize(TabType.chat), const Size(34, 21));
      expect(navBarTabIconSize(TabType.network), const Size(23, 23));
      expect(navBarTabIconSize(TabType.users), const Size(30, 18.34));
      expect(navBarTabIconSize(TabType.public), const Size(31, 26));
    });

    test('throws for account tab', () {
      expect(() => navBarTabIconSize(TabType.account), throwsStateError);
    });
  });

  group('QaulNavBar notification badge', () {
    testWidgets('uses 16px badge positioned 4px from bottom/end', (
      tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: QaulNavBar(
              vertical: false,
              selectedTab: TabType.chat,
              onTabSelected: (_) {},
              onOverflowSelected: (_) {},
              overflowMenuLabels: const {},
              tabTooltips: const {
                TabType.account: 'Account',
                TabType.public: 'Public',
                TabType.users: 'Users',
                TabType.chat: 'Chat',
                TabType.network: 'Network',
              },
              chatNotificationCount: 5,
            ),
          ),
        ),
      );

      final badge = tester.widget<badges.Badge>(find.byType(badges.Badge));
      expect(badge.position?.bottom, 4);
      expect(badge.position?.end, 4);
      expect(badge.badgeStyle.padding, EdgeInsets.zero);

      final badgeContent = badge.badgeContent;
      expect(badgeContent, isA<SizedBox>());
      final box = badgeContent as SizedBox;
      expect(box.width, 16);
      expect(box.height, 16);
    });
  });
}
