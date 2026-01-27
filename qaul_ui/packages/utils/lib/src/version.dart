// From https://github.com/dartninja/version
// Copyright (c) 2021, Matthew Barbour.
// Licensed under BSD-3-Clause. See docs/licenses/THIRD_PARTY_LICENSES.md

/// Provides immutable storage and comparison of semantic version numbers.
class Version {
  static final RegExp _versionRegex =
      RegExp(r"^([\d.]+)(-([0-9A-Za-z\-.]+))?(\+([0-9A-Za-z\-.]+))?$");
  static final RegExp _buildRegex = RegExp(r"^[0-9A-Za-z\-.]+$");
  static final RegExp _preReleaseRegex = RegExp(r"^[0-9A-Za-z\-]+$");

  /// The major number of the version, incremented when making breaking changes.
  final int major;

  /// The minor number of the version, incremented when adding new functionality in a backwards-compatible manner.
  final int minor;

  /// The patch number of the version, incremented when making backwards-compatible bug fixes.
  final int patch;

  /// Build information relevant to the version. Does not contribute to sorting.
  final String build;

  final List<String> _preRelease;


  /// Creates a new instance of [Version].
  ///
  /// [major], [minor], and [patch] are all required, all must be greater than 0 and not null, and at least one must be greater than 0.
  /// [preRelease] is optional, but if specified must be a [List] of [String] and must not be null. Each element in the list represents one of the period-separated segments of the pre-release information, and may only contain [0-9A-Za-z-].
  /// [build] is optional, but if specified must be a [String]. must contain only [0-9A-Za-z-.], and must not be null.
  /// Throws a [FormatException] if the [String] content does not follow the character constraints defined above.
  /// Throes an [ArgumentError] if any of the other conditions are violated.
  Version(this.major, this.minor, this.patch,
      {List<String> preRelease = const <String>[], this.build = ""})
      : _preRelease = preRelease {
    for (int i = 0; i < _preRelease.length; i++) {
      if (_preRelease[i].toString().trim().isEmpty) {
        throw ArgumentError("preRelease segments must not be empty");
      }
      // Just in case
      _preRelease[i] = _preRelease[i].toString();
      if (!_preReleaseRegex.hasMatch(_preRelease[i])) {
        throw FormatException(
            "preRelease segments must only contain [0-9A-Za-z-]");
      }
    }
    if (build.isNotEmpty && !_buildRegex.hasMatch(build)) {
      throw FormatException("build must only contain [0-9A-Za-z-.]");
    }

    if (major < 0 || minor < 0 || patch < 0) {
      throw ArgumentError("Version numbers must be greater than 0");
    }
  }

  @override
  int get hashCode => toString().hashCode;


  /// Determines whether the left-hand [Version] represents an equal precedence to the right-hand [Version].
  @override
  bool operator ==(Object other) {
    if (identical(this, other)) return true;
    return other is Version && _compare(this, other) == 0;
  }


  /// Returns a [String] representation of the [Version].
  ///
  /// Uses the format "$major.$minor.$patch".
  /// If [preRelease] has segments available they are appended as "-segmentOne.segmentTwo", with each segment separated by a period.
  /// If [build] is specified, it is appended as "+build.info" where "build.info" is whatever value [build] is set to.
  /// If all [preRelease] and [build] are specified, then both are appended, [preRelease] first and [build] second.
  /// An example of such output would be "1.0.0-preRelease.segment+build.info".
  @override
  String toString() {
    final StringBuffer output = StringBuffer("$major.$minor.$patch");
    if (_preRelease.isNotEmpty) {
      output.write("-${_preRelease.join('.')}");
    }
    if (build.trim().isNotEmpty) {
      output.write("+${build.trim()}");
    }
    return output.toString();
  }

  /// Creates a [Version] instance from a string.
  ///
  /// The string must conform to the specification at http://semver.org/
  /// Throws [FormatException] if the string is empty or does not conform to the spec.
  static Version parse(String versionString) {
    if (versionString.trim().isEmpty) {
      throw FormatException("Cannot parse empty string into version");
    }
    if (!_versionRegex.hasMatch(versionString)) {
      throw FormatException("Not a properly formatted version string");
    }
    final Match m = _versionRegex.firstMatch(versionString)!;
    final String version = m.group(1)!;

    int? major, minor, patch;
    final List<String> parts = version.split(".");
    major = int.parse(parts[0]);
    if (parts.length > 1) {
      minor = int.parse(parts[1]);
      if (parts.length > 2) {
        patch = int.parse(parts[2]);
      }
    }

    final String preReleaseString = m.group(3) ?? "";
    List<String> preReleaseList = <String>[];
    if (preReleaseString.trim().isNotEmpty) {
      preReleaseList = preReleaseString.split(".");
    }
    final String build = m.group(5) ?? "";

    return Version(major, minor ?? 0, patch ?? 0,
        build: build, preRelease: preReleaseList);
  }

  static int _compare(Version? a, Version? b) {
    if (a == null) {
      throw ArgumentError.notNull("a");
    }

    if (b == null) {
      throw ArgumentError.notNull("b");
    }

    if (a.major > b.major) return 1;
    if (a.major < b.major) return -1;

    if (a.minor > b.minor) return 1;
    if (a.minor < b.minor) return -1;

    if (a.patch > b.patch) return 1;
    if (a.patch < b.patch) return -1;

    if (a._preRelease.isEmpty) {
      if (b._preRelease.isEmpty) {
        return 0;
      } else {
        return 1;
      }
    } else if (b._preRelease.isEmpty) {
      return -1;
    } else {
      int preReleaseMax = a._preRelease.length;
      if (b._preRelease.length > a._preRelease.length) {
        preReleaseMax = b._preRelease.length;
      }

      for (int i = 0; i < preReleaseMax; i++) {
        if (b._preRelease.length <= i) {
          return 1;
        } else if (a._preRelease.length <= i) {
          return -1;
        }

        if (a._preRelease[i] == b._preRelease[i]) continue;

        final bool aNumeric = _isNumeric(a._preRelease[i]);
        final bool bNumeric = _isNumeric(b._preRelease[i]);

        if (aNumeric && bNumeric) {
          final double aNumber = double.parse(a._preRelease[i]);
          final double bNumber = double.parse(b._preRelease[i]);
          if (aNumber > bNumber) {
            return 1;
          } else {
            return -1;
          }
        } else if (bNumeric) {
          return 1;
        } else if (aNumeric) {
          return -1;
        } else {
          return a._preRelease[i].compareTo(b._preRelease[i]);
        }
      }
    }
    return 0;
  }

  static bool _isNumeric(String? s) {
    if (s == null) {
      return false;
    }
    return double.tryParse(s) != null;
  }
}
