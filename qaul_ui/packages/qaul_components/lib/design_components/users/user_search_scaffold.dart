import 'package:flutter/material.dart';

/// Shell for screens that search users: optional [title] app bar, search field,
/// and a [body] driven by the parent (list, loading, etc.).
class UserSearchScaffold extends StatelessWidget {
  const UserSearchScaffold({
    super.key,
    required this.searchHint,
    required this.onQueryChanged,
    required this.onClear,
    required this.body,
    this.title,
    this.leading,
    this.controller,
  });

  final String? title;
  final String searchHint;
  final Widget? leading;
  final TextEditingController? controller;
  final ValueChanged<String> onQueryChanged;
  final VoidCallback onClear;
  final Widget body;

  @override
  Widget build(BuildContext context) {
    final topPadding = MediaQuery.paddingOf(context).top;
    final searchBar = PreferredSize(
      preferredSize: Size(double.maxFinite, 40 + topPadding),
      child: SafeArea(
        top: true,
        bottom: false,
        left: false,
        right: false,
        child: SizedBox(
          height: 40,
          child: TextField(
            controller: controller,
            decoration: InputDecoration(
              prefixIcon: const Icon(Icons.search),
              hintText: searchHint,
              border: const UnderlineInputBorder(),
              focusedBorder: const UnderlineInputBorder(
                borderSide: BorderSide(color: Colors.white),
              ),
              suffixIcon: IconButton(
                onPressed: () {
                  controller?.clear();
                  onClear();
                },
                splashRadius: 16,
                icon: const Icon(Icons.clear_rounded),
              ),
            ),
            onChanged: onQueryChanged,
          ),
        ),
      ),
    );

    return Scaffold(
      appBar: title == null
          ? searchBar
          : AppBar(
              title: Text(title!),
              centerTitle: false,
              leading: leading,
              bottom: searchBar,
            ),
      body: body,
    );
  }
}
