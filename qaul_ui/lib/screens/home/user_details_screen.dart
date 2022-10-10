// ignore_for_file: use_build_context_synchronously
import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intersperse/intersperse.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../../decorators/disabled_state_decorator.dart';
import '../../decorators/loading_decorator.dart';
import '../../widgets/widgets.dart';
import 'tabs/chat/widgets/chat.dart';

class UserDetailsScreen extends HookConsumerWidget {
  const UserDetailsScreen({
    Key? key,
    required this.user,
    this.showChatButton = true,
  }) : super(key: key);
  final User user;
  final bool showChatButton;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final loading = useState(false);
    final isMounted = useIsMounted();

    var theme = Theme.of(context).textTheme;
    final l18ns = AppLocalizations.of(context)!;
    return LoadingDecorator(
      isLoading: loading.value,
      child: Scaffold(
        appBar: AppBar(
          leading: const IconButtonFactory(),
          title: !showChatButton ? null : Row(
            mainAxisAlignment: MainAxisAlignment.end,
            children: [
              Tooltip(
                message: l18ns.newChatTooltip,
                child: IconButton(
                  splashRadius: 26,
                  onPressed: () {
                    final defaultUser = ref.watch(defaultUserProvider)!;
                    final newRoom = ChatRoom.blank(otherUser: user);
                    Navigator.pop(context);
                    openChat(
                      newRoom,
                      ref: ref,
                      context: context,
                      user: defaultUser,
                      otherUser: user,
                    );
                  },
                  icon: SvgPicture.asset(
                    'assets/icons/comment.svg',
                    width: 24,
                    height: 24,
                    color: Theme.of(context).appBarTheme.iconTheme?.color ??
                        Theme.of(context).iconTheme.color,
                  ),
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
                  // disabledForegroundColor: Colors.white.withOpacity(0.38),
                  // disabledBackgroundColor: Colors.white.withOpacity(0.12),
                  surfaceTintColor: Colors.white.withOpacity(0.38),
                  shadowColor: Colors.white.withOpacity(0.12),
                  textStyle: theme.headline6,
                ),
              ),
            ),
            child: Builder(builder: (context) {
              return SingleChildScrollView(
                child: Padding(
                  padding: const EdgeInsets.symmetric(
                      horizontal: 16.0, vertical: 32.0),
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.center,
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      QaulAvatar.large(user: user),
                      const SizedBox(height: 28.0),
                      Text(user.name, style: theme.headline3),
                      const SizedBox(height: 8.0),
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 20.0),
                        child: Text('${l18ns.userID}: ${user.idBase58}',
                            style: theme.headline5),
                      ),
                      const SizedBox(height: 40.0),
                      Padding(
                        padding: const EdgeInsets.symmetric(horizontal: 20.0),
                        child: Text('${l18ns.publicKey}:\n${user.keyBase58}',
                            style: theme.headline5),
                      ),
                      const SizedBox(height: 40.0),
                      DisabledStateDecorator(
                        isDisabled: blocked,
                        child: _RoundedRectButton(
                          color:
                              verified ? Colors.green.shade300 : Colors.green,
                          onPressed: blocked
                              ? null
                              : () async {
                                  loading.value = true;

                                  final worker = ref.read(qaulWorkerProvider);
                                  verified
                                      ? await worker.unverifyUser(user)
                                      : await _verifyUser(context, user: user);

                                  loading.value = false;
                                  if (!isMounted()) return;
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
                        color:
                            blocked ? Colors.red.shade300 : Colors.red.shade400,
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
                          blocked
                              ? await worker.unblockUser(user)
                              : await worker.blockUser(user);

                          loading.value = false;
                          if (!isMounted()) return;
                          Navigator.pop(context);
                        },
                        child:
                            Text(blocked ? l18ns.unblockUser : l18ns.blockUser),
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

Future<void> _verifyUser(BuildContext context, {required User user}) async {
  return await showDialog(
    context: context,
    builder: (c) => _VerifyUserDialog(user),
  );
}

class _VerifyUserDialog extends HookConsumerWidget {
  const _VerifyUserDialog(this.user, {Key? key}) : super(key: key);
  final User user;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final securityNo = ref.watch(currentSecurityNoProvider);

    final isLoading = useState(false);

    final fetchSecurityNo = useCallback(
      () async {
        isLoading.value = true;
        final worker = ref.read(qaulWorkerProvider);
        for (var i = 0; i < 60; i++) {
          if (i % 10 == 0) worker.getUserSecurityNumber(user);

          await worker.getDefaultUserAccount();
          await Future.delayed(const Duration(milliseconds: 1000));
          final no = ref.read(currentSecurityNoProvider);
          if (no != null) break;
        }
        isLoading.value = false;
      },
      [],
    );

    useEffect(() {
      if (!(securityNo?.userId.equals(user.id) ?? false)) fetchSecurityNo();
      return () {};
    }, []);

    final l10n = AppLocalizations.of(context)!;

    return LoadingDecorator(
      isLoading: isLoading.value,
      child: AlertDialog(
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(20.0),
        ),
        title: Row(
          mainAxisAlignment: MainAxisAlignment.end,
          children: [
            IconButton(
              icon: const Icon(Icons.close),
              onPressed: () => Navigator.pop(context),
            ),
          ],
        ),
        content: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 60.0),
          child: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                l10n.securityNumberDialogDesc,
                style: Theme.of(context).textTheme.subtitle1,
                textAlign: TextAlign.center,
              ),
              const SizedBox(height: 24),
              if (securityNo != null && securityNo.userId.equals(user.id)) ...[
                _SecurityNumberDisplay(securityNo: securityNo),
              ],
              const SizedBox(height: 24),
              _RoundedRectButton(
                color: Colors.lightBlue,
                size: const Size(280, 80),
                onPressed: () {
                  final worker = ref.read(qaulWorkerProvider);
                  worker.verifyUser(user);
                  Navigator.pop(context);
                },
                child: Text(l10n.okDialogButton),
              ),
              const SizedBox(height: 12),
              _RoundedRectButton(
                color: Colors.red.shade400,
                size: const Size(280, 80),
                onPressed: () => Navigator.pop(context),
                child: Text(l10n.cancelDialogButton),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _SecurityNumberDisplay extends StatelessWidget {
  const _SecurityNumberDisplay({
    Key? key,
    required this.securityNo,
  }) : super(key: key);

  final SecurityNumber securityNo;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return Column(
      children: [
        Text(
          l10n.securityNumber,
          style: Theme.of(context).textTheme.titleLarge,
        ),
        const SizedBox(height: 8),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: _buildSecurityCodeRow(0)
              .intersperse(const SizedBox(width: 16))
              .toList(),
        ),
        const SizedBox(height: 8),
        Row(
          mainAxisAlignment: MainAxisAlignment.center,
          children: _buildSecurityCodeRow(1)
              .intersperse(const SizedBox(width: 16))
              .toList(),
        ),
      ],
    );
  }

  List<Widget> _buildSecurityCodeRow(int row) {
    return List<Widget>.generate(
      4,
      (index) => Container(
        padding: const EdgeInsets.all(2),
        decoration: BoxDecoration(
          border: Border.all(color: Colors.grey.withOpacity(.5)),
        ),
        child: Text(securityNo.securityCode[(row * 4) + index]),
      ),
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
