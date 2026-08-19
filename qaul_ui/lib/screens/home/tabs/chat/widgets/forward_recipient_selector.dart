part of 'chat.dart';

class _ForwardRecipientSelectorScreen extends StatefulHookConsumerWidget {
  const _ForwardRecipientSelectorScreen({
    required this.defaultUser,
    required this.forwardedText,
  });

  final User defaultUser;
  final String forwardedText;

  @override
  ConsumerState<_ForwardRecipientSelectorScreen> createState() =>
      _ForwardRecipientSelectorScreenState();
}

class _ForwardRecipientSelectorScreenState
    extends ConsumerState<_ForwardRecipientSelectorScreen> {
  late final TextEditingController _searchController;
  String _query = '';

  @override
  void initState() {
    super.initState();
    _searchController = TextEditingController();
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final rooms = ref.watch(chatRoomsProvider);
    final users = _filteredForwardUsers(
      ref.watch(usersStoreProvider),
      defaultUser: widget.defaultUser,
      rooms: rooms,
      query: _query,
    );
    final groups = _filteredForwardGroups(rooms, query: _query);
    final itemCount = users.length + groups.length + 2;

    return Scaffold(
      appBar: AppBar(
        title: const Text('Forward to'),
        centerTitle: false,
        leading: const IconButtonFactory(),
      ),
      body: Column(
        children: [
          Padding(
            padding: const EdgeInsets.fromLTRB(16, 8, 8, 8),
            child: TextField(
              key: const ValueKey('forward-recipient-search'),
              controller: _searchController,
              decoration: InputDecoration(
                prefixIcon: const Icon(Icons.search),
                hintText: 'Search users and groups...',
                border: const UnderlineInputBorder(),
                suffixIcon: IconButton(
                  onPressed: () {
                    _searchController.clear();
                    setState(() => _query = '');
                  },
                  splashRadius: 16,
                  icon: const Icon(Icons.clear_rounded),
                ),
              ),
              onChanged: (value) => setState(() => _query = value),
            ),
          ),
          Expanded(
            child: ListView.separated(
              itemCount: itemCount,
              separatorBuilder: (_, index) =>
                  index == 0 || index == groups.length + 1
                  ? const SizedBox.shrink()
                  : const Divider(height: 12),
              itemBuilder: (context, index) {
                if (index == 0) {
                  return const _ForwardRecipientSectionHeader(label: 'Groups');
                }

                if (index <= groups.length) {
                  final group = groups[index - 1];
                  return QaulListTile.group(
                    group,
                    key: ValueKey('forward-group-${group.idBase58}'),
                    onTap: () => _selectRoom(group),
                    trailingIcon: const Icon(Icons.radio_button_unchecked),
                  );
                }

                if (index == groups.length + 1) {
                  return const _ForwardRecipientSectionHeader(
                    label: 'Users / Contacts',
                  );
                }

                final user = users[index - groups.length - 2];
                final canOpenUser = _canOpenForwardUser(user, rooms);
                return QaulListTile.user(
                  user,
                  key: ValueKey('forward-user-${user.idBase58}'),
                  onTap: canOpenUser ? () => _selectUser(user) : null,
                  avatarTapRoutesToDetailsScreen: false,
                  trailingIcon: const Icon(Icons.radio_button_unchecked),
                );
              },
            ),
          ),
        ],
      ),
    );
  }

  void _selectUser(User user) {
    final room = _existingDirectRoom(user) ?? ChatRoom.blank(otherUser: user);
    _selectRoom(room, otherUser: user);
  }

  ChatRoom? _existingDirectRoom(User user) {
    return _existingDirectRoomForUser(user, ref.read(chatRoomsProvider));
  }

  void _selectRoom(ChatRoom room, {User? otherUser}) {
    ref.read(_pendingForwardDraftProvider.notifier).state = _ForwardDraft(
          roomIdBase58: room.idBase58,
          text: widget.forwardedText,
        );

    if (Responsiveness.isMobile(context)) {
      Navigator.pushReplacement(
        context,
        MaterialPageRoute(
          builder: (_) => ChatScreen(
            room,
            widget.defaultUser,
            otherUser: otherUser,
            initialMessageText: widget.forwardedText,
          ),
          settings: const RouteSettings(name: _kChatRouteName),
        ),
      );
      return;
    }

    ref.read(currentOpenChatRoom.notifier).state = room;
    ref.read(homeScreenControllerProvider.notifier).goToTab(TabType.chat);
    Navigator.pop(context);
  }
}

