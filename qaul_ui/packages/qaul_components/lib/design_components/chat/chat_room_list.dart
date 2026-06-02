import 'package:flutter/material.dart';

/// Scrollable chat room list with an optional search field, pull-to-refresh,
/// and loading / empty states. Room tiles are built via [itemBuilder].
class ChatRoomList extends StatelessWidget {
  const ChatRoomList({
    super.key,
    required this.itemCount,
    required this.itemBuilder,
    required this.onRefresh,
    this.scrollController,
    this.searchHint,
    this.searchController,
    this.onQueryChanged,
    this.onClear,
    this.isLoading = false,
    this.isEmpty = false,
    this.emptyMessage,
    this.separatorBuilder,
  });

  final int itemCount;
  final IndexedWidgetBuilder itemBuilder;
  final Future<void> Function() onRefresh;
  final ScrollController? scrollController;
  final String? searchHint;
  final TextEditingController? searchController;
  final ValueChanged<String>? onQueryChanged;
  final VoidCallback? onClear;
  final bool isLoading;
  final bool isEmpty;
  final String? emptyMessage;
  final IndexedWidgetBuilder? separatorBuilder;

  bool get _showsSearch =>
      searchHint != null && onQueryChanged != null && onClear != null;

  @override
  Widget build(BuildContext context) {
    final list = RefreshIndicator(
      onRefresh: onRefresh,
      child: isEmpty && emptyMessage != null
          ? ListView(
              controller: scrollController,
              physics: const AlwaysScrollableScrollPhysics(),
              children: [
                if (_showsSearch) _SearchField(
                  hint: searchHint!,
                  controller: searchController,
                  onChanged: onQueryChanged!,
                  onClear: onClear!,
                ),
                SizedBox(
                  height: MediaQuery.sizeOf(context).height * 0.5,
                  child: Center(child: Text(emptyMessage!)),
                ),
              ],
            )
          : ListView.separated(
              controller: scrollController,
              physics: const AlwaysScrollableScrollPhysics(),
              itemCount: itemCount + (_showsSearch ? 1 : 0),
              separatorBuilder: (context, index) {
                if (_showsSearch && index == 0) {
                  return const SizedBox.shrink();
                }
                final itemIndex = _showsSearch ? index - 1 : index;
                if (separatorBuilder != null) {
                  return separatorBuilder!(context, itemIndex);
                }
                return const Divider(height: 12.0);
              },
              itemBuilder: (context, index) {
                if (_showsSearch && index == 0) {
                  return _SearchField(
                    hint: searchHint!,
                    controller: searchController,
                    onChanged: onQueryChanged!,
                    onClear: onClear!,
                  );
                }
                final itemIndex = _showsSearch ? index - 1 : index;
                return itemBuilder(context, itemIndex);
              },
            ),
    );

    if (!isLoading) return list;

    return Stack(
      children: [
        list,
        const Positioned(
          left: 0,
          right: 0,
          top: 0,
          child: LinearProgressIndicator(minHeight: 2),
        ),
      ],
    );
  }
}

class _SearchField extends StatelessWidget {
  const _SearchField({
    required this.hint,
    required this.onChanged,
    required this.onClear,
    this.controller,
  });

  final String hint;
  final TextEditingController? controller;
  final ValueChanged<String> onChanged;
  final VoidCallback onClear;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 8, 8, 4),
      child: TextField(
        controller: controller,
        decoration: InputDecoration(
          prefixIcon: const Icon(Icons.search),
          hintText: hint,
          border: const UnderlineInputBorder(),
          suffixIcon: IconButton(
            onPressed: () {
              controller?.clear();
              onClear();
            },
            splashRadius: 16,
            icon: const Icon(Icons.clear_rounded),
          ),
        ),
        onChanged: onChanged,
      ),
    );
  }
}
