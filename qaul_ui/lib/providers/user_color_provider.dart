import 'dart:ui';

import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

final userColorProvider = Provider<Color?>((ref) {
  final user = ref.watch(defaultUserProvider).state;
  user == null ? null : colorGenerationStrategy(user.idBase58);
});
