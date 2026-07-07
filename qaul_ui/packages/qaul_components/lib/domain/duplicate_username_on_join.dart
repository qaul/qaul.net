/// Pure domain logic for duplicate-username detection when a member joins a group.
///
/// UI-free and app-free: callers translate their own room/message models into
/// the snapshot types below, and translate the resulting notifications into
/// whatever presentation layer they use.
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

  /// When false (e.g. a pending, not-yet-accepted invite), the member is
  /// ignored for collision checks.
  final bool countsTowardDuplicateCheck;
}

/// A join event extracted from the persisted group timeline.
class GroupJoinEventSnapshot {
  const GroupJoinEventSnapshot({
    required this.messageIdBase58,
    required this.userIdBase58,
    required this.userName,
    required this.sentAt,
    required this.receivedAt,
    this.skipJoinedAnnouncement = false,
  });

  final String messageIdBase58;
  final String userIdBase58;
  final String userName;

  /// When the join was sent. Defines the order in which joins are evaluated,
  /// so a collision is judged against who was already present at that point.
  final DateTime sentAt;
  final DateTime receivedAt;

  /// Mirrors the UI rule that suppresses "joined" for pending-invite members.
  final bool skipJoinedAnnouncement;
}

/// Synthetic metadata to surface after a join when display names collide.
class DuplicateUsernameOnJoinNotification {
  const DuplicateUsernameOnJoinNotification({
    required this.afterMessageIdBase58,
    required this.joiningUserIdBase58,
    required this.baseName,
    required this.disambiguatedName,
    required this.receivedAt,
  });

  /// The join message this notification should be rendered directly after.
  final String afterMessageIdBase58;
  final String joiningUserIdBase58;
  final String baseName;
  final String disambiguatedName;
  final DateTime receivedAt;

  /// Stable id for the synthetic timeline row derived from the join.
  String get syntheticMessageIdBase58 => 'dup-username:$afterMessageIdBase58';
}

/// NFKC-normalized, trimmed, lowercased key used for display-name equality.
String normalizeGroupDisplayNameForComparison(String name) =>
    unorm.nfkc(name.trim()).toLowerCase();

/// Whether [a] and [b] are the same display name (NFKC + case-insensitive).
bool groupDisplayNamesCollide(String a, String b) =>
    normalizeGroupDisplayNameForComparison(a) ==
    normalizeGroupDisplayNameForComparison(b);

/// Builds a local disambiguated label for [baseName] using a short suffix of
/// [userIdBase58] (best-effort — not guaranteed globally unique).
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

/// Detects joins whose display name was *already* held when they occurred, and
/// flags only the later joiner — never the member who held the name first.
///
/// The collision is judged point-in-time by walking joins in send order and
/// tracking which active members hold each name, rather than comparing against
/// the final roster (which would retroactively flag the original holder and be
/// sensitive to who happens to remain a member now).
List<DuplicateUsernameOnJoinNotification> detectDuplicateUsernameOnJoin({
  required List<GroupJoinEventSnapshot> joins,
  required List<GroupMemberNameSnapshot> members,
}) {
  final countingMemberIds = {
    for (final m in members)
      if (m.countsTowardDuplicateCheck) m.idBase58,
  };
  final joinUserIds = {for (final j in joins) j.userIdBase58};

  // normalized display name -> ids of active members currently holding it.
  final holdersByName = <String, Set<String>>{};
  void claim(String userId, String rawName) {
    final key = normalizeGroupDisplayNameForComparison(rawName);
    if (key.isEmpty) return;
    (holdersByName[key] ??= <String>{}).add(userId);
  }

  // Seed members who predate the visible join timeline (e.g. the group
  // creator, who has no join event): they hold their name from the start.
  for (final m in members) {
    if (m.countsTowardDuplicateCheck && !joinUserIds.contains(m.idBase58)) {
      claim(m.idBase58, m.name);
    }
  }

  // Ascending send order: a join can only collide with names already present.
  final orderedJoins = joins.toList()
    ..sort((a, b) => a.sentAt.compareTo(b.sentAt));

  final notifications = <DuplicateUsernameOnJoinNotification>[];
  for (final join in orderedJoins) {
    if (join.skipJoinedAnnouncement) continue;

    final joiningName = join.userName.trim();
    if (joiningName.isEmpty) continue;

    final key = normalizeGroupDisplayNameForComparison(joiningName);
    final priorHolders = holdersByName[key];
    final collidesWithOther =
        priorHolders != null &&
        priorHolders.any((id) => id != join.userIdBase58);

    // Only members still in the group keep holding their name for later joins;
    // a joiner who has since left can't cause a downstream collision.
    if (countingMemberIds.contains(join.userIdBase58)) {
      claim(join.userIdBase58, joiningName);
    }

    if (!collidesWithOther) continue;

    notifications.add(
      DuplicateUsernameOnJoinNotification(
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
