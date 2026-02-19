import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/nav_bar/constants.dart';
import 'package:qaul_ui/providers/providers.dart';

void main() {
  group('navBarIconPath', () {
    test('returns path for menu icon', () {
      expect(navBarIconPath('menu'), 'assets/icons/nav_bar/menu.svg');
    });

    test('returns path for any name', () {
      expect(navBarIconPath('foo'), 'assets/icons/nav_bar/foo.svg');
    });
  });

  group('navBarTabIconPath', () {
    test('returns outlined path when not selected', () {
      expect(
        navBarTabIconPath(TabType.chat, false),
        'assets/icons/nav_bar/chat-outlined.svg',
      );
    });

    test('returns filled path when selected', () {
      expect(
        navBarTabIconPath(TabType.chat, true),
        'assets/icons/nav_bar/chat-filled.svg',
      );
    });

    test('returns empty string for account tab', () {
      expect(navBarTabIconPath(TabType.account, false), '');
      expect(navBarTabIconPath(TabType.account, true), '');
    });

    test('returns correct paths for all tab types with icons', () {
      expect(navBarTabIconPath(TabType.public, false), 'assets/icons/nav_bar/public-outlined.svg');
      expect(navBarTabIconPath(TabType.users, false), 'assets/icons/nav_bar/people-outlined.svg');
      expect(navBarTabIconPath(TabType.network, false), 'assets/icons/nav_bar/network-outlined.svg');
    });
  });

  group('navBarColors', () {
    test('returns dark theme colors for Brightness.dark', () {
      final theme = ThemeData.dark();
      final (selected, icon, active) = navBarColors(theme);
      expect(selected, kNavBarSelectedBackgroundDark);
      expect(icon, theme.iconTheme.color);
      expect(active, theme.navigationBarTheme.surfaceTintColor ?? theme.iconTheme.color);
    });

    test('returns light theme colors for Brightness.light', () {
      final theme = ThemeData.light();
      final (selected, icon, active) = navBarColors(theme);
      expect(selected, kNavBarSelectedBackgroundLight);
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
      expect(
        () => navBarTabIconSize(TabType.account),
        throwsStateError,
      );
    });
  });
}
