// ignore: unused_import
import 'package:intl/intl.dart' as intl;
import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Russian (`ru`).
class AppLocalizationsRu extends AppLocalizations {
  AppLocalizationsRu([String locale = 'ru']) : super(locale);

  @override
  String get okDialogButton => 'ОК';

  @override
  String get cancelDialogButton => 'ОТМЕНА';

  @override
  String get backButtonTooltip => 'Назад';

  @override
  String get sendTooltip => 'Отправить';

  @override
  String get sendFileTooltip => 'Отправить файл';

  @override
  String get sendAudioTooltip => 'Записать аудиосообщение';

  @override
  String get start => 'Пуск';

  @override
  String get createUserAccount => 'Создать профиль пользователя';

  @override
  String get learnMore => 'Узнайте о qaul';

  @override
  String get userAccountNavButtonTooltip => 'Ваша учетная запись';

  @override
  String get publicNavButtonTooltip => 'Общедоступный';

  @override
  String get usersNavButtonTooltip => 'Пользователи';

  @override
  String get chatNavButtonTooltip => 'Чат';

  @override
  String get network => 'Сеть';

  @override
  String get createPublicPostTooltip => 'Создать сообщение';

  @override
  String get submitPostTooltip => 'Отправить';

  @override
  String get newChatTooltip => 'Новый чат';

  @override
  String get createGroupHint => 'Название группы';

  @override
  String get publicNotificationsEnabled => 'Общедоступные сообщения';

  @override
  String get chatNotificationsEnabled => 'Сообщения чата';

  @override
  String get notifyOnlyForVerifiedUsers =>
      'Показывать уведомления только проверенных пользователей';

  @override
  String get settings => 'Настройки';

  @override
  String get about => 'О программе';

  @override
  String get theme => 'Тема';

  @override
  String get lightTheme => 'Светлая тема';

  @override
  String get darkTheme => 'Темная тема';

  @override
  String get internetNodes => 'Интернет-узлы';

  @override
  String get address => 'Адрес';

  @override
  String get name => 'Имя';

  @override
  String get options => 'Options';

  @override
  String get useIpv6 => 'Use IPv6';

  @override
  String get useQuic => 'Use Quic Protocol';

  @override
  String get connections => 'Соединения';

  @override
  String get allConnectionsFilterLabel => 'Показать всю сеть';

  @override
  String get ping => 'Пинг';

  @override
  String get hopCount => 'Количество переходов';

  @override
  String get via => 'Через';

  @override
  String get language => 'Язык';

  @override
  String get username => 'Имя пользователя';

  @override
  String get userID => 'Идентификатор пользователя';

  @override
  String get publicKey => 'Открытый ключ';

  @override
  String get unknown => 'Неизвестно';

  @override
  String get verify => 'Проверка';

  @override
  String get unverify => 'Удалить  подтвержденный статус';

  @override
  String get blockUser => 'Заблокировать пользователя';

  @override
  String get unblockUser => 'Разблокировать пользователя';

  @override
  String get addNodeCTA => 'Добавить интернет-узел';

  @override
  String get chatEmptyMessageHint => 'Ваше сообщение...';

  @override
  String get emptyPublicList => 'Общедоступных сообщений еще нет';

  @override
  String get emptyUsersList => 'Пользователи еще не добавлены';

  @override
  String get emptyChatsList => 'Пока нет чатов';

  @override
  String get genericEmptyState => 'Здесь пока ничего нет';

  @override
  String get verifyUserConfirmationMessage =>
      'Вы хотите подтвердить этого пользователя?';

  @override
  String get unverifyUserConfirmationMessage =>
      'Вы хотите удалить статус подтверждения у этого пользователя?';

  @override
  String get blockUserConfirmationMessage =>
      'Вы хотите заблокировать этого пользователя?';

  @override
  String get unblockUserConfirmationMessage =>
      'Вы хотите разблокировать этого пользователя?';

  @override
  String get useSystemDefaultMessage =>
      'Использовать системное сообщение по умолчанию';

  @override
  String get genericErrorMessage => 'Произошла ошибка';

  @override
  String get fieldRequiredErrorMessage => 'Требуется поле.';

  @override
  String get timeoutErrorMessage => 'Тайм-аут';

  @override
  String get notFoundErrorMessage => 'Не найдено';

  @override
  String get noneAvailableMessage => 'Недоступно';

  @override
  String get invalidIPMessage => 'Недопустимый IP';

  @override
  String get invalidPortMessage => 'Недопустимый порт';

  @override
  String get usernameLengthMessage =>
      'Имя пользователя должно содержать не менее 2 символов.';

  @override
  String get pleaseRestartApp => 'Пожалуйста, перезапустите приложение';

  @override
  String get gotoSupport => 'Перейти в службу поддержки';

  @override
  String get continueDialogButton => 'Продолжить';

  @override
  String get removeUser => 'Удалить пользователя';

  @override
  String get removeUserDialogContent =>
      'Вы уверены, что хотите удалить этого пользователя из группы?';

  @override
  String get support => 'Служба поддержки';

  @override
  String get enableLogging => 'Включить ведение журнала:';

