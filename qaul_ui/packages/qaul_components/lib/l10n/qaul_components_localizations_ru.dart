// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Russian (`ru`).
class QaulComponentsLocalizationsRu extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsRu([String locale = 'ru']) : super(locale);

  @override
  String get navTabAccountTooltip => 'Ваша учетная запись';

  @override
  String get navTabPublicTooltip => 'Общедоступный';

  @override
  String get navTabUsersTooltip => 'Пользователи';

  @override
  String get navTabChatTooltip => 'Чат';

  @override
  String get navTabNetworkTooltip => 'Сеть';

  @override
  String get navOverflowSettings => 'Настройки';

  @override
  String get navOverflowAbout => 'О программе';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => 'Служба поддержки';

  @override
  String get navOverflowRoutingTable => 'Таблица данных маршрутизации';

  @override
  String get navOverflowFileHistory => 'История файлов';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count участника',
      many: '$count участников',
      few: '$count участника',
      one: '1 участник',
    );
    return '$_temp0';
  }
}
