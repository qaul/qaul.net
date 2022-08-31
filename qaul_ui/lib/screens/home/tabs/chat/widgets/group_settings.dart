part of 'chat.dart';

class _GroupSettingsPage extends HookConsumerWidget {
  _GroupSettingsPage(this.room, {Key? key}) : super(key: key);

  final ChatRoom room;

  final _nameKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final defaultUser = ref.watch(defaultUserProvider)!;
    final group = ref.watch(chatRoomsProvider).firstWhere((r) => r == room);

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
          title: const Text('Group Settings'),
        ),
        floatingActionButton: !isAdmin
            ? const SizedBox()
            : FloatingActionButton(
                onPressed: () async {
                  await Navigator.push(
                    context,
                    MaterialPageRoute(
                      builder: (_) => InviteUsersToGroupDialog(room: group),
                    ),
                  );
                },
                child: const Icon(Icons.person_add),
              ),
        body: ListView(
          padding: const EdgeInsets.symmetric(vertical: 32, horizontal: 16),
          children: [
            CircleAvatar(
              radius: 80.0,
              child: Text(
                initials(group.name ?? 'GROUP'),
                style: const TextStyle(
                  fontSize: 68,
                  color: Colors.white,
                  fontWeight: FontWeight.w700,
                ),
              ),
              backgroundColor: colorGenerationStrategy(room.idBase58),
            ),
            const SizedBox(height: 28.0),
            !isAdmin
                ? Text(room.name ?? '', style: theme.headline3)
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
            const Text('Members'),
            const SizedBox(height: 4),
            ListView.separated(
              shrinkWrap: true,
              itemCount: group.members.length,
              separatorBuilder: (_, __) => const Divider(),
              itemBuilder: (c, i) {
                final user = group.members[i];
                // TODO add option to remove user (if admin)
                return UserListTile(
                  user,
                  trailingIcon:
                      Icon(_mapInvitationStateToIcon(user.invitationState)),
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
}
