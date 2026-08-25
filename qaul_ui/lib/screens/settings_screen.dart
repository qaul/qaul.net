import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';
import 'package:utils/utils.dart';

import '../coordinators/account_management_coordinator.dart';
import '../decorators/cron_task_decorator.dart';
import '../dialogs/android_background_execution_dialog.dart';
import '../helpers/user_prefs_helper.dart';
import '../l10n/app_localizations.dart';
import '../widgets/widgets.dart';

const _kSettingsIconSize = 25.0;

class _SettingsPngIcon extends StatelessWidget {
  const _SettingsPngIcon(
    this.assetName, {
    this.width = _kSettingsIconSize,
    this.height = _kSettingsIconSize,
  });

  final String assetName;
  final double width;
  final double height;

  @override
  Widget build(BuildContext context) {
    return Image.asset(
      assetName,
      width: width,
      height: height,
      color: IconTheme.of(context).color ?? kQaulSettingsTextColor,
      colorBlendMode: BlendMode.srcIn,
    );
  }
}

class SettingsScreen extends HookConsumerWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context)!;
    final user = ref.watch(defaultUserProvider);

    return ResponsiveScaffold(
      title: l10n.settings,
      titleIcon: const _SettingsPngIcon(
        'assets/icons/settings/settings_cog.png',
        width: 19,
        height: 20,
      ),
      backgroundColor: qaulSettingsBackgroundColor(context),
      bodyAlignment: Alignment.topCenter,
      scrollHorizontalPadding: 0,
      scrollTopPadding: 0,
      wrapWithScrollable: true,
      body: Column(
        children: [
          ValueListenableBuilder<Locale?>(
            valueListenable: UserPrefsHelper.instance.localeNotifier,
            builder: (context, locale, _) => QaulSettingsMenuItem(
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_language.png',
              ),
              title: l10n.language,
              value: locale == null
                  ? _systemDefaultLabel(l10n)
                  : lookupAppLocalizations(locale).languageName,
              onTap: () => _pushSettingsDetail(
                context,
                icon: const _SettingsPngIcon(
                  'assets/icons/settings/settings_language.png',
                ),
                title: l10n.language,
                child: const SettingsLanguageList(),
              ),
            ),
          ),
          ValueListenableBuilder<ThemeMode>(
            valueListenable: UserPrefsHelper.instance.themeModeNotifier,
            builder: (context, themeMode, _) => QaulSettingsMenuItem(
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_theme.png',
              ),
              title: l10n.theme,
              value: _themeLabel(l10n, themeMode),
              onTap: () => _pushSettingsDetail(
                context,
                icon: const _SettingsPngIcon(
                  'assets/icons/settings/settings_theme.png',
                ),
                title: l10n.theme,
                child: const _ThemeSettingsList(),
              ),
            ),
          ),
          QaulSettingsMenuItem(
            icon: const _SettingsPngIcon(
              'assets/icons/settings/settings_notificatons.png',
              width: 23,
              height: 27,
            ),
            title: l10n.notifications,
            onTap: () => _pushSettingsDetail(
              context,
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_notificatons.png',
                width: 23,
                height: 27,
              ),
              title: l10n.notifications,
              child: const Padding(
                padding: kQaulSettingsContentPadding,
                child: _NotificationOptions(),
              ),
            ),
          ),
          QaulSettingsMenuItem(
            icon: const _SettingsPngIcon(
              'assets/icons/settings/settings_network.png',
              width: 22,
              height: 20,
            ),
            title: l10n.network,
            onTap: () => _pushSettingsDetail(
              context,
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_network.png',
                width: 22,
                height: 20,
              ),
              title: l10n.network,
              child: const _InternetNodesList(),
            ),
          ),
          QaulSettingsMenuItem(
            icon: const _SettingsPngIcon(
              'assets/icons/settings/settings_usr.png',
            ),
            title: 'Account Management',
            enabled: user != null,
            onTap: () => _pushSettingsDetail(
              context,
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_usr.png',
              ),
              title: 'Account Management',
              child: user == null
                  ? const Padding(
                      padding: kQaulSettingsContentPadding,
                      child: _SettingsPlaceholder(label: 'Account Management'),
                    )
                  : Padding(
                      padding: kQaulSettingsContentPadding,
                      child: QaulAccountSettingsSection(
                        showHeader: false,
                        showPasswordAction: false,
                        onExportAccount: () =>
                            AccountManagementCoordinator.showExportFlow(
                              context,
                              ref,
                            ),
                        onLogout: () =>
                            AccountManagementCoordinator.logout(context, ref),
                        onDeleteAccount: () =>
                            AccountManagementCoordinator.showDeleteFlow(
                              context,
                              ref,
                            ),
                      ),
                    ),
            ),
          ),
          if (Platform.isAndroid)
            QaulSettingsMenuItem(
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_info_privacy.png',
              ),
              title: 'Enhanced Privacy',
              onTap: () => _pushSettingsDetail(
                context,
                icon: const _SettingsPngIcon(
                  'assets/icons/settings/settings_info_privacy.png',
                ),
                title: 'Enhanced Privacy',
                child: const Padding(
                  padding: kQaulSettingsContentPadding,
                  child: _EnhancedPrivacyOptions(),
                ),
              ),
            ),
          if (Platform.isAndroid)
            QaulSettingsMenuItem(
              icon: const _SettingsPngIcon(
                'assets/icons/settings/settings_info_privacy.png',
              ),
              title: l10n.aboutBackgroundExecution,
              onTap: () => _pushSettingsDetail(
                context,
                icon: const _SettingsPngIcon(
                  'assets/icons/settings/settings_info_privacy.png',
                ),
                title: l10n.aboutBackgroundExecution,
                child: const Padding(
                  padding: kQaulSettingsContentPadding,
                  child: _AndroidOptions(),
                ),
              ),
            ),
        ],
      ),
    );
  }
}

