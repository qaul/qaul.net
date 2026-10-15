// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:qaul_components/qaul_components.dart';

import '../helpers/user_prefs_helper.dart';

/// Applies persisted text zoom and keyboard shortcuts to desktop app routes.
class DesktopTextZoom extends StatelessWidget {
  const DesktopTextZoom({super.key, required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    final platform = Theme.of(context).platform;
    if (platform != TargetPlatform.macOS &&
        platform != TargetPlatform.windows &&
        platform != TargetPlatform.linux) {
      return child;
    }

    final prefs = UserPrefsHelper.instance;
    final useMeta = platform == TargetPlatform.macOS;
    SingleActivator shortcut(LogicalKeyboardKey key, {bool shift = false}) =>
        SingleActivator(key, control: !useMeta, meta: useMeta, shift: shift);

    return Shortcuts(
      shortcuts: {
        shortcut(LogicalKeyboardKey.equal): const _TextZoomIntent(1),
        shortcut(LogicalKeyboardKey.equal, shift: true): const _TextZoomIntent(1),
        shortcut(LogicalKeyboardKey.add): const _TextZoomIntent(1),
        shortcut(LogicalKeyboardKey.add, shift: true): const _TextZoomIntent(1),
        shortcut(LogicalKeyboardKey.numpadAdd): const _TextZoomIntent(1),
        shortcut(LogicalKeyboardKey.minus): const _TextZoomIntent(-1),
        shortcut(LogicalKeyboardKey.numpadSubtract): const _TextZoomIntent(-1),
        shortcut(LogicalKeyboardKey.digit0): const _TextZoomIntent(0),
        shortcut(LogicalKeyboardKey.numpad0): const _TextZoomIntent(0),
      },
      child: Actions(
        actions: {
          _TextZoomIntent: CallbackAction<_TextZoomIntent>(
            onInvoke: (intent) => prefs.setTextScale(
              intent.step == 0 ? 1.0 : prefs.textScale + intent.step / 10,
            ),
          ),
        },
        child: Focus(
          autofocus: true,
          skipTraversal: true,
          child: ValueListenableBuilder<double>(
            valueListenable: prefs.textScaleNotifier,
            child: child,
            builder: (context, scale, child) {
              final mediaQuery = MediaQuery.of(context);
              return MediaQuery(
                data: mediaQuery.copyWith(
                  textScaler: QaulTextScaler(
                    systemTextScaler: mediaQuery.textScaler,
                    zoom: scale,
                  ),
                ),
                child: child!,
              );
            },
          ),
        ),
      ),
    );
  }
}

/// A zero step restores the default; other steps adjust zoom by one tenth.
class _TextZoomIntent extends Intent {
  const _TextZoomIntent(this.step);

  final int step;
}
