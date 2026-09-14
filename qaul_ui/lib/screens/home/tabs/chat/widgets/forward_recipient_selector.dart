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
  static const _recentRecipientsLimit = 5;

  late final TextEditingController _userSearchController;
  late final TextEditingController _groupSearchController;
  late final FocusNode _userSearchFocusNode;
  late final FocusNode _groupSearchFocusNode;
  String _userQuery = '';
  String _groupQuery = '';
  bool _isUserSearchVisible = false;
  bool _isGroupSearchVisible = false;

  @override
  void initState() {
    super.initState();
    _userSearchController = TextEditingController();
    _groupSearchController = TextEditingController();
    _userSearchFocusNode = FocusNode();
    _groupSearchFocusNode = FocusNode();
  }

  @override
  void dispose() {
    _userSearchController.dispose();
    _groupSearchController.dispose();
    _userSearchFocusNode.dispose();
    _groupSearchFocusNode.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final rooms = ref.watch(chatRoomsProvider);
    final users = _filteredForwardUsers(
      ref.watch(usersStoreProvider),
      defaultUser: widget.defaultUser,
      rooms: rooms,
      query: _userQuery,
      limit: _userQuery.trim().isEmpty ? _recentRecipientsLimit : null,
    );
    final groups = _filteredForwardGroups(
      rooms,
      query: _groupQuery,
      limit: _groupQuery.trim().isEmpty ? _recentRecipientsLimit : null,
    );

    return Scaffold(
      appBar: AppBar(
        title: const Text('Forward to'),
        centerTitle: false,
        leading: const IconButtonFactory(),
      ),
      body: ListView(
        children: [
          _ForwardRecipientSectionHeader(
            label: 'Users / Contacts',
            isSearchVisible: _isUserSearchVisible,
            searchTooltip: 'Search users / contacts',
            onSearchPressed: _toggleUserSearch,
          ),
          if (_isUserSearchVisible)
            _ForwardRecipientSearchField(
              key: const ValueKey('forward-user-search'),
              controller: _userSearchController,
              focusNode: _userSearchFocusNode,
              hintText: 'Search users / contacts...',
              onChanged: (value) => setState(() => _userQuery = value),
              onClear: _clearUserSearch,
            ),
          for (final user in users) ...[
            _buildUserTile(user, rooms),
            const Divider(height: 12),
          ],
          _ForwardRecipientSectionHeader(
            label: 'Groups',
            isSearchVisible: _isGroupSearchVisible,
            searchTooltip: 'Search groups',
            onSearchPressed: _toggleGroupSearch,
          ),
          if (_isGroupSearchVisible)
            _ForwardRecipientSearchField(
              key: const ValueKey('forward-group-search'),
              controller: _groupSearchController,
              focusNode: _groupSearchFocusNode,
              hintText: 'Search groups...',
              onChanged: (value) => setState(() => _groupQuery = value),
              onClear: _clearGroupSearch,
            ),
          for (final group in groups) ...[
            QaulListTile.group(
              group,
              key: ValueKey('forward-group-${group.idBase58}'),
              onTap: () => _selectRoom(group),
              trailingIcon: const Icon(Icons.radio_button_unchecked),
            ),
            const Divider(height: 12),
          ],
        ],
      ),
    );
  }

  Widget _buildUserTile(User user, List<ChatRoom> rooms) {
    final canOpenUser = _canOpenForwardUser(user, rooms);
    return QaulListTile.user(
      user,
      key: ValueKey('forward-user-${user.idBase58}'),
      onTap: canOpenUser ? () => _selectUser(user) : null,
      avatarTapRoutesToDetailsScreen: false,
      trailingIcon: const Icon(Icons.radio_button_unchecked),
    );
  }

  void _toggleUserSearch() {
    setState(() => _isUserSearchVisible = !_isUserSearchVisible);
    if (_isUserSearchVisible) {
      _focusSearchField(_userSearchFocusNode);
    } else {
      _clearUserSearch();
    }
  }

  void _toggleGroupSearch() {
    setState(() => _isGroupSearchVisible = !_isGroupSearchVisible);
    if (_isGroupSearchVisible) {
      _focusSearchField(_groupSearchFocusNode);
    } else {
      _clearGroupSearch();
    }
  }

  void _focusSearchField(FocusNode focusNode) {
    WidgetsBinding.instance.addPostFrameCallback((_) {
      if (mounted) focusNode.requestFocus();
    });
  }

  void _clearUserSearch() {
    _userSearchController.clear();
    if (_userQuery.isNotEmpty) setState(() => _userQuery = '');
  }

  void _clearGroupSearch() {
    _groupSearchController.clear();
    if (_groupQuery.isNotEmpty) setState(() => _groupQuery = '');
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
  const _ForwardRecipientSectionHeader({
    required this.label,
    required this.isSearchVisible,
    required this.searchTooltip,
    required this.onSearchPressed,
  });

  final String label;
  final bool isSearchVisible;
  final String searchTooltip;
  final VoidCallback onSearchPressed;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(20, 20, 8, 8),
      child: Row(
        children: [
          Expanded(
            child: Text(label, style: Theme.of(context).textTheme.titleSmall),
          ),
          IconButton(
            key: ValueKey(
              'forward-${label == 'Groups' ? 'group' : 'user'}-search-toggle',
            ),
            tooltip: searchTooltip,
            onPressed: onSearchPressed,
            icon: Icon(isSearchVisible ? Icons.close : Icons.search),
          ),
        ],
      ),
    );
  }
}

