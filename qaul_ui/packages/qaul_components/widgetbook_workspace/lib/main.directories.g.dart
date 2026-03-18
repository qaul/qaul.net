// dart format width=80
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_import, prefer_relative_imports, directives_ordering

// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AppGenerator
// **************************************************************************

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'package:qaul_components_widgetbook/use_cases/qaul_chat_bubble.dart'
    as _qaul_components_widgetbook_use_cases_qaul_chat_bubble;
import 'package:qaul_components_widgetbook/use_cases/qaul_fab.dart'
    as _qaul_components_widgetbook_use_cases_qaul_fab;
import 'package:qaul_components_widgetbook/use_cases/qaul_navbar.dart'
    as _qaul_components_widgetbook_use_cases_qaul_navbar;
import 'package:widgetbook/widgetbook.dart' as _widgetbook;

final directories = <_widgetbook.WidgetbookNode>[
  _widgetbook.WidgetbookFolder(
    name: 'widgets',
    children: [
      _widgetbook.WidgetbookComponent(
        name: 'QaulChatBubble',
        useCases: [
          _widgetbook.WidgetbookUseCase(
            name: 'Conversation preview',
            builder: _qaul_components_widgetbook_use_cases_qaul_chat_bubble
                .buildChatBubbleConversationUseCase,
          ),
        ],
      ),
      _widgetbook.WidgetbookComponent(
        name: 'QaulFAB',
        useCases: [
          _widgetbook.WidgetbookUseCase(
            name: 'Default',
            builder: _qaul_components_widgetbook_use_cases_qaul_fab
                .buildQaulFabDefaultUseCase,
          ),
          _widgetbook.WidgetbookUseCase(
            name: 'Small (chat)',
            builder: _qaul_components_widgetbook_use_cases_qaul_fab
                .buildQaulFabSmallUseCase,
          ),
        ],
      ),
      _widgetbook.WidgetbookComponent(
        name: 'QaulNavBar',
        useCases: [
          _widgetbook.WidgetbookUseCase(
            name: 'Horizontal (mobile)',
            builder: _qaul_components_widgetbook_use_cases_qaul_navbar
                .buildNavBarHorizontalUseCase,
          ),
          _widgetbook.WidgetbookUseCase(
            name: 'Vertical (tablet/desktop)',
            builder: _qaul_components_widgetbook_use_cases_qaul_navbar
                .buildNavBarVerticalUseCase,
          ),
        ],
      ),
    ],
  ),
];
