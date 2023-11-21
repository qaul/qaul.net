// ignore_for_file: use_build_context_synchronously
part of '../../tab.dart';

class _CreateNewRoomDialog extends StatelessWidget {
  const _CreateNewRoomDialog({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return SearchUserDecorator(
      title: AppLocalizations.of(context)!.newChatTooltip,
      builder: (_, users) {
        return ListView.separated(
          padding: const EdgeInsets.all(8),
          itemCount: users.length + 1,
          separatorBuilder: (_, __) => const Divider(height: 12.0),
          itemBuilder: (context, i) {
            if (i == 0) {
              return ListTile(
                leading: Stack(
                  alignment: AlignmentDirectional.topStart,
                  children: [
                    QaulAvatar.groupSmall(),
                    const Icon(Icons.add, size: 12),
                  ],
                ),
                title: Text(l10n.createNewGroup),
                onTap: () async {
                  final result = await Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (_) => _CreateNewGroupDialog(),
                    ),
                  );
                  Navigator.pop(context, result);
                },
              );
            }
            final usr = users[i - 1];
            return QaulListTile.user(
              usr,
              onTap: () => Navigator.pop(context, usr),
            );
          },
        );
      },
    );
  }
}

class _CreateNewGroupDialog extends HookConsumerWidget {
  _CreateNewGroupDialog({Key? key}) : super(key: key);

  final _nameKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final worker = ref.read(qaulWorkerProvider);

    final loading = useState(false);
    final nameCtrl = useTextEditingController();

    final isMounted = useIsMounted();

    final l10n = AppLocalizations.of(context)!;

    return Scaffold(
      appBar: AppBar(
        title: Text(l10n.groupInvite),
        centerTitle: false,
        leading: IconButtonFactory.close(),
      ),
      body: Stack(
        children: [
          ListView(
            padding: const EdgeInsets.symmetric(horizontal: 40, vertical: 8),
            children: [
              const SizedBox(height: 140, width: double.maxFinite),
              QaulAvatar.groupLarge(),
              const SizedBox(height: 20),
              LayoutBuilder(builder: (context, constraints) {
                return SizedBox(
                  width: constraints.constrainWidth(400),
                  child: TextFormField(
                    key: _nameKey,
                    controller: nameCtrl,
                    validator: (val) {
                      if (val == null || val.isEmpty) {
                        return l10n.fieldRequiredErrorMessage;
                      }
                      return null;
                    },
                    decoration: InputDecoration(
                      hintText: l10n.createGroupHint,
                    ),
                  ),
                );
              }),
              const SizedBox(height: 20),
              QaulButton(
                label: l10n.createButtonHint,
                onPressed: () async {
                  if (!_nameKey.currentState!.validate()) return;
                  loading.value = true;

                  var name = nameCtrl.text;
                  await createChatGroup(name, worker, ref);
                  if (!isMounted()) return;

                  final group = ref
                      .read(chatRoomsProvider)
                      .firstWhereOrNull((g) => g.name == name);
                  if (group == null) {
                    ScaffoldMessenger.of(context).showSnackBar(SnackBar(
                      content: Text(l10n.genericErrorMessage),
                    ));
                    Navigator.pop(context);
                    return;
                  }

                  await Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (_) => InviteUsersToGroupDialog(room: group),
                    ),
                  );

                  if (!isMounted()) return;
                  var chatRoom = ref
                      .read(chatRoomsProvider)
                      .firstWhereOrNull((g) => g.name == name);

                  Navigator.pop(context, chatRoom);
                },
              ),
            ],
          ),
          if (loading.value)
            Positioned.fill(
              child: Container(
                color: Colors.black38,
                child: const LoadingIndicator(),
              ),
            ),
        ],
      ),
    );
  }

  Future<void> createChatGroup(
      String name, LibqaulWorker worker, WidgetRef ref) async {
    worker.createGroup(name);
    for (var i = 0; i < 60; i++) {
      final groups = ref.read(chatRoomsProvider);
      if (groups.where((g) => g.name == name).isNotEmpty) break;

      worker.getAllChatRooms();
      await Future.delayed(const Duration(milliseconds: 1000));
    }
  }
}

class InviteUsersToGroupDialog extends StatefulHookConsumerWidget {
  const InviteUsersToGroupDialog({
    Key? key,
    required this.room,
  }) : super(key: key);

  final ChatRoom room;

  @override
  ConsumerState<InviteUsersToGroupDialog> createState() =>
      _InviteUsersToGroupDialogState();
}

class _InviteUsersToGroupDialogState
    extends ConsumerState<InviteUsersToGroupDialog> {
  final selected = <User>{};

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    return SearchUserDecorator(
      title: AppLocalizations.of(context)!.inviteUser,
      builder: (_, users) {
        return Stack(
          children: [
            ListView.separated(
              padding: const EdgeInsets.fromLTRB(8, 8, 8, 40),
              itemCount: users.length,
              separatorBuilder: (_, __) => const Divider(height: 12.0),
              itemBuilder: (context, i) {
                final usr = users[i];
                return QaulListTile.user(
                  usr,
                  onTap: () {
                    if (selected.contains(usr)) {
                      setState(() => selected.remove(usr));
                      return;
                    }
                    setState(() => selected.add(usr));
                  },
                  trailingIcon: Checkbox(
                    value: selected.contains(usr),
                    onChanged: (ok) {
                      if (ok ?? false) {
                        setState(() => selected.add(usr));
                        return;
                      }
                      setState(() => selected.remove(usr));
                    },
                  ),
                );
              },
            ),
            Positioned.directional(
              textDirection: Directionality.of(context),
              end: 12,
              bottom: 8,
              child: QaulButton(
                backgroundColor: Theme.of(context).scaffoldBackgroundColor,
                onPressed: () async {
                  final worker = ref.read(qaulWorkerProvider);
                  for (final user in selected) {
                    worker.inviteUserToGroup(user, widget.room);
                  }
                  Navigator.pop(context);
                },
                label: l10n.invite,
              ),
            ),
          ],
        );
      },
    );
  }
}

class _InviteDetailsDialog extends HookConsumerWidget {
  const _InviteDetailsDialog(this.invite, {Key? key}) : super(key: key);

  final GroupInvite invite;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final users = ref.watch(usersProvider);

    final replyInvite = useCallback(({required bool accepted}) {
      final worker = ref.read(qaulWorkerProvider);
      worker.replyToGroupInvite(
        invite.groupDetails.conversationId,
        accepted: accepted,
      );

      Navigator.pop(context);
    }, []);

    final sender = users.firstWhereOrNull((s) => s.id.equals(invite.senderId));

    final l10n = AppLocalizations.of(context)!;
    return QaulDialog(
      title: l10n.groupInvite,
      content: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text('${l10n.groupName}: ${invite.groupDetails.name}'),
          Text('${l10n.createdAt}: ${invite.groupDetails.createdAt}'),
          Text(
            '${l10n.noOfMembers}: ${invite.groupDetails.members.length}',
          ),
          if (sender != null) ...[
            Text('${l10n.invitedBy}: ${sender.name}'),
          ],
        ],
      ),
      button1Label: l10n.accept,
      onButton1Pressed: () => replyInvite(accepted: true),
      button2Label: l10n.decline,
      onButton2Pressed: () => replyInvite(accepted: false),
    );
  }
}
