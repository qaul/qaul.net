/// A method returns a human readable string representing a file _size
String fileSize(dynamic size, [int round = 2]) {
  /** 
   * [size] can be passed as number or as string
   *
   * the optional parameter [round] specifies the number 
   * of digits after comma/point (default is 2)
   */
  var divider = 1024;
  int parsedSize;
  try {
    parsedSize = int.parse(size.toString());
  } catch (e) {
    throw ArgumentError('Can not parse the size parameter: $e');
  }

  if (parsedSize < divider) {
    return '$parsedSize B';
  }

  if (parsedSize < divider * divider && parsedSize % divider == 0) {
    return '${(parsedSize / divider).toStringAsFixed(0)} KB';
  }

  if (parsedSize < divider * divider) {
    return '${(parsedSize / divider).toStringAsFixed(round)} KB';
  }

  if (parsedSize < divider * divider * divider && parsedSize % divider == 0) {
    return '${(parsedSize / (divider * divider)).toStringAsFixed(0)} MB';
  }

  if (parsedSize < divider * divider * divider) {
    return '${(parsedSize / divider / divider).toStringAsFixed(round)} MB';
  }

  if (parsedSize < divider * divider * divider * divider && parsedSize % divider == 0) {
    return '${(parsedSize / (divider * divider * divider)).toStringAsFixed(0)} GB';
  }

  if (parsedSize < divider * divider * divider * divider) {
    return '${(parsedSize / divider / divider / divider).toStringAsFixed(round)} GB';
  }

  if (parsedSize < divider * divider * divider * divider * divider &&
      parsedSize % divider == 0) {
    num r = parsedSize / divider / divider / divider / divider;
    return '${r.toStringAsFixed(0)} TB';
  }

  if (parsedSize < divider * divider * divider * divider * divider) {
    num r = parsedSize / divider / divider / divider / divider;
    return '${r.toStringAsFixed(round)} TB';
  }

  if (parsedSize < divider * divider * divider * divider * divider * divider &&
      parsedSize % divider == 0) {
    num r = parsedSize / divider / divider / divider / divider / divider;
    return '${r.toStringAsFixed(0)} PB';
  } else {
    num r = parsedSize / divider / divider / divider / divider / divider;
    return '${r.toStringAsFixed(round)} PB';
  }
}
