/// Formats a capacity given in megabytes (MiB) for display.
///
/// Whole gigabytes render without decimals (`1 GB`), other values of a
/// gigabyte or more use one decimal (`1.5 GB`), and anything smaller stays in
/// megabytes (`512 MB`).
String storageSizeLabel(int megabytes) {
  const divider = 1024;

  if (megabytes < divider) return '$megabytes MB';
  if (megabytes % divider == 0) return '${megabytes ~/ divider} GB';

  final gigabytes = (megabytes / divider).toStringAsFixed(1);
  return '${gigabytes.endsWith('.0') ? gigabytes.substring(0, gigabytes.length - 2) : gigabytes} GB';
}
