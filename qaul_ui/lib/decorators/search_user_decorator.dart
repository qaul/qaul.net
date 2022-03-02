import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../widgets/widgets.dart';

typedef SearchUserResultBuilder = Widget Function(BuildContext context, List<User> users);

final searchKeyProvider = StateProvider.autoDispose<String>((ref) {
  return '';
});

final _userSearchProvider = StateProvider.autoDispose<List<User>>((ref) {
  final defaultUser = ref.watch(defaultUserProvider)!;
  final chatRooms = ref.watch(chatRoomsProvider);
  final users = ref
      .watch(usersProvider)
      .where((u) =>
          !u.id.equals(defaultUser.id) &&
          !(u.isBlocked ?? false) &&
          chatRooms.indexWhere((c) => c.conversationId.equals(u.id)).isNegative)
      .toList()
    ..sort();

  final key = ref.watch(searchKeyProvider).toLowerCase();
  if (key.isEmpty) return users;

  return users.where((user) => user.name.toLowerCase().contains(key)).toList();
});

class SearchUserDecorator extends HookConsumerWidget {
  const SearchUserDecorator({
    Key? key,
    required this.title,
    required this.builder,
  }) : super(key: key);
  final String title;
  final SearchUserResultBuilder builder;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(
        title: Text(title),
        centerTitle: false,
        leading: const DefaultBackButton(),
        bottom: PreferredSize(
          preferredSize: const Size(double.maxFinite, 40),
          child: Padding(
            padding: EdgeInsets.zero,
            child: TextFormField(
              decoration: const InputDecoration(
                hintText: 'Search user...',
                border: UnderlineInputBorder(),
                focusedBorder: UnderlineInputBorder(
                  borderSide: BorderSide(color: Colors.white),
                ),
                contentPadding: EdgeInsets.symmetric(horizontal: 20),
              ),
              onChanged: (newValue) {
                ref.read(searchKeyProvider.notifier).state = newValue;
              },
            ),
          ),
        ),
      ),
      body: Consumer(
        builder: (context, ref, ___) {
          final users = ref.watch(_userSearchProvider.notifier).state;
          return builder(context, users);
        },
      ),
    );
  }
}
