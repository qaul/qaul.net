import 'package:flutter/widgets.dart';

import '../design_components/shell/qaul_navbar.dart';
import 'qaul_components_localizations.dart';

/// Localized default labels for [QaulNavBar] overflow menu entries.
Map<NavBarOverflowOption, String> qaulNavBarOverflowMenuLabels(
  BuildContext context,
) {
  final l10n = QaulComponentsLocalizations.of(context)!;
  return {
    NavBarOverflowOption.settings: l10n.navOverflowSettings,
    NavBarOverflowOption.about: l10n.navOverflowAbout,
    NavBarOverflowOption.license: l10n.navOverflowLicense,
    NavBarOverflowOption.support: l10n.navOverflowSupport,
    NavBarOverflowOption.oldNetwork: l10n.navOverflowRoutingTable,
    NavBarOverflowOption.files: l10n.navOverflowFileHistory,
  };
}

/// Localized default tooltips for [QaulNavBar] tabs.
Map<TabType, String> qaulNavBarDefaultTabTooltips(BuildContext context) {
  final l10n = QaulComponentsLocalizations.of(context)!;
  return {
    TabType.account: l10n.navTabAccountTooltip,
    TabType.public: l10n.navTabPublicTooltip,
    TabType.users: l10n.navTabUsersTooltip,
    TabType.chat: l10n.navTabChatTooltip,
    TabType.network: l10n.navTabNetworkTooltip,
  };
}

String qaulChatHeaderMembersCountLabel(BuildContext context, int count) {
  return QaulComponentsLocalizations.of(context)!.membersCount(count);
}
