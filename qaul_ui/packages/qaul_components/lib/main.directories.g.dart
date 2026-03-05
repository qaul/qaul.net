// dart format width=80
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_import, prefer_relative_imports, directives_ordering

// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AppGenerator
// **************************************************************************

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'package:qaul_components/force_update_dialog.dart'
    as _qaul_components_force_update_dialog;
import 'package:qaul_components/qaul_nav_bar_use_case.dart'
    as _qaul_components_qaul_nav_bar_use_case;
import 'package:widgetbook/widgetbook.dart' as _widgetbook;

final directories = <_widgetbook.WidgetbookNode>[
  _widgetbook.WidgetbookComponent(
    name: 'ForceUpdateDialog',
    useCases: [
      _widgetbook.WidgetbookUseCase(
        name: 'Default',
        builder: _qaul_components_force_update_dialog.buildCoolButtonUseCase,
      ),
    ],
  ),
  _widgetbook.WidgetbookFolder(
    name: 'widgets',
    children: [
      _widgetbook.WidgetbookComponent(
        name: 'QaulNavBarWidget',
        useCases: [
          _widgetbook.WidgetbookUseCase(
            name: 'Horizontal (mobile)',
            builder: _qaul_components_qaul_nav_bar_use_case
                .buildNavBarHorizontalUseCase,
          ),
          _widgetbook.WidgetbookUseCase(
            name: 'Vertical (tablet/desktop)',
            builder: _qaul_components_qaul_nav_bar_use_case
                .buildNavBarVerticalUseCase,
          ),
        ],
      ),
    ],
  ),
];
