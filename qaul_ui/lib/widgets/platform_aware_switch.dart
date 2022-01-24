import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/widgets/platform_aware_builder.dart';

class PlatformAwareSwitch extends PlatformAwareBuilder {
  const PlatformAwareSwitch({
    Key? key,
    required this.value,
    required this.onChanged,
  }) : super(key: key);

  final bool value;

  final Function(bool)? onChanged;

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return Switch(value: value, onChanged: onChanged);
  }

  @override
  Widget iosBuilder(BuildContext context, WidgetRef ref) {
    return CupertinoSwitch(value: value, onChanged: onChanged);
  }
}
