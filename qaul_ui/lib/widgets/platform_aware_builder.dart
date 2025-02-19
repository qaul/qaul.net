part of 'widgets.dart';

/// Overriding [defaultBuilder] is required. It's passed as the base value of all
/// other builders.
///
/// Override all other platform builders that should have a unique UI implementation.
///
/// Overriding the [build] method would render this superclass useless, as it's there that
/// the Platform selection occurs.
abstract class PlatformAwareBuilder extends HookConsumerWidget {
  const PlatformAwareBuilder({super.key});

  Widget defaultBuilder(BuildContext context, WidgetRef ref);

  Widget androidBuilder(BuildContext context, WidgetRef ref) => defaultBuilder(context, ref);

  Widget linuxBuilder(BuildContext context, WidgetRef ref) => defaultBuilder(context, ref);

  Widget windowsBuilder(BuildContext context, WidgetRef ref) => defaultBuilder(context, ref);

  Widget iosBuilder(BuildContext context, WidgetRef ref) => defaultBuilder(context, ref);

  Widget macosBuilder(BuildContext context, WidgetRef ref) => defaultBuilder(context, ref);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    if (Platform.isAndroid) return androidBuilder(context, ref);
    if (Platform.isLinux) return linuxBuilder(context, ref);
    if (Platform.isWindows) return windowsBuilder(context, ref);
    if (Platform.isIOS) return iosBuilder(context, ref);
    if (Platform.isMacOS) return macosBuilder(context, ref);

    return defaultBuilder(context, ref);
  }
}
