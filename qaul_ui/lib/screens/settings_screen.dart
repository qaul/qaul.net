import 'dart:io';

import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:font_awesome_flutter/font_awesome_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../decorators/cron_task_decorator.dart';
import '../helpers/navigation_helper.dart';
import '../helpers/user_prefs_helper.dart';
import '../widgets/widgets.dart';

class SettingsScreen extends HookConsumerWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final l10n = AppLocalizations.of(context)!;

    return ResponsiveScaffold(
      title: l10n.settings,
      icon: Icons.settings,
      wrapWithScrollable: true,
      body: Column(
        mainAxisAlignment: MainAxisAlignment.start,
        children: [
          const LanguageSelectDropDown(),
          const SizedBox(height: 20),
          const ThemeSelectDropdown(),
          const SizedBox(height: 20),
          SettingsSection(
            name: l10n.notifications,
            icon: const FaIcon(FontAwesomeIcons.solidBell),
            content: const _NotificationOptions(),
          ),
          const SizedBox(height: 20),
          SettingsSection(
            name: l10n.network,
            icon: const FaIcon(FontAwesomeIcons.networkWired),
            content: const Padding(
              padding: EdgeInsets.only(top: 20),
              child: _InternetNodesList(),
            ),
          ),
          if (Platform.isAndroid) ...[
            const SizedBox(height: 20),
            SettingsSection(
              name: l10n.androidOptions,
              icon: const FaIcon(FontAwesomeIcons.android),
              content: const _AndroidOptions(),
            ),
          ]
        ],
      ),
    );
  }
}

class _NotificationOptions extends StatefulWidget {
  const _NotificationOptions({Key? key}) : super(key: key);

  @override
  State<_NotificationOptions> createState() => _NotificationOptionsState();
}

