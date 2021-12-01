import 'package:flutter/services.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:libqaul/libqaul.dart';

void main() {
  const MethodChannel channel = MethodChannel('libqaul');

  TestWidgetsFlutterBinding.ensureInitialized();

  setUp(() {
    channel.setMockMethodCallHandler((MethodCall methodCall) async {
      return '42';
    });
  });

  tearDown(() {
    channel.setMockMethodCallHandler(null);
  });

  test('getPlatformVersion', () async {
    expect(await Libqaul.platformVersion, '42');
  });
}
