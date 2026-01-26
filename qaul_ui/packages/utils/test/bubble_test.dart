import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  group('Bubble', () {
    testWidgets('renders with default values and child', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Bubble(child: const Text('Test')),
          ),
        ),
      );

      expect(find.text('Test'), findsOneWidget);
      expect(find.byType(Bubble), findsOneWidget);
    });

    testWidgets('renders without child', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(body: Bubble()),
        ),
      );

      expect(find.byType(Bubble), findsOneWidget);
    });

    testWidgets('renders all nip types', (WidgetTester tester) async {
      for (final nip in BubbleNip.values) {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Bubble(
                nip: nip,
                child: const Text('Test'),
              ),
            ),
          ),
        );
        expect(find.text('Test'), findsOneWidget);
      }
    });

    testWidgets('applies custom properties', (WidgetTester tester) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(
            body: Bubble(
              color: Colors.blue,
              radius: const Radius.circular(20),
              elevation: 0,
              nip: BubbleNip.leftBottom,
              nipWidth: 10,
              nipHeight: 15,
              nipRadius: 2,
              padding: const BubbleEdges.all(16),
              margin: const BubbleEdges.symmetric(horizontal: 8),
              child: const Text('Custom'),
            ),
          ),
        ),
      );

      expect(find.text('Custom'), findsOneWidget);
    });
  });

  group('BubbleEdges', () {
    test('fromLTRB sets all values', () {
      const edges = BubbleEdges.fromLTRB(1, 2, 3, 4);
      expect(edges.left, 1);
      expect(edges.top, 2);
      expect(edges.right, 3);
      expect(edges.bottom, 4);
    });

    test('all sets same value for all sides', () {
      const edges = BubbleEdges.all(10);
      expect(edges.left, 10);
      expect(edges.top, 10);
      expect(edges.right, 10);
      expect(edges.bottom, 10);
    });

    test('only sets specified values, others are null', () {
      const edges = BubbleEdges.only(left: 5, top: 10);
      expect(edges.left, 5);
      expect(edges.top, 10);
      expect(edges.right, null);
      expect(edges.bottom, null);
    });

    test('symmetric sets vertical and horizontal values', () {
      const edges = BubbleEdges.symmetric(vertical: 8, horizontal: 12);
      expect(edges.left, 12);
      expect(edges.top, 8);
      expect(edges.right, 12);
      expect(edges.bottom, 8);
    });

    test('edgeInsets converts null values to 0', () {
      const edges = BubbleEdges.only(left: 5, top: 10);
      final edgeInsets = edges.edgeInsets;
      expect(edgeInsets.left, 5);
      expect(edgeInsets.top, 10);
      expect(edgeInsets.right, 0);
      expect(edgeInsets.bottom, 0);
    });

    test('edgeInsets converts all values correctly', () {
      const edges = BubbleEdges.fromLTRB(1, 2, 3, 4);
      final edgeInsets = edges.edgeInsets;
      expect(edgeInsets.left, 1);
      expect(edgeInsets.top, 2);
      expect(edgeInsets.right, 3);
      expect(edgeInsets.bottom, 4);
    });
  });
}
