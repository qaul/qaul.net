import 'package:flutter/material.dart';

enum ForwardRecipientKind { public, user, group }

@immutable
class ForwardRecipient {
  const ForwardRecipient({
    required this.id,
    required this.displayName,
    required this.kind,
    this.initials,
    this.avatarColor,
    this.isOnline = false,
  });

  final String id;
  final String displayName;
  final ForwardRecipientKind kind;
  final String? initials;
  final Color? avatarColor;
  final bool isOnline;
}

enum ForwardSearchFilter { user, group, id, chat, trustedVerified }

/// Recipient selection state used before forwarding a message.
///
/// Selection is intentionally single-recipient. The host receives changes via
/// [onRecipientSelected] and can replace this component with a multi-select
/// implementation later without coupling it to a contact data source.
class ForwardRecipientSelector extends StatefulWidget {
  const ForwardRecipientSelector({
    super.key,
    required this.recipients,
    required this.onRecipientSelected,
    required this.onSearchChanged,
    required this.onCancel,
    this.initialSelectedRecipientId,
    this.initialSearchOpen = false,
    this.onSearchFilterSelected,
    this.onMore,
    this.title = 'Forward message to:',
  });

  final List<ForwardRecipient> recipients;
  final ValueChanged<ForwardRecipient> onRecipientSelected;
  final ValueChanged<String> onSearchChanged;
  final VoidCallback onCancel;
  final ValueChanged<ForwardSearchFilter>? onSearchFilterSelected;
  final VoidCallback? onMore;
  final String? initialSelectedRecipientId;
  final bool initialSearchOpen;
  final String title;

  @override
  State<ForwardRecipientSelector> createState() =>
      _ForwardRecipientSelectorState();
}

class _ForwardRecipientSelectorState extends State<ForwardRecipientSelector> {
  late String? _selectedRecipientId = widget.initialSelectedRecipientId;
  late bool _searchOpen = widget.initialSearchOpen;
  final _searchController = TextEditingController();

