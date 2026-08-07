import 'package:badges/badges.dart' as badges;
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:flutter_svg/svg.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:utils/utils.dart';

void main() {
  Widget wrap(Widget child) {
    return MaterialApp(home: Scaffold(body: Center(child: child)));
  }

  group('QaulAvatar', () {
    testWidgets('renders initials with the shared user color strategy', (
      tester,
    ) async {
      const id = '12D3KooWG78qQyC7QLdzpHjFq9UbqZFChkw6MM8XiNncCRhmdpKU';

      await tester.pumpWidget(
        wrap(
          const QaulAvatar(
            name: 'Gustavo Silva',
            id: id,
          ),
        ),
      );

      final avatar = tester.widget<CircleAvatar>(find.byType(CircleAvatar));
      expect(find.text('GS'), findsOneWidget);
      expect(avatar.radius, QaulAvatarSize.small.radius);
      expect(avatar.backgroundColor, colorGenerationStrategy(id));
    });

    testWidgets('uses size tokens for each variant', (tester) async {
      await tester.pumpWidget(
        wrap(
          const Row(
            children: [
              QaulAvatar(name: 'Tiny User', id: 'tiny', size: QaulAvatarSize.tiny),
              QaulAvatar(name: 'Small User', id: 'small'),
              QaulAvatar(name: 'Large User', id: 'large', size: QaulAvatarSize.large),
            ],
          ),
        ),
      );

      final avatars = tester.widgetList<CircleAvatar>(
        find.byType(CircleAvatar),
      );
      expect(
        avatars.map((avatar) => avatar.radius),
        [
          QaulAvatarSize.tiny.radius,
          QaulAvatarSize.small.radius,
          QaulAvatarSize.large.radius,
        ],
      );
    });

    testWidgets('keeps large emoji avatars centered in the circle', (
      tester,
    ) async {
      await tester.pumpWidget(
        wrap(
          const QaulAvatar(
            name: '🦋 Max',
            id: 'emoji',
            size: QaulAvatarSize.large,
          ),
        ),
      );

      final avatarCenter = tester.getCenter(find.byType(CircleAvatar));
      final emojiCenter = tester.getCenter(find.text('🦋'));
      expect((avatarCenter.dx - emojiCenter.dx).abs(), lessThan(1));
      expect((avatarCenter.dy - emojiCenter.dy).abs(), lessThan(1));
    });

    testWidgets('applies the compact emoji offset to small avatars', (
      tester,
    ) async {
      await tester.pumpWidget(
        wrap(
          const QaulAvatar(
            name: '😘 Small',
            id: 'small-emoji',
          ),
        ),
      );

      final avatarCenter = tester.getCenter(find.byType(CircleAvatar));
      final emojiCenter = tester.getCenter(find.text('😘'));
      expect(
        emojiCenter.dx - avatarCenter.dx,
        moreOrLessEquals(kQaulAvatarCompactEmojiOffset.dx, epsilon: 1),
      );
      expect(
        emojiCenter.dy - avatarCenter.dy,
        moreOrLessEquals(kQaulAvatarCompactEmojiOffset.dy, epsilon: 1),
      );
    });

    testWidgets('uses the shared online badge color when visible', (
      tester,
    ) async {
      await tester.pumpWidget(
        wrap(
          const QaulAvatarBadge(
            child: QaulAvatar(name: 'Online User', id: 'online'),
          ),
        ),
      );

      final badge = tester.widget<badges.Badge>(find.byType(badges.Badge));
      expect(badge.badgeStyle.badgeColor, kQaulAvatarOnlineBadgeColor);
    });

    testWidgets('does not wrap the child when badge is hidden', (tester) async {
      await tester.pumpWidget(
        wrap(
          const QaulAvatarBadge(
            isVisible: false,
            child: QaulAvatar(name: 'Offline User', id: 'offline'),
          ),
        ),
      );

      expect(find.byType(badges.Badge), findsNothing);
      expect(find.text('OU'), findsOneWidget);
    });

    testWidgets('renders shared SVG fallbacks for group and blank avatars', (
      tester,
    ) async {
      await tester.pumpWidget(
        wrap(
          const Row(
            children: [
              QaulAvatar.group(),
              QaulAvatar.blank(),
            ],
          ),
        ),
      );

      expect(find.byType(SvgPicture), findsNWidgets(2));
      final pictures = tester.widgetList<SvgPicture>(find.byType(SvgPicture));
      final loaders = pictures.map((picture) => picture.bytesLoader);
      expect(loaders, everyElement(isA<SvgAssetLoader>()));
      expect(
        loaders.cast<SvgAssetLoader>().map((loader) => loader.assetName),
        ['assets/icons/group.svg', 'assets/icons/user.svg'],
      );
    });
  });
}