class _ForwardRecipientSectionHeader extends StatelessWidget {
  const _ForwardRecipientSectionHeader({required this.label});

  final String label;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 20, 20, 8),
      child: Text(
        label,
        style: Theme.of(context).textTheme.titleSmall,
      ),
    );
  }
}

List<User> _filteredForwardUsers(
  List<User> users, {
  required User defaultUser,
  required List<ChatRoom> rooms,
  required String query,
}) {
  final normalizedQuery = query.trim().toLowerCase();
  final filtered = users.where((user) {
    if (user.id.equals(defaultUser.id)) return false;
    if (user.isBlocked ?? false) return false;
    if (normalizedQuery.isEmpty) return true;
    return _matchesForwardQuery(user.name, user.idBase58, normalizedQuery);
  }).toList();

  filtered.sort(
    (a, b) => _compareForwardUsers(a, b, rooms, normalizedQuery),
  );
  return filtered;
}

List<ChatRoom> _filteredForwardGroups(
  List<ChatRoom> rooms, {
  required String query,
}) {
  final normalizedQuery = query.trim().toLowerCase();
  final filtered = rooms.where((room) {
    if (!room.isGroupChatRoom) return false;
    if (normalizedQuery.isEmpty) return true;
    return _matchesForwardQuery(
      room.name ?? '',
      room.idBase58,
      normalizedQuery,
    );
  }).toList();

  filtered.sort(
    (a, b) =>
        _compareByForwardRelevance(a.name ?? '', b.name ?? '', normalizedQuery),
  );
  return filtered;
}

int _compareForwardUsers(
  User a,
  User b,
  List<ChatRoom> rooms,
  String query,
) {
  final searchRank = _compareByForwardRelevance(a.name, b.name, query);
  if (query.isNotEmpty && searchRank != 0) return searchRank;

  final aHasConversation = _hasForwardConversation(a, rooms);
  final bHasConversation = _hasForwardConversation(b, rooms);
  if (aHasConversation != bHasConversation) {
    return aHasConversation ? -1 : 1;
  }

  if (a.isConnected != b.isConnected) return a.isConnected ? -1 : 1;

  return a.name.toLowerCase().compareTo(b.name.toLowerCase());
}

int _compareByForwardRelevance(String a, String b, String query) {
  final lowerA = a.toLowerCase();
  final lowerB = b.toLowerCase();
  if (query.isEmpty) return lowerA.compareTo(lowerB);

  final aStarts = lowerA.startsWith(query);
  final bStarts = lowerB.startsWith(query);
  if (aStarts != bStarts) return aStarts ? -1 : 1;
  return lowerA.compareTo(lowerB);
}

bool _matchesForwardQuery(String name, String idBase58, String query) {
  return name.toLowerCase().contains(query) ||
      idBase58.toLowerCase().contains(query);
}

bool _canOpenForwardUser(User user, List<ChatRoom> rooms) {
  return user.conversationId != null ||
      _existingDirectRoomForUser(user, rooms) != null;
}

bool _hasForwardConversation(User user, List<ChatRoom> rooms) {
  return _existingDirectRoomForUser(user, rooms) != null;
}

ChatRoom? _existingDirectRoomForUser(User user, List<ChatRoom> rooms) {
  return rooms.firstWhereOrNull(
    (room) =>
        !room.isGroupChatRoom &&
        room.members.any((member) => member.id.equals(user.id)),
  );
}
