import 'package:collection/collection.dart';
import 'package:hooks_riverpod/legacy.dart';

import '../generated/services/dtn/dtn_rpc.pb.dart';
import 'models.dart';

final dtnConfigurationProvider = StateProvider<DTNConfiguration?>((_) => null);

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
