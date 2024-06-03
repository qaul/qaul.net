import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_ui/force_update.dart';
import 'package:version/version.dart';

void main() {
  test('current: 2.0.0-beta+15 ; target: 2.0.0-beta+18 ; true', () async {
    final target = Version(2, 0, 0, preRelease: ['beta'], build: '18');
    final current = Version(2, 0, 0, preRelease: ['beta'], build: '15');

    expect(forceUpdateRequired(current, target), true);
  });

  test('current: 2.0.0-beta+17 ; target: 2.0.0-beta+18 ; true', () async {
    final target = Version(2, 0, 0, preRelease: ['beta'], build: '18');
    final current = Version(2, 0, 0, preRelease: ['beta'], build: '17');

    expect(forceUpdateRequired(current, target), true);
  });

  test('current: 2.0.0-beta+18 ; target: 2.0.0-beta+18 ; false', () async {
    final target = Version(2, 0, 0, preRelease: ['beta'], build: '18');
    final current = Version(2, 0, 0, preRelease: ['beta'], build: '18');

    expect(forceUpdateRequired(current, target), false);
  });

  test('current: 2.0.0-beta+19 ; target: 2.0.0-beta+18 ; false', () async {
    final target = Version(2, 0, 0, preRelease: ['beta'], build: '18');
    final current = Version(2, 0, 0, preRelease: ['beta'], build: '19');

    expect(forceUpdateRequired(current, target), false);
  });
}
