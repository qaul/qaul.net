/// Pure domain logic for duplicate-username detection when a member joins a group.
library;

import 'package:unorm_dart/unorm_dart.dart' as unorm;

/// A group member considered for name-collision checks at join time.
class GroupMemberNameSnapshot {
  const GroupMemberNameSnapshot({
    required this.idBase58,
    required this.name,
    this.countsTowardDuplicateCheck = true,
  });

  final String idBase58;
  final String name;

  /// When false (e.g. pending invite only), the member is ignored for collisions.
  final bool countsTowardDuplicateCheck;
}

/// A join event extracted from the persisted group timeline.
class GroupJoinEventSnapshot {
  const GroupJoinEventSnapshot({
    required this.messageIdBase58,
    required this.userIdBase58,
    required this.userName,
    required this.receivedAt,
    this.skipJoinedAnnouncement = false,
  });

  final String messageIdBase58;
  final String userIdBase58;
  final String userName;
  final DateTime receivedAt;

  /// Mirrors UI rules that suppress "joined" for invite-only members.
  final bool skipJoinedAnnouncement;
}

/// Synthetic metadata appended after a join when names collide.
class DuplicateUsernameOnJoinNotification {
  const DuplicateUsernameOnJoinNotification({
    required this.syntheticMessageIdBase58,
    required this.afterMessageIdBase58,
    required this.joiningUserIdBase58,
    required this.baseName,
    required this.disambiguatedName,
    required this.receivedAt,
  });

  final String syntheticMessageIdBase58;
  final String afterMessageIdBase58;
  final String joiningUserIdBase58;
  final String baseName;
  final String disambiguatedName;
  final DateTime receivedAt;
}

/// NFKC-normalized, trimmed, lowercased key used for display-name equality.
String normalizeGroupDisplayNameForComparison(String name) =>
    unorm.nfkc(name.trim()).toLowerCase();

/// Returns true when [a] and [b] are the same display name (NFKC + case-insensitive).
bool groupDisplayNamesCollide(String a, String b) =>
    normalizeGroupDisplayNameForComparison(a) ==
    normalizeGroupDisplayNameForComparison(b);

/// Builds a local disambiguated label for [baseName] using a short [userIdBase58] suffix.
String disambiguateGroupDisplayName({
  required String baseName,
  required String userIdBase58,
}) {
  final trimmedId = userIdBase58.trim();
  final suffix = trimmedId.length <= 3
      ? trimmedId
      : trimmedId.substring(trimmedId.length - 3);
  return '$baseName $suffix';
}

/// Detects join events whose username already exists among other active members.
List<DuplicateUsernameOnJoinNotification> detectDuplicateUsernameOnJoin({
  required List<GroupJoinEventSnapshot> joins,
  required List<GroupMemberNameSnapshot> members,
}) {
  final activeMembers = members
      .where((m) => m.countsTowardDuplicateCheck)
      .toList(growable: false);

  final notifications = <DuplicateUsernameOnJoinNotification>[];

  for (final join in joins) {
    if (join.skipJoinedAnnouncement) continue;

    final joiningName = join.userName.trim();
    if (joiningName.isEmpty) continue;

    final hasCollision = activeMembers.any(
      (member) =>
          member.idBase58 != join.userIdBase58 &&
          groupDisplayNamesCollide(member.name, joiningName),
    );
    if (!hasCollision) continue;

    notifications.add(
      DuplicateUsernameOnJoinNotification(
        syntheticMessageIdBase58: 'dup-username:${join.messageIdBase58}',
        afterMessageIdBase58: join.messageIdBase58,
        joiningUserIdBase58: join.userIdBase58,
        baseName: joiningName,
        disambiguatedName: disambiguateGroupDisplayName(
          baseName: joiningName,
          userIdBase58: join.userIdBase58,
        ),
        receivedAt: join.receivedAt,
      ),
    );
  }

  return notifications;
}