  @override
  String get totalLogsSize => 'Общий размер журналов:';

  @override
  String get deleteLogs => 'Удалить журналы';

  @override
  String get logsDescription1 =>
      'Всякий раз, когда возникает ошибка, создается журнал.';

  @override
  String get logsDescription2 => 'Вы можете сообщить о них или удалить их.';

  @override
  String get sendLogs => 'Отправить журналы';

  @override
  String get noLogsAvailable => 'Журналы недоступны';

  @override
  String get routingDataTable => 'Таблица данных маршрутизации';

  @override
  String get knownAddresses => 'Известные адреса';

  @override
  String get noOpenChats => 'Нет открытых чатов';

  @override
  String get groupInvite => 'Приглашение в группу';

  @override
  String get createNewGroup => 'Создать новую группу';

  @override
  String get invite => 'Пригласить';

  @override
  String get groupName => 'Название группы';

  @override
  String get createdAt => 'Создано в';

  @override
  String get noOfMembers => 'Количество участников';

  @override
  String get invitedBy => 'Приглашен';

  @override
  String get accept => 'Принять';

  @override
  String get decline => 'Отклонить';

  @override
  String get groupSettings => 'Настройки группы';

  @override
  String get members => 'Участники';

  @override
  String get showAllFiles => 'Show All Files';

  @override
  String get searchUser => 'Поиск пользователя...';

  @override
  String get storageUsers => 'Пользователи хранилища';

  @override
  String get addStorageUser => 'Добавить пользователя хранилища';

  @override
  String get publicNoteHintText => 'Публичная запись';

  @override
  String get createButtonHint => 'Создать';

  @override
  String get chatEmptyState => 'Здесь еще нет сообщений';

  @override
  String get securityNumber => 'Номер безопасности';

  @override
  String get securityNumberDialogDesc =>
      'Пожалуйста, убедитесь, что пользователь, которого вы пытаетесь подтвердить, видит тот же номер безопасности на своем экране при попытке подтверждения.';

  @override
  String get groupStateEventCreated => 'Группа создана';

  @override
  String get groupStateEventClosed => 'Группа закрыта';

  @override
  String groupEventInvited(String username) {
    return '\"$username\" имеет был приглашен группу';
  }

  @override
  String get aboutBackgroundExecution => 'About the background execution';

  @override
  String get agplLicense => 'AGPL License';

  @override
  String get androidOptions => 'Android options';

  @override
  String get androidPrivacyPolicy => 'Privacy Policy';

  @override
  String get backgroundExecutionDialog1 =>
      'This app uses background execution to receive and send messages when the app is running in the background.';

  @override
  String get backgroundExecutionDialog2 =>
      'On older Android devices, we ask location permissions and background location permission in order to communicate via Bluetooth Low Energy. This is due to a missing separation between bluetooth permissions and location permissions. Only bluetooth is used, the location is not used by the app at all.';

  @override
  String get backgroundExecutionDialog3 =>
      'This is completely optional, and you can disable this behavior at any time through the Android settings.';

  @override
  String get backgroundExecutionDialogConfirmButton => 'I understand';

  @override
  String get createAccountHeading => 'Выберите имя пользователя';

  @override
  String get currentVersion => 'currently installed version:';

  @override
  String get emptyNodeName => 'Без имени';

  @override
  String get fileHistory => 'История файлов';

  @override
  String get forceUpdateConfirmationDialog =>
      'All your existing data will be deleted.';

  @override
  String get forceUpdateCreateAccount => 'Create new account';

  @override
  String get forceUpdateDescription1 =>
      'info: qaul has a new database format. Users of qaul 2.0.0-beta.17 and earlier who wish to keep their existing account need to migrate their data to the new format.';

  @override
  String get forceUpdateDescription2 =>
      'To migrate an existing database, download qaul 2.0.0-beta.18 and run it';

  @override
  String get forceUpdateDescription3 =>
      'If you don\'t wish to keep your existing data base';

  @override
  String get forceUpdateDisclaimer =>
      '(you will lose all your data and accounts)';

  @override
  String get forceUpdateDownloadQaul18 => 'Download qaul 2.0.0-beta.18';

  @override
  String get forceUpdateRequired => 'Upgrade Required';

  @override
  String get groupChatMessageHint => 'Сообщение группового чата';

  @override
  String groupEventInviteAccepted(String username) {
    return '\"$username\" имеет принял приглашение группу';
  }

  @override
  String groupEventJoined(String username) {
    return '\"$username\" имеет присоединился группу';
  }

  @override
  String groupEventLeft(String username) {
    return '\"$username\" имеет вышел группу';
  }

  @override
  String groupEventRemoved(String username) {
    return '\"$username\" имеет был удален из группу';
  }

  @override
  String get inviteUser => 'Пригласить пользователя';

  @override
  String get languageName => 'Русский';

  @override
  String get notifications => 'Уведомления';

  @override
  String get previousVersion => 'formerly installed version:';

  @override
  String get securePrivateMessageHint => 'Защищенное личное сообщение';

  @override
  String get userDocumentation => 'User Documentation';
}
