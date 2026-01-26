import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  group('intersperse', () {
    group('function intersperse', () {
      test('empty list returns empty list', () {
        expect(intersperse(2, <int>[]).toList(), isEmpty);
      });

      test('single element list returns same list', () {
        expect(intersperse(2, [0]).toList(), [0]);
      });

      test('two elements returns elements with separator', () {
        expect(intersperse(2, [0, 1]).toList(), [0, 2, 1]);
      });

      test('multiple elements returns elements with separators', () {
        expect(intersperse(2, [0, 1, 2]).toList(), [0, 2, 1, 2, 2]);
      });

      test('works with strings', () {
        expect(intersperse(',', ['a', 'b', 'c']).toList(), ['a', ',', 'b', ',', 'c']);
      });

      test('works with different types', () {
        expect(intersperse('separator', ['first', 'second']).toList(), ['first', 'separator', 'second']);
      });

      test('works with iterables that are not lists', () {
        final set = {1, 2, 3};
        final result = intersperse(0, set).toList();
        expect(result.length, 5);
        expect(result.first, 1);
        expect(result.last, 3);
        expect(result.where((e) => e == 0).length, 2);
      });
    });

    group('extension IntersperseExtensions', () {
      test('empty list returns empty list', () {
        expect(<int>[].intersperse(2).toList(), isEmpty);
      });

      test('single element list returns same list', () {
        expect([0].intersperse(2).toList(), [0]);
      });

      test('two elements returns elements with separator', () {
        expect([0, 1].intersperse(2).toList(), [0, 2, 1]);
      });

      test('multiple elements returns elements with separators', () {
        expect([0, 1, 2].intersperse(2).toList(), [0, 2, 1, 2, 2]);
      });

      test('works with strings', () {
        expect(['a', 'b', 'c'].intersperse(',').toList(), ['a', ',', 'b', ',', 'c']);
      });

      test('works with widgets (SizedBox)', () {
        final widgets = <Widget>[
          const Text('A'),
          const Text('B'),
          const Text('C'),
        ];
        final result = widgets.intersperse(const SizedBox(width: 8)).toList();
        expect(result.length, 5);
        expect(result[0], isA<Text>());
        expect(result[1], isA<SizedBox>());
        expect(result[2], isA<Text>());
        expect(result[3], isA<SizedBox>());
        expect(result[4], isA<Text>());
      });

      test('works with iterables that are not lists', () {
        final set = {1, 2, 3};
        final result = set.intersperse(0).toList();
        expect(result.length, 5);
        expect(result.first, 1);
        expect(result.last, 3);
        expect(result.where((e) => e == 0).length, 2);
      });

      test('preserves lazy evaluation', () {
        var callCount = 0;
        final iterable = [1, 2, 3].map((e) {
          callCount++;
          return e;
        });
        final interspersed = iterable.intersperse(0);
        expect(callCount, 0);
        final result = interspersed.toList();
        expect(callCount, 3);
        expect(result, [1, 0, 2, 0, 3]);
      });
    });
  });
}
