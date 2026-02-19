import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:hooks_riverpod/legacy.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../l10n/app_localizations.dart';

import '../widgets/widgets.dart';

typedef SearchUserResultBuilder = Widget Function(
    BuildContext context, List<User> users);

final _searchKeyProvider = StateProvider.autoDispose<String>((ref) {
  return '';
});

final _userSearchProvider = StateProvider.autoDispose<List<User>>((ref) {
  final defaultUser = ref.watch(defaultUserProvider)!;
  final users = ref
      .watch(usersProvider)
      .where((u) => !u.id.equals(defaultUser.id) && !(u.isBlocked ?? false))
      .toList()
    ..sort();

  final key = ref.watch(_searchKeyProvider).toLowerCase();
  if (key.isEmpty) return users;

  return users.where((user) => user.name.toLowerCase().contains(key)).toList();
});

class SearchUserDecorator extends HookConsumerWidget {
  const SearchUserDecorator({super.key, required this.builder, this.title});
  final String? title;
  final SearchUserResultBuilder builder;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();
    final searchKeyNotifier = _searchKeyProvider.notifier;

    final l10n = AppLocalizations.of(context)!;
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
          child: Padding(
            padding: EdgeInsets.zero,
            child: TextField(
          controller: controller,
          decoration: InputDecoration(
            prefixIcon: const Icon(Icons.search),
            hintText: l10n.searchUser,
            border: const UnderlineInputBorder(),
            focusedBorder: const UnderlineInputBorder(
              borderSide: BorderSide(color: Colors.white),
            ),
            suffixIcon: IconButton(
              onPressed: () {
                controller.clear();
                ref.read(searchKeyNotifier).state = '';
              },
              splashRadius: 16,
              icon: const Icon(Icons.clear_rounded),
            ),
          ),
          onChanged: (val) => ref.read(searchKeyNotifier).state = val,
            ),
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
              leading: const IconButtonFactory(),
              bottom: searchBar,
            ),
      body: Consumer(
        builder: (context, ref, _) {
          final users = ref.watch(_userSearchProvider.notifier).state;
          return builder(context, users);
        },
      ),
    );
  }
}
