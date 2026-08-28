import 'package:flutter/material.dart';

const double kQaulAuthItemGap = 6;
const double kQaulAuthItemRadius = 10;
const double kQaulAuthIconSize = 25;
const double kQaulAuthSectionHeaderHeight = 40;
const double kQaulAuthAccountRowHeight = 45;
const double kQaulAuthMoreRowHeight = 35;
const double kQaulAuthAvatarTextGap = 18;

const Color kQaulAuthBackgroundColor = Colors.black;
const Color kQaulAuthRowBackgroundColor = Color(0xFF262626);
const Color kQaulAuthLightRowBackgroundColor = Color(0xFFF2F2F2);
const Color kQaulAuthPrimaryTextColor = Colors.white;
const Color kQaulAuthLightPrimaryTextColor = Color(0xFF666666);
const Color kQaulAuthSecondaryTextColor = Color(0xFF999999);
const Color kQaulAuthDarkHeaderDividerColor = Color(0xFF333333);
const Color kQaulAuthLightHeaderDividerColor = Color(0x33000000);

bool qaulAuthIsDark(BuildContext context) {
  return Theme.of(context).brightness == Brightness.dark;
}

Color qaulAuthBackgroundColor(BuildContext context) {
  return qaulAuthIsDark(context) ? Colors.black : Colors.white;
}

Color qaulAuthRowBackgroundColor(BuildContext context) {
  return qaulAuthIsDark(context)
      ? kQaulAuthRowBackgroundColor
      : kQaulAuthLightRowBackgroundColor;
}

Color qaulAuthPrimaryTextColor(BuildContext context) {
  return qaulAuthIsDark(context)
      ? kQaulAuthPrimaryTextColor
      : kQaulAuthLightPrimaryTextColor;
}

Color qaulAuthSecondaryTextColor(BuildContext context) {
  return kQaulAuthSecondaryTextColor;
}

Color qaulAuthHeaderDividerColor(BuildContext context) {
  return qaulAuthIsDark(context)
      ? kQaulAuthDarkHeaderDividerColor
      : kQaulAuthLightHeaderDividerColor;
}

const TextStyle kQaulAuthLabelTextStyle = TextStyle(
  color: kQaulAuthPrimaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w600,
  height: 1.2,
  letterSpacing: 1.5,
);

const TextStyle kQaulAuthSecondaryTextStyle = TextStyle(
  color: kQaulAuthSecondaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w600,
  height: 1.2,
  letterSpacing: 1.5,
);

const TextStyle kQaulAuthAccountTextStyle = TextStyle(
  color: kQaulAuthSecondaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w400,
  height: 1.2,
  letterSpacing: 0.5,
);

TextStyle qaulAuthLabelTextStyle(BuildContext context, {Color? color}) {
  return kQaulAuthLabelTextStyle.copyWith(
    color: color ?? qaulAuthPrimaryTextColor(context),
  );
}

TextStyle qaulAuthSecondaryTextStyle(BuildContext context, {Color? color}) {
  return kQaulAuthSecondaryTextStyle.copyWith(
    color: color ?? qaulAuthSecondaryTextColor(context),
  );
}

TextStyle qaulAuthAccountTextStyle(BuildContext context, {Color? color}) {
  return kQaulAuthAccountTextStyle.copyWith(
    color: color ?? qaulAuthSecondaryTextColor(context),
  );
}
