import 'package:flutter/foundation.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final currentSecurityNoProvider =
    NotifierProvider<CurrentSecurityNoNotifier, SecurityNumber?>(
        CurrentSecurityNoNotifier.new);

class CurrentSecurityNoNotifier extends Notifier<SecurityNumber?> {
  @override
  SecurityNumber? build() => null;

  void setSecurityNumber(SecurityNumber? value) => state = value;
}

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
