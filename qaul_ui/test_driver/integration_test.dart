import 'package:integration_test/integration_test_driver.dart';

Future<void> main() {
  return integrationDriver();

  // final FlutterDriver driver = await FlutterDriver.connect();
  // await integrationDriver(
  //   driver: driver,
  //   onScreenshot: (String screenshotName, List<int> screenshotBytes) async {
  //    // Return false if the screenshot is invalid.
  //    return true;
  //   },
  // );
}
