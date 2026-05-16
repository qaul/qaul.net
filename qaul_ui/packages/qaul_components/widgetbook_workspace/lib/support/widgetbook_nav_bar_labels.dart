import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';

import '../l10n/app_localizations.dart';

Map<TabType, String> widgetbookNavBarTabTooltips(BuildContext context) {
  final l10n = AppLocalizations.of(context)!;
  return {
    TabType.account: l10n.userAccountNavButtonTooltip,
    TabType.public: l10n.publicNavButtonTooltip,
    TabType.users: l10n.usersNavButtonTooltip,
    TabType.chat: l10n.chatNavButtonTooltip,
    TabType.network: l10n.network,
  };
}

Map<NavBarOverflowOption, String> widgetbookNavBarOverflowLabels(
  BuildContext context,
) {
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
