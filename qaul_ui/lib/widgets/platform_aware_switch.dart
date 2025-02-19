part of 'widgets.dart';

class PlatformAwareSwitch extends PlatformAwareBuilder {
  const PlatformAwareSwitch({
    super.key,
    required this.value,
    required this.onChanged,
  });

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

  @override
  Widget macosBuilder(BuildContext context, WidgetRef ref) {
    return CupertinoSwitch(value: value, onChanged: onChanged);
  }
}
