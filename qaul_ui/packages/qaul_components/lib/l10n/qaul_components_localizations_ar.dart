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
  String get accountCreateUserProfile => 'Create user profile';

  @override
  String get accountRestoreAccount => 'Restore account';

  @override
  String get accountLoginExistingAccount => 'Login with existing account';

  @override
  String get accountLearnMore => 'Learn about qaul';

  @override
  String get accountManageAccount => 'Manage account';

  @override
  String get accountPublicKey => 'Public Key';

  @override
  String get accountUnknown => 'Unknown';

  @override
  String get accountLogout => 'Logout';

  @override
  String get accountExportAccount => 'Export Account';

  @override
  String get accountChangeOrRemovePassword => 'Change or remove password';

  @override
  String get accountRemoveAccount => 'Remove Account';

  @override
  String get accountCancel => 'CANCEL';

  @override
  String get accountRestoreContinue => 'Continue';

  @override
  String get accountChooseExportFile => 'Choose export file';

  @override
  String get accountRestoreDescription =>
      'Select a .qaul_export archive to restore this account on this node.';

  @override
  String get accountRestoreFilePickerDescription =>
      'Choose the .qaul_export archive you want to restore on this node.';

  @override
  String get accountRestoreFilePickerPlaceholder => 'my_account.qaul_export';

  @override
  String get accountExportDescription =>
      'Create a .qaul_export archive that can be restored later.';

  @override
  String get accountDeleteExportPrompt =>
      'Would you like to export your account before removing it from this node?';

  @override
  String get accountExportFirst => 'Export first';

  @override
  String get accountDeleteWithoutExport => 'Remove without export';

  @override
  String get accountDeleteFinalWarning =>
      'This permanently removes the account from this node. This action cannot be undone.';

  @override
  String get accountDeletePermanently => 'Remove permanently';

  @override
  String get accountNewPassword => 'New password';

  @override
  String get accountRemovePassword => 'Remove password';

  @override
  String get accountSetPassword => 'Set password';

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
