import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../providers/providers.dart';
import '../screens/home/tabs/tab.dart';
import '../widgets/widgets.dart';

class QaulNavBarDecorator extends StatefulWidget {
  const QaulNavBarDecorator({super.key, required this.child});

  final Widget Function(GlobalKey pageViewKey) child;

  @override
  State<QaulNavBarDecorator> createState() => _QaulNavBarDecoratorState();
}

class _QaulNavBarDecoratorState extends State<QaulNavBarDecorator> {
  final _pageViewKey = GlobalKey();

  @override
  Widget build(BuildContext context) {
    return ResponsiveLayout(
      mobileBody: Column(
        children: [
          Expanded(child: widget.child(_pageViewKey)),
          const _ConnectedNavBar(vertical: false),
        ],
      ),
      tabletBody: Row(
        children: [
          const _ConnectedNavBar(vertical: true),
          Expanded(child: widget.child(_pageViewKey)),
        ],
      ),
    );
  }
}

class _ConnectedNavBar extends ConsumerStatefulWidget {
  const _ConnectedNavBar({required this.vertical});

  final bool vertical;

  @override
  ConsumerState<_ConnectedNavBar> createState() => _ConnectedNavBarState();
}

class _ConnectedNavBarState extends ConsumerState<_ConnectedNavBar> {
  PublicNotificationController? _publicController;
  ChatNotificationController? _chatController;

  @override
  void dispose() {
    _publicController?.newNotificationCount.removeListener(_onNotificationChanged);
    _chatController?.newNotificationCount.removeListener(_onNotificationChanged);
    super.dispose();
  }

  void _onNotificationChanged() {
    if (mounted) setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    final publicController = ref.read(publicNotificationControllerProvider);
    final chatController = ref.read(chatNotificationControllerProvider);
    if (_publicController != publicController || _chatController != chatController) {
      _publicController?.newNotificationCount.removeListener(_onNotificationChanged);
      _chatController?.newNotificationCount.removeListener(_onNotificationChanged);
      _publicController = publicController;
      _chatController = chatController;
      publicController.newNotificationCount.addListener(_onNotificationChanged);
      chatController.newNotificationCount.addListener(_onNotificationChanged);
    }

    final currentTab = ref.watch(homeScreenControllerProvider);
    final tabController = ref.read(homeScreenControllerProvider.notifier);
    final user = ref.watch(defaultUserProvider);
    final l10n = AppLocalizations.of(context)!;

    final avatarChild = user != null
        ? CircleAvatar(
            radius: kNavBarAccountSize / 2,
            backgroundColor: colorGenerationStrategy(user.idBase58),
            child: Text(
              initials(user.name),
              style: kNavBarAvatarTextStyle,
            ),
          )
        : null;

    final tabTooltips = {
      TabType.account: l10n.userAccountNavButtonTooltip,
      TabType.public: l10n.publicNavButtonTooltip,
      TabType.users: l10n.usersNavButtonTooltip,
      TabType.chat: l10n.chatNavButtonTooltip,
      TabType.network: l10n.network,
    };

    final publicCount = publicController.newNotificationCount.value;
    final chatCount = chatController.newNotificationCount.value;

    return QaulNavBar(
      vertical: widget.vertical,
      overflowMenuLabels: navBarOverflowMenuLabels(context),
      onOverflowSelected: (option) =>
          handleNavBarOverflowSelected(context, option),
      selectedTab: currentTab,
      onTabSelected: (tab) {
        tabController.goToTab(tab);
        if (tab == TabType.public) {
          publicController.removeNotifications();
        } else if (tab == TabType.chat) {
          chatController.removeNotifications();
        }
      },
      avatarChild: avatarChild,
      publicNotificationCount: publicCount,
      chatNotificationCount: chatCount,
      tabTooltips: tabTooltips,
    );
  }
}

Map<NavBarOverflowOption, String> navBarOverflowMenuLabels(BuildContext context) {
  final l10n = AppLocalizations.of(context)!;
  return {
    NavBarOverflowOption.settings: l10n.settings,
    NavBarOverflowOption.about: l10n.about,
    NavBarOverflowOption.license: l10n.agplLicense,
    NavBarOverflowOption.support: l10n.support,
    NavBarOverflowOption.oldNetwork: l10n.routingDataTable,
    NavBarOverflowOption.files: l10n.fileHistory,
  };
}

void handleNavBarOverflowSelected(
    BuildContext context, NavBarOverflowOption option) {
  switch (option) {
    case NavBarOverflowOption.settings:
      Navigator.pushNamed(context, NavigationHelper.settings);
      break;
    case NavBarOverflowOption.about:
      Navigator.pushNamed(context, NavigationHelper.about);
      break;
    case NavBarOverflowOption.license:
      Navigator.pushNamed(context, NavigationHelper.license);
      break;
    case NavBarOverflowOption.support:
      Navigator.pushNamed(context, NavigationHelper.support);
      break;
    case NavBarOverflowOption.oldNetwork:
      Navigator.push(context, MaterialPageRoute(builder: (_) {
        return Scaffold(
          appBar: AppBar(
            leading: const IconButtonFactory(),
            title: Row(
              children: [
                const Icon(Icons.language),
                const SizedBox(width: 8),
                Text(AppLocalizations.of(context)!.routingDataTable),
              ],
            ),
          ),
          body: BaseTab.network(),
        );
      }));
      break;
    case NavBarOverflowOption.files:
      Navigator.pushNamed(context, NavigationHelper.fileHistory);
      break;
  }
}
