// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for French (`fr`).
class QaulComponentsLocalizationsFr extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsFr([String locale = 'fr']) : super(locale);

  @override
  String get navTabAccountTooltip => 'Votre compte';

  @override
  String get navTabPublicTooltip => 'Public';

  @override
  String get navTabUsersTooltip => 'Utilisateurs';

  @override
  String get navTabChatTooltip => 'Discussion';

  @override
  String get navTabNetworkTooltip => 'Réseau';

  @override
  String get navOverflowSettings => 'Paramètres';

  @override
  String get navOverflowAbout => 'À propos de nous';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => 'Support';

  @override
  String get navOverflowRoutingTable => 'Tableau des données de routage';

  @override
  String get navOverflowFileHistory => 'Historique du fichier';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count membres',
      one: '1 membre',
    );
    return '$_temp0';
  }
}
