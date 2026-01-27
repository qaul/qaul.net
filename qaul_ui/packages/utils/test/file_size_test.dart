import 'package:flutter_test/flutter_test.dart';
import 'package:utils/utils.dart';

void main() {
  group('fileSize', () {
    final testcases = <MapEntry<num, String>>[
      MapEntry(0, '0 B'),
      MapEntry(1, '1 B'),
      MapEntry(512, '512 B'),
      MapEntry(1023, '1023 B'),
      MapEntry(1024, '1.00 KB'),
      MapEntry(1536, '1.50 KB'),
      MapEntry(2048, '2.00 KB'),
      MapEntry(10240, '10.00 KB'),
      MapEntry(1048575, '1024.00 KB'),
      MapEntry(1048576, '1.00 MB'),
      MapEntry(1572864, '1.50 MB'),
      MapEntry(10485760, '10.00 MB'),
      MapEntry(1073741823, '1024.00 MB'),
      MapEntry(1073741824, '1.00 GB'),
      MapEntry(1610612736, '1.50 GB'),
      MapEntry(10737418240, '10.00 GB'),
    ];

    for (final tc in testcases) {
      test('${tc.key} becomes ${tc.value}', () {
        expect(fileSize(tc.key), tc.value);
      });
    }
  });
}
