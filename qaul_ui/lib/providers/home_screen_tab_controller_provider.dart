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

  void _goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    state = TabType.values[index];
    if (_pageController.hasClients) {
      _pageController.jumpToPage(state.index);
    }
  }

  void goToTab(TabType tab) => _goToIndex(TabType.values.indexOf(tab));
}
