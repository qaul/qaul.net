import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';
import 'package:url_launcher/url_launcher.dart';

import '../coordinators/account_management_coordinator.dart';
import '../helpers/navigation_helper.dart';
import '../helpers/user_prefs_helper.dart';
import '../l10n/app_localizations.dart';
import '../providers/account_session_provider.dart';

const _kAuthItemGap = 6.0;
const _kAuthItemRadius = 10.0;
const _kAuthIconSize = 25.0;
const _kAuthSectionHeaderHeight = 40.0;
const _kAuthAccountRowHeight = 45.0;
const _kAuthMoreRowHeight = 35.0;
const _kAuthAvatarTextGap = 18.0;
const _kAuthRowBackgroundColor = Color(0xFF262626);
const _kAuthPrimaryTextColor = Colors.white;
const _kAuthSecondaryTextColor = Colors.grey;

const _kAuthLabelTextStyle = TextStyle(
  color: _kAuthPrimaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w600,
  height: 1.2,
  letterSpacing: 1.5,
);

const _kAuthSecondaryTextStyle = TextStyle(
  color: _kAuthSecondaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w600,
  height: 1.2,
  letterSpacing: 1.5,
);

const _kAuthAccountTextStyle = TextStyle(
  color: _kAuthSecondaryTextColor,
  fontSize: 16,
  fontWeight: FontWeight.w400,
  height: 1.2,
  letterSpacing: 0.5,
);

class SplashScreen extends ConsumerWidget {
  const SplashScreen() : super(key: widgetKey);

  static const widgetKey = ValueKey('SplashScreen');
  static const createUserButtonKey = ValueKey('createUserAccountButton');

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    ref.listen(accountSessionProvider, (_, snapshot) {
      if (snapshot.isLoading || snapshot.hasError) return;
      if (snapshot.value != QaulAccountSessionState.signedIn) return;
      WidgetsBinding.instance.addPostFrameCallback((_) {
        if (!context.mounted) return;
        Navigator.of(
          context,
          rootNavigator: true,
        ).pushReplacementNamed(NavigationHelper.home);
      });
    });

    final session = ref.watch(accountSessionProvider);

    return session.when(
      data: (state) {
        if (state == QaulAccountSessionState.signedIn) {
          return const _SplashLoadingState();
        }

        return _AuthAccountLoader(state: state);
      },
      error: (error, stackTrace) =>
          const _AuthAccountLoader(state: QaulAccountSessionState.noLocalAccount),
      loading: () => const _SplashLoadingState(),
    );
  }
}

class _AuthAccountLoader extends ConsumerStatefulWidget {
  const _AuthAccountLoader({required this.state});

  final QaulAccountSessionState state;

  @override
  ConsumerState<_AuthAccountLoader> createState() => _AuthAccountLoaderState();
}

class _AuthAccountLoaderState extends ConsumerState<_AuthAccountLoader> {
  late final Future<List<LocalAccount>> _accountsFuture;

  @override
  void initState() {
    super.initState();
    _accountsFuture = ref.read(qaulWorkerProvider).getLocalAccounts();
  }

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<List<LocalAccount>>(
      future: _accountsFuture,
      builder: (context, snapshot) {
        if (snapshot.connectionState == ConnectionState.waiting) {
          return const _SplashLoadingState();
        }

        return _AuthLanding(
          state: widget.state,
          accounts: snapshot.data ?? const [],
        );
      },
    );
  }
}

class _AuthLanding extends ConsumerWidget {
  const _AuthLanding({
    required this.state,
    required this.accounts,
  });

  final QaulAccountSessionState state;
  final List<LocalAccount> accounts;

  static const _tutorialUrl = 'https://qaul.net/tutorials/onboarding/';
  static const _contentPadding = EdgeInsets.fromLTRB(26, 88, 26, 32);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final hasAccounts =
        state == QaulAccountSessionState.signedOut && accounts.isNotEmpty;

