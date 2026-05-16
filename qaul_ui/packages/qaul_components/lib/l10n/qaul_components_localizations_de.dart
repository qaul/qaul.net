// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for German (`de`).
class QaulComponentsLocalizationsDe extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsDe([String locale = 'de']) : super(locale);

  @override
  String get navTabAccountTooltip => 'Dein Profil';

  @override
  String get navTabPublicTooltip => 'Öffentlich';

  @override
  String get navTabUsersTooltip => 'Kontakte';

  @override
  String get navTabChatTooltip => 'Chat';

  @override
  String get navTabNetworkTooltip => 'Netzwerk';

  @override
  String get navOverflowSettings => 'Einstellungen';

  @override
  String get navOverflowAbout => 'Über';

  @override
  String get navOverflowLicense => 'AGPL Lizenz';

  @override
  String get navOverflowSupport => 'Support';

  @override
  String get navOverflowRoutingTable => 'Routing-Tabelle';

  @override
  String get navOverflowFileHistory => 'Alle Chat-Dateien';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count Mitglieder',
      one: '1 Mitglied',
    );
    return '$_temp0';
  }
}
