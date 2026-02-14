// ignore_for_file: no_logic_in_create_state
import 'dart:math' as math;

import 'package:collection/collection.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:intl/intl.dart';
import 'package:intl/locale.dart';
import 'package:logging/logging.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';

import '../../../decorators/cron_task_decorator.dart';
import '../../../decorators/disabled_state_decorator.dart';
import '../../../decorators/empty_state_text_decorator.dart';
import '../../../decorators/search_user_decorator.dart';
import '../../../l10n/app_localizations.dart';
import '../../../providers/providers.dart';
import '../../../stores/stores.dart';
import '../../../utils.dart';
import '../../../widgets/qaul_dialog.dart';
import '../../../widgets/qaul_fab.dart';
import '../../../widgets/widgets.dart';
import 'chat/widgets/chat.dart';

part 'chat/chat_tab.dart';

part 'chat/dialogs/dialogs.dart';

part 'public_tab.dart';

part 'network_tab.dart';

part 'users_tab.dart';

abstract class BaseTab extends StatefulHookConsumerWidget {
  const BaseTab({super.key});

  factory BaseTab.chat({Key? key}) => _Chat(key: key);

  factory BaseTab.public(
          {Key? key, required ValueNotifier<bool> disablePageViewScroll}) =>
      _Public(key: key, disablePageViewScroll: disablePageViewScroll);

  factory BaseTab.network({Key? key}) => _Network(key: key);

  factory BaseTab.users({Key? key}) => _Users(key: key);

  @override
  ConsumerState<BaseTab> createState() => _BaseTabState();
}

class _BaseTabState<T extends BaseTab> extends ConsumerState<T>
    with AutomaticKeepAliveClientMixin<T> {
  @protected
  ValueNotifier<bool> get loading => _loading;
  final _loading = ValueNotifier(false);

  @override
  bool get wantKeepAlive => true;

  @mustCallSuper
  @override
  Widget build(BuildContext context) {
    super.build(context);
    return const SizedBox();
  }
}
