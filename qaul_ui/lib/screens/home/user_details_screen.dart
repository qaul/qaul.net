import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:qaul_ui/decorators/disabled_state_decorator.dart';
import 'package:qaul_ui/decorators/loading_decorator.dart';
import 'package:qaul_ui/widgets/default_back_button.dart';
import 'package:qaul_ui/widgets/user_avatar.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class UserDetailsScreen extends HookConsumerWidget {
  const UserDetailsScreen({Key? key, required this.user}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loading = useState(false);

    var theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context)!;
    return LoadingDecorator(
      isLoading: loading.value,
      child: Scaffold(
        appBar: AppBar(
          leading: const DefaultBackButton(),
          title: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              Tooltip(
                message: l18ns.newChatTooltip,
                child: SvgPicture.asset(
                  'assets/icons/comment.svg',
                  width: 24,
                  height: 24,
                  color: Theme.of(context).appBarTheme.iconTheme?.color ??
                      Theme.of(context).iconTheme.color,
                ),
              ),
            ],
          ),
        ),
        body: SizedBox.expand(
          child: Theme(
            data: Theme.of(context).copyWith(
              elevatedButtonTheme: ElevatedButtonThemeData(
                style: ElevatedButton.styleFrom(
                  fixedSize: Size(MediaQuery.of(context).size.width * .8, 48),
                  shape: RoundedRectangleBorder(
                    borderRadius: BorderRadius.circular(24.0),
                  ),
                  textStyle: theme.headline6,
                  onSurface: Colors.white,
                ),
              ),
            ),
            child: Builder(builder: (context) {
              return SingleChildScrollView(
                child: Padding(
                  padding: const EdgeInsets.symmetric(horizontal: 16.0, vertical: 32.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      UserAvatar.large(user: user),
                      const SizedBox(height: 28.0),
                      Text(user.name, style: theme.headline3),
                      const SizedBox(height: 8.0),
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 20.0),
                        child: Text('${l18ns.userID}: ${user.idBase58}', style: theme.headline5),
                      ),
                      const SizedBox(height: 40.0),
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 20.0),
                        child:
                            Text('${l18ns.publicKey}:\n${user.keyBase58}', style: theme.headline5),
                      ),
                      const SizedBox(height: 40.0),
                      DisabledStateDecorator(
                        isDisabled: blocked,
                        child: _RoundedRectButton(
                          color: verified ? Colors.green.shade300 : Colors.green,
                          onPressed: blocked
                              ? null
                              : () async {
                                  final res = await _confirmAction(
                                    context,
                                    description: verified
                                        ? l18ns.unverifyUserConfirmationMessage
                                        : l18ns.verifyUserConfirmationMessage,
                                  );

                                  if (res is! bool || !res) return;
                                  loading.value = true;

                                  final worker = ref.read(qaulWorkerProvider);
                                  verified
                                      ? await worker.unverifyUser(user)
                                      : await worker.verifyUser(user);

                                  loading.value = false;
                                  Navigator.pop(context);
                                },
                          child: Row(
                            mainAxisAlignment: MainAxisAlignment.center,
                            children: [
                              if (!verified) ...[
                                const Icon(Icons.check, size: 32),
                                const SizedBox(width: 4),
                              ],
                              Text(verified ? l18ns.unverify : l18ns.verify),
                            ],
                          ),
                        ),
                      ),
                      const SizedBox(height: 28.0),
                      _RoundedRectButton(
                        color: blocked ? Colors.red.shade300 : Colors.red.shade400,
                        onPressed: () async {
                          final res = await _confirmAction(
                            context,
                            description: blocked
                                ? l18ns.unblockUserConfirmationMessage
                                : l18ns.blockUserConfirmationMessage,
                          );

                          if (res is! bool || !res) return;
                          loading.value = true;

                          final worker = ref.read(qaulWorkerProvider);
                          blocked ? await worker.unblockUser(user) : await worker.blockUser(user);

                          loading.value = false;
                          Navigator.pop(context);
                        },
                        child: Text(blocked ? l18ns.unblockUser : l18ns.blockUser),
                      ),
                    ],
                  ),
                ),
              );
            }),
          ),
        ),
      ),
    );
  }

  bool get verified => (user.isVerified ?? false);

  bool get blocked => (user.isBlocked ?? false);

  Future<bool?> _confirmAction(
    BuildContext context, {
    required String description,
  }) async {
    void pop({bool res = false}) => Navigator.pop(context, res);

    return await showDialog(
      context: context,
      builder: (c) {
        final l18ns = AppLocalizations.of(context);
        return AlertDialog(
          shape: RoundedRectangleBorder(
            borderRadius: BorderRadius.circular(20.0),
          ),
          title: Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              IconButton(icon: const Icon(Icons.close), onPressed: pop),
            ],
          ),
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                description,
                style: Theme.of(context).textTheme.subtitle1,
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              _RoundedRectButton(
                color: Colors.lightBlue,
                size: const Size(280, 80),
                onPressed: () => pop(res: true),
                child: Text(l18ns!.okDialogButton),
              ),
              const SizedBox(height: 12),
              _RoundedRectButton(
                color: Colors.lightBlue,
                size: const Size(280, 80),
                onPressed: pop,
                child: Text(l18ns.cancelDialogButton),
              ),
            ],
          ),
        );
      },
    );
  }
}

class _RoundedRectButton extends StatelessWidget {
  const _RoundedRectButton({
    Key? key,
    required this.color,
    required this.onPressed,
    required this.child,
    this.size,
  }) : super(key: key);
  final Color color;
  final VoidCallback? onPressed;
  final Widget child;
  final Size? size;

  @override
  Widget build(BuildContext context) {
    return ConstrainedBox(
      constraints: const BoxConstraints(maxWidth: 500),
      child: ElevatedButton(
        onPressed: onPressed,
        style: Theme.of(context).elevatedButtonTheme.style!.copyWith(
              foregroundColor: MaterialStateProperty.all(Colors.white),
              backgroundColor: MaterialStateProperty.all(color),
              maximumSize: MaterialStateProperty.all(size),
            ),
        child: child,
      ),
    );
  }
}
