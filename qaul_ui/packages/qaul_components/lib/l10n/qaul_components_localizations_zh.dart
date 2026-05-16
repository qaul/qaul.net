// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'qaul_components_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Chinese (`zh`).
class QaulComponentsLocalizationsZh extends QaulComponentsLocalizations {
  QaulComponentsLocalizationsZh([String locale = 'zh']) : super(locale);

  @override
  String get navTabAccountTooltip => '您的账号';

  @override
  String get navTabPublicTooltip => '公开';

  @override
  String get navTabUsersTooltip => '用户';

  @override
  String get navTabChatTooltip => '聊天';

  @override
  String get navTabNetworkTooltip => '网络';

  @override
  String get navOverflowSettings => '设置';

  @override
  String get navOverflowAbout => '关于';

  @override
  String get navOverflowLicense => 'AGPL License';

  @override
  String get navOverflowSupport => '帮助与支持';

  @override
  String get navOverflowRoutingTable => '路由表';

  @override
  String get navOverflowFileHistory => 'File history';

  @override
  String membersCount(int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count 位成员',
    );
    return '$_temp0';
  }
}
