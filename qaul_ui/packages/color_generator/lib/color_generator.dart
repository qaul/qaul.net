library color_generator;

import 'dart:ui';

import 'package:flutter/material.dart';

// Added as a separate package as running tests on root project instantiates a libqaul shell for some reason, which in return fails the test.
Color colorGenerationStrategy(String first) {
  final colors = <Color>[...Colors.primaries.map((e) => e.shade700), ...Colors.accents.map((e) => e.shade700)];
  return colors[first.hashCode % colors.length];
}
