import 'package:flutter/material.dart';
import 'package:flutter_svg/svg.dart';
import 'package:package_info_plus/package_info_plus.dart';
import 'package:url_launcher/url_launcher.dart';

import '../helpers/navigation_helper.dart';
import '../l10n/app_localizations.dart';
import '../widgets/widgets.dart';

class AboutScreen extends StatelessWidget {
  const AboutScreen({super.key});

  @override
  Widget build(BuildContext context) {
    final l10n = AppLocalizations.of(context)!;
    final defaultTextStyle = Theme.of(context).textTheme.bodyMedium!;

    return ResponsiveScaffold(
      icon: Icons.info_outline_rounded,
      title: l10n.about,
      body: SingleChildScrollView(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            children: [
              SvgPicture.asset('assets/logo/logo.svg', width: 200, height: 200),
              const SizedBox(height: 16),
              FutureBuilder(
                future: PackageInfo.fromPlatform(),
                builder: (context, snapshot) {
                  if (snapshot.hasError || !snapshot.hasData) {
                    return const Text("qaul");
                  }
                  return Text("qaul version ${snapshot.data!.version}");
                },
              ),
              const SizedBox(height: 16),
              const _LinkButton(
                urlLabel: "https://qaul.net",
                url: "https://qaul.net",
              ),
              const SizedBox(height: 16),
              _LinkButton(
                urlLabel: l10n.userDocumentation,
                url: "https://qaul.net/tutorials/user-documentation/",
              ),
              const SizedBox(height: 16),
              _LinkButton(
                urlLabel: l10n.learnMore,
                url: "https://qaul.net/tutorials/onboarding/",
              ),
              const SizedBox(height: 16),
              RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  style: defaultTextStyle,
                  text:
                      'qaul is a fully free and open source software. It is published under the ',
                  children: <InlineSpan>[
                    WidgetSpan(
                      alignment: PlaceholderAlignment.baseline,
                      baseline: TextBaseline.alphabetic,
                      child: _LinkButton(
                        urlLabel: "AGPLv3",
                        url: "",
                        onPressed: () => Navigator.of(context)
                            .pushNamed(NavigationHelper.license),
                      ),
                    ),
                  ],
                ),
              ),
              RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: '© ',
                  style: defaultTextStyle,
                  children: const <InlineSpan>[
                    WidgetSpan(
                      alignment: PlaceholderAlignment.baseline,
                      baseline: TextBaseline.alphabetic,
                      child: _LinkButton(
                          urlLabel: "Open Community Projects Association",
                          url: "https://ocpa.ch"),
                    ),
                    TextSpan(
                      text: '.',
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 16),
              RichText(
                textAlign: TextAlign.center,
                text: TextSpan(
                  text: 'Logo & project name\n© ',
                  style: defaultTextStyle,
                  children: const <InlineSpan>[
                    WidgetSpan(
                      alignment: PlaceholderAlignment.baseline,
                      baseline: TextBaseline.alphabetic,
                      child: _LinkButton(
                        urlLabel: "Christoph Wachter & Mathias Jud",
                        url: "http://wachter-jud.net",
                      ),
                    ),
                    TextSpan(
                      text: '.',
                    ),
                  ],
                ),
              ),
              const SizedBox(height: 16),
              const _LinkButton(
                urlLabel: "Source code",
                url: "https://github.com/qaul/qaul.net/",
              ),
            ],
          ),
        ),
      ),
    );
  }
}

class _LinkButton extends StatelessWidget {
  const _LinkButton({
    required this.urlLabel,
    required this.url,
    this.onPressed,
  });

  final String urlLabel;
  final String url;

  /// [onPressed] defines the behavior for the interaction with this button.
  ///
  /// If null, will default to opening the [url] using the url_launcher package.
  final VoidCallback? onPressed;

  Future<void> _launchUrl(String url) async {
    final Uri uri = Uri.parse(url);

    if (!await launchUrl(uri)) {
      throw 'Could not launch $uri';
    }
  }

  @override
  Widget build(BuildContext context) {
    return TextButton(
      style: TextButton.styleFrom(
        padding: EdgeInsets.zero,
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(0),
        ),
        tapTargetSize: MaterialTapTargetSize.shrinkWrap,
        visualDensity: VisualDensity.compact,
        minimumSize: const Size(0, 0),
        textStyle: Theme.of(context).textTheme.bodySmall,
      ),
      onPressed: onPressed ?? () => _launchUrl(url),
      child: Text(urlLabel),
    );
  }
}
