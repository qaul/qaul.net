import 'package:flutter/material.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../screens/home/tabs/tab.dart';
import '../widgets/widgets.dart';

import 'constants.dart';

enum NavBarOverflowOption {
  settings,
  about,
  license,
  support,
  oldNetwork,
  files,
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
                const SizedBox(width: kNavBarHorizontalPadding),
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
