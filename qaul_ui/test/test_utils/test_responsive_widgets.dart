part of 'test_utils.dart';

class ScreenSize {
  const ScreenSize(this.name, this.width, this.height, this.pixelDensity);

  final String name;
  final double width, height, pixelDensity;

  @override
  String toString() =>
      'ScreenSize(name: "$name", width: $width, height: $height, pixelDensity: $pixelDensity)';
}

const iPhone8 = ScreenSize('iPhone_8', 414, 736, 3);
const iPhone13ProMax = ScreenSize('iPhone_13_Pro_Max', 414, 896, 3);
const desktop = ScreenSize('Desktop', 1920, 1080, 1);

final responsiveVariant = ValueVariant<ScreenSize>({
  iPhone8,
  iPhone13ProMax,
  desktop,
});

final desktopVariant = ValueVariant<ScreenSize>({desktop});

@isTest
void testResponsiveWidgets(
  String description,
  WidgetTesterCallback callback, {
  Future<void> Function(String sizeName, WidgetTester tester)? goldenCallback,
  bool? skip,
  Timeout? timeout,
  bool semanticsEnabled = true,
  ValueVariant<ScreenSize>? breakpoints,
  dynamic tags,
}) {
  final variant = breakpoints ?? responsiveVariant;
  testWidgets(
    '$description\n',
    (tester) async {
      await tester.setScreenSize(variant.currentValue!);
      await callback(tester);
      if (goldenCallback != null) {
        await goldenCallback(variant.currentValue!.name, tester);
      }
    },
    skip: skip,
    timeout: timeout,
    semanticsEnabled: semanticsEnabled,
    variant: variant,
    tags: tags,
  );
}

extension ScreenSizeManager on WidgetTester {
  Future<void> setScreenSize(ScreenSize screenSize) async {
    return _setScreenSize(
      width: screenSize.width,
      height: screenSize.height,
      pixelDensity: screenSize.pixelDensity,
    );
  }

  Future<void> _setScreenSize({
    required double width,
    required double height,
    required double pixelDensity,
  }) async {
    final size = Size(width, height);
    await binding.setSurfaceSize(size);
    binding.platformDispatcher.implicitView?.physicalSize = size;
    binding.platformDispatcher.implicitView?.devicePixelRatio = pixelDensity;
  }
}
