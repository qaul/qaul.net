/// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the tests.
library color_generator;

import 'dart:ui';

import 'package:flutter/material.dart';

Color colorGenerationStrategy(String first) {
  final colors = <Color>[...Colors.primaries.map((e) => e.shade700), ...Colors.accents.map((e) => e.shade700)];
  return colors[first.hashCode % colors.length];
}

/// Given a string containing values separated by space (" "), yields a string of length 2
/// containing the first letter of the first and last word, respectively, in uppercase.
///
/// If the provided string has no spaces, returns its first two letters - also uppercase.
String initials(String name) {
  if (name.replaceAll(' ', '').length < 2) throw ArgumentError.value(name, 'Name', 'not enough charactes to form initials string');
  if (name.contains(' ')) {
    final ws = name.split(' ').where((e) => e.isNotEmpty).toList();
    if (ws.length > 1) return '${ws.first[0]}${ws.last[0]}'.toUpperCase();
  }
  return name.substring(0, 2).toUpperCase();
}