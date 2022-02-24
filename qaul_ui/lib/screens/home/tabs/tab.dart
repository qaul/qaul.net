// ignore_for_file: no_logic_in_create_state
import 'dart:math' as math;

import 'package:collection/collection.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:flutter/cupertino.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:intl/intl.dart';
import 'package:intl/locale.dart';
import 'package:qaul_ui/decorators/cron_task_decorator.dart';
import 'package:qaul_ui/decorators/disabled_state_decorator.dart';
import 'package:qaul_ui/decorators/loading_decorator.dart';
import 'package:qaul_ui/screens/chat/chat.dart';
import 'package:qaul_ui/decorators/empty_state_text_decorator.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:utils/utils.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

import '../../../widgets/widgets.dart';
import '../user_details_screen.dart';

part 'chat_tab.dart';

part 'feed_tab.dart';

part 'network_tab.dart';

part 'users_tab.dart';

abstract class BaseTab extends StatefulHookConsumerWidget {
  const BaseTab({Key? key}) : super(key: key);

  factory BaseTab.chat({Key? key}) => _Chat(key: key);

  factory BaseTab.feed({Key? key}) => _Feed(key: key);

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
