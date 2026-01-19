import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_ar.dart';
import 'app_localizations_de.dart';
import 'app_localizations_en.dart';
import 'app_localizations_es.dart';
import 'app_localizations_fa.dart';
import 'app_localizations_fr.dart';
import 'app_localizations_id.dart';
import 'app_localizations_it.dart';
import 'app_localizations_pt.dart';
import 'app_localizations_ru.dart';
import 'app_localizations_uk.dart';
import 'app_localizations_zh.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
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
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations? of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations);
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

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
    Locale('fa'),
    Locale('fr'),
    Locale('id'),
    Locale('it'),
    Locale('pt'),
    Locale('ru'),
    Locale('uk'),
    Locale('zh', 'TW'),
    Locale('zh'),
  ];

  /// No description provided for @okDialogButton.
  ///
  /// In en, this message translates to:
  /// **'OK'**
  String get okDialogButton;

  /// No description provided for @cancelDialogButton.
  ///
  /// In en, this message translates to:
  /// **'CANCEL'**
  String get cancelDialogButton;

  /// No description provided for @backButtonTooltip.
  ///
  /// In en, this message translates to:
  /// **'Back'**
  String get backButtonTooltip;

  /// No description provided for @sendTooltip.
  ///
  /// In en, this message translates to:
  /// **'Send'**
  String get sendTooltip;

  /// No description provided for @sendFileTooltip.
  ///
  /// In en, this message translates to:
  /// **'Send File'**
  String get sendFileTooltip;

  /// No description provided for @sendAudioTooltip.
  ///
  /// In en, this message translates to:
  /// **'Record audio message'**
  String get sendAudioTooltip;

  /// No description provided for @start.
  ///
  /// In en, this message translates to:
  /// **'Start'**
  String get start;

  /// No description provided for @createUserAccount.
  ///
  /// In en, this message translates to:
  /// **'Create user profile'**
  String get createUserAccount;

  /// No description provided for @learnMore.
  ///
  /// In en, this message translates to:
  /// **'Learn about qaul'**
  String get learnMore;

  /// No description provided for @userAccountNavButtonTooltip.
  ///
  /// In en, this message translates to:
  /// **'Your account'**
  String get userAccountNavButtonTooltip;

  /// No description provided for @publicNavButtonTooltip.
  ///
  /// In en, this message translates to:
  /// **'Public'**
  String get publicNavButtonTooltip;

  /// No description provided for @usersNavButtonTooltip.
  ///
  /// In en, this message translates to:
  /// **'Users'**
  String get usersNavButtonTooltip;

  /// No description provided for @chatNavButtonTooltip.
  ///
  /// In en, this message translates to:
  /// **'Chat'**
  String get chatNavButtonTooltip;

  /// No description provided for @network.
  ///
  /// In en, this message translates to:
  /// **'Network'**
  String get network;

  /// No description provided for @createPublicPostTooltip.
  ///
  /// In en, this message translates to:
  /// **'Create post'**
  String get createPublicPostTooltip;

  /// No description provided for @submitPostTooltip.
  ///
  /// In en, this message translates to:
  /// **'Submit'**
  String get submitPostTooltip;

  /// No description provided for @newChatTooltip.
  ///
  /// In en, this message translates to:
  /// **'Chat'**
  String get newChatTooltip;

  /// No description provided for @createGroupHint.
  ///
  /// In en, this message translates to:
  /// **'Group name'**
  String get createGroupHint;

  /// No description provided for @publicNotificationsEnabled.
  ///
  /// In en, this message translates to:
  /// **'Public Posts'**
  String get publicNotificationsEnabled;

  /// No description provided for @chatNotificationsEnabled.
  ///
  /// In en, this message translates to:
  /// **'Chat Messages'**
  String get chatNotificationsEnabled;

  /// No description provided for @notifyOnlyForVerifiedUsers.
  ///
  /// In en, this message translates to:
  /// **'Only Show Notifications of Verified Users'**
  String get notifyOnlyForVerifiedUsers;

  /// No description provided for @settings.
  ///
  /// In en, this message translates to:
  /// **'Settings'**
  String get settings;

  /// No description provided for @about.
  ///
  /// In en, this message translates to:
  /// **'About'**
  String get about;

  /// No description provided for @theme.
  ///
  /// In en, this message translates to:
  /// **'Theme'**
  String get theme;

  /// No description provided for @lightTheme.
  ///
  /// In en, this message translates to:
  /// **'Light Theme'**
  String get lightTheme;

  /// No description provided for @darkTheme.
  ///
  /// In en, this message translates to:
  /// **'Dark Theme'**
  String get darkTheme;

  /// No description provided for @internetNodes.
  ///
  /// In en, this message translates to:
  /// **'Internet Nodes'**
  String get internetNodes;

  /// No description provided for @address.
  ///
  /// In en, this message translates to:
  /// **'Address'**
  String get address;

  /// No description provided for @name.
  ///
  /// In en, this message translates to:
  /// **'Name'**
  String get name;

  /// No description provided for @options.
  ///
  /// In en, this message translates to:
  /// **'Options'**
  String get options;

  /// No description provided for @useIpv6.
  ///
  /// In en, this message translates to:
  /// **'Use IPv6'**
  String get useIpv6;

  /// No description provided for @useQuic.
  ///
  /// In en, this message translates to:
  /// **'Use Quic Protocol'**
  String get useQuic;

  /// No description provided for @connections.
  ///
  /// In en, this message translates to:
  /// **'Connections'**
  String get connections;

  /// No description provided for @allConnectionsFilterLabel.
  ///
  /// In en, this message translates to:
  /// **'Show Entire Network'**
  String get allConnectionsFilterLabel;

  /// No description provided for @ping.
  ///
  /// In en, this message translates to:
  /// **'Ping'**
  String get ping;

  /// No description provided for @hopCount.
  ///
  /// In en, this message translates to:
  /// **'Hop Count'**
  String get hopCount;

  /// No description provided for @via.
  ///
  /// In en, this message translates to:
  /// **'Via'**
  String get via;

  /// No description provided for @language.
  ///
  /// In en, this message translates to:
  /// **'Language'**
  String get language;

  /// No description provided for @username.
  ///
  /// In en, this message translates to:
  /// **'Username'**
  String get username;

  /// No description provided for @userID.
  ///
  /// In en, this message translates to:
  /// **'User ID'**
  String get userID;

  /// No description provided for @publicKey.
  ///
  /// In en, this message translates to:
  /// **'Public Key'**
  String get publicKey;

  /// No description provided for @unknown.
  ///
  /// In en, this message translates to:
  /// **'Unknown'**
  String get unknown;

  /// No description provided for @verify.
  ///
  /// In en, this message translates to:
  /// **'Verify'**
  String get verify;

  /// No description provided for @unverify.
  ///
  /// In en, this message translates to:
  /// **'Remove verified status'**
  String get unverify;

  /// No description provided for @blockUser.
  ///
  /// In en, this message translates to:
  /// **'Block User'**
  String get blockUser;

  /// No description provided for @unblockUser.
  ///
  /// In en, this message translates to:
  /// **'Unblock User'**
  String get unblockUser;

  /// No description provided for @addNodeCTA.
  ///
  /// In en, this message translates to:
  /// **'Add internet node'**
  String get addNodeCTA;

  /// No description provided for @chatEmptyMessageHint.
  ///
  /// In en, this message translates to:
  /// **'Your message...'**
  String get chatEmptyMessageHint;

  /// No description provided for @emptyPublicList.
  ///
  /// In en, this message translates to:
  /// **'No public messages yet'**
  String get emptyPublicList;

  /// No description provided for @emptyUsersList.
  ///
  /// In en, this message translates to:
  /// **'No users added yet'**
  String get emptyUsersList;

  /// No description provided for @emptyChatsList.
  ///
  /// In en, this message translates to:
  /// **'No chat rooms yet'**
  String get emptyChatsList;

  /// No description provided for @genericEmptyState.
  ///
  /// In en, this message translates to:
  /// **'Nothing here yet'**
  String get genericEmptyState;

  /// No description provided for @verifyUserConfirmationMessage.
  ///
  /// In en, this message translates to:
  /// **'Do you want to verify this user?'**
  String get verifyUserConfirmationMessage;

  /// No description provided for @unverifyUserConfirmationMessage.
  ///
  /// In en, this message translates to:
  /// **'Do you want to remove the verified status from this user?'**
  String get unverifyUserConfirmationMessage;

  /// No description provided for @blockUserConfirmationMessage.
  ///
  /// In en, this message translates to:
  /// **'Do you want to block this user?'**
  String get blockUserConfirmationMessage;

  /// No description provided for @unblockUserConfirmationMessage.
  ///
  /// In en, this message translates to:
  /// **'Do you want to unblock this user?'**
  String get unblockUserConfirmationMessage;

  /// No description provided for @useSystemDefaultMessage.
  ///
  /// In en, this message translates to:
  /// **'Use system\'s default'**
  String get useSystemDefaultMessage;

  /// No description provided for @genericErrorMessage.
  ///
  /// In en, this message translates to:
  /// **'An error occurred'**
  String get genericErrorMessage;

  /// No description provided for @fieldRequiredErrorMessage.
  ///
  /// In en, this message translates to:
  /// **'Field required.'**
  String get fieldRequiredErrorMessage;

  /// No description provided for @timeoutErrorMessage.
  ///
  /// In en, this message translates to:
  /// **'Timeout'**
  String get timeoutErrorMessage;

  /// No description provided for @notFoundErrorMessage.
  ///
  /// In en, this message translates to:
  /// **'not found'**
  String get notFoundErrorMessage;

  /// No description provided for @noneAvailableMessage.
  ///
  /// In en, this message translates to:
  /// **'None available'**
  String get noneAvailableMessage;

  /// No description provided for @invalidIPMessage.
  ///
  /// In en, this message translates to:
  /// **'Invalid IP'**
  String get invalidIPMessage;

  /// No description provided for @invalidPortMessage.
  ///
  /// In en, this message translates to:
  /// **'Invalid Port'**
  String get invalidPortMessage;

  /// No description provided for @usernameLengthMessage.
  ///
  /// In en, this message translates to:
  /// **'Username must have at least 2 characters.'**
  String get usernameLengthMessage;

  /// No description provided for @pleaseRestartApp.
  ///
  /// In en, this message translates to:
  /// **'Please restart the application'**
  String get pleaseRestartApp;

  /// No description provided for @gotoSupport.
  ///
  /// In en, this message translates to:
  /// **'Go to support'**
  String get gotoSupport;

  /// No description provided for @continueDialogButton.
  ///
  /// In en, this message translates to:
  /// **'Continue'**
  String get continueDialogButton;

  /// No description provided for @removeUser.
  ///
  /// In en, this message translates to:
  /// **'Remove user'**
  String get removeUser;

  /// No description provided for @removeUserDialogContent.
  ///
  /// In en, this message translates to:
  /// **'Are you sure you\'d like to remove this user from the group?'**
  String get removeUserDialogContent;

  /// No description provided for @support.
  ///
  /// In en, this message translates to:
  /// **'Support'**
  String get support;

  /// No description provided for @enableLogging.
  ///
  /// In en, this message translates to:
  /// **'Enable Logging:'**
  String get enableLogging;

  /// No description provided for @totalLogsSize.
  ///
  /// In en, this message translates to:
  /// **'Total logs size:'**
  String get totalLogsSize;

  /// No description provided for @deleteLogs.
  ///
  /// In en, this message translates to:
  /// **'Delete logs'**
  String get deleteLogs;

  /// No description provided for @logsDescription1.
  ///
  /// In en, this message translates to:
  /// **'Whenever an error occurs, a log is created.'**
  String get logsDescription1;

  /// No description provided for @logsDescription2.
  ///
  /// In en, this message translates to:
  /// **'You can choose to report them or delete them.'**
  String get logsDescription2;

  /// No description provided for @sendLogs.
  ///
  /// In en, this message translates to:
  /// **'Send Logs'**
  String get sendLogs;

  /// No description provided for @noLogsAvailable.
  ///
  /// In en, this message translates to:
  /// **'No Logs Available'**
  String get noLogsAvailable;

  /// No description provided for @routingDataTable.
  ///
  /// In en, this message translates to:
  /// **'Routing Data Table'**
  String get routingDataTable;

  /// No description provided for @knownAddresses.
  ///
  /// In en, this message translates to:
  /// **'Known Addresses'**
  String get knownAddresses;

  /// No description provided for @noOpenChats.
  ///
  /// In en, this message translates to:
  /// **'No open chats'**
  String get noOpenChats;

  /// No description provided for @groupInvite.
  ///
  /// In en, this message translates to:
  /// **'Group Invite'**
  String get groupInvite;

  /// No description provided for @createNewGroup.
  ///
  /// In en, this message translates to:
  /// **'Create new group'**
  String get createNewGroup;

  /// No description provided for @invite.
  ///
  /// In en, this message translates to:
  /// **'Invite'**
  String get invite;

  /// No description provided for @groupName.
  ///
  /// In en, this message translates to:
  /// **'Group Name'**
  String get groupName;

  /// No description provided for @createdAt.
  ///
  /// In en, this message translates to:
  /// **'Created at'**
  String get createdAt;

  /// No description provided for @noOfMembers.
  ///
  /// In en, this message translates to:
  /// **'Number of members'**
  String get noOfMembers;

  /// No description provided for @invitedBy.
  ///
  /// In en, this message translates to:
  /// **'Invited by'**
  String get invitedBy;

  /// No description provided for @accept.
  ///
  /// In en, this message translates to:
  /// **'Accept'**
  String get accept;

  /// No description provided for @decline.
  ///
  /// In en, this message translates to:
  /// **'Decline'**
  String get decline;

  /// No description provided for @groupSettings.
  ///
  /// In en, this message translates to:
  /// **'Group Settings'**
  String get groupSettings;

  /// No description provided for @members.
  ///
  /// In en, this message translates to:
  /// **'Members'**
  String get members;

  /// No description provided for @showAllFiles.
  ///
  /// In en, this message translates to:
  /// **'Show All Files'**
  String get showAllFiles;

  /// No description provided for @searchUser.
  ///
  /// In en, this message translates to:
  /// **'Search user...'**
  String get searchUser;

  /// No description provided for @storageUsers.
  ///
  /// In en, this message translates to:
  /// **'Storage Users'**
  String get storageUsers;

  /// No description provided for @addStorageUser.
  ///
  /// In en, this message translates to:
  /// **'Add storage user'**
  String get addStorageUser;

  /// No description provided for @publicNoteHintText.
  ///
  /// In en, this message translates to:
  /// **'Public note'**
  String get publicNoteHintText;

  /// No description provided for @createButtonHint.
  ///
  /// In en, this message translates to:
  /// **'Create'**
  String get createButtonHint;

  /// No description provided for @chatEmptyState.
  ///
  /// In en, this message translates to:
  /// **'No messages here yet'**
  String get chatEmptyState;

  /// No description provided for @securityNumber.
  ///
  /// In en, this message translates to:
  /// **'Security Number'**
  String get securityNumber;

  /// No description provided for @securityNumberDialogDesc.
  ///
  /// In en, this message translates to:
  /// **'Please ensure that the person you\'re trying to verify sees the same security number on their screen when attempting to verify you.'**
  String get securityNumberDialogDesc;

  /// No description provided for @groupStateEventCreated.
  ///
  /// In en, this message translates to:
  /// **'The group has been created'**
  String get groupStateEventCreated;

  /// No description provided for @groupStateEventClosed.
  ///
  /// In en, this message translates to:
  /// **'The group has been closed'**
  String get groupStateEventClosed;

  /// No description provided for @groupEventInvited.
  ///
  /// In en, this message translates to:
  /// **'\"{username}\" has been invited to the group'**
  String groupEventInvited(String username);

  /// No description provided for @aboutBackgroundExecution.
  ///
  /// In en, this message translates to:
  /// **'About the background execution'**
  String get aboutBackgroundExecution;

  /// No description provided for @agplLicense.
  ///
  /// In en, this message translates to:
  /// **'AGPL License'**
  String get agplLicense;

  /// No description provided for @androidOptions.
  ///
  /// In en, this message translates to:
  /// **'Android options'**
  String get androidOptions;

  /// No description provided for @androidPrivacyPolicy.
  ///
  /// In en, this message translates to:
  /// **'Privacy Policy'**
  String get androidPrivacyPolicy;

  /// No description provided for @backgroundExecutionDialog1.
  ///
  /// In en, this message translates to:
  /// **'This app uses background execution to receive and send messages when the app is running in the background.'**
  String get backgroundExecutionDialog1;

  /// No description provided for @backgroundExecutionDialog2.
  ///
  /// In en, this message translates to:
  /// **'On older Android devices, we ask location permissions and background location permission in order to communicate via Bluetooth Low Energy. This is due to a missing separation between bluetooth permissions and location permissions. Only bluetooth is used, the location is not used by the app at all.'**
  String get backgroundExecutionDialog2;

  /// No description provided for @backgroundExecutionDialog3.
  ///
  /// In en, this message translates to:
  /// **'This is completely optional, and you can disable this behavior at any time through the Android settings.'**
  String get backgroundExecutionDialog3;

  /// No description provided for @backgroundExecutionDialogConfirmButton.
  ///
  /// In en, this message translates to:
  /// **'I understand'**
  String get backgroundExecutionDialogConfirmButton;

  /// CTA rendered on top of text field
  ///
  /// In en, this message translates to:
  /// **'Choose a user name'**
  String get createAccountHeading;

  /// No description provided for @currentVersion.
  ///
  /// In en, this message translates to:
  /// **'currently installed version:'**
  String get currentVersion;

  /// No description provided for @emptyNodeName.
  ///
  /// In en, this message translates to:
  /// **'No name'**
  String get emptyNodeName;

  /// No description provided for @fileHistory.
  ///
  /// In en, this message translates to:
  /// **'File history'**
  String get fileHistory;

  /// No description provided for @forceUpdateConfirmationDialog.
  ///
  /// In en, this message translates to:
  /// **'All your existing data will be deleted.'**
  String get forceUpdateConfirmationDialog;

  /// No description provided for @forceUpdateCreateAccount.
  ///
  /// In en, this message translates to:
  /// **'Create new account'**
  String get forceUpdateCreateAccount;

  /// No description provided for @forceUpdateDescription1.
  ///
  /// In en, this message translates to:
  /// **'info: qaul has a new database format. Users of qaul 2.0.0-beta.17 and earlier who wish to keep their existing account need to migrate their data to the new format.'**
  String get forceUpdateDescription1;

  /// No description provided for @forceUpdateDescription2.
  ///
  /// In en, this message translates to:
  /// **'To migrate an existing database, download qaul 2.0.0-beta.18 and run it'**
  String get forceUpdateDescription2;

  /// No description provided for @forceUpdateDescription3.
  ///
  /// In en, this message translates to:
  /// **'If you don\'t wish to keep your existing data base'**
  String get forceUpdateDescription3;

  /// No description provided for @forceUpdateDisclaimer.
  ///
  /// In en, this message translates to:
  /// **'(you will lose all your data and accounts)'**
  String get forceUpdateDisclaimer;

  /// No description provided for @forceUpdateDownloadQaul18.
  ///
  /// In en, this message translates to:
  /// **'Download qaul 2.0.0-beta.18'**
  String get forceUpdateDownloadQaul18;

  /// No description provided for @forceUpdateRequired.
  ///
  /// In en, this message translates to:
  /// **'Upgrade Required'**
  String get forceUpdateRequired;

  /// No description provided for @groupChatMessageHint.
  ///
  /// In en, this message translates to:
  /// **'Group chat message'**
  String get groupChatMessageHint;

  /// No description provided for @groupEventInviteAccepted.
  ///
  /// In en, this message translates to:
  /// **'\"{username}\" accepted the invite to the group'**
  String groupEventInviteAccepted(String username);

  /// No description provided for @groupEventJoined.
  ///
  /// In en, this message translates to:
  /// **'\"{username}\" has joined the group'**
  String groupEventJoined(String username);

  /// No description provided for @groupEventLeft.
  ///
  /// In en, this message translates to:
  /// **'\"{username}\" has left the group'**
  String groupEventLeft(String username);

  /// No description provided for @groupEventRemoved.
  ///
  /// In en, this message translates to:
  /// **'\"{username}\" was removed from the group'**
  String groupEventRemoved(String username);

  /// No description provided for @inviteUser.
  ///
  /// In en, this message translates to:
  /// **'Invite user'**
  String get inviteUser;

  /// No description provided for @languageName.
  ///
  /// In en, this message translates to:
  /// **'English'**
  String get languageName;

  /// No description provided for @notifications.
  ///
  /// In en, this message translates to:
  /// **'Notifications'**
  String get notifications;

  /// No description provided for @previousVersion.
  ///
  /// In en, this message translates to:
  /// **'formerly installed version:'**
  String get previousVersion;

  /// No description provided for @securePrivateMessageHint.
  ///
  /// In en, this message translates to:
  /// **'Secure private message'**
  String get securePrivateMessageHint;

  /// No description provided for @userDocumentation.
  ///
  /// In en, this message translates to:
  /// **'User Documentation'**
  String get userDocumentation;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) => <String>[
    'ar',
    'de',
    'en',
    'es',
    'fa',
    'fr',
    'id',
    'it',
    'pt',
    'ru',
    'uk',
    'zh',
  ].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when language+country codes are specified.
  switch (locale.languageCode) {
    case 'zh':
      {
        switch (locale.countryCode) {
          case 'TW':
            return AppLocalizationsZhTw();
        }
        break;
      }
  }

  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'ar':
      return AppLocalizationsAr();
    case 'de':
      return AppLocalizationsDe();
    case 'en':
      return AppLocalizationsEn();
    case 'es':
      return AppLocalizationsEs();
    case 'fa':
      return AppLocalizationsFa();
    case 'fr':
      return AppLocalizationsFr();
    case 'id':
      return AppLocalizationsId();
    case 'it':
      return AppLocalizationsIt();
    case 'pt':
      return AppLocalizationsPt();
    case 'ru':
      return AppLocalizationsRu();
    case 'uk':
      return AppLocalizationsUk();
    case 'zh':
      return AppLocalizationsZh();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
