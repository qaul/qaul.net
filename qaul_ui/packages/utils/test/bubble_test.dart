import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  group('Bubble', () {
    testWidgets('renders with default values and child', (
      WidgetTester tester,
    ) async {
      await tester.pumpWidget(
        MaterialApp(
          home: Scaffold(body: Bubble(child: const Text('Test'))),
        ),
      );

      expect(find.text('Test'), findsOneWidget);
      expect(find.byType(Bubble), findsOneWidget);
    });

    testWidgets('renders without child', (WidgetTester tester) async {
      await tester.pumpWidget(MaterialApp(home: Scaffold(body: Bubble())));

      expect(find.byType(Bubble), findsOneWidget);
    });

    testWidgets('renders all nip types', (WidgetTester tester) async {
      for (final nip in BubbleNip.values) {
        await tester.pumpWidget(
          MaterialApp(
            home: Scaffold(
              body: Bubble(nip: nip, child: const Text('Test')),
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
              padding: const EdgeInsets.all(16),
              margin: const EdgeInsets.symmetric(horizontal: 8),
              child: const Text('Custom'),
            ),
          ),
        ),
      );

      expect(find.text('Custom'), findsOneWidget);
    });
  });
}