void _pushSettingsDetail(
  BuildContext context, {
  required Widget icon,
  required String title,
  required Widget child,
}) {
  Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => _SettingsDetailScreen(
        icon: icon,
        title: title,
        child: child,
      ),
    ),
  );
}

String _systemDefaultLabel(AppLocalizations l10n) =>
    l10n.useSystemDefaultMessage
        .replaceFirst('Use ', '')
        .replaceFirst('system', 'System');

String _themeLabel(AppLocalizations l10n, ThemeMode mode) {
  switch (mode) {
    case ThemeMode.light:
      return l10n.lightTheme.replaceFirst('Theme', 'mode');
    case ThemeMode.dark:
      return l10n.darkTheme.replaceFirst('Theme', 'mode');
    case ThemeMode.system:
      return _systemDefaultLabel(l10n);
  }
}

class _SettingsDetailScreen extends StatelessWidget {
  const _SettingsDetailScreen({
    required this.icon,
    required this.title,
    required this.child,
  });

  final Widget icon;
  final String title;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ResponsiveScaffold(
      title: title,
      titleIcon: icon,
      backgroundColor: qaulSettingsBackgroundColor(context),
      bodyAlignment: Alignment.topCenter,
      scrollHorizontalPadding: 0,
      scrollTopPadding: 0,
      wrapWithScrollable: true,
      body: child,
    );
  }
}

class SettingsLanguageScreen extends StatelessWidget {
  const SettingsLanguageScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return _SettingsDetailScreen(
      icon: const _SettingsPngIcon(
        'assets/icons/settings/settings_language.png',
      ),
      title: l10n.language,
      child: const SettingsLanguageList(),
    );
  }
}

class SettingsLanguageList extends StatelessWidget {
  const SettingsLanguageList({super.key});

  String _languageName(Locale l) => lookupAppLocalizations(l).languageName;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final items = <Locale?>[null, ...AppLocalizations.supportedLocales];

    return ValueListenableBuilder<Locale?>(
      valueListenable: UserPrefsHelper.instance.localeNotifier,
      builder: (context, currentLocale, _) {
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            for (final locale in items)
              QaulSettingsOptionItem(
                label: locale == null
                    ? _systemDefaultLabel(l10n)
                    : _languageName(locale),
                selected: locale == currentLocale,
                onTap: () => UserPrefsHelper.instance.setDefaultLocale(locale),
              ),
          ],
        );
      },
    );
  }
}

class _ThemeSettingsList extends StatelessWidget {
  const _ThemeSettingsList();

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return ValueListenableBuilder<ThemeMode>(
      valueListenable: UserPrefsHelper.instance.themeModeNotifier,
      builder: (context, currentMode, _) {
        return Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            for (final mode in ThemeMode.values)
              QaulSettingsOptionItem(
                label: _themeLabel(l10n, mode),
                selected: mode == currentMode,
                onTap: () => UserPrefsHelper.instance.setThemeMode(mode),
              ),
          ],
        );
      },
    );
  }
}

class _SettingsPlaceholder extends StatelessWidget {
  const _SettingsPlaceholder({required this.label});

  final String label;

