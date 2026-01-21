import 'dart:typed_data';

import 'package:equatable/equatable.dart';
import 'package:fast_base58/fast_base58.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

import '../../qaul_rpc.dart';
import '../generated/services/group/group_rpc.pb.dart';

final groupInvitesProvider =
    NotifierProvider<GroupInviteListNotifier, List<GroupInvite>>(
        GroupInviteListNotifier.new);

class GroupInvite extends Equatable {
  GroupInvite({
    required this.senderId,
    required this.receivedAt,
    required this.groupDetails,
  }) : senderIdBase58 = Base58Encode(senderId);

  final Uint8List senderId;
  final DateTime receivedAt;
  final ChatRoom groupDetails;

  final String senderIdBase58;

  factory GroupInvite.fromRpcGroupInvited(GroupInvited i, List<User> users) {
    return GroupInvite(
      senderId: Uint8List.fromList(i.senderId),
      receivedAt: DateTime.fromMillisecondsSinceEpoch(i.receivedAt.toInt()),
      groupDetails: ChatRoom.fromRpcGroupInfo(i.group, users),
    );
  }

  @override
  List<Object?> get props => [senderIdBase58, groupDetails.idBase58];
}

class GroupInviteListNotifier extends Notifier<List<GroupInvite>> {
  @override
  List<GroupInvite> build() => [];

  void add(GroupInvite invite) => state = [invite, ...state];

  void update(GroupInvite invite) {
    assert(contains(invite), 'State does not contain invite $invite');
    final filtered = state.where((r) => r != invite);
    state = [invite, ...filtered];
  }

  bool contains(GroupInvite invite) => state.contains(invite);

  void filterInvitesNotIn(List<GroupInvite> list) {
    state = [...state.where((invite) => list.contains(invite))];
  }
}