    return Scaffold(
      backgroundColor: Colors.black,
      body: SafeArea(
        child: Scrollbar(
          child: ListView(
            padding: EdgeInsets.zero,
            children: [
              Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 420),
                  child: Padding(
                    padding: _contentPadding,
                    child: Column(
                      children: [
                        const Center(
                          child: SizedBox(
                            width: 232,
                            height: 232,
                            child: QaulAccountLogo(),
                          ),
                        ),
                        const SizedBox(height: 44),
                        if (hasAccounts)
                          _LoginSection(
                            accounts: accounts,
                            onLogin: (account) =>
                                AccountManagementCoordinator.loginLocalAccount(
                              context,
                              ref,
                              account,
                            ),
                            onMore: () => _pushManageAccounts(
                              context,
                              accounts: accounts,
                              showLogin: true,
                            ),
                          )
                        else
                          _WelcomeSection(
                            onCreateAccount: () =>
                                Navigator.pushReplacementNamed(
                              context,
                              NavigationHelper.createAccount,
                            ),
                          ),
                        SizedBox(height: hasAccounts ? _kAuthItemGap : 55),
                        if (!hasAccounts) ...[
                          _AuthActionRow(
                            icon: Icons.supervisor_account_outlined,
                            label: 'Manage accounts',
                            labelColor: _kAuthSecondaryTextColor,
                            onTap: () => _pushManageAccounts(
                              context,
                              accounts: accounts,
                              showLogin: false,
                            ),
                          ),
                          const SizedBox(height: _kAuthItemGap),
                        ],
                        _AuthLanguageTile(
                          onTap: () => _pushAuthLanguage(context),
                        ),
                        const SizedBox(height: _kAuthItemGap),
                        _AuthActionRow(
                          icon: Icons.open_in_new,
                          label: 'Learn about qaul',
                          labelColor: _kAuthSecondaryTextColor,
                          onTap: () => launchUrl(Uri.parse(_tutorialUrl)),
                          trailing: const Icon(
                            Icons.open_in_new,
                            color: Colors.grey,
                          ),
                        ),
                      ],
                    ),
                  ),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _WelcomeSection extends StatelessWidget {
  const _WelcomeSection({required this.onCreateAccount});

  final VoidCallback onCreateAccount;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        const Text(
          'Welcome',
          style: TextStyle(
            color: Colors.white,
            fontWeight: FontWeight.w800,
            letterSpacing: 1.1,
          ),
        ),
        const SizedBox(height: 12),
        InkWell(
          onTap: onCreateAccount,
          borderRadius: BorderRadius.circular(48),
          hoverColor: Colors.transparent,
          highlightColor: Colors.transparent,
          splashColor: Colors.transparent,
          child: Column(
            children: [
              Container(
                width: 64,
                height: 64,
                decoration: BoxDecoration(
                  shape: BoxShape.circle,
                  border: Border.all(color: Colors.white, width: 2),
                ),
                child: const Icon(
                  Icons.accessibility_new,
                  color: Colors.white,
                  size: 40,
                ),
              ),
              const SizedBox(height: 12),
              const Text(
                'Create User Profile',
                style: TextStyle(
                  color: Colors.grey,
                  fontWeight: FontWeight.w800,
                  letterSpacing: 1.1,
                ),
              ),
            ],
          ),
        ),
      ],
    );
  }
}

class _LoginSection extends StatelessWidget {
  const _LoginSection({
    required this.accounts,
    required this.onLogin,
    required this.onMore,
  });

  final List<LocalAccount> accounts;
  final ValueChanged<LocalAccount> onLogin;
  final VoidCallback onMore;

  @override
  Widget build(BuildContext context) {
    final visibleAccounts = accounts.take(2).toList();

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        const _AuthSectionTitle(icon: Icons.accessibility_new, label: 'Login'),
        const SizedBox(height: _kAuthItemGap),
        _AuthSegmentedList(
          children: [
            for (final account in visibleAccounts)
              _AccountLoginTile(account: account, onTap: () => onLogin(account)),
            _MoreAccountsTile(onTap: onMore),
          ],
        ),
      ],
    );
  }
}

class _AuthSegmentedList extends StatelessWidget {
  const _AuthSegmentedList({required this.children});

  final List<Widget> children;

  @override
  Widget build(BuildContext context) {
    return Column(
      children: [
        for (var i = 0; i < children.length; i++) ...[
          ClipRRect(
            borderRadius: _borderRadiusFor(i, children.length),
            child: Material(
              color: _kAuthRowBackgroundColor,
              child: children[i],
            ),
          ),
          if (i < children.length - 1) const SizedBox(height: _kAuthItemGap),
        ],
      ],
    );
  }

