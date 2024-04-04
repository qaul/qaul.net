// ignore_for_file: use_build_context_synchronously
import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intersperse/intersperse.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../../decorators/disabled_state_decorator.dart';
import '../../decorators/loading_decorator.dart';
import '../../widgets/qaul_dialog.dart';
import '../../widgets/user_details_banner.dart';
import '../../widgets/widgets.dart';
import 'tabs/chat/widgets/chat.dart';

class UserDetailsScreen extends HookConsumerWidget {
  const UserDetailsScreen({
    Key? key,
    required this.user,
  }) : super(key: key);
  final User user;

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
        appBar: AppBar(leading: const IconButtonFactory()),
        body: ListView(
          padding: MediaQuery.of(context)
              .viewPadding
              .add(const EdgeInsets.fromLTRB(16, 32, 16, 8)),
          children: [
            UserDetailsHeading(user),
            ResponsiveLayout(
              mobileBody: Column(
                children: [
                  SizedBox(
                    width: 280,
                    child: QaulButton(
                      label: l10n.newChatTooltip,
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
                    ),
                  ),
                  const SizedBox(height: 20),
                  SizedBox(
                    width: 280,
                    child: DisabledStateDecorator(
                      isDisabled: blocked,
                      child: QaulButton(
                        onPressed: blocked ? null : onVerifyUserPressed,
                        label: verified ? l10n.unverify : l10n.verify,
                      ),
                    ),
                  ),
                  const SizedBox(height: 20),
                  SizedBox(
                    width: 280,
                    child: QaulButton(
                      onPressed: onBlockUserPressed,
                      label: blocked ? l10n.unblockUser : l10n.blockUser,
                    ),
                  ),
                ],
              ),
              tabletBody: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  SizedBox(
                    width: 480,
                    child: QaulButton(
                      label: l10n.newChatTooltip,
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
                    ),
                  ),
                  const SizedBox(height: 20),
                  SizedBox(
                    width: 480,
                    child: DisabledStateDecorator(
                      isDisabled: blocked,
                      child: QaulButton(
                        onPressed: blocked ? null : onVerifyUserPressed,
                        label: verified ? l10n.unverify : l10n.verify,
                      ),
                    ),
                  ),
                  const SizedBox(height: 20),
                  SizedBox(
                    width: 480,
                    child: QaulButton(
                      onPressed: onBlockUserPressed,
                      label: blocked ? l10n.unblockUser : l10n.blockUser,
                    ),
                  ),
                ],
              ),
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
    return await showDialog(
      context: context,
      builder: (context) {
        final l10n = AppLocalizations.of(context)!;

        return QaulDialog(
          content: Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              Text(
                description,
                style: Theme.of(context).textTheme.titleMedium,
                textAlign: TextAlign.center,
              ),
            ],
          ),
          button1Label: l10n.okDialogButton,
          onButton1Pressed: () => Navigator.pop(context, true),
          button2Label: l10n.cancelDialogButton,
          onButton2Pressed: () => Navigator.pop(context),
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
      child: QaulDialog(
        content: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            if (securityNo != null && securityNo.userId.equals(user.id)) ...[
              _SecurityNumberDisplay(securityNo: securityNo),
            ],
            const SizedBox(height: 24),
            Text(
              l10n.securityNumberDialogDesc,
              style: Theme.of(context).textTheme.titleMedium,
              textAlign: TextAlign.center,
            ),
          ],
        ),
        button1Label: l10n.okDialogButton,
        onButton1Pressed: () {
          final worker = ref.read(qaulWorkerProvider);
          worker.verifyUser(user);
          Navigator.pop(context);
        },
        button2Label: l10n.cancelDialogButton,
        onButton2Pressed: () => Navigator.pop(context),
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
        Container(
          decoration: BoxDecoration(
            border: Border.all(color: Colors.grey.withOpacity(.5)),
          ),
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 8.0, vertical: 8.0),
            child: Column(
              children: [
                Row(
                  mainAxisSize: MainAxisSize.min,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: _buildSecurityCodeRow(0)
                      .intersperse(const SizedBox(width: 16))
                      .toList(),
                ),
                const SizedBox(height: 8),
                Row(
                  mainAxisSize: MainAxisSize.min,
                  mainAxisAlignment: MainAxisAlignment.center,
                  children: _buildSecurityCodeRow(1)
                      .intersperse(const SizedBox(width: 16))
                      .toList(),
                ),
              ],
            ),
          ),
        ),
      ],
    );
  }

  List<Widget> _buildSecurityCodeRow(int row) {
    return List<Widget>.generate(
      4,
      (index) => Container(
        padding: const EdgeInsets.all(2),
        child: Text(securityNo.securityCode[(row * 4) + index]),
      ),
    );
  }
}
