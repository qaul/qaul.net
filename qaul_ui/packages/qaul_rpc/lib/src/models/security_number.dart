import 'package:flutter/foundation.dart';

class SecurityNumber {
  SecurityNumber({
    required this.userId,
    required this.securityHash,
    required this.securityNumberBlocks,
  });

  final Uint8List userId;
  final Uint8List securityHash;
  final List<int> securityNumberBlocks;

  List<String> get securityCode => securityNumberBlocks
      .map((n) => n.toString())
      .map((n) => n.padLeft(5, '0'))
      .toList();
}