  BorderRadius _borderRadiusFor(int index, int total) {
    const radius = Radius.circular(_kAuthItemRadius);

    if (total == 1) {
      return const BorderRadius.all(radius);
    }

    if (index == 0) {
      return const BorderRadius.vertical(top: radius);
    }

    if (index == total - 1) {
      return const BorderRadius.vertical(bottom: radius);
    }

    return BorderRadius.zero;
  }
}

class _AccountLoginTile extends StatelessWidget {
  const _AccountLoginTile({
    required this.account,
    required this.onTap,
  });

  final LocalAccount account;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: _kAuthAccountRowHeight,
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              QaulAvatar(
                name: account.username,
                id: account.userIdBase58,
                size: QaulAvatarSize.tiny,
              ),
              const SizedBox(width: _kAuthAvatarTextGap),
              Expanded(
                child: Text(
                  account.username,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  style: _kAuthAccountTextStyle,
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _MoreAccountsTile extends StatelessWidget {
  const _MoreAccountsTile({required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: _kAuthMoreRowHeight,
        child: const Padding(
          padding: EdgeInsets.symmetric(horizontal: 12),
          child: Row(
            children: [
              Icon(
                Icons.add_circle_outline,
                color: _kAuthSecondaryTextColor,
                size: _kAuthIconSize,
              ),
              SizedBox(width: _kAuthAvatarTextGap),
              Expanded(
                child: Text(
                  'more',
                  style: _kAuthAccountTextStyle,
                ),
              ),
              Icon(Icons.keyboard_arrow_down, color: _kAuthSecondaryTextColor),
            ],
          ),
        ),
      ),
    );
  }
}

class _AuthLanguageTile extends StatelessWidget {
  const _AuthLanguageTile({required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;

    return ValueListenableBuilder<Locale?>(
      valueListenable: UserPrefsHelper.instance.localeNotifier,
      builder: (context, locale, _) {
        return _AuthActionRow(
          leading: SvgPicture.asset(
            'assets/icons/language.svg',
            width: _kAuthIconSize,
            height: _kAuthIconSize,
            colorFilter: const ColorFilter.mode(
              Colors.white,
              BlendMode.srcATop,
            ),
          ),
          label: l10n.language,
          value: locale == null
              ? _authSystemDefaultLabel(l10n)
              : lookupAppLocalizations(locale).languageName,
          onTap: onTap,
          trailing: const Icon(Icons.chevron_right, color: Colors.grey),
        );
      },
    );
  }
}

class _AuthActionRow extends StatelessWidget {
  const _AuthActionRow({
    required this.label,
    this.icon,
    this.leading,
    this.value,
    this.trailing,
    this.labelColor,
    this.onTap,
  });

  final String label;
  final IconData? icon;
  final Widget? leading;
  final String? value;
  final Widget? trailing;
  final Color? labelColor;
  final VoidCallback? onTap;

  @override
  Widget build(BuildContext context) {
    return InkWell(
      onTap: onTap,
      child: SizedBox(
        height: _kAuthSectionHeaderHeight,
        child: Row(
          children: [
            SizedBox(
              width: _kAuthIconSize + 17,
              child: leading ??
                  Icon(
                    icon,
                    color: _kAuthSecondaryTextColor,
                    size: _kAuthIconSize,
                  ),
            ),
            Expanded(
              child: Text(
                label,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                style: _kAuthLabelTextStyle.copyWith(color: labelColor),
              ),
            ),
            if (value != null)
              Flexible(
                child: Text(
                  value!,
                  maxLines: 1,
                  overflow: TextOverflow.ellipsis,
                  textAlign: TextAlign.end,
                  style: _kAuthSecondaryTextStyle,
                ),
              ),
            if (trailing != null) ...[
              const SizedBox(width: 8),
              trailing!,
            ],
          ],
        ),
      ),
    );
  }
}

class _AuthSectionTitle extends StatelessWidget {
  const _AuthSectionTitle({required this.icon, required this.label});

  final IconData icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return SizedBox(
      height: _kAuthSectionHeaderHeight,
      child: Row(
        children: [
          Icon(icon, color: _kAuthPrimaryTextColor, size: _kAuthIconSize),
          const SizedBox(width: _kAuthAvatarTextGap),
          Text(
            label,
            style: _kAuthLabelTextStyle,
          ),
        ],
      ),
    );
  }
}

void _pushManageAccounts(
  BuildContext context, {
  required List<LocalAccount> accounts,
  required bool showLogin,
}) {
  Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => _ManageAccountsScreen(
        accounts: accounts,
        showLogin: showLogin,
      ),
    ),
  );
}

