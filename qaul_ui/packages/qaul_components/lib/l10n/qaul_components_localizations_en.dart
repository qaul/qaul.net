// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class QaulComponentsLocalizationsEn extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get navTabAccountTooltip => 'Your account';

  @override
  String get navTabPublicTooltip => 'Public';

  @override
  String get navTabUsersTooltip => 'Users';

  @override
  String get navTabChatTooltip => 'Chat';

  @override
  String get navTabNetworkTooltip => 'Network';

  @override
  String get navOverflowSettings => 'Settings';

  @override
  String get navOverflowAbout => 'About';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => 'Support';

  @override
  String get navOverflowRoutingTable => 'Routing Data Table';

  @override
  String get navOverflowFileHistory => 'File history';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count members',
      one: '1 member',
    );
    return '$_temp0';
  }
}
