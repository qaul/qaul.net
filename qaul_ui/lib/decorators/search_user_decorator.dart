import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

import '../l10n/app_localizations.dart';
import '../stores/stores.dart';

import '../widgets/widgets.dart';

typedef SearchUserResultBuilder = Widget Function(
    BuildContext context, List<User> users);

List<User> _localUsersForPicker(WidgetRef ref) {
  final defaultUser = ref.watch(defaultUserProvider)!;
  return ref
      .watch(usersStoreProvider)
      .where((u) => !u.id.equals(defaultUser.id) && !(u.isBlocked ?? false))
      .toList()
    ..sort();
}

class SearchUserDecorator extends HookConsumerWidget {
  const SearchUserDecorator({super.key, required this.builder, this.title});
  final String? title;
  final SearchUserResultBuilder builder;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final controller = useTextEditingController();

    useEffect(() {
      return () => ref.read(usersSearchProvider.notifier).clear();
    }, const []);

    final search = ref.watch(usersSearchProvider);
    final users = search.isActive ? search.results : _localUsersForPicker(ref);

    final l10n = AppLocalizations.of(context)!;
    return UserSearchScaffold(
      title: title,
      leading: title == null ? null : const IconButtonFactory(),
      searchHint: l10n.searchUser,
      controller: controller,
      onQueryChanged: ref.read(usersSearchProvider.notifier).setQuery,
      onClear: () {
        controller.clear();
        ref.read(usersSearchProvider.notifier).clear();
      },
      body: builder(context, users),
    );
  }
}
