part of 'widgets.dart';

class DropdownBuilder<T> extends PlatformAwareBuilder {
  const DropdownBuilder({
    super.key,
    required this.value,
    required this.itemsLength,
    required this.itemBuilder,
    this.onChanged,
  });
  final T value;
  final int itemsLength;
  final DropdownMenuItem<T> Function(BuildContext, int) itemBuilder;
  final void Function(T?)? onChanged;

  @override
  Widget defaultBuilder(BuildContext context, WidgetRef ref) {
    return InputDecorator(
      decoration: InputDecoration(
        contentPadding: const EdgeInsets.all(8),
        border: OutlineInputBorder(borderRadius: BorderRadius.circular(4)),
      ),
      child: Theme(
        data: Theme.of(context).copyWith(
          hintColor: Colors.transparent,
          hoverColor: Colors.transparent,
          focusColor: Colors.transparent,
          splashColor: Colors.transparent,
          highlightColor: Colors.transparent,
        ),
        child: DropdownButton<T>(
          value: value,
          isExpanded: true,
          onChanged: onChanged,
          underline: const SizedBox(),
          focusColor: Colors.transparent,
          items: List.generate(itemsLength, (i) => itemBuilder(context, i)),
        ),
      ),
    );
  }
}
