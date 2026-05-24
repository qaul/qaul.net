// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Arabic (`ar`).
class QaulComponentsLocalizationsAr extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsAr([String locale = 'ar']) : super(locale);

  @override
  String get navTabAccountTooltip => 'حسابك';

  @override
  String get navTabPublicTooltip => 'عام';

  @override
  String get navTabUsersTooltip => 'المستخدمين';

  @override
  String get navTabChatTooltip => 'دردشة';

  @override
  String get navTabNetworkTooltip => 'الشبكة';

  @override
  String get navOverflowSettings => 'الإعدادات';

  @override
  String get navOverflowAbout => 'عن';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => 'الدعم';

  @override
  String get navOverflowRoutingTable => 'جدول بيانات التوجيه';

  @override
  String get navOverflowFileHistory => 'تاريخ الملف';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count أعضاء',
      one: 'عضو واحد',
    );
    return '$_temp0';
  }
}
