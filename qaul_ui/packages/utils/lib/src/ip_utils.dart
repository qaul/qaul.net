import 'dart:math';

import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';

bool isValidIPv4(String? s) {
  if (s == null || s.isEmpty) return false;
  return RegExp(r'^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)(\.(?!$)|$)){4}$')
      .hasMatch(s);
}

bool isValidIPv6(String? s) {
  if (s == null || s.isEmpty) return false;
  try {
    final _ = Uri.parseIPv6Address(s);
    return true;
  } on FormatException catch (_) {
    return false;
  }
}

bool isValidPort(String? s) {
  if (s == null || int.tryParse(s) == null) return false;
  return int.parse(s) >= 0 && int.parse(s) < pow(2, 16);
}

abstract class IPTextInputFormatter extends TextInputFormatter {
  @protected
  FilteringTextInputFormatter get filter;

  int get maxLength;

  String get separator;

  int get maxSectionLength;

  int get numberOfSections;

  @protected
  bool userIsErasing(TextEditingValue oldValue, TextEditingValue newValue) =>
      oldValue.text.length > newValue.text.length;

  @protected
  String filterInvalidCharacters(
      TextEditingValue oldValue, TextEditingValue newValue, String output) {
    var value = TextEditingValue(text: output, selection: newValue.selection);
    return filter.formatEditUpdate(oldValue, value).text;
  }

  @protected
  String validateLastSection(List<String> sections) {
    var lastSection = sections.last;
    if (lastSection.length > maxSectionLength) {
      lastSection = lastSection.substring(0, lastSection.length - 1);
    }
    if (lastSection.length == maxSectionLength &&
        sections.length < numberOfSections) lastSection += separator;
    return lastSection;
  }
}

class IPv4TextInputFormatter extends IPTextInputFormatter {
  @override
  FilteringTextInputFormatter get filter =>
      FilteringTextInputFormatter.allow(RegExp(r'[0-9\.]'));

  @override
  String get separator => '.';

  @override
  int get maxLength => 15;

  @override
  int get maxSectionLength => 4;

  @override
  int get numberOfSections => 4;

  @override
  TextEditingValue formatEditUpdate(
      TextEditingValue oldValue, TextEditingValue newValue) {
    if (newValue.text.isEmpty) return newValue;
    if (userIsErasing(oldValue, newValue)) return newValue;

    var output = newValue.text;

    if (output.contains(',')) output = output.replaceAll(',', separator);
    if (output.endsWith('..')) {
      return TextEditingValue(
        text: output.substring(0, output.length - 1),
        selection: TextSelection.collapsed(offset: output.length - 1),
      );
    }

    output = filterInvalidCharacters(oldValue, newValue, output);

    var sections = output.split(separator);
    if (sections.length > numberOfSections) {
      sections = sections.getRange(0, numberOfSections).toList();
    }
    sections.last = validateLastSection(sections);

    output = sections.join(separator);
    if (output.length > maxLength) output = output.substring(0, maxLength);

    return TextEditingValue(
      text: output,
      selection: TextSelection.collapsed(offset: output.length),
    );
  }
}

class IPv6TextInputFormatter extends IPTextInputFormatter {
  @override
  FilteringTextInputFormatter get filter =>
      FilteringTextInputFormatter.allow(RegExp(r'[0-9a-fA-F:]'));

  @override
  String get separator => ':';

  @override
  int get maxLength => 39;

  @override
  int get maxSectionLength => 4;

  @override
  int get numberOfSections => 8;

  @override
  TextEditingValue formatEditUpdate(
      TextEditingValue oldValue, TextEditingValue newValue) {
    if (newValue.text.isEmpty) return newValue;
    if (_userIsErasing(oldValue, newValue)) return newValue;

    var output = newValue.text;

    if (output.contains(',')) output = output.replaceAll(',', separator);
    if (output.contains('.')) output = output.replaceAll('.', separator);

    if (output.endsWith(':::') ||
        (output.endsWith('::') && '::'.allMatches(output).length > 1)) {
      return TextEditingValue(
        text: output.substring(0, output.length - 1),
        selection: TextSelection.collapsed(offset: output.length - 1),
      );
    }

    output = _filterInvalidCharacters(oldValue, newValue, output);

    var sections = output.split(separator);
    if (sections.length > numberOfSections) {
      sections = sections.getRange(0, numberOfSections).toList();
    }
    sections.last = _validateLastSection(sections);

    output = sections.join(separator);
    if (output.length > maxLength) output = output.substring(0, maxLength);

    return TextEditingValue(
      text: output,
      selection: TextSelection.collapsed(offset: output.length),
    );
  }

  bool _userIsErasing(TextEditingValue oldValue, TextEditingValue newValue) =>
      oldValue.text.length > newValue.text.length;

  String _filterInvalidCharacters(
      TextEditingValue oldValue, TextEditingValue newValue, String output) {
    var value = TextEditingValue(text: output, selection: newValue.selection);
    return filter.formatEditUpdate(oldValue, value).text;
  }

  String _validateLastSection(List<String> sections) {
    var lastSection = sections.last;
    if (lastSection.length > maxSectionLength) {
      lastSection = lastSection.substring(0, lastSection.length - 1);
    }
    if (lastSection.length == maxSectionLength &&
        sections.length < numberOfSections) lastSection += separator;

    return lastSection;
  }
}
