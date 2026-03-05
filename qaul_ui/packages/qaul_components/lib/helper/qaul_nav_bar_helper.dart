enum TabType { account, public, users, chat, network }

enum NavBarOverflowOption {
  settings,
  about,
  license,
  support,
  oldNetwork,
  files,
}

const Map<NavBarOverflowOption, String> kNavBarOverflowMenuLabelsEn = {
  NavBarOverflowOption.settings: 'Settings',
  NavBarOverflowOption.about: 'About',
  NavBarOverflowOption.license: 'AGPL License',
  NavBarOverflowOption.support: 'Support',
  NavBarOverflowOption.oldNetwork: 'Routing table',
  NavBarOverflowOption.files: 'File history',
};

Map<NavBarOverflowOption, String> navBarOverflowMenuLabelsDefault() =>
    Map.from(kNavBarOverflowMenuLabelsEn);
