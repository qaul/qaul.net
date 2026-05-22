import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/domain/duplicate_username_on_join.dart';

void main() {
  group('normalizeGroupDisplayNameForComparison', () {
    test('applies NFKC before lowercasing', () {
      const composed = 'café';
      const decomposed = 'cafe\u0301';

      expect(
        normalizeGroupDisplayNameForComparison(composed),
        normalizeGroupDisplayNameForComparison(decomposed),
      );
    });
  });

  group('groupDisplayNamesCollide', () {
    test('matches case-insensitively', () {
      expect(groupDisplayNamesCollide('Alice', 'alice'), isTrue);
      expect(groupDisplayNamesCollide(' Bob ', 'bob'), isTrue);
      expect(groupDisplayNamesCollide('Alice', 'Bob'), isFalse);
    });

    test('matches composed and decomposed unicode forms', () {
      expect(groupDisplayNamesCollide('café', 'cafe\u0301'), isTrue);
    });
  });

  group('disambiguateGroupDisplayName', () {
    test('appends a short id suffix', () {
      expect(
        disambiguateGroupDisplayName(
          baseName: 'Nickname',
          userIdBase58: 'QmNickname3fr',
        ),
        'Nickname 3fr',
      );
    });
  });

  group('detectDuplicateUsernameOnJoin', () {
    final members = [
      const GroupMemberNameSnapshot(
        idBase58: 'admin',
        name: 'Nickname',
      ),
      const GroupMemberNameSnapshot(
        idBase58: 'QmNickname3fr',
        name: 'Nickname',
      ),
    ];

    test('emits metadata when joiner collides with an existing member', () {
      final joins = [
        GroupJoinEventSnapshot(
          messageIdBase58: 'join-1',
          userIdBase58: 'QmNickname3fr',
          userName: 'Nickname',
          receivedAt: DateTime(2026, 4, 17, 10),
        ),
      ];

      final result = detectDuplicateUsernameOnJoin(
        joins: joins,
        members: members,
      );

      expect(result, hasLength(1));
      expect(result.first.afterMessageIdBase58, 'join-1');
      expect(result.first.baseName, 'Nickname');
      expect(result.first.disambiguatedName, 'Nickname 3fr');
      expect(result.first.syntheticMessageIdBase58, 'dup-username:join-1');
    });

    test('does not emit when names are unique', () {
      final joins = [
        GroupJoinEventSnapshot(
          messageIdBase58: 'join-2',
          userIdBase58: 'unique-user',
          userName: 'Unique',
          receivedAt: DateTime(2026, 4, 17, 11),
        ),
      ];

      final result = detectDuplicateUsernameOnJoin(
        joins: joins,
        members: [
          ...members,
          const GroupMemberNameSnapshot(
            idBase58: 'unique-user',
            name: 'Unique',
          ),
        ],
      );

      expect(result, isEmpty);
    });

    test('ignores members that do not count toward duplicate checks', () {
      final joins = [
        GroupJoinEventSnapshot(
          messageIdBase58: 'join-3',
          userIdBase58: 'QmNickname3fr',
          userName: 'Nickname',
          receivedAt: DateTime(2026, 4, 17, 12),
        ),
      ];

      final result = detectDuplicateUsernameOnJoin(
        joins: joins,
        members: [
          const GroupMemberNameSnapshot(
            idBase58: 'admin',
            name: 'Nickname',
            countsTowardDuplicateCheck: false,
          ),
          const GroupMemberNameSnapshot(
            idBase58: 'QmNickname3fr',
            name: 'Nickname',
          ),
        ],
      );

      expect(result, isEmpty);
    });

    test('skips joins flagged as invite-only announcements', () {
      final joins = [
        GroupJoinEventSnapshot(
          messageIdBase58: 'join-4',
          userIdBase58: 'QmNickname3fr',
          userName: 'Nickname',
          receivedAt: DateTime(2026, 4, 17, 13),
          skipJoinedAnnouncement: true,
        ),
      ];

      final result = detectDuplicateUsernameOnJoin(
        joins: joins,
        members: members,
      );

      expect(result, isEmpty);
    });
  });
}
