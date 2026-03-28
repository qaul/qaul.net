part of 'providers.dart';

final homeScreenControllerProvider =
    NotifierProvider<HomeScreenTabController, TabType>(
      HomeScreenTabController.new,
    );

class HomeScreenTabController extends Notifier<TabType> {
  @override
  TabType build() {
    return TabType.public;
  }

  final _pageController = PageController(initialPage: 1);

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
