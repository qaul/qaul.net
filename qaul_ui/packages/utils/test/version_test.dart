import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  group('Version', () {
    group('constructor', () {
      test('creates version with correct properties', () {
        final version = Version(1, 2, 3);
        expect(version.major, 1);
        expect(version.minor, 2);
        expect(version.patch, 3);
        expect(version.build, '');
      });

      test('creates version with preRelease and build', () {
        final version = Version(2, 0, 0, preRelease: ['beta'], build: '18');
        expect(version.major, 2);
        expect(version.minor, 0);
        expect(version.patch, 0);
        expect(version.build, '18');
      });

      test('validates and rejects invalid inputs', () {
        expect(() => Version(-1, 0, 0), throwsArgumentError);
        expect(() => Version(0, -1, 0), throwsArgumentError);
        expect(() => Version(0, 0, -1), throwsArgumentError);
        expect(() => Version(1, 0, 0, preRelease: ['beta.1']), throwsFormatException);
        expect(() => Version(1, 0, 0, preRelease: ['']), throwsArgumentError);
        expect(() => Version(1, 0, 0, build: 'build@123'), throwsFormatException);
      });
    });

    group('toString', () {
      test('formats basic version correctly', () {
        expect(Version(1, 2, 3).toString(), '1.2.3');
      });

      test('formats version with preRelease and build', () {
        final testCases = [
          (Version(1, 0, 0, preRelease: ['beta']), '1.0.0-beta'),
          (Version(1, 0, 0, build: '18'), '1.0.0+18'),
          (Version(2, 0, 0, preRelease: ['beta'], build: '18'), '2.0.0-beta+18'),
          (Version(1, 0, 0, preRelease: ['beta', '1']), '1.0.0-beta.1'),
        ];

        for (final (version, expected) in testCases) {
          expect(version.toString(), expected);
        }
      });
    });

    group('parse and round-trip', () {
      test('parses valid version strings correctly', () {
        final testCases = [
          ('1.2.3', Version(1, 2, 3)),
          ('1.0.0-beta', Version(1, 0, 0, preRelease: ['beta'])),
          ('1.0.0+18', Version(1, 0, 0, build: '18')),
          ('2.0.0-beta+18', Version(2, 0, 0, preRelease: ['beta'], build: '18')),
          ('1.0.0-beta.1', Version(1, 0, 0, preRelease: ['beta', '1'])),
        ];

        for (final (input, expected) in testCases) {
          final parsed = Version.parse(input);
          expect(parsed.major, expected.major);
          expect(parsed.minor, expected.minor);
          expect(parsed.patch, expected.patch);
          expect(parsed.build, expected.build);
        }
      });

      test('parse and toString are consistent', () {
        final testCases = [
          '1.2.3',
          '1.0.0-beta',
          '1.0.0+18',
          '2.0.0-beta+18',
          '1.0.0-beta.1',
        ];

        for (final input in testCases) {
          final version = Version.parse(input);
          expect(version.toString(), input);
        }
      });

      test('rejects invalid version strings', () {
        expect(() => Version.parse(''), throwsFormatException);
        expect(() => Version.parse('   '), throwsFormatException);
        expect(() => Version.parse('invalid'), throwsFormatException);
        expect(() => Version.parse('not.a.version'), throwsFormatException);
        expect(() => Version.parse('abc.def.ghi'), throwsFormatException);
      });
    });

    group('equality', () {
      test('equal versions are equal regardless of build', () {
        final v1 = Version(1, 2, 3);
        final v2 = Version(1, 2, 3);
        final v3 = Version(1, 2, 3, build: '18');
        final v4 = Version(1, 2, 3, build: '19');

        expect(v1 == v2, true);
        expect(v1.hashCode, v2.hashCode);
        expect(v1 == v3, true);
        expect(v3 == v4, true);
      });

      test('different version numbers are not equal', () {
        final base = Version(1, 0, 0);
        expect(base == Version(2, 0, 0), false);
        expect(base == Version(1, 1, 0), false);
        expect(base == Version(1, 0, 1), false);
      });

      test('preRelease affects equality', () {
        final withPreRelease = Version(1, 0, 0, preRelease: ['beta']);
        final withoutPreRelease = Version(1, 0, 0);
        final samePreRelease = Version(1, 0, 0, preRelease: ['beta']);

        expect(withPreRelease == withoutPreRelease, false);
        expect(withPreRelease == samePreRelease, true);
      });
    });

    group('force update use case', () {
      test('compares versions correctly for force update logic', () {
        final target = Version(2, 0, 0, preRelease: ['beta'], build: '18');
        final currentOld = Version(2, 0, 0, preRelease: ['beta'], build: '15');
        final currentSame = Version(2, 0, 0, preRelease: ['beta'], build: '18');
        final currentNewer = Version(2, 0, 0, preRelease: ['beta'], build: '19');

        expect(currentOld == target, true);
        expect(int.parse(currentOld.build) < int.parse(target.build), true);

        expect(currentSame == target, true);
        expect(int.parse(currentSame.build) < int.parse(target.build), false);

        expect(currentNewer == target, true);
        expect(int.parse(currentNewer.build) < int.parse(target.build), false);
      });

      test('parses version from file format used in force update', () {
        final versionString = '2.0.0-beta+18';
        final version = Version.parse(versionString);
        expect(version.toString(), versionString);
        expect(version.major, 2);
        expect(version.minor, 0);
        expect(version.patch, 0);
        expect(version.build, '18');
      });
    });
  });
}
