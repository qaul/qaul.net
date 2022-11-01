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
import '../../widgets/user_details_banner.dart';
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
    final isMounted = useIsMounted();

    final l10n = AppLocalizations.of(context)!;

    final onVerifyUserPressed = useCallback(() async {
      final worker = ref.read(qaulWorkerProvider);
      verified
          ? await worker.unverifyUser(user)
          : await _verifyUser(context, user: user);

      if (!isMounted()) return;
      Navigator.pop(context);
    }, [ref, isMounted]);

    final onBlockUserPressed = useCallback(() async {
      final res = await _confirmAction(
        context,
        description: blocked
            ? l10n.unblockUserConfirmationMessage
            : l10n.blockUserConfirmationMessage,
      );

      if (res is! bool || !res) return;

      final worker = ref.read(qaulWorkerProvider);
      blocked ? await worker.unblockUser(user) : await worker.blockUser(user);
      if (!isMounted()) return;
      Navigator.pop(context);
    }, [l10n, ref, isMounted]);

    return Scaffold(
        appBar: AppBar(
          leading: const IconButtonFactory(),
          title: !showChatButton
              ? null
              : Row(
                  mainAxisAlignment: MainAxisAlignment.end,
                  children: [
                    Tooltip(
                      message: l10n.newChatTooltip,
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
                          color:
                              Theme.of(context).appBarTheme.iconTheme?.color ??
                                  Theme.of(context).iconTheme.color,
                        ),
                      ),
                    ),
                  ],
                ),
        ),
        body: ListView(
          padding: MediaQuery.of(context)
              .viewPadding
              .add(const EdgeInsets.fromLTRB(16, 32, 16, 8)),
          children: [
            UserDetailsHeading(user),
            Row(
              children: [
                Expanded(
                  child: DisabledStateDecorator(
                    isDisabled: blocked,
                    child: QaulButton(
                      onPressed: blocked ? null : onVerifyUserPressed,
                      label: verified ? l10n.unverify : l10n.verify,
                    ),
                  ),
                ),
                const SizedBox(width: 20),
                Expanded(
                  child: QaulButton(
                    onPressed: onBlockUserPressed,
                    label: blocked ? l10n.unblockUser : l10n.blockUser,
                  ),
                ),
              ],
            )
          ],
        ));
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
              QaulButton(
                onPressed: () => pop(res: true),
                label: l18ns!.okDialogButton,
              ),
              const SizedBox(height: 12),
              QaulButton(
                onPressed: pop,
                label: l18ns.cancelDialogButton,
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
              QaulButton(
                onPressed: () {
                  final worker = ref.read(qaulWorkerProvider);
                  worker.verifyUser(user);
                  Navigator.pop(context);
                },
                label: l10n.okDialogButton,
              ),
              const SizedBox(height: 12),
              QaulButton(
                onPressed: () => Navigator.pop(context),
                label: l10n.cancelDialogButton,
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
