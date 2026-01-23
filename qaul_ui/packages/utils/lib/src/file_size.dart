String fileSize(dynamic size) {
  const divider = 1024;
  final parsedSize = int.parse(size.toString());

  if (parsedSize < divider) {
    return '$parsedSize B';
  }

  if (parsedSize < divider * divider) {
    return '${(parsedSize / divider).toStringAsFixed(2)} KB';
  }

  if (parsedSize < divider * divider * divider) {
    return '${(parsedSize / (divider * divider)).toStringAsFixed(2)} MB';
  }

  return '${(parsedSize / (divider * divider * divider)).toStringAsFixed(2)} GB';
}
