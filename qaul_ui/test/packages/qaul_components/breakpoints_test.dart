import 'package:flutter/material.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:qaul_components/breakpoints.dart';

Widget buildWithSize(Size size, Widget child) {
  return MediaQuery(
    data: MediaQueryData(size: size),
    child: child,
  );
}

void main() {
  group('QaulBreakpoints', () {
    testWidgets('isMobile is true below tablet width', (tester) async {
      await tester.pumpWidget(
        buildWithSize(
          const Size(400, 800),
          MaterialApp(
            home: Builder(
              builder: (context) {
                expect(QaulBreakpoints.isMobile(context), isTrue);
                expect(QaulBreakpoints.isTablet(context), isFalse);
                expect(QaulBreakpoints.isDesktop(context), isFalse);
                return const SizedBox();
              },
            ),
          ),
        ),
      );
    });

    testWidgets('isTablet is true between tablet and desktop width', (tester) async {
      await tester.pumpWidget(
        buildWithSize(
          const Size(1000, 800),
          MaterialApp(
            home: Builder(
              builder: (context) {
                expect(QaulBreakpoints.isMobile(context), isFalse);
                expect(QaulBreakpoints.isTablet(context), isTrue);
                expect(QaulBreakpoints.isDesktop(context), isFalse);
                return const SizedBox();
              },
            ),
          ),
        ),
      );
    });

    testWidgets('isDesktop is true at or above desktop width', (tester) async {
      await tester.pumpWidget(
        buildWithSize(
          const Size(1600, 900),
          MaterialApp(
            home: Builder(
              builder: (context) {
                expect(QaulBreakpoints.isMobile(context), isFalse);
                expect(QaulBreakpoints.isTablet(context), isFalse);
                expect(QaulBreakpoints.isDesktop(context), isTrue);
                return const SizedBox();
              },
            ),
          ),
        ),
      );
    });
  });

  group('kDesignerBreakpoints', () {
    test('has expected viewports', () {
      expect(kDesignerBreakpoints.length, 5);
      expect(kDesignerBreakpoints.any((v) => v.name == kBreakpointIphone16), isTrue);
      expect(kDesignerBreakpoints.any((v) => v.name == kBreakpointFullHd), isTrue);
    });
  });
}
