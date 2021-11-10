import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:qaul_ui/helpers/navigation_helper.dart';
import 'package:qaul_ui/widgets/language_select_dropdown.dart';

class CreateAccountScreen extends HookConsumerWidget {
  CreateAccountScreen({Key? key}) : super(key: key);

  final _fieldKey = GlobalKey<FormFieldState>();

  static final _usernameProvider = StateProvider<String?>((ref) => null);

  final _sendRequestProvider = FutureProvider<bool?>((ref) async {
    final name = ref.watch(_usernameProvider).state;
    if (name == null) return null;

    final rpcUserAccounts = RpcUserAccounts(ref.read);
    await rpcUserAccounts.createUserAccount(name);

    await Future.delayed(const Duration(seconds: 3));
    await rpcUserAccounts.getDefaultUserAccount();
    await Future.delayed(const Duration(seconds: 3));

    final libqaul = ref.read(libqaulProvider);
    // DEBUG: how many messages are queued by libqaul
    final queued = await libqaul.checkReceiveQueue();
    // check for rpc messages
    if (queued > 0) await libqaul.receiveRpc();

    await Future.delayed(const Duration(seconds: 3));
    final user = ref.read(defaultUserProvider).state;
    return user != null;
  });

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final nameCtrl = useTextEditingController();

    final loading = useState(false);

    ref.listen(
      _sendRequestProvider,
      (AsyncValue<bool?> data) {
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
                  title: const Text('Timeout'),
                  content: const Text('An error occurred'),
                  actions: [
                    TextButton(
                      onPressed: () => Navigator.pop(context),
                      child: const Text('OK'),
                    ),
                  ],
                );
              },
            );
          },
        );
      },
    );

    return Scaffold(
        resizeToAvoidBottomInset: false,
        body: Stack(
          children: [
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 28.0),
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  const Padding(
                    padding: EdgeInsets.all(16.0),
                    child: LanguageSelectDropDown(),
                  ),
                  const SizedBox(height: 24.0),
                  Text(
                    'Choose a\nuser name',
                    style: Theme.of(context).textTheme.headline4,
                    textAlign: TextAlign.center,
                  ),
                  const SizedBox(height: 12.0),
                  TextFormField(
                    key: _fieldKey,
                    controller: nameCtrl,
                    validator: _validateUserName,
                    decoration: const InputDecoration(
                      border: OutlineInputBorder(),
                    ),
                  ),
                  const SizedBox(height: 12.0),
                  Material(
                      type: MaterialType.transparency,
                      child: Ink(
                        decoration: BoxDecoration(
                          border: Border.all(color: Colors.black38),
                          shape: BoxShape.circle,
                        ),
                        child: InkWell(
                          borderRadius: BorderRadius.circular(80.0),
                          onTap: () {
                            var valid = _fieldKey.currentState?.validate();
                            if (!(valid ?? false)) return;

                            loading.value = true;
                            ref.read(_usernameProvider).state = nameCtrl.text;
                          },
                          child: const Padding(
                            padding: EdgeInsets.all(20.0),
                            child: Icon(Icons.arrow_forward_ios_rounded),
                          ),
                        ),
                      )),
                ],
              ),
            ),
            if (loading.value) ...[
              SizedBox.expand(
                child: IgnorePointer(
                  ignoring: true,
                  child: Container(
                    color: Colors.black54,
                    child: const Center(child: CircularProgressIndicator()),
                  ),
                ),
              )
            ],
          ],
        ));
  }

  String? _validateUserName(String? value) {
    if (value == null || value.isEmpty) return 'Field required.';
    return null;
  }
}
