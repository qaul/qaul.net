// Barrel file
library widgets;

import 'dart:io';
import 'dart:math';

import 'package:adaptive_theme/adaptive_theme.dart';
import 'package:badges/badges.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../helpers/user_prefs_helper.dart';

part 'default_back_button.dart';

part 'language_select_dropdown.dart';

part 'loading_indicator.dart';

part 'platform_aware_builder.dart';

part 'platform_aware_switch.dart';

part 'theme_select_dropdown.dart';

part 'user_avatar.dart';

part 'user_list_tile.dart';
