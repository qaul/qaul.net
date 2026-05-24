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
