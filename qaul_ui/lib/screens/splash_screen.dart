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
import 'settings_screen.dart';

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
      backgroundColor: kQaulAuthBackgroundColor,
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
                          QaulAuthWelcomeSection(
                            onCreateAccount: () =>
                                Navigator.pushReplacementNamed(
                              context,
                              NavigationHelper.createAccount,
                            ),
                          ),
                        SizedBox(height: hasAccounts ? kQaulAuthItemGap : 55),
                        if (!hasAccounts) ...[
                          QaulAuthActionRow(
                            icon: Icons.supervisor_account_outlined,
                            label: 'Manage accounts',
                            labelColor: kQaulAuthSecondaryTextColor,
                            onTap: () => _pushManageAccounts(
                              context,
                              accounts: accounts,
                              showLogin: false,
                            ),
                          ),
                          const SizedBox(height: kQaulAuthItemGap),
                        ],
                        _AuthLanguageTile(
                          onTap: () => _pushAuthLanguage(context),
                        ),
                        const SizedBox(height: kQaulAuthItemGap),
                        QaulAuthActionRow(
                          icon: Icons.open_in_new,
                          label: 'Learn about qaul',
                          labelColor: kQaulAuthSecondaryTextColor,
                          onTap: () => launchUrl(Uri.parse(_tutorialUrl)),
                          trailing: const Icon(
                            Icons.open_in_new,
                            color: kQaulAuthSecondaryTextColor,
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
        const SizedBox(height: kQaulAuthItemGap),
        QaulAuthSegmentedList(
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

class _AccountLoginTile extends StatelessWidget {
  const _AccountLoginTile({
    required this.account,
    required this.onTap,
  });

  final LocalAccount account;
  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return QaulAuthAccountTile(
      avatar: QaulAvatar(
        name: account.username,
        id: account.userIdBase58,
        size: QaulAvatarSize.tiny,
      ),
      name: account.username,
      onTap: onTap,
    );
  }
}

class _MoreAccountsTile extends StatelessWidget {
  const _MoreAccountsTile({required this.onTap});

  final VoidCallback onTap;

  @override
  Widget build(BuildContext context) {
    return QaulAuthMoreTile(onTap: onTap);
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
        return QaulAuthActionRow(
          leading: SvgPicture.asset(
            'assets/icons/language.svg',
            width: kQaulAuthIconSize,
            height: kQaulAuthIconSize,
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
          trailing: const Icon(
            Icons.chevron_right,
            color: kQaulAuthSecondaryTextColor,
          ),
        );
      },
    );
  }
}

class _AuthSectionTitle extends StatelessWidget {
  const _AuthSectionTitle({required this.icon, required this.label});

  final IconData icon;
  final String label;

  @override
  Widget build(BuildContext context) {
    return QaulAuthSectionTitle(icon: icon, label: label);
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
    return QaulAuthPageScaffold(
      child: ListView(
        padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
        children: [
          if (showLogin && accounts.isNotEmpty) ...[
            const _AuthSectionTitle(
              icon: Icons.accessibility_new,
              label: 'Login',
            ),
            const SizedBox(height: kQaulAuthItemGap),
            ConstrainedBox(
              constraints: const BoxConstraints(maxHeight: 260),
              child: SingleChildScrollView(
                child: QaulAuthSegmentedList(
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
            const SizedBox(height: kQaulAuthItemGap),
          ],
          QaulAuthActionRow(
            icon: Icons.supervisor_account_outlined,
            label: 'Import account',
            onTap: () => AccountManagementCoordinator.showRestoreFlow(
              context,
              ref,
            ),
          ),
          const SizedBox(height: kQaulAuthItemGap),
          QaulAuthActionRow(
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

void _pushAuthLanguage(BuildContext context) {
  Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => const SettingsLanguageScreen(),
    ),
  );
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
