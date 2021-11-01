import 'dart:ui';

import 'package:color_generator/color_generator.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

final userColorProvider = Provider<Color?>((ref) {
  final user = ref.watch(defaultUserProvider).state;
  user == null ? null : colorGenerationStrategy(user.idBase58);
});
