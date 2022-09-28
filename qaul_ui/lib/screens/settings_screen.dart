import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../decorators/cron_task_decorator.dart';
import '../helpers/user_prefs_helper.dart';
import '../widgets/widgets.dart';

class SettingsScreen extends StatefulHookConsumerWidget {
  const SettingsScreen({Key? key}) : super(key: key);

  @override
  ConsumerState<SettingsScreen> createState() => _SettingsScreenState();
}

class _SettingsScreenState extends ConsumerState<SettingsScreen> {
  @override
  Widget build(BuildContext context) {
    final l18ns = AppLocalizations.of(context)!;
    return Scaffold(
      appBar: AppBar(
        leading: const IconButtonFactory(),
        title: Row(
          children: [
            const Icon(Icons.settings),
            const SizedBox(width: 8),
            Text(l18ns.settings),
          ],
        ),
      ),
      body: SingleChildScrollView(
        child: Padding(
          padding:
              MediaQuery.of(context).viewPadding.copyWith(left: 20, right: 20),
          child: Column(
            children: [
              const LanguageSelectDropDown(),
              const SizedBox(height: 20),
              const ThemeSelectDropdown(),
              const SizedBox(height: 20),
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(l18ns.publicNotificationsEnabled),
                  PlatformAwareSwitch(
                    value: UserPrefsHelper().publicTabNotificationsEnabled,
                    onChanged: (val) {
                      UserPrefsHelper().publicTabNotificationsEnabled = val;
                      setState(() {});
                    },
                  ),
                ],
              ),
              const SizedBox(height: 20),
              Row(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.spaceBetween,
                children: [
                  Text(l18ns.chatNotificationsEnabled),
                  PlatformAwareSwitch(
                    value: UserPrefsHelper().chatNotificationsEnabled,
                    onChanged: (val) {
                      UserPrefsHelper().chatNotificationsEnabled = val;
                      setState(() {});
                    },
                  ),
                ],
              ),
              if (_notificationsAreEnabled) ...[
                const SizedBox(height: 20),
                Row(
                  crossAxisAlignment: CrossAxisAlignment.center,
                  mainAxisAlignment: MainAxisAlignment.spaceBetween,
                  children: [
                    Text(l18ns.notifyOnlyForVerifiedUsers),
                    PlatformAwareSwitch(
                      value: UserPrefsHelper().notifyOnlyForVerifiedUsers,
                      onChanged: (val) {
                        UserPrefsHelper().notifyOnlyForVerifiedUsers = val;
                        setState(() {});
                      },
                    ),
                  ],
                ),
              ],
              const Divider(),
              const SizedBox(height: 80),
              const _InternetNodesList(),
            ],
          ),
        ),
      ),
    );
  }

  bool get _notificationsAreEnabled =>
      UserPrefsHelper().chatNotificationsEnabled ||
      UserPrefsHelper().publicTabNotificationsEnabled;
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

    final addNode = useCallback((String nodeAddress) {
      final worker = ref.read(qaulWorkerProvider);
      worker.addNode(nodeAddress);
    }, []);

    final setNodeState = useCallback((String address, bool value) {
      final worker = ref.read(qaulWorkerProvider);
      worker.setNodeState(address, active: value);
    }, []);

    final refreshNodes = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      await worker.requestNodes();
    }, []);

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

              if (res is! String) return;

              addNode(res);
            },
            rowBuilder: (context, i) {
              var node = nodes[i];
              var nodeAddr = node.address;
              return ListTile(
                contentPadding: const EdgeInsets.all(4.0),
                title: Text(
                  nodeAddr,
                  style: Theme.of(context).textTheme.subtitle2,
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
                  String? res;
                  if (nodeAddr.contains('/ip4/')) {
                    final ip =
                        nodeAddr.replaceAll('/ip4/', '').split('/').first;
                    final port = nodeAddr.split('/').last;
                    res = await showDialog(
                      context: context,
                      builder: (_) => _AddNodeDialog(ip: ip, port: port),
                    );
                  }
                  if (nodeAddr.contains('/ip6/')) {
                    final ip =
                        nodeAddr.replaceAll('/ip6/', '').split('/').first;
                    final port = nodeAddr.split('/').last;
                    res = await showDialog(
                      context: context,
                      builder: (_) => _AddIPv6NodeDialog(ip: ip, port: port),
                    );
                  }

                  if (res is! String) return;
                  removeNode(nodeAddr);
                  addNode(res);
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
                      context: context, builder: (_) => _AddIPv6NodeDialog());

                  if (res is! String) return;

                  addNode(res);
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

class _AddNodeDialog extends HookWidget {
  _AddNodeDialog({
    Key? key,
    this.ip,
    this.port,
  }) : super(key: key);

  final String? ip;
  final String? port;

  final _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    final ipCtrl = useTextEditingController(text: ip);
    final portCtrl = useTextEditingController(text: port);

    final l18ns = AppLocalizations.of(context)!;
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
            return l18ns.invalidPortMessage;
          },
        ),
      ),
    ];

    return AlertDialog(
      title:
          orientation == Orientation.landscape ? null : Text(l18ns.addNodeCTA),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('/ip4/', style: _fixedTextStyle),
                _spacer,
                Expanded(
                  child: TextFormField(
                    autofocus: true,
                    controller: ipCtrl,
                    inputFormatters: [IPv4TextInputFormatter()],
                    decoration: _decoration('ip', hint: '000.000.000.000'),
                    validator: (val) {
                      if (isValidIPv4(val)) return null;
                      return l18ns.invalidIPMessage;
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
          child: Text(l18ns.okDialogButton),
          onPressed: () {
            if (!(_formKey.currentState?.validate() ?? false)) return;
            Navigator.pop(context, '/ip4/${ipCtrl.text}/tcp/${portCtrl.text}');
          },
        ),
        TextButton(
          child: Text(l18ns.cancelDialogButton),
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

class _AddIPv6NodeDialog extends HookWidget {
  _AddIPv6NodeDialog({this.ip, this.port});

  final String? ip;
  final String? port;

  final _formKey = GlobalKey<FormState>();

  @override
  Widget build(BuildContext context) {
    final ipCtrl = useTextEditingController(text: ip);
    final portCtrl = useTextEditingController(text: port);

    final l18ns = AppLocalizations.of(context)!;
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
            return l18ns.invalidPortMessage;
          },
        ),
      ),
    ];

    return AlertDialog(
      title:
          orientation == Orientation.landscape ? null : Text(l18ns.addNodeCTA),
      content: Form(
        key: _formKey,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            Row(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Text('/ip6/', style: _fixedTextStyle),
                _spacer,
                Expanded(
                  child: TextFormField(
                    autofocus: true,
                    controller: ipCtrl,
                    inputFormatters: [IPv6TextInputFormatter()],
                    decoration: _decoration(
                      'ip',
                      hint: '0000:0000:0000:0000:0000:0000:0000:0000',
                    ),
                    validator: (val) {
                      if (isValidIPv6(val)) return null;
                      return l18ns.invalidIPMessage;
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
          child: Text(l18ns.okDialogButton),
          onPressed: () {
            if (!(_formKey.currentState?.validate() ?? false)) return;
            Navigator.pop(context, '/ip6/${ipCtrl.text}/tcp/${portCtrl.text}');
          },
        ),
        TextButton(
          child: Text(l18ns.cancelDialogButton),
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