class _NotificationOptionsState extends State<_NotificationOptions> {
  bool get _notificationsAreEnabled =>
      UserPrefsHelper().chatNotificationsEnabled ||
      UserPrefsHelper().publicTabNotificationsEnabled;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return DefaultTextStyle(
      maxLines: 2,
      style: Theme.of(context)
          .textTheme
          .labelLarge!
          .copyWith(overflow: TextOverflow.ellipsis),
      child: Column(
        children: [
          _buildConfigurationOption(
            label: l10n.publicNotificationsEnabled,
            value: UserPrefsHelper().publicTabNotificationsEnabled,
            onValueChanged: (val) =>
                UserPrefsHelper().publicTabNotificationsEnabled = val,
          ),
          const SizedBox(height: 20),
          _buildConfigurationOption(
            label: l10n.chatNotificationsEnabled,
            value: UserPrefsHelper().chatNotificationsEnabled,
            onValueChanged: (val) =>
                UserPrefsHelper().chatNotificationsEnabled = val,
          ),
          if (_notificationsAreEnabled) ...[
            const SizedBox(height: 20),
            _buildConfigurationOption(
              label: l10n.notifyOnlyForVerifiedUsers,
              value: UserPrefsHelper().notifyOnlyForVerifiedUsers,
              onValueChanged: (val) =>
                  UserPrefsHelper().notifyOnlyForVerifiedUsers = val,
            ),
          ],
        ],
      ),
    );
  }

  Widget _buildConfigurationOption(
      {required String label,
      required bool value,
      required Function(bool newValue) onValueChanged}) {
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

    final textTheme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context);
    return CronTaskDecorator(
      callback: refreshNodes,
      schedule: const Duration(milliseconds: 1000),
      child: Column(
        children: [
          QaulTable(
            titleIcon: CupertinoIcons.globe,
            title: l10n!.internetNodes,
            addRowLabel: l10n.addNodeCTA,
            rowCount: nodes.length,
            onAddRowPressed: () async {
              final res = await showDialog(
                  context: context, builder: (_) => _AddNodeDialog());

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
                subtitle: Text(
                  nodeAddr,
                  style: textTheme.titleSmall,
                ),
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
                    ),
                  );

                  if (res is! _AddNodeDialogResponse) return;
                  removeNode(nodeAddr);
                  addNode(res.address, res.name);
                },
              );
            },
          ),
          Row(
            children: [
              IconButton(
                icon: const Icon(Icons.add),
                splashRadius: 24,
                onPressed: () async {
                  final res = await showDialog(
                    context: context,
                    builder: (_) => _AddNodeDialog(isIPv4: false),
                  );

                  if (res is! _AddNodeDialogResponse) return;
                  addNode(res.address, res.name);
                },
              ),
              const SizedBox(width: 12.0),
              Text(l10n.addIPv6NodeCTA),
            ],
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
  _AddNodeDialog({
    Key? key,
    this.name,
    this.ip,
    this.port,
    this.isIPv4 = true,
  }) : super(key: key);

  final String? name;
  final String? ip;
  final String? port;

  /// If [false], will be considered IPv6
  final bool isIPv4;

  final _formKey = GlobalKey<FormState>();

  String get _descriptor => isIPv4 ? '/ip4/' : '/ip6/';

  TextInputFormatter get _formatter =>
      isIPv4 ? IPv4TextInputFormatter() : IPv6TextInputFormatter();

  String get _hint =>
      isIPv4 ? '000.000.000.000' : '0000:0000:0000:0000:0000:0000:0000:0000';

  bool _isValidIP(String? value) =>
      isIPv4 ? isValidIPv4(value) : isValidIPv6(value);

  _AddNodeDialogResponse _buildIPAddress(
      {required String ip, required String port, required String name}) {
    return _AddNodeDialogResponse(
      address: isIPv4 ? '/ip4/$ip/tcp/$port' : '/ip6/$ip/tcp/$port',
      name: name,
    );
  }

  @override
  Widget build(BuildContext context) {
    final nameCtrl = useTextEditingController(text: name);
    final ipCtrl = useTextEditingController(text: ip);
    final portCtrl = useTextEditingController(text: port);

    final l10n = AppLocalizations.of(context)!;
    var orientation = MediaQuery.of(context).orientation;
    final tcpField = [
      _spacer,
      Text('/tcp/', style: _fixedTextStyle),
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
    ];

    return AlertDialog(
      title:
          orientation == Orientation.landscape ? null : Text(l10n.addNodeCTA),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
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
                Text(_descriptor, style: _fixedTextStyle),
                _spacer,
                Expanded(
                  child: TextFormField(
                    controller: ipCtrl,
                    inputFormatters: [_formatter],
                    decoration: _decoration('ip', hint: _hint),
                    validator: (val) {
                      if (_isValidIP(val)) return null;
                      return l10n.invalidIPMessage;
                    },
                    keyboardType: const TextInputType.numberWithOptions(
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
          ],
        ),
      ),
      actions: [
        TextButton(
          child: Text(l10n.okDialogButton),
          onPressed: () {
            if (!(_formKey.currentState?.validate() ?? false)) return;
            Navigator.pop(
              context,
              _buildIPAddress(
                ip: ipCtrl.text,
                port: portCtrl.text,
                name: nameCtrl.text,
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
  }

  SizedBox get _spacer => const SizedBox(width: 4, height: 4);

  TextStyle get _fixedTextStyle => TextStyle(
      fontSize: 26, fontWeight: FontWeight.w500, color: Colors.grey.shade500);

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
  const _AndroidOptions({Key? key}) : super(key: key);

  @override
  State<_AndroidOptions> createState() => _AndroidOptionsState();
}

class _AndroidOptionsState extends State<_AndroidOptions> {
  bool _isBgExecutionEnabled = true;

  void _enableBackgroundExecution() {
    const MethodChannel('libqaul').invokeMethod('enableBackgroundExecution');
    setState(() => _isBgExecutionEnabled = true);
  }

  void _disableBackgroundExecution() {
    const MethodChannel('libqaul').invokeMethod('disableBackgroundExecution');
    setState(() => _isBgExecutionEnabled = false);
  }

  @override
  void initState() {
    super.initState();
    if (!Platform.isAndroid) {
      throw UnimplementedError('invalid platform making use of AndroidOptions');
    }
    WidgetsBinding.instance.addPostFrameCallback((_) async {
      final isEnabled = await const MethodChannel('libqaul')
          .invokeMethod('isBackgroundExecutionEnabled');
      setState(() => _isBgExecutionEnabled = isEnabled);
    });
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return DefaultTextStyle(
      maxLines: 2,
      style: Theme.of(context)
          .textTheme
          .labelLarge!
          .copyWith(overflow: TextOverflow.ellipsis),
      child: Column(
        children: [
          Row(
            crossAxisAlignment: CrossAxisAlignment.center,
            mainAxisAlignment: MainAxisAlignment.spaceBetween,
            children: [
              Text(l10n.backgroundExecution),
              IconButton(
                iconSize: 18,
                splashRadius: 9,
                icon: const Icon(Icons.help_outline),
                onPressed: () => Navigator.pushNamed(
                  context,
                  NavigationHelper.aboutAndroidBackground,
                ),
              ),
              const Expanded(child: SizedBox.shrink()),
              PlatformAwareSwitch(
                value: _isBgExecutionEnabled,
                onChanged: (val) {
                  val
                      ? _enableBackgroundExecution()
                      : _disableBackgroundExecution();
                },
              ),
            ],
          ),
        ],
      ),
    );
  }
}
