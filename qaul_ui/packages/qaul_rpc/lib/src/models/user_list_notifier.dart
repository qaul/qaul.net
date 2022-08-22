import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../qaul_rpc.dart';

class UserListNotifier extends StateNotifier<List<User>> {
  UserListNotifier({List<User>? users}) : super(users ?? []);

  void add(User u) {
    state = [...state, u];
  }

  void update(User u) {
    state = [
      for (final usr in state)
        if (usr.id == u.id || usr.idBase58 == u.idBase58)
          User(
            name: usr.name == 'Name Undefined' ? u.name : usr.name,
            id: u.id,
            conversationId: u.conversationId ?? usr.conversationId,
            status:
            u.status == ConnectionStatus.offline ? usr.status : u.status,
            keyBase58: u.keyBase58 ?? usr.keyBase58,
            isBlocked: u.isBlocked ?? usr.isBlocked,
            isVerified: u.isVerified ?? usr.isVerified,
            availableTypes: u.availableTypes ?? usr.availableTypes,
          )
        else
          usr,
    ];
  }

  bool contains(User usr) => !state
      .indexWhere((u) => u.id == usr.id || u.idBase58 == usr.idBase58)
      .isNegative;
}
