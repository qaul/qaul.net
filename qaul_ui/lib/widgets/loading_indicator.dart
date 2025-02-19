part of 'widgets.dart';

class LoadingIndicator extends PlatformAwareBuilder {
  const LoadingIndicator({super.key});

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
