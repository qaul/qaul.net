part of 'providers.dart';

final homeScreenControllerProvider = StateNotifierProvider<HomeScreenTabController, int>(
    (ref) => HomeScreenTabController(initialTab: 1));

enum TabType { account, public, users, chat, network }

class HomeScreenTabController extends StateNotifier<int> {
  HomeScreenTabController({int? initialTab})
      : _initialTab = initialTab ?? 1,
        super(initialTab ?? 1) {
    _initialize();
  }

  int get index => state;

  TabType get currentTab => TabType.values[state];

  PageController get pageController => _pageController;
  final _pageController = PageController(initialPage: 1);

  int get initialTab => _initialTab;
  final int _initialTab;

  void _initialize() {
    _pageController.addListener(() {
      if (_pageController.page == null) return;
      state = _pageController.page!.round();
    });
  }

  void goToIndex(int index) {
    assert(!index.isNegative && index < TabType.values.length);
    state = index;
    _pageController.jumpToPage(state);
  }

  void goToTab(TabType tab) => goToIndex(TabType.values.indexOf(tab));

  void goToNext() {
    index == TabType.values.length - 1 ? goToIndex(0) : goToIndex(index + 1);
  }

  void goToPrevious() {
    index == 0 ? goToIndex(TabType.values.length - 1) : goToIndex(index - 1);
  }
}
