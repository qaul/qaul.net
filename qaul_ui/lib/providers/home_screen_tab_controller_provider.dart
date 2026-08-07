part of 'providers.dart';

final homeScreenControllerProvider =
    NotifierProvider<HomeScreenTabController, TabType>(
      HomeScreenTabController.new,
    );

class HomeScreenTabController extends Notifier<TabType> {
  @override
  TabType build() {
    // Non-state resource, created here because Riverpod reuses the Notifier
    // instance across invalidations — see lib/session/session_scope.dart. A
    // field initializer would hand the next session a disposed controller.
    _pageController = PageController(initialPage: 1);
    ref.onDispose(_pageController.dispose);
    return TabType.public;
  }

  late PageController _pageController;

  PageController controller() {
    return _pageController;
  }

  void _setTabIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    state = TabType.values[index];
  }

  void setTabFromPageIndex(int index) => _setTabIndex(index);

  void goToTab(TabType tab) {
    final index = TabType.values.indexOf(tab);
    _setTabIndex(index);
    if (_pageController.hasClients) {
      _pageController.jumpToPage(index);
    }
  }
}
