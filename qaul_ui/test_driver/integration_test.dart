import 'package:integration_test/integration_test_driver.dart';
// import 'package:integration_test/integration_test_driver_extended.dart';

Future<void> main() {
  return integrationDriver();

  // return await integrationDriver(
  //   onScreenshot: (screenshotName, screenshotBytes) async {
  //     final image = await File('test_results/$screenshotName.png').create(
  //       // Create the folder "test_results" if it doesn't exist.
  //       recursive: true,
  //     );
  //
  //     image.writeAsBytesSync(screenshotBytes);
  //
  //     // Return false if the screenshot is invalid.
  //     return true;
  //   },
  // );
}
