import 'dart:async';

import 'package:flutter/material.dart';

/// Does not add anything visually, so child is passed on directly to the build method.
///
/// This needs to be a Widget to have access to the application's lifecycle events,
/// enabling to remove the callback when the app is suspended and reschedule the callback
/// when the app is active.
class CronTaskDecorator extends StatefulWidget {
  const CronTaskDecorator({
    super.key,
    required this.schedule,
    required this.callback,
    required this.child,
  });
  final Duration schedule;
  final VoidCallback callback;
  final Widget child;

  @override
  State<CronTaskDecorator> createState() => _CronTaskDecoratorState();
}

class _CronTaskDecoratorState extends State<CronTaskDecorator>
    with WidgetsBindingObserver {
  Timer? _callbackTimer;

  void setTimer() {
    cancelTimer();
    _callbackTimer = Timer.periodic(widget.schedule, (_) => widget.callback());
  }

  void cancelTimer() {
    _callbackTimer?.cancel();
    _callbackTimer = null;
  }

  @override
  void initState() {
    super.initState();
    WidgetsBinding.instance.addObserver(this);
    setTimer();
  }

  @override
  void dispose() {
    cancelTimer();
    WidgetsBinding.instance.removeObserver(this);
    super.dispose();
  }

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    state == AppLifecycleState.resumed ? setTimer() : cancelTimer();
  }

  @override
  Widget build(BuildContext context) => widget.child;
}