class _ForwardRecipientSearchField extends StatelessWidget {
  const _ForwardRecipientSearchField({
    super.key,
    required this.controller,
    required this.focusNode,
    required this.hintText,
    required this.onChanged,
    required this.onClear,
  });

  final TextEditingController controller;
  final FocusNode focusNode;
  final String hintText;
  final ValueChanged<String> onChanged;
  final VoidCallback onClear;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 0, 8, 8),
      child: TextField(
        controller: controller,
        focusNode: focusNode,
        decoration: InputDecoration(
          prefixIcon: const Icon(Icons.search),
          hintText: hintText,
          border: const UnderlineInputBorder(),
          suffixIcon: IconButton(
            onPressed: onClear,
            splashRadius: 16,
            icon: const Icon(Icons.clear_rounded),
          ),
        ),
        onChanged: onChanged,
      ),
    );
  }
}

List<User> _filteredForwardUsers(
  List<User> users, {
  required User defaultUser,
  required List<ChatRoom> rooms,
  required String query,
  int? limit,
}) {
  final normalizedQuery = query.trim().toLowerCase();
  final filtered = users.where((user) {
    if (user.id.equals(defaultUser.id)) return false;
    if (user.isBlocked ?? false) return false;
    if (normalizedQuery.isEmpty) return true;
    return _matchesForwardQuery(user.name, user.idBase58, normalizedQuery);
  }).toList();

  filtered.sort((a, b) => _compareForwardUsers(a, b, rooms, normalizedQuery));
  return limit == null ? filtered : filtered.take(limit).toList();
}

List<ChatRoom> _filteredForwardGroups(
  List<ChatRoom> rooms, {
  required String query,
  int? limit,
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

  filtered.sort((a, b) => _compareForwardGroups(a, b, normalizedQuery));
  return limit == null ? filtered : filtered.take(limit).toList();
}

int _compareForwardUsers(User a, User b, List<ChatRoom> rooms, String query) {
  final searchRank = _compareByForwardRelevance(a.name, b.name, query);
  if (query.isNotEmpty && searchRank != 0) return searchRank;

  final aLastMessageTime = _lastMessageTimeForUser(a, rooms);
  final bLastMessageTime = _lastMessageTimeForUser(b, rooms);
  final recentComparison = _compareForwardRecency(
    aLastMessageTime,
    bLastMessageTime,
  );
  if (recentComparison != 0) return recentComparison;

  final aHasConversation =
      aLastMessageTime != null || _hasForwardConversation(a, rooms);
  final bHasConversation =
      bLastMessageTime != null || _hasForwardConversation(b, rooms);
  if (aHasConversation != bHasConversation) {
    return aHasConversation ? -1 : 1;
  }

  if (a.isConnected != b.isConnected) return a.isConnected ? -1 : 1;

  return a.name.toLowerCase().compareTo(b.name.toLowerCase());
}

int _compareForwardGroups(ChatRoom a, ChatRoom b, String query) {
  final searchRank = _compareByForwardRelevance(
    a.name ?? '',
    b.name ?? '',
    query,
  );
  if (query.isNotEmpty && searchRank != 0) return searchRank;

  final recentComparison = _compareForwardRecency(
    a.lastMessageTime,
    b.lastMessageTime,
  );
  if (recentComparison != 0) return recentComparison;

  return (a.name ?? '').toLowerCase().compareTo((b.name ?? '').toLowerCase());
}

int _compareForwardRecency(DateTime? a, DateTime? b) {
  if (a == null && b == null) return 0;
  if (a == null) return 1;
  if (b == null) return -1;
  return b.compareTo(a);
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

DateTime? _lastMessageTimeForUser(User user, List<ChatRoom> rooms) {
  return _existingDirectRoomForUser(user, rooms)?.lastMessageTime;
}

ChatRoom? _existingDirectRoomForUser(User user, List<ChatRoom> rooms) {
  return rooms.firstWhereOrNull(
    (room) =>
        !room.isGroupChatRoom &&
        room.members.any((member) => member.id.equals(user.id)),
  );
}
