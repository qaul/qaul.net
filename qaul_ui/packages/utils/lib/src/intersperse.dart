// From https://github.com/modulovalue/dart_intersperse
// Copyright (c) 2019 Modestas Valauskas
// Licensed under MIT. See docs/licenses/THIRD_PARTY_LICENSES.md

extension IntersperseExtension<T> on Iterable<T> {
  /// Puts [element] between every element in [list].
  ///
  /// Example:
  ///
  ///     final list1 = <int>[].intersperse(2); // [];
  ///     final list2 = [0].intersperse(2); // [0];
  ///     final list3 = [0, 0].intersperse(2); // [0, 2, 0];
  ///
  Iterable<T> intersperse(T element) sync* {
    final iterator = this.iterator;
    if (iterator.moveNext()) {
      yield iterator.current;
      while (iterator.moveNext()) {
        yield element;
        yield iterator.current;
      }
    }
  }
}
