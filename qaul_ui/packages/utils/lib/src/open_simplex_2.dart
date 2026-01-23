import 'open_simplex_2f.dart';

/// Abstract base class for OpenSimplex2 noise.
abstract class OpenSimplex2 {
  /// 4D noise, classic lattice orientation.
  double noise4Classic(double x, double y, double z, double w);
}
