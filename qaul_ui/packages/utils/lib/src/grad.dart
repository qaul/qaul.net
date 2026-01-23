/// 2-dimensional gradient.
class Grad2 {
  /// Creates a 2-dimensional gradient from its components.
  const Grad2(this.dx, this.dy);

  /// The x component of the gradient.
  final double dx;

  /// The y component of the gradient.
  final double dy;
}

/// 3-dimensional gradient.
class Grad3 {
  /// Creates a 3-dimensional gradient from its components.
  const Grad3(this.dx, this.dy, this.dz);

  /// The x component of the gradient.
  final double dx;

  /// The y component of the gradient.
  final double dy;

  /// The z component of the gradient.
  final double dz;
}

/// 4-dimensional gradient.
class Grad4 {
  /// Creates a 4-dimensional gradient from its components.
  const Grad4(this.dx, this.dy, this.dz, this.dw);

  /// The x component of the gradient.
  final double dx;

  /// The y component of the gradient.
  final double dy;

  /// The z component of the gradient.
  final double dz;

  /// The w component of the gradinet.
  final double dw;
}
