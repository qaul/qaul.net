import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/domain/duplicate_username_on_join.dart';

void main() {
  // "cafe" + precomposed accented e (U+00E9) vs. ASCII 'e' + combining acute
  // (U+0301). Built from code points so the two byte sequences stay distinct
  // regardless of how this source file is normalized on disk.
  final composedCafe = String.fromCharCodes([0x63, 0x61, 0x66, 0x00E9]);
  final decomposedCafe = String.fromCharCodes([0x63, 0x61, 0x66, 0x65, 0x0301]);

  group('normalizeGroupDisplayNameForComparison', () {
    test('applies NFKC before lowercasing', () {
      expect(
        normalizeGroupDisplayNameForComparison(composedCafe),
        normalizeGroupDisplayNameForComparison(decomposedCafe),
      );
    });
  });

  group('groupDisplayNamesCollide', () {
    test('matches case-insensitively and trims', () {
      expect(groupDisplayNamesCollide('Alice', 'alice'), isTrue);
      expect(groupDisplayNamesCollide(' Bob ', 'bob'), isTrue);
      expect(groupDisplayNamesCollide('Alice', 'Bob'), isFalse);
    });

    test('matches composed and decomposed unicode forms', () {
      expect(groupDisplayNamesCollide(composedCafe, decomposedCafe), isTrue);
    });
  });

  group('disambiguateGroupDisplayName', () {
    test('appends the last three id characters', () {
      expect(
        disambiguateGroupDisplayName(
          baseName: 'Nickname',
          userIdBase58: 'QmNickname3fr',
        ),
        'Nickname 3fr',
      );
    });

    test('uses the whole id when it is three characters or shorter', () {
      expect(
        disambiguateGroupDisplayName(baseName: 'X', userIdBase58: 'ab'),
        'X ab',
      );
    });
  });

  group('detectDuplicateUsernameOnJoin', () {
    GroupMemberNameSnapshot member(
      String id,
      String name, {
      bool counts = true,
    }) => GroupMemberNameSnapshot(
      idBase58: id,
      name: name,
      countsTowardDuplicateCheck: counts,
    );

    GroupJoinEventSnapshot join(
      String msgId,
      String userId,
      String name, {
      bool skip = false,
      DateTime? sentAt,
    }) => GroupJoinEventSnapshot(
      messageIdBase58: msgId,
      userIdBase58: userId,
      userName: name,
      sentAt: sentAt ?? DateTime(2026, 1, 1),
      receivedAt: sentAt ?? DateTime(2026, 1, 1),
      skipJoinedAnnouncement: skip,
    );

    test('flags a join whose name collides with another active member', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob2', 'Bob')],
        members: [member('uBob1', 'Bob'), member('uBob2', 'Bob')],
      );

      expect(result, hasLength(1));
      expect(result.single.joiningUserIdBase58, 'uBob2');
      expect(result.single.baseName, 'Bob');
      expect(result.single.disambiguatedName, 'Bob ob2');
      expect(result.single.afterMessageIdBase58, 'm1');
      expect(result.single.syntheticMessageIdBase58, 'dup-username:m1');
    });

    test('flags only the later joiner when two members share a name', () {
      // Both Bob1 and Bob2 have join events and both remain members. The
      // original holder's earlier join must not be retroactively flagged.
      final result = detectDuplicateUsernameOnJoin(
        joins: [
          join('m0', 'uBob1', 'Bob', sentAt: DateTime(2026, 1, 1, 9)),
          join('m1', 'uBob2', 'Bob', sentAt: DateTime(2026, 1, 1, 10)),
        ],
        members: [member('uBob1', 'Bob'), member('uBob2', 'Bob')],
      );

      expect(result, hasLength(1));
      expect(result.single.joiningUserIdBase58, 'uBob2');
      expect(result.single.afterMessageIdBase58, 'm1');
    });

    test('evaluates joins in send order regardless of input order', () {
      // The room timeline is delivered newest-first; the domain must still
      // leave the earlier joiner alone and flag only the later one.
      final result = detectDuplicateUsernameOnJoin(
        joins: [
          join('m1', 'uBob2', 'Bob', sentAt: DateTime(2026, 1, 1, 10)),
          join('m0', 'uBob1', 'Bob', sentAt: DateTime(2026, 1, 1, 9)),
        ],
        members: [member('uBob1', 'Bob'), member('uBob2', 'Bob')],
      );

      expect(result, hasLength(1));
      expect(result.single.joiningUserIdBase58, 'uBob2');
    });

    test('flags a joiner colliding with a member that has no join event', () {
      // The pre-existing holder (e.g. group creator) predates the visible
      // join timeline and is seeded from the roster.
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob2', 'Bob', sentAt: DateTime(2026, 1, 1, 10))],
        members: [member('uCreator', 'Bob'), member('uBob2', 'Bob')],
      );

      expect(result, hasLength(1));
      expect(result.single.joiningUserIdBase58, 'uBob2');
    });

    test('does not flag a name reused after its earlier holder left', () {
      // Bob1 joined then left (absent from the roster); Bob2 later reuses the
      // name. With no live holder, there is no collision.
      final result = detectDuplicateUsernameOnJoin(
        joins: [
          join('m0', 'uBob1', 'Bob', sentAt: DateTime(2026, 1, 1, 9)),
          join('m1', 'uBob2', 'Bob', sentAt: DateTime(2026, 1, 1, 10)),
        ],
        members: [member('uBob2', 'Bob')],
      );

      expect(result, isEmpty);
    });

    test('does not flag a unique name', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uCarol', 'Carol')],
        members: [member('uBob', 'Bob'), member('uCarol', 'Carol')],
      );
      expect(result, isEmpty);
    });

    test('never collides a member with itself', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob', 'Bob')],
        members: [member('uBob', 'Bob')],
      );
      expect(result, isEmpty);
    });

    test('ignores pending-invite members when checking collisions', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob2', 'Bob')],
        members: [
          member('uBob1', 'Bob', counts: false),
          member('uBob2', 'Bob'),
        ],
      );
      expect(result, isEmpty);
    });

    test('skips joins marked as suppressed announcements', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob2', 'Bob', skip: true)],
        members: [member('uBob1', 'Bob'), member('uBob2', 'Bob')],
      );
      expect(result, isEmpty);
    });

    test('skips joins with a blank display name', () {
      final result = detectDuplicateUsernameOnJoin(
        joins: [join('m1', 'uBob2', '   ')],
        members: [member('uBob1', '   '), member('uBob2', '   ')],
      );
      expect(result, isEmpty);
    });
  });
}
