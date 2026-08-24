import 'package:flutter/material.dart';

const kQaulSettingsTextColor = Color(0xFF999999);
const kQaulSettingsDarkHoverColor = Color(0xFF181C1E);
const kQaulSettingsLightHoverColor = Color(0xFFF2F2F2);

Color qaulSettingsItemColor(BuildContext context, {bool selected = false}) {
  if (!selected) return kQaulSettingsTextColor;

  return Theme.of(context).brightness == Brightness.dark
      ? Colors.white
      : Colors.black;
}

Color qaulSettingsBackgroundColor(BuildContext context) {
  return Theme.of(context).brightness == Brightness.dark
      ? Colors.black
      : Colors.white;
}

Color qaulSettingsHoverColor(BuildContext context) {
  return Theme.of(context).brightness == Brightness.dark
      ? kQaulSettingsDarkHoverColor
      : kQaulSettingsLightHoverColor;
}
