String fileSize(num size) {
  const divider = 1024;

  if (size < divider) {
    return '$size B';
  }

  if (size < divider * divider) {
    return '${(size / divider).toStringAsFixed(2)} KB';
  }

  if (size < divider * divider * divider) {
    return '${(size / (divider * divider)).toStringAsFixed(2)} MB';
  }

  return '${(size / (divider * divider * divider)).toStringAsFixed(2)} GB';
}
