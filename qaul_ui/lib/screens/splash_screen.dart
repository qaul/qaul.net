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

class _AuthLanding extends ConsumerStatefulWidget {
  const _AuthLanding({
    required this.state,
    required this.accounts,
  });

  final QaulAccountSessionState state;
  final List<LocalAccount> accounts;

  static const _tutorialUrl = 'https://qaul.net/tutorials/onboarding/';
  static const _contentPadding = EdgeInsets.fromLTRB(26, 88, 26, 32);

  @override
  ConsumerState<_AuthLanding> createState() => _AuthLandingState();
}

class _AuthLandingState extends ConsumerState<_AuthLanding> {
  late final ScrollController _scrollController;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final hasAccounts =
        widget.state == QaulAccountSessionState.signedOut &&
        widget.accounts.isNotEmpty;

    return Scaffold(
      backgroundColor: kQaulAuthBackgroundColor,
      body: SafeArea(
        child: Scrollbar(
          controller: _scrollController,
          thumbVisibility: _showPersistentScrollbar(context),
          child: ListView(
            controller: _scrollController,
            padding: EdgeInsets.zero,
            children: [
              Center(
                child: ConstrainedBox(
                  constraints: const BoxConstraints(maxWidth: 420),
                  child: Padding(
                    padding: _AuthLanding._contentPadding,
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
                            accounts: widget.accounts,
                            onLogin: (account) =>
                                AccountManagementCoordinator.loginLocalAccount(
                              context,
                              ref,
                              account,
                            ),
                            onMore: () => _pushManageAccounts(
                              context,
                              accounts: widget.accounts,
                              showLogin: true,
                            ),
                          )
                        else
                          QaulAuthWelcomeSection(
                            createAccountIcon: const _AuthPngIcon(
                              'assets/icons/auth/avatar_auth.png',
                              color: Colors.white,
                              width: 65,
                              height: 65,
                            ),
                            onCreateAccount: () =>
                                Navigator.pushNamed(
                              context,
                              NavigationHelper.createAccount,
                            ),
                          ),
                        SizedBox(height: hasAccounts ? kQaulAuthItemGap : 55),
                        if (!hasAccounts) ...[
                          QaulAuthActionRow(
                            leading: const _AuthPngIcon(
                              'assets/icons/auth/manage_account.png',
                            ),
                            label: l10n.manageAccounts,
                            labelColor: kQaulAuthSecondaryTextColor,
                            onTap: () => _pushManageAccounts(
                              context,
                              accounts: widget.accounts,
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
                          leading: const _AuthPngIcon(
                            'assets/icons/auth/qaul_small.png',
                            useThemeColor: false,
                          ),
                          label: l10n.learnMore,
                          labelColor: kQaulAuthSecondaryTextColor,
                          onTap: () =>
                              launchUrl(Uri.parse(_AuthLanding._tutorialUrl)),
                          trailing: Image.asset(
                            'assets/icons/auth/extern-link.png',
                            width: kQaulAuthIconSize,
                            height: kQaulAuthIconSize,
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
        _AuthSectionTitle(
          leading: const _AuthPngIcon(
            'assets/icons/auth/manage_account.png',
            color: kQaulAuthPrimaryTextColor,
          ),
          label: AppLocalizations.of(context)!.login,
        ),
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
              ? l10n.systemDefault
              : lookupAppLocalizations(locale).languageName,
          onTap: onTap,
          trailing: SvgPicture.asset(
            'assets/icons/arrow_right.svg',
            width: 9.206,
            height: 18.407,
            colorFilter: const ColorFilter.mode(
              kQaulAuthSecondaryTextColor,
              BlendMode.srcIn,
            ),
          ),
        );
      },
    );
  }
}

class _AuthSectionTitle extends StatelessWidget {
  const _AuthSectionTitle({
    required this.leading,
    required this.label,
  });

  final Widget leading;
  final String label;

  @override
  Widget build(BuildContext context) {
    return QaulAuthSectionTitle(leading: leading, label: label);
  }
}

class _AuthPngIcon extends StatelessWidget {
  const _AuthPngIcon(
    this.assetName, {
    this.color,
    this.width = kQaulAuthIconSize,
    this.height = kQaulAuthIconSize,
    this.useThemeColor = true,
  });

  final String assetName;
  final Color? color;
  final double width;
  final double height;
  final bool useThemeColor;

  @override
  Widget build(BuildContext context) {
    final iconColor =
        color ??
        (useThemeColor
            ? Theme.of(context).brightness == Brightness.dark
                  ? kQaulAuthSecondaryTextColor
                  : Colors.black
            : null);

    return Image.asset(
      assetName,
      width: width,
      height: height,
      color: iconColor,
      colorBlendMode: iconColor == null ? null : BlendMode.srcIn,
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

class _ManageAccountsScreen extends ConsumerStatefulWidget {
  const _ManageAccountsScreen({
    required this.accounts,
    required this.showLogin,
  });

  final List<LocalAccount> accounts;
  final bool showLogin;

  @override
  ConsumerState<_ManageAccountsScreen> createState() =>
      _ManageAccountsScreenState();
}

class _ManageAccountsScreenState extends ConsumerState<_ManageAccountsScreen> {
  late final ScrollController _scrollController;
  late List<LocalAccount> _accounts;
  var _showAllAccounts = false;

  @override
  void initState() {
    super.initState();
    _scrollController = ScrollController();
    _accounts = List.of(widget.accounts);
  }

  @override
  void dispose() {
    _scrollController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final shouldCollapseAccounts = _accounts.length > 2;
    final visibleAccounts = shouldCollapseAccounts && !_showAllAccounts
        ? _accounts.take(2)
        : _accounts;

    return QaulAuthPageScaffold(
      child: Scrollbar(
        controller: _scrollController,
        thumbVisibility: _showPersistentScrollbar(context),
        child: ListView(
          controller: _scrollController,
          padding: const EdgeInsets.fromLTRB(28, 32, 28, 32),
          children: [
            if (widget.showLogin && _accounts.isNotEmpty) ...[
              _AuthSectionTitle(
                leading: const _AuthPngIcon(
                  'assets/icons/auth/manage_account.png',
                  color: kQaulAuthPrimaryTextColor,
                ),
                label: l10n.login,
              ),
              const SizedBox(height: kQaulAuthItemGap),
              QaulAuthSegmentedList(
                children: [
                  for (final account in visibleAccounts)
                    _AccountLoginTile(
                      account: account,
                      onTap: () =>
                          AccountManagementCoordinator.loginLocalAccount(
                        context,
                        ref,
                        account,
                      ),
                    ),
                  if (shouldCollapseAccounts && !_showAllAccounts)
                    QaulAuthExpandTile(
                      onTap: () => setState(() => _showAllAccounts = true),
                    ),
                ],
              ),
              const SizedBox(height: kQaulAuthItemGap),
            ],
            QaulAuthActionRow(
              leading: const _AuthPngIcon(
                'assets/icons/auth/import_account.png',
              ),
              label: l10n.importAccount,
              onTap: () => AccountManagementCoordinator.showRestoreFlow(
                context,
                ref,
              ),
            ),
            const SizedBox(height: kQaulAuthItemGap),
            QaulAuthActionRow(
              leading: const _AuthPngIcon('assets/icons/auth/add_account.png'),
              label: l10n.createUserAccount,
              onTap: () => Navigator.pushNamed(
                context,
                NavigationHelper.createAccount,
              ),
            ),
          ],
        ),
      ),
    );
  }
}

bool _showPersistentScrollbar(BuildContext context) {
  switch (Theme.of(context).platform) {
    case TargetPlatform.macOS:
    case TargetPlatform.linux:
    case TargetPlatform.windows:
      return true;
    case TargetPlatform.android:
    case TargetPlatform.fuchsia:
    case TargetPlatform.iOS:
      return false;
  }
}

void _pushAuthLanguage(BuildContext context) {
  Navigator.of(context).push(
    MaterialPageRoute<void>(
      builder: (_) => const SettingsLanguageScreen(),
    ),
  );
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
