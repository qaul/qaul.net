// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

import 'dart:async';

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:qaul_components/qaul_components.dart';

import '../helpers/user_prefs_helper.dart';
import '../l10n/app_localizations.dart';

/// Applies persisted text zoom and keyboard shortcuts to desktop app routes.
class DesktopTextZoom extends StatefulWidget {
  const DesktopTextZoom({super.key, required this.child});

  final Widget child;

  /// How long the reset popup stays visible after a zoom change.
  static const popupDuration = Duration(seconds: 3);

  /// Fade duration used when the reset popup appears or disappears.
  static const popupFadeDuration = Duration(milliseconds: 200);

  @override
  State<DesktopTextZoom> createState() => _DesktopTextZoomState();
}

class _DesktopTextZoomState extends State<DesktopTextZoom> {
  final _prefs = UserPrefsHelper.instance;
  Timer? _popupTimer;
  bool _showPopup = false;
  // Kept while fading out so the label does not jump to 100%.
  double _popupScale = 1.0;

  @override
  void dispose() {
    _popupTimer?.cancel();
    super.dispose();
  }

  /// Applies [scale] and shows the reset popup briefly while it is not the default.
  void _setScale(double scale) {
    unawaited(_prefs.setTextScale(scale));
    _popupTimer?.cancel();
    final show = _prefs.textScale != 1.0;
    if (show) {
      _popupScale = _prefs.textScale;
      _popupTimer = Timer(DesktopTextZoom.popupDuration, () {
        if (mounted) setState(() => _showPopup = false);
      });
    }
    setState(() => _showPopup = show);
  }

  @override
  Widget build(BuildContext context) {
    final platform = Theme.of(context).platform;
    if (platform != TargetPlatform.macOS &&
        platform != TargetPlatform.windows &&
        platform != TargetPlatform.linux) {
      return widget.child;
    }

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
            onInvoke: (intent) => _setScale(
              intent.step == 0 ? 1.0 : _prefs.textScale + intent.step / 10,
            ),
          ),
        },
        child: Focus(
          autofocus: true,
          skipTraversal: true,
          child: ValueListenableBuilder<double>(
            valueListenable: _prefs.textScaleNotifier,
            child: widget.child,
            builder: (context, scale, child) {
              final mediaQuery = MediaQuery.of(context);
              return Stack(
                fit: StackFit.expand,
                children: [
                  MediaQuery(
                    data: mediaQuery.copyWith(
                      textScaler: QaulTextScaler(
                        systemTextScaler: mediaQuery.textScaler,
                        zoom: scale,
                      ),
                    ),
                    child: child!,
                  ),
                  Positioned(
                    top: 12,
                    left: 12,
                    right: 12,
                    child: SafeArea(
                      bottom: false,
                      child: Align(
                        alignment: Alignment.topCenter,
                        child: IgnorePointer(
                          ignoring: !_showPopup,
                          child: AnimatedOpacity(
                            key: const ValueKey('desktop-text-zoom-popup'),
                            opacity: _showPopup ? 1 : 0,
                            duration: DesktopTextZoom.popupFadeDuration,
                            child: Material(
                              elevation: 8,
                              color: Theme.of(context).colorScheme.surface,
                              borderRadius: BorderRadius.circular(12),
                              child: Padding(
                                padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                                child: Wrap(
                                  spacing: 16,
                                  crossAxisAlignment: WrapCrossAlignment.center,
                                  children: [
                                    Text('${(_popupScale * 100).round()}%'),
                                    TextButton(
                                      onPressed: () => _setScale(1.0),
                                      child: Text(AppLocalizations.of(context)!.resetToDefault),
                                    ),
                                  ],
                                ),
                              ),
                            ),
                          ),
                        ),
                      ),
                    ),
                  ),
                ],
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
