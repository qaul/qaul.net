import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../widgets/widgets.dart';

class _UsernameNotifier extends Notifier<String?> {
  @override
  String? build() => null;

  void setUsername(String? value) => state = value;
}

class CreateAccountScreen extends HookConsumerWidget {
  CreateAccountScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');

  static const submitButtonKey = ValueKey('SplashScreen.submitButton');

  final _fieldKey = GlobalKey<FormFieldState>();

  static final _log = Logger('CreateAccountScreen');

  static final _usernameProvider =
      NotifierProvider<_UsernameNotifier, String?>(_UsernameNotifier.new);

  final _sendRequestProvider = FutureProvider<bool?>((ref) async {
    final name = ref.watch(_usernameProvider);
    if (name == null) return null;

    final worker = ref.read(qaulWorkerProvider);
    // TODO(brenodt): Decrease logs from config to finer/finest once tested
    _log.config('Starting create account request...');

    for (var i = 0; i < 60; i++) {
      if (i % 10 == 0) {
        _log.config('Attempt ${i ~/ 10} - Send createUserAccount to libqaul');
        await worker.createUserAccount(name);
      }
      await worker.getDefaultUserAccount();
      await Future.delayed(const Duration(milliseconds: 1000));
      final user = ref.read(defaultUserProvider);
      _log.config('\tAttempt $i - Fetch defaultUserAccount yields "$user"');
      if (user != null) return true;
    }
    return false;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nameCtrl = useTextEditingController();

    final loading = useState(false);

    ref.listen(
      _sendRequestProvider,
      (AsyncValue<bool?>? previous, AsyncValue<bool?> data) {
        data.whenData(
          (created) {
            if (created == null) return;
            loading.value = false;
            if (created) {
              Navigator.pushReplacementNamed(context, NavigationHelper.home);
              return;
            }
            showDialog(
              context: context,
              builder: (c) {
                return AlertDialog(
                  title:
                      Text(AppLocalizations.of(context)!.timeoutErrorMessage),
                  content:
                      Text(AppLocalizations.of(context)!.genericErrorMessage),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context),
                      child: Text(AppLocalizations.of(context)!.okDialogButton),
                    ),
                  ],
                );
              },
            );
          },
        );
      },
    );

    final i10n = AppLocalizations.of(context)!;

    return Scaffold(
        resizeToAvoidBottomInset: false,
        body: Stack(
          children: [
            Padding(
              padding:
                  const EdgeInsets.symmetric(horizontal: 40, vertical: 120),
              child: Column(
                children: [
                  const SizedBox(width: double.maxFinite),
                  QaulAvatar.large(),
                  const SizedBox(height: 28),
                  LayoutBuilder(builder: (context, constraints) {
                    return SizedBox(
                      width: constraints.constrainWidth(400),
                      child: TextFormField(
                        key: _fieldKey,
                        controller: nameCtrl,
                        validator: (s) => _validateUserName(context, s),
                        onFieldSubmitted: (_) {
                          _submitUsername(ref, nameCtrl, loading);
                        },
                        decoration: InputDecoration(
                          hintText: i10n.createAccountHeading,
                        ),
                      ),
                    );
                  }),
                  const SizedBox(height: 28),
                  QaulButton(
                    key: submitButtonKey,
                    label: i10n.start,
                    onPressed: () => _submitUsername(ref, nameCtrl, loading),
                  ),
                ],
              ),
            ),
            if (loading.value) ...[
              SizedBox.expand(
                child: IgnorePointer(
                  ignoring: true,
                  child: Container(
                    color: Colors.black54,
                    child: const LoadingIndicator(),
                  ),
                ),
              )
            ],
          ],
        ));
  }

  String? _validateUserName(BuildContext context, String? value) {
    if (value == null || value.isEmpty) {
      return AppLocalizations.of(context)!.fieldRequiredErrorMessage;
    }
    if (value.length < 2) {
      return AppLocalizations.of(context)!.usernameLengthMessage;
    }
    return null;
  }

  void _submitUsername(
    WidgetRef ref,
    TextEditingController nameCtrl,
    ValueNotifier<bool> loading,
  ) {
    var valid = _fieldKey.currentState?.validate() ?? false;
    if (!valid) return;

    loading.value = true;
    ref.read(_usernameProvider.notifier).setUsername(nameCtrl.text);
  }
}
