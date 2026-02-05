part of 'chat.dart';

class _GroupSettingsPage extends HookConsumerWidget {
  _GroupSettingsPage(this.room);

  final ChatRoom room;

  final _nameKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(defaultUserProvider)!;
    final group = ref.watch(chatRoomsProvider).firstWhere((r) => r.idBase58 == room.idBase58);

    final nameCtrl = useTextEditingController(text: room.name);

    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context)!;

    final isAdmin = group.groupAdminIdBase58 == defaultUser.idBase58;

    return CronTaskDecorator(
      schedule: const Duration(milliseconds: 500),
      callback: () {
        ref.read(qaulWorkerProvider).getGroupInfo(room.conversationId);
      },
      child: Scaffold(
        appBar: AppBar(
          leading: const IconButtonFactory(),
          title: Text(l10n.groupSettings),
        ),
        body: ListView(
          padding: const EdgeInsets.symmetric(vertical: 32, horizontal: 16),
          children: [
            QaulAvatar.groupLarge(),
            const SizedBox(height: 28.0),
            !isAdmin
                ? Text(room.name ?? '', style: theme.displaySmall)
                : TextFormField(
                    key: _nameKey,
                    controller: nameCtrl,
                    style: theme.bodyLarge,
                    validator: (val) {
                      if (val == null || val.isEmpty) {
                        return l10n.fieldRequiredErrorMessage;
                      }
                      return null;
                    },
                    decoration: InputDecoration(
                      suffixIcon: IconButton(
                        splashRadius: 20,
                        icon: const Icon(Icons.send),
                        onPressed: () {
                          if (!_nameKey.currentState!.validate()) return;

                          var worker = ref.read(qaulWorkerProvider);
                          worker.renameGroup(room, nameCtrl.text);
                        },
                      ),
                    ),
                  ),
            const SizedBox(height: 20),
            QaulTable(
              title: l10n.members,
              addButtonEnabled: isAdmin,
              addRowLabel: l10n.inviteUser,
              rowCount: group.members.length,
              titleIcon: Icons.people_outlined,
              rowBuilder: (c, i) {
                final user = group.members[i];
                final isNotDefaultUser = user.idBase58 != defaultUser.idBase58;

                return QaulListTile.user(
                  user,
                  trailingIcon: !isNotDefaultUser
                      ? const SizedBox()
                      : Row(
                    mainAxisSize: MainAxisSize.min,
                    children: [
                      Icon(_mapInvitationStateToIcon(
                          user.invitationState)),
                      const SizedBox(width: 12),
                      if (isAdmin)
                        IconButton(
                          onPressed: () async {
                            var ok =
                            await showConfirmDialog(context, l10n);
                            if (!(ok ?? false)) return;
                            var worker = ref.read(qaulWorkerProvider);
                            worker.removeUserFromGroup(user, room);
                          },
                          splashRadius: 18,
                          color: Colors.red.shade400,
                          icon: const Icon(Icons.person_remove_rounded),
                        )
                    ],
                  ),
                  tapRoutesToDetailsScreen: isNotDefaultUser,
                  avatarTapRoutesToDetailsScreen: isNotDefaultUser,
                );
              },
              onAddRowPressed: () async {
                await Navigator.push(
                  context,
                  MaterialPageRoute(
                    builder: (_) => InviteUsersToGroupDialog(room: group),
                  ),
                );
              },
            ),
          ],
        ),
      ),
    );
  }

  IconData _mapInvitationStateToIcon(InvitationState s) {
    switch (s) {
      case InvitationState.sent:
        return Icons.check;
      case InvitationState.received:
        return Icons.done_all;
      case InvitationState.accepted:
        return Icons.handshake;
      case InvitationState.unknown:
        return Icons.help;
    }
  }

  Future<bool?> showConfirmDialog(
      BuildContext context, AppLocalizations l10n) async {
    // set up the buttons
    Widget cancelButton = TextButton(
      child: Text(l10n.cancelDialogButton),
      onPressed: () => Navigator.pop(context, false),
    );
    Widget continueButton = TextButton(
      child: Text(l10n.continueDialogButton),
      onPressed: () => Navigator.pop(context, true),
    );

    // set up the AlertDialog
    AlertDialog alert = AlertDialog(
      title: Text(l10n.removeUser),
      content: Text(l10n.removeUserDialogContent),
      actions: [
        cancelButton,
        continueButton,
      ],
    );

    // show the dialog
    return showDialog<bool?>(
      context: context,
      builder: (BuildContext context) {
        return alert;
      },
    );
  }
}