  @override
  Widget build(BuildContext context) {
    return Text(
      label,
      style: Theme.of(context).textTheme.titleSmall?.copyWith(
            color: qaulSettingsItemColor(context),
            fontWeight: FontWeight.w600,
            letterSpacing: 1.8,
          ),
    );
  }
}

class _EnhancedPrivacyOptions extends StatelessWidget {
  const _EnhancedPrivacyOptions();

  @override
  Widget build(BuildContext context) {
    if (!Platform.isAndroid) return const SizedBox.shrink();

    return const _PrivacyPolicyOption();
  }
}

class _PrivacyPolicyOption extends StatelessWidget {
  const _PrivacyPolicyOption();

  static const privacyPolicyURL =
      "https://qaul.net/legal/privacy-policy-android/";

  Future<void> _openPrivacyPolicy() async {
    final uri = Uri.parse(privacyPolicyURL);
    if (!(await canLaunchUrl(uri))) return;
    launchUrl(uri);
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return DefaultTextStyle(
      maxLines: 2,
      style: Theme.of(
        context,
      ).textTheme.labelLarge!.copyWith(overflow: TextOverflow.ellipsis),
      child: InkWell(
        onTap: _openPrivacyPolicy,
        child: Padding(
          padding: const EdgeInsets.symmetric(vertical: 8.0),
          child: Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisAlignment: MainAxisAlignment.start,
            children: [
              const Icon(Icons.policy),
              const SizedBox(width: 8),
              Text(l10n.androidPrivacyPolicy),
            ],
          ),
        ),
      ),
    );
  }
}

class _NotificationOptions extends StatefulWidget {
  const _NotificationOptions();

  @override
  State<_NotificationOptions> createState() => _NotificationOptionsState();
}

class _NotificationOptionsState extends State<_NotificationOptions> {
  bool get _notificationsAreEnabled =>
      UserPrefsHelper.instance.chatNotificationsEnabled ||
      UserPrefsHelper.instance.publicTabNotificationsEnabled;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return DefaultTextStyle(
      maxLines: 2,
      style: Theme.of(
        context,
      ).textTheme.labelLarge!.copyWith(overflow: TextOverflow.ellipsis),
      child: Column(
        children: [
          _buildConfigurationOption(
            label: l10n.publicNotificationsEnabled,
            value: UserPrefsHelper.instance.publicTabNotificationsEnabled,
            onValueChanged: (val) =>
                UserPrefsHelper.instance.setPublicTabNotificationsEnabled(val),
          ),
          const SizedBox(height: 20),
          _buildConfigurationOption(
            label: l10n.chatNotificationsEnabled,
            value: UserPrefsHelper.instance.chatNotificationsEnabled,
            onValueChanged: (val) =>
                UserPrefsHelper.instance.setChatNotificationsEnabled(val),
          ),
          if (_notificationsAreEnabled) ...[
            const SizedBox(height: 20),
            _buildConfigurationOption(
              label: l10n.notifyOnlyForVerifiedUsers,
              value: UserPrefsHelper.instance.notifyOnlyForVerifiedUsers,
              onValueChanged: (val) =>
                  UserPrefsHelper.instance.setNotifyOnlyForVerifiedUsers(val),
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildConfigurationOption({
    required String label,
    required bool value,
    required void Function(bool newValue) onValueChanged,
  }) {
    return Row(
      crossAxisAlignment: CrossAxisAlignment.center,
      mainAxisAlignment: MainAxisAlignment.spaceBetween,
      children: [
        Expanded(child: Text(label)),
        PlatformAwareSwitch(
          value: value,
          onChanged: (val) {
            onValueChanged(val);
            setState(() {});
          },
        ),
      ],
    );
  }
}

class _InternetNodesList extends HookConsumerWidget {
  const _InternetNodesList();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nodes = ref.watch(connectedNodesProvider);
    final isLoading = useState(true);

    final removeNode = useCallback((String nodeAddress) {
      final worker = ref.read(qaulWorkerProvider);
      worker.removeNode(nodeAddress);
    }, []);

    final addNode = useCallback((String nodeAddress, [String? name]) {
      final worker = ref.read(qaulWorkerProvider);
      worker.addNode(nodeAddress, name);
    }, []);

    final setNodeState = useCallback((String address, bool value) {
      final worker = ref.read(qaulWorkerProvider);
      worker.setNodeState(address, active: value);
    }, []);

    final refreshNodes = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      await worker.requestNodes();
    }, []);

    useEffect(() {
      var isDisposed = false;

      Future<void>(() async {
        try {
          await refreshNodes();
        } finally {
          if (!isDisposed) {
            isLoading.value = false;
          }
        }
      });

      return () => isDisposed = true;
    }, const []);

    final textTheme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context);

    if (isLoading.value) {
      return const Padding(
        padding: EdgeInsets.fromLTRB(28, 24, 28, 0),
        child: Align(
          alignment: Alignment.topCenter,
          child: CircularProgressIndicator(),
        ),
      );
    }

    return CronTaskDecorator(
      callback: refreshNodes,
      schedule: const Duration(milliseconds: 1000),
      child: Column(
        children: [
          QaulTable(
            titleIcon: CupertinoIcons.globe,
            title: l10n!.internetNodes,
            showTitle: false,
            contentPadding: kQaulSettingsContentPadding,
            addRowLabel: l10n.addNodeCTA,
            rowCount: nodes.length,
            onAddRowPressed: () async {
              final res = await showDialog(
                context: context,
                builder: (_) => const _AddNodeDialog(),
              );

              if (res is! _AddNodeDialogResponse) return;

              addNode(res.address, res.name);
            },
            rowBuilder: (context, i) {
              var node = nodes[i];
              var nodeAddr = node.address;

              return ListTile(
                contentPadding: const EdgeInsets.all(4.0),
                title: Text(
                  node.name.isNotEmpty && node.name != 'undefined'
                      ? node.name
                      : l10n.emptyNodeName,
                  style: textTheme.titleMedium,
                ),
                subtitle: Text(nodeAddr, style: textTheme.titleSmall),
                trailing: Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    PlatformAwareSwitch(
                      value: node.isActive,
                      onChanged: (val) => setNodeState(nodeAddr, val),
                    ),
                    IconButton(
                      splashRadius: 24,
                      iconSize: 20,
                      icon: const Icon(CupertinoIcons.delete),
                      onPressed: () async => removeNode(nodeAddr),
                    ),
                  ],
                ),
                onTap: () async {
                  final res = await showDialog(
                    context: context,
                    builder: (_) => _AddNodeDialog(
                      ip: node.ip,
                      port: node.port,
                      name: node.name,
                      isIPv4: node.isIPv4,
                      usesQuic: node.isQuic,
                    ),
                  );

                  if (res is! _AddNodeDialogResponse) return;
                  removeNode(nodeAddr);
                  addNode(res.address, res.name);
                },
              );
            },
          ),
        ],
      ),
    );
  }
}

