import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../decorators/loading_decorator.dart';
import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../widgets/widgets.dart';

class CreateAccountScreen extends HookConsumerWidget {
  CreateAccountScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');

  static const submitButtonKey = ValueKey('SplashScreen.submitButton');

  final _fieldKey = GlobalKey<FormFieldState>();

  static final _log = Logger('CreateAccountScreen');

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nameCtrl = useTextEditingController();

    final loading = useState(false);

    final i10n = AppLocalizations.of(context)!;

    return Scaffold(
      resizeToAvoidBottomInset: false,
      body: SizedBox.expand(
        child: LoadingDecorator(
          isLoading: loading.value,
          backgroundColor: Colors.black54,
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 120),
            child: Column(
              children: [
                const SizedBox(width: double.maxFinite),
                QaulAvatar.large(),
                const SizedBox(height: 28),
                LayoutBuilder(
                  builder: (context, constraints) {
                    return SizedBox(
                      width: constraints.constrainWidth(400),
                      child: TextFormField(
                        key: _fieldKey,
                        controller: nameCtrl,
                        validator: (s) => _validateUserName(context, s),
                        onFieldSubmitted: (_) {
                          _submitUsername(context, ref, nameCtrl, loading);
                        },
                        decoration: InputDecoration(
                          hintText: i10n.createAccountHeading,
                        ),
                      ),
                    );
                  },
                ),
                const SizedBox(height: 28),
                QaulButton(
                  key: submitButtonKey,
                  label: i10n.start,
                  onPressed: () =>
                      _submitUsername(context, ref, nameCtrl, loading),
                ),
              ],
            ),
          ),
        ),
      ),
    );
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

  Future<void> _submitUsername(
    BuildContext context,
    WidgetRef ref,
    TextEditingController nameCtrl,
    ValueNotifier<bool> loading,
  ) async {
    if (loading.value) return;

    var valid = _fieldKey.currentState?.validate() ?? false;
    if (!valid) return;

    loading.value = true;
    // TODO(brenodt): Decrease logs from config to finer/finest once tested
    _log.config('Starting create account request...');

    final createdUser = await ref
        .read(qaulWorkerProvider)
        .createUserAccount(nameCtrl.text);
    _log.config('Create user account result: "$createdUser"');

    if (!context.mounted) return;
    loading.value = false;

    if (createdUser != null) {
      Navigator.pushReplacementNamed(context, NavigationHelper.home);
      return;
    }

    showDialog(
      context: context,
      builder: (c) {
        return AlertDialog(
          title: Text(AppLocalizations.of(context)!.timeoutErrorMessage),
          content: Text(AppLocalizations.of(context)!.genericErrorMessage),
          actions: [
            TextButton(
              onPressed: () => Navigator.pop(context),
              child: Text(AppLocalizations.of(context)!.okDialogButton),
            ),
          ],
        );
      },
    );
  }
}
