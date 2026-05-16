// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Spanish Castilian (`es`).
class QaulComponentsLocalizationsEs extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsEs([String locale = 'es']) : super(locale);

  @override
  String get navTabAccountTooltip => 'Tu cuenta';

  @override
  String get navTabPublicTooltip => 'Público';

  @override
  String get navTabUsersTooltip => 'Usuarios';

  @override
  String get navTabChatTooltip => 'Chat';

  @override
  String get navTabNetworkTooltip => 'Red';

  @override
  String get navOverflowSettings => 'Configuración';

  @override
  String get navOverflowAbout => 'Acerca de';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => 'Soporte';

  @override
  String get navOverflowRoutingTable => 'Tabla de datos de enrutamiento';

  @override
  String get navOverflowFileHistory => 'Historial del archivo';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count miembros',
      one: '1 miembro',
    );
    return '$_temp0';
  }
}
