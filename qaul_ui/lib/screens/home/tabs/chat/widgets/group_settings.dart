part of 'chat.dart';

class _GroupSettingsPage extends HookConsumerWidget {
  _GroupSettingsPage(this.room, {Key? key}) : super(key: key);

  final ChatRoom room;

  final _nameKey = GlobalKey<FormFieldState>();

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final group = ref.watch(chatRoomsProvider).firstWhere((r) => r == room);

    final nameCtrl = useTextEditingController(text: room.name);

    final theme = Theme.of(context).textTheme;
    final l10n = AppLocalizations.of(context)!;

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
        floatingActionButton: FloatingActionButton(
          onPressed: () {
            // TODO: add invite person logic (if admin)
          },
          child: const Icon(Icons.person_add),
        ),
        body: SizedBox.expand(
          child: SingleChildScrollView(
            child: Padding(
              padding:
                  const EdgeInsets.symmetric(horizontal: 16.0, vertical: 32.0),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.center,
                mainAxisAlignment: MainAxisAlignment.center,
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
                  Row(
                    children: [
                      TextFormField(
                        key: _nameKey,
                        controller: nameCtrl,
                        style: theme.headline3,
                        validator: (val) {
                          if (val == null || val.isEmpty) {
                            return l10n.fieldRequiredErrorMessage;
                          }
                          return null;
                        },
                      ),
                      // TODO: only update name if admin
                      IconButton(
                        icon: const Icon(Icons.send),
                        onPressed: () {
                          if (!_nameKey.currentState!.validate()) return;

                          var worker = ref.read(qaulWorkerProvider);
                          worker.renameGroup(room, nameCtrl.text);
                          worker.getGroupInfo(room.conversationId);
                        },
                      ),
                    ],
                  ),
                  const SizedBox(height: 20),
                  const Text('Members'),
                  const SizedBox(height: 4),
                  ListView.separated(
                    itemCount: group.members.length,
                    separatorBuilder: (_, __) => const Divider(),
                    itemBuilder: (c, i) {
                      final user = group.members[i];
                      // TODO add option to remove user (if admin)
                      return UserListTile(user);
                    },
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
