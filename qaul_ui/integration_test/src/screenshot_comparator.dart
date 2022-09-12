import 'dart:developer';

import 'package:flutter/foundation.dart';

import 'package:flutter_test/flutter_test.dart';
import 'package:path/path.dart' as path;

const double _kGoldenDiffTolerance = 0.18;

Future<void> expectGoldenMatches(
  dynamic actual,
  String goldenFileNameWithExtension, {
  String? subPath,
  String? reason,
  dynamic skip = false, // true or a String
}) {
  String goldenPath;
  if (subPath == null || subPath.isEmpty) {
    goldenPath = path.join('goldens', goldenFileNameWithExtension);
  } else {
    goldenPath = path.join('goldens', subPath, goldenFileNameWithExtension);
  }
  goldenFileComparator = _CustomFileComparator(path.join(
    (goldenFileComparator as LocalFileComparator).basedir.toString(),
    goldenFileNameWithExtension,
  ));
  return expectLater(actual, matchesGoldenFile(goldenPath),
      reason: reason, skip: skip);
}

class _CustomFileComparator extends LocalFileComparator {
  _CustomFileComparator(String testFile) : super(Uri.parse(testFile));

  @override
  Future<bool> compare(Uint8List imageBytes, Uri golden) async {
    final result = await GoldenFileComparator.compareLists(
      imageBytes,
      await getGoldenBytes(golden),
    );

    if (!result.passed && result.diffPercent > _kGoldenDiffTolerance) {
      final error = await generateFailureOutput(result, golden, basedir);
      throw FlutterError(error);
    }
    if (!result.passed) {
      log('A tolerable difference of ${result.diffPercent * 100}% was found when '
          'comparing $golden.');
    }
    return result.passed || result.diffPercent <= _kGoldenDiffTolerance;
  }
}
