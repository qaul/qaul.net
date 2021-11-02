import 'package:flutter/material.dart';
import 'package:flutter_hooks/flutter_hooks.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/decorators/qaul_nav_bar_decorator.dart';
import 'package:qaul_ui/providers/providers.dart';
import 'package:qaul_ui/widgets/user_avatar.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:timeago/timeago.dart' as timeago;

part 'tabs/chat_tab.dart';
part 'tabs/feed_tab.dart';
part 'tabs/network_tab.dart';
part 'tabs/user_account_tab.dart';
part 'tabs/users_tab.dart';

class HomeScreen extends HookConsumerWidget {
  const HomeScreen({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final tabCtrl = ref.watch(selectedTabProvider);
    final pageCtrl = usePageController();

    useEffect(
      () {
        final subscription = tabCtrl.stream.listen(
          (i) => pageCtrl.animateToPage(i,
              duration: _pageTransitionDuration, curve: Curves.decelerate),
        );
        return subscription.cancel;
      },
      [tabCtrl.index],
    );

    return Scaffold(
      body: QaulNavBarDecorator(
        child: PageView(
          physics: const NeverScrollableScrollPhysics(),
          controller: pageCtrl,
          children: const [
            _UserAccountTab(),
            _FeedTab(),
            _UsersTab(),
            _ChatTab(),
            _NetworkTab(),
          ],
        ),
      ),
    );
  }

  Duration get _pageTransitionDuration => const Duration(milliseconds: 230);
}