class _ManageAccountsScreen extends ConsumerWidget {
  const _ManageAccountsScreen({
    required this.accounts,
    required this.showLogin,
  });

  final List<LocalAccount> accounts;
  final bool showLogin;

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return _DarkAuthScaffold(
      child: ListView(
        padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
        children: [
          if (showLogin && accounts.isNotEmpty) ...[
            const _AuthSectionTitle(
              icon: Icons.accessibility_new,
              label: 'Login',
            ),
            const SizedBox(height: _kAuthItemGap),
            ConstrainedBox(
              constraints: const BoxConstraints(maxHeight: 260),
              child: SingleChildScrollView(
                child: _AuthSegmentedList(
                  children: [
                    for (final account in accounts)
                      _AccountLoginTile(
                        account: account,
                        onTap: () =>
                            AccountManagementCoordinator.loginLocalAccount(
                          context,
                          ref,
                          account,
                        ),
                      ),
                  ],
                ),
              ),
            ),
            const SizedBox(height: _kAuthItemGap),
          ],
          _AuthActionRow(
            icon: Icons.supervisor_account_outlined,
            label: 'Import account',
            onTap: () => AccountManagementCoordinator.showRestoreFlow(
              context,
              ref,
            ),
          ),
          const SizedBox(height: _kAuthItemGap),
          _AuthActionRow(
            icon: Icons.person_add_alt,
            label: 'Create user profile',
            onTap: () => Navigator.pushReplacementNamed(
              context,
              NavigationHelper.createAccount,
            ),
          ),
        ],
      ),
    );
  }
}

class _DarkAuthScaffold extends StatelessWidget {
  const _DarkAuthScaffold({required this.child});

  final Widget child;

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.black,
      appBar: AppBar(
        backgroundColor: Colors.black,
        foregroundColor: Colors.grey,
        elevation: 0,
        actions: const [
          Icon(Icons.more_vert),
          SizedBox(width: 12),
        ],
      ),
      body: SafeArea(
        child: Center(
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 420),
            child: child,
          ),
        ),
      ),
    );
  }
}

void _pushAuthLanguage(BuildContext context) {
  Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => const _AuthLanguageScreen(),
    ),
  );
}

class _AuthLanguageScreen extends StatelessWidget {
  const _AuthLanguageScreen();

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final items = <Locale?>[null, ...AppLocalizations.supportedLocales];

    return _DarkAuthScaffold(
      child: ValueListenableBuilder<Locale?>(
        valueListenable: UserPrefsHelper.instance.localeNotifier,
        builder: (context, currentLocale, _) {
          return ListView(
            padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
            children: [
              const _AuthSectionTitle(
                icon: Icons.translate,
                label: 'Language',
              ),
              const SizedBox(height: 18),
              for (final locale in items)
                InkWell(
                  onTap: () =>
                      UserPrefsHelper.instance.setDefaultLocale(locale),
                  child: SizedBox(
                    height: 48,
                    child: Row(
                      children: [
                        Expanded(
                          child: Text(
                            locale == null
                                ? _authSystemDefaultLabel(l10n)
                                : lookupAppLocalizations(locale).languageName,
                            style: TextStyle(
                              color: locale == currentLocale
                                  ? Colors.white
                                  : Colors.grey,
                              fontWeight: FontWeight.w800,
                              letterSpacing: 1.1,
                            ),
                          ),
                        ),
                        if (locale == currentLocale)
                          const Icon(Icons.check, color: Colors.white),
                      ],
                    ),
                  ),
                ),
            ],
          );
        },
      ),
    );
  }
}

String _authSystemDefaultLabel(AppLocalizations l10n) {
  final label = l10n.useSystemDefaultMessage.replaceFirst('Use ', '');
  if (label.isEmpty) return label;
  return label[0].toUpperCase() + label.substring(1);
}

class _SplashLoadingState extends StatelessWidget {
  const _SplashLoadingState();

  @override
  Widget build(BuildContext context) {
    return const Scaffold(
      body: Center(
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            SizedBox(width: 320, height: 320, child: QaulAccountLogo()),
            SizedBox(height: 24),
            QaulLoadingIndicator(),
          ],
        ),
      ),
    );
  }
}