class _AddNodeDialogResponse {
  final String address;
  final String name;

  _AddNodeDialogResponse({required this.address, required this.name});
}

class _AddNodeDialog extends HookWidget {
  const _AddNodeDialog({
    this.name,
    this.ip,
    this.port,
    this.isIPv4 = true,
    this.usesQuic = true,
  });

  final String? name;
  final String? ip;
  final String? port;

  /// If [false], will be considered IPv6
  final bool isIPv4;

  /// If [true], will assume the address uses the quic protocol
  final bool usesQuic;

  _AddNodeDialogResponse _buildIPAddress({
    required String ip,
    required String port,
    required String name,
    required bool useIPv6,
    required bool useQuic,
  }) {
    var address = useIPv6 ? '/ip6/$ip' : '/ip4/$ip';
    if (useQuic) {
      address += '/udp/$port/quic-v1';
    } else {
      address += '/tcp/$port';
    }
    return _AddNodeDialogResponse(address: address, name: name);
  }

  @override
  Widget build(BuildContext context) {
    final ttheme = Theme.of(context).textTheme;

    final nameCtrl = useTextEditingController(text: name);
    final ipCtrl = useTextEditingController(text: ip);
    final portCtrl = useTextEditingController(text: port);

    final l10n = AppLocalizations.of(context)!;
    final orientation = MediaQuery.of(context).orientation;

    final isIPv6 = useState(isIPv4 == false);
    final isQuic = useState(usesQuic);

    final tcpField = useMemoized(
      () => [
        _spacer,
        Text(isQuic.value ? '/udp/' : '/tcp/', style: _fixedTextStyle),
        _spacer,
        Expanded(
          child: TextFormField(
            controller: portCtrl,
            decoration: _decoration('port', hint: '9229'),
            keyboardType: TextInputType.number,
            validator: (val) {
              if (isValidPort(val)) return null;
              return l10n.invalidPortMessage;
            },
          ),
        ),
        if (isQuic.value) ...[
          _spacer,
          Text('/quic-v1', style: _fixedTextStyle),
        ],
      ],
      [portCtrl, isQuic.value],
    );

    return Form(
      child: Builder(
        builder: (context) {
          return AlertDialog(
            title: Text(l10n.addNodeCTA),
            content: SingleChildScrollView(
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  // used to force the dialog to fill the available horizontal space
                  const Row(
                    mainAxisSize: MainAxisSize.max,
                    children: [SizedBox(width: double.maxFinite)],
                  ),

                  TextField(
                    autofocus: true,
                    controller: nameCtrl,
                    decoration: _decoration(l10n.name),
                    keyboardType: TextInputType.name,
                  ),
                  const SizedBox(height: 20),
                  Row(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      Text(
                        isIPv6.value ? '/ip6/' : '/ip4/',
                        style: _fixedTextStyle,
                      ),
                      _spacer,
                      Expanded(
                        child: TextFormField(
                          controller: ipCtrl,
                          inputFormatters: [
                            isIPv6.value
                                ? IPv6TextInputFormatter()
                                : IPv4TextInputFormatter(),
                          ],
                          decoration: _decoration(
                            'ip',
                            hint: isIPv6.value
                                ? '0000:0000:0000:0000:0000:0000:0000:0000'
                                : '000.000.000.000',
                          ),
                          validator: (v) {
                            if (isIPv6.value
                                ? isValidIPv6(v)
                                : isValidIPv4(v)) {
                              return null;
                            }
                            return l10n.invalidIPMessage;
                          },
                          keyboardType: isIPv6.value
                              ? TextInputType.text
                              : const TextInputType.numberWithOptions(
                                  decimal: true,
                                ),
                          enableInteractiveSelection: false,
                        ),
                      ),
                      if (orientation == Orientation.landscape) ...tcpField,
                    ],
                  ),
                  if (orientation == Orientation.portrait) ...[
                    const SizedBox(height: 20),
                    Row(children: tcpField),
                  ],

                  const SizedBox(height: 8),
                  const Divider(),

                  Padding(
                    padding: const EdgeInsets.symmetric(vertical: 8),
                    child: Text(l10n.options, style: ttheme.titleMedium),
                  ),
                  SwitchListTile(
                    value: isIPv6.value,
                    onChanged: (v) => {isIPv6.value = v},
                    title: Text(l10n.useIpv6),
                  ),
                  SwitchListTile(
                    value: isQuic.value,
                    onChanged: (v) => {isQuic.value = v},
                    title: Text(l10n.useQuic),
                  ),
                ],
              ),
            ),
            actions: [
              TextButton(
                child: Text(l10n.okDialogButton),
                onPressed: () {
                  if (Form.of(context).validate() == false) return;
                  Navigator.pop(
                    context,
                    _buildIPAddress(
                      ip: ipCtrl.text,
                      port: portCtrl.text,
                      name: nameCtrl.text,
                      useIPv6: isIPv6.value,
                      useQuic: isQuic.value,
                    ),
                  );
                },
              ),
              TextButton(
                child: Text(l10n.cancelDialogButton),
                onPressed: () => Navigator.pop(context),
              ),
            ],
          );
        },
      ),
    );
  }

  SizedBox get _spacer => const SizedBox(width: 4, height: 4);

  TextStyle get _fixedTextStyle => TextStyle(
    fontSize: 26,
    fontWeight: FontWeight.w500,
    color: Colors.grey.shade500,
  );

  InputDecoration _decoration(String label, {String? hint}) => InputDecoration(
    isDense: true,
    hintText: hint,
    labelText: label,
    border: const OutlineInputBorder(),
    contentPadding: const EdgeInsets.all(12),
    floatingLabelBehavior: FloatingLabelBehavior.always,
  );
}

class _AndroidOptions extends StatefulWidget {
  const _AndroidOptions();

  @override
  State<_AndroidOptions> createState() => _AndroidOptionsState();
}

class _AndroidOptionsState extends State<_AndroidOptions> {
  void _showPrivacyDialog() async {
    showDialog(
      context: context,
      builder: (context) => const AndroidBackgroundExecutionDialog(),
    );
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return DefaultTextStyle(
      maxLines: 2,
      style: Theme.of(
        context,
      ).textTheme.labelLarge!.copyWith(overflow: TextOverflow.ellipsis),
      child: Column(
        children: [
          InkWell(
            onTap: _showPrivacyDialog,
            child: Padding(
              padding: const EdgeInsets.symmetric(vertical: 8.0),
              child: Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.start,
                children: [
                  const Icon(Icons.info),
                  const SizedBox(width: 8),
                  Text(l10n.aboutBackgroundExecution),
                ],
              ),
            ),
          ),
          const SizedBox(height: 40),
        ],
      ),
    );
  }
}