  @override
  void didUpdateWidget(covariant ForwardRecipientSelector oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.initialSelectedRecipientId !=
        widget.initialSelectedRecipientId) {
      _selectedRecipientId = widget.initialSelectedRecipientId;
    }
    if (oldWidget.initialSearchOpen != widget.initialSearchOpen) {
      _searchOpen = widget.initialSearchOpen;
    }
  }

  @override
  void dispose() {
    _searchController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final publicRecipients = _ofKind(ForwardRecipientKind.public);
    final users = _ofKind(ForwardRecipientKind.user);
    final groups = _ofKind(ForwardRecipientKind.group);
    final backgroundColor = Theme.of(context).brightness == Brightness.dark
        ? Colors.black
        : Colors.white;

    return Scaffold(
      backgroundColor: backgroundColor,
      appBar: AppBar(
        backgroundColor: backgroundColor,
        surfaceTintColor: Colors.transparent,
        shadowColor: Colors.transparent,
        elevation: 0,
        scrolledUnderElevation: 0,
        shape: const Border(),
        leading: IconButton(
          tooltip: 'Back',
          onPressed: widget.onCancel,
          icon: const Icon(Icons.arrow_back),
        ),
        actions: [
          IconButton(
            tooltip: 'More options',
            onPressed: widget.onMore ?? () {},
            icon: const Icon(Icons.more_vert),
          ),
        ],
      ),
      body: Stack(
        children: [
          ListView(
            padding: const EdgeInsets.fromLTRB(20, 12, 20, 24),
            children: [
              Text(
                widget.title,
                style: Theme.of(
                  context,
                ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700),
              ),
              const SizedBox(height: 12),
              if (publicRecipients.isNotEmpty) ...[
                _section(context, 'Public', publicRecipients),
                const Divider(height: 28),
              ],
              _section(context, 'Users / Contacts', users, searchable: true),
              const Divider(height: 28),
              _section(context, 'Groups', groups, searchable: true),
            ],
          ),
          if (_searchOpen) _buildSearchOverlay(context),
        ],
      ),
    );
  }

  List<ForwardRecipient> _ofKind(ForwardRecipientKind kind) =>
      widget.recipients.where((recipient) => recipient.kind == kind).toList();

  Widget _section(
    BuildContext context,
    String title,
    List<ForwardRecipient> recipients, {
    bool searchable = false,
  }) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.stretch,
      children: [
        Row(
          children: [
            Expanded(
              child: Text(
                title,
                style: Theme.of(
                  context,
                ).textTheme.titleMedium?.copyWith(fontWeight: FontWeight.w700),
              ),
            ),
            if (searchable)
              IconButton(
                tooltip: 'Search $title',
                onPressed: () => setState(() => _searchOpen = true),
                icon: const Icon(Icons.search),
              ),
          ],
        ),
        ...recipients.map(_recipientRow),
      ],
    );
  }

  Widget _recipientRow(ForwardRecipient recipient) {
    final selected = recipient.id == _selectedRecipientId;
    return Semantics(
      selected: selected,
      button: true,
      child: ListTile(
        contentPadding: EdgeInsets.zero,
        leading: _RecipientAvatar(recipient: recipient),
        title: Text(recipient.displayName),
        trailing: selected
            ? const Icon(Icons.check_circle_outline)
            : const Icon(Icons.radio_button_unchecked),
        onTap: () {
          setState(() => _selectedRecipientId = recipient.id);
          widget.onRecipientSelected(recipient);
        },
      ),
    );
  }

  Widget _buildSearchOverlay(BuildContext context) {
    final colors = Theme.of(context).colorScheme;
    return Positioned.fill(
      child: ColoredBox(
        color: colors.scrim.withValues(alpha: 0.45),
        child: Align(
          alignment: Alignment.topCenter,
          child: SafeArea(
            minimum: const EdgeInsets.fromLTRB(20, 12, 20, 0),
            child: Material(
              color: colors.surfaceContainerHighest,
              elevation: 8,
              borderRadius: BorderRadius.circular(20),
              clipBehavior: Clip.antiAlias,
              child: Padding(
                padding: const EdgeInsets.fromLTRB(12, 8, 8, 12),
                child: Column(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    TextField(
                      autofocus: true,
                      controller: _searchController,
                      onChanged: widget.onSearchChanged,
                      decoration: InputDecoration(
                        hintText: 'Search recipients',
                        prefixIcon: const Icon(Icons.search),
                        suffixIcon: IconButton(
                          tooltip: 'Close search',
                          onPressed: _closeSearch,
                          icon: const Icon(Icons.close),
                        ),
                      ),
                    ),
                    const SizedBox(height: 8),
                    ...ForwardSearchFilter.values.map(
                      (filter) => ListTile(
                        dense: true,
                        leading: const Icon(Icons.search),
                        title: Text(_filterLabel(filter)),
                        onTap: () {
                          widget.onSearchFilterSelected?.call(filter);
                          _closeSearch();
                        },
                      ),
                    ),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }

  void _closeSearch() {
    _searchController.clear();
    widget.onSearchChanged('');
    setState(() => _searchOpen = false);
  }

  String _filterLabel(ForwardSearchFilter filter) => switch (filter) {
    ForwardSearchFilter.user => 'User',
    ForwardSearchFilter.group => 'Group',
    ForwardSearchFilter.id => 'ID',
    ForwardSearchFilter.chat => 'Chat',
    ForwardSearchFilter.trustedVerified => 'Trusted / verified',
  };
}

class _RecipientAvatar extends StatelessWidget {
  const _RecipientAvatar({required this.recipient});

  final ForwardRecipient recipient;

  @override
  Widget build(BuildContext context) {
    if (recipient.kind == ForwardRecipientKind.public) {
      return const CircleAvatar(child: Icon(Icons.campaign_outlined));
    }

    final avatar = CircleAvatar(
      backgroundColor: recipient.avatarColor,
      child: recipient.kind == ForwardRecipientKind.group
          ? const Icon(Icons.groups, color: Colors.white)
          : Text(
              _avatarLabel(recipient),
              style: const TextStyle(color: Colors.white),
            ),
    );

    if (!recipient.isOnline) return avatar;
    return SizedBox.square(
      dimension: 40,
      child: Stack(
        clipBehavior: Clip.none,
        children: [
          Positioned.fill(child: avatar),
          Positioned(
            right: -1,
            bottom: -1,
            child: Container(
              key: const ValueKey('forward-recipient-online-indicator'),
              width: 12,
              height: 12,
              decoration: BoxDecoration(
                color: Colors.greenAccent.shade700,
                shape: BoxShape.circle,
                border: Border.all(
                  color: Theme.of(context).scaffoldBackgroundColor,
                  width: 2,
                ),
              ),
            ),
          ),
        ],
      ),
    );
  }

  String _avatarLabel(ForwardRecipient recipient) {
    final initials = recipient.initials?.trim();
    if (initials != null && initials.isNotEmpty) return initials;

    final displayName = recipient.displayName.trim();
    return displayName.isEmpty ? '?' : displayName.characters.first;
  }
}
