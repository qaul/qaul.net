import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'qaul_components_localizations_ar.dart';
import 'qaul_components_localizations_de.dart';
import 'qaul_components_localizations_en.dart';
import 'qaul_components_localizations_es.dart';
import 'qaul_components_localizations_fr.dart';
import 'qaul_components_localizations_ru.dart';
import 'qaul_components_localizations_zh.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of QaulComponentsLocalizations
/// returned by `QaulComponentsLocalizations.of(context)`.
///
/// Applications need to include `QaulComponentsLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/qaul_components_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: QaulComponentsLocalizations.localizationsDelegates,
///   supportedLocales: QaulComponentsLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the QaulComponentsLocalizations.supportedLocales
/// property.
abstract class QaulComponentsLocalizations {
  QaulComponentsLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static QaulComponentsLocalizations? of(BuildContext context) {
    return Localizations.of<QaulComponentsLocalizations>(
      context,
      QaulComponentsLocalizations,
    );
  }

  static const LocalizationsDelegate<QaulComponentsLocalizations> delegate =
      _QaulComponentsLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('ar'),
    Locale('de'),
    Locale('en'),
    Locale('es'),
    Locale('fr'),
    Locale('ru'),
    Locale('zh'),
  ];

  /// No description provided for @navTabAccountTooltip.
  ///
  /// In en, this message translates to:
  /// **'Your account'**
  String get navTabAccountTooltip;

  /// No description provided for @navTabPublicTooltip.
  ///
  /// In en, this message translates to:
  /// **'Public'**
  String get navTabPublicTooltip;

  /// No description provided for @navTabUsersTooltip.
  ///
  /// In en, this message translates to:
  /// **'Users'**
  String get navTabUsersTooltip;

  /// No description provided for @navTabChatTooltip.
  ///
  /// In en, this message translates to:
  /// **'Chat'**
  String get navTabChatTooltip;

  /// No description provided for @navTabNetworkTooltip.
  ///
  /// In en, this message translates to:
  /// **'Network'**
  String get navTabNetworkTooltip;

  /// No description provided for @navOverflowSettings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get navOverflowSettings;

  /// No description provided for @navOverflowAbout.
  ///
  /// In en, this message translates to:
  /// **'About'**
  String get navOverflowAbout;

  /// No description provided for @navOverflowLicense.
  ///
  /// In en, this message translates to:
  /// **'AGPL License'**
  String get navOverflowLicense;

  /// No description provided for @navOverflowSupport.
  ///
  /// In en, this message translates to:
  /// **'Support'**
  String get navOverflowSupport;

  /// No description provided for @navOverflowRoutingTable.
  ///
  /// In en, this message translates to:
  /// **'Routing Data Table'**
  String get navOverflowRoutingTable;

  /// No description provided for @navOverflowFileHistory.
  ///
  /// In en, this message translates to:
  /// **'File history'**
  String get navOverflowFileHistory;

  /// No description provided for @accountCreateUserProfile.
  ///
  /// In en, this message translates to:
  /// **'Create user profile'**
  String get accountCreateUserProfile;

  /// No description provided for @accountRestoreAccount.
  ///
  /// In en, this message translates to:
  /// **'Restore account'**
  String get accountRestoreAccount;

  /// No description provided for @accountLoginExistingAccount.
  ///
  /// In en, this message translates to:
  /// **'Login with existing account'**
  String get accountLoginExistingAccount;

  /// No description provided for @accountLearnMore.
  ///
  /// In en, this message translates to:
  /// **'Learn about qaul'**
  String get accountLearnMore;

  /// No description provided for @accountManageAccount.
  ///
  /// In en, this message translates to:
  /// **'Manage account'**
  String get accountManageAccount;

  /// No description provided for @accountPublicKey.
  ///
  /// In en, this message translates to:
  /// **'Public Key'**
  String get accountPublicKey;

  /// No description provided for @accountUnknown.
  ///
  /// In en, this message translates to:
  /// **'Unknown'**
  String get accountUnknown;

  /// No description provided for @accountLogout.
  ///
  /// In en, this message translates to:
  /// **'Logout'**
  String get accountLogout;

  /// No description provided for @accountExportAccount.
  ///
  /// In en, this message translates to:
  /// **'Export Account'**
  String get accountExportAccount;

  /// No description provided for @accountChangeOrRemovePassword.
  ///
  /// In en, this message translates to:
  /// **'Change or remove password'**
  String get accountChangeOrRemovePassword;

  /// No description provided for @accountRemoveAccount.
  ///
  /// In en, this message translates to:
  /// **'Remove Account'**
  String get accountRemoveAccount;

  /// No description provided for @accountCancel.
  ///
  /// In en, this message translates to:
  /// **'CANCEL'**
  String get accountCancel;

  /// No description provided for @accountRestoreContinue.
  ///
  /// In en, this message translates to:
  /// **'Continue'**
  String get accountRestoreContinue;

  /// No description provided for @accountChooseExportFile.
  ///
  /// In en, this message translates to:
  /// **'Choose export file'**
  String get accountChooseExportFile;

  /// No description provided for @accountRestoreDescription.
  ///
  /// In en, this message translates to:
  /// **'Select a .qaul_export archive to restore this account on this node.'**
  String get accountRestoreDescription;

  /// No description provided for @accountRestoreFilePickerDescription.
  ///
  /// In en, this message translates to:
  /// **'Choose the .qaul_export archive you want to restore on this node.'**
  String get accountRestoreFilePickerDescription;

  /// No description provided for @accountRestoreFilePickerPlaceholder.
  ///
  /// In en, this message translates to:
  /// **'my_account.qaul_export'**
  String get accountRestoreFilePickerPlaceholder;

  /// No description provided for @accountExportDescription.
  ///
  /// In en, this message translates to:
  /// **'Create a .qaul_export archive that can be restored later.'**
  String get accountExportDescription;

  /// No description provided for @accountDeleteExportPrompt.
  ///
  /// In en, this message translates to:
  /// **'Would you like to export your account before removing it from this node?'**
  String get accountDeleteExportPrompt;

  /// No description provided for @accountExportFirst.
  ///
  /// In en, this message translates to:
  /// **'Export first'**
  String get accountExportFirst;

  /// No description provided for @accountDeleteWithoutExport.
  ///
  /// In en, this message translates to:
  /// **'Remove without export'**
  String get accountDeleteWithoutExport;

  /// No description provided for @accountDeleteFinalWarning.
  ///
  /// In en, this message translates to:
  /// **'This permanently removes the account from this node. This action cannot be undone.'**
  String get accountDeleteFinalWarning;

  /// No description provided for @accountDeletePermanently.
  ///
  /// In en, this message translates to:
  /// **'Remove permanently'**
  String get accountDeletePermanently;

  /// No description provided for @accountNewPassword.
  ///
  /// In en, this message translates to:
  /// **'New password'**
  String get accountNewPassword;

  /// No description provided for @accountRemovePassword.
  ///
  /// In en, this message translates to:
  /// **'Remove password'**
  String get accountRemovePassword;

  /// No description provided for @accountSetPassword.
  ///
  /// In en, this message translates to:
  /// **'Set password'**
  String get accountSetPassword;

  /// No description provided for @membersCount.
  ///
  /// In en, this message translates to:
  /// **'{count, plural, =1{1 member} other{{count} members}}'**
  String membersCount(int count);
}

class _QaulComponentsLocalizationsDelegate
    extends LocalizationsDelegate<QaulComponentsLocalizations> {
  const _QaulComponentsLocalizationsDelegate();

  @override
  Future<QaulComponentsLocalizations> load(Locale locale) {
    return SynchronousFuture<QaulComponentsLocalizations>(
      lookupQaulComponentsLocalizations(locale),
    );
  }

  @override
  bool isSupported(Locale locale) => <String>[
    'ar',
    'de',
    'en',
    'es',
    'fr',
    'ru',
    'zh',
  ].contains(locale.languageCode);

  @override
  bool shouldReload(_QaulComponentsLocalizationsDelegate old) => false;
}

QaulComponentsLocalizations lookupQaulComponentsLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'ar':
      return QaulComponentsLocalizationsAr();
    case 'de':
      return QaulComponentsLocalizationsDe();
    case 'en':
      return QaulComponentsLocalizationsEn();
    case 'es':
      return QaulComponentsLocalizationsEs();
    case 'fr':
      return QaulComponentsLocalizationsFr();
    case 'ru':
      return QaulComponentsLocalizationsRu();
    case 'zh':
      return QaulComponentsLocalizationsZh();
  }

  throw FlutterError(
    'QaulComponentsLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
