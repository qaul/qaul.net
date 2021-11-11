import 'package:flutter/material.dart';
import 'package:flutter_svg/flutter_svg.dart';
import 'package:hive_flutter/hive_flutter.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';
import 'package:qaul_ui/helpers/user_prefs_helper.dart';
import 'package:flutter_gen/gen_l10n/app_localizations.dart';

class LanguageSelectDropDown extends ConsumerWidget {
  const LanguageSelectDropDown({
    Key? key,
  }) : super(key: key);

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Row(
      children: [
        SvgPicture.asset(
          'assets/icons/language.svg',
          width: 24,
          height: 24,
          color: Theme.of(context).iconTheme.color,
        ),
        const SizedBox(width: 8.0),
        Text(AppLocalizations.of(context)!.language),
        const SizedBox(width: 12.0),
        Expanded(
          child: ValueListenableBuilder(
            valueListenable: Hive.box(UserPrefsHelper.hiveBoxName).listenable(),
            builder: (context, box, _) => DropdownButton<Locale?>(
              isExpanded: true,
              value: UserPrefsHelper().defaultLocale,
              items: UserPrefsHelper().supportedLocales.map((value) {
                return DropdownMenuItem<Locale?>(
                  value: value,
                  child: Text(
                    value == null
                        ? AppLocalizations.of(context)!.useSystemDefaultMessage
                        : value.toLanguageTag(),
                  ),
                );
              }).toList(),
              onChanged: (val) => UserPrefsHelper().defaultLocale = val,
            ),
          ),
        ),
      ],
    );
  }
}
