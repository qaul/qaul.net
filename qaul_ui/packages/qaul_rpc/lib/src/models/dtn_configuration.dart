import 'package:collection/collection.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../generated/services/dtn/dtn_rpc.pb.dart';
import 'models.dart';

final dtnConfigurationProvider =
    NotifierProvider<DtnConfigurationNotifier, DTNConfiguration?>(
        DtnConfigurationNotifier.new);

class DtnConfigurationNotifier extends Notifier<DTNConfiguration?> {
  @override
  DTNConfiguration? build() => null;
}

class DTNConfiguration {
  DTNConfiguration._(this.totalSize, this.users);

  final int totalSize;
  final List<User> users;

  factory DTNConfiguration.fromRpcConfigResponse(
    DtnConfigResponse res,
    List<User> users,
  ) {
    final usrs = <User>[];
    for (final id in res.users) {
      final usr = users.firstWhereOrNull((element) => element.id.equals(id));
      if (usr != null) usrs.add(usr);
    }

    return DTNConfiguration._(res.totalSize, usrs.toList());
  }
}
