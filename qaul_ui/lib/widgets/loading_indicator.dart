import 'package:fluent_ui/fluent_ui.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/widgets/platform_aware_builder.dart';

class LoadingIndicator extends PlatformAwareBuilder {
  const LoadingIndicator({Key? key}) : super(key: key);

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return const Center(child: CircularProgressIndicator());
  }

  @override
  Widget iosBuilder(BuildContext context, WidgetRef ref) {
    return const Center(child: CupertinoActivityIndicator());
  }

  @override
  Widget macosBuilder(BuildContext context, WidgetRef ref) {
    return const Center(child: CupertinoActivityIndicator());
  }
}
