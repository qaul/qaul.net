import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/qaul_components.dart';

void main() {
  group('NavBarOverflowOption', () {
    test('has six values', () {
      expect(NavBarOverflowOption.values.length, 6);
    });

    test('contains all expected options', () {
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.settings));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.about));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.license));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.support));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.oldNetwork));
      expect(NavBarOverflowOption.values, contains(NavBarOverflowOption.files));
    });
  });
}
