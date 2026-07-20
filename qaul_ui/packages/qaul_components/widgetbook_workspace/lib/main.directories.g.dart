// dart format width=80
// coverage:ignore-file
// ignore_for_file: type=lint
// ignore_for_file: unused_import, prefer_relative_imports, directives_ordering

// GENERATED CODE - DO NOT MODIFY BY HAND

// **************************************************************************
// AppGenerator
// **************************************************************************

// ignore_for_file: no_leading_underscores_for_library_prefixes
import 'package:qaul_components_widgetbook/use_cases/design/account/account_management.dart'
    as _qaul_components_widgetbook_use_cases_design_account_account_management;
import 'package:qaul_components_widgetbook/use_cases/design_components/chat/chat_footer.dart'
    as _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer;
import 'package:qaul_components_widgetbook/use_cases/design_components/chat/chat_header.dart'
    as _qaul_components_widgetbook_use_cases_design_components_chat_chat_header;
import 'package:qaul_components_widgetbook/use_cases/design_components/chat/chat_timeline.dart'
    as _qaul_components_widgetbook_use_cases_design_components_chat_chat_timeline;
import 'package:qaul_components_widgetbook/use_cases/design_components/chat/forward_recipient_selector.dart'
    as _qaul_components_widgetbook_use_cases_design_components_chat_forward_recipient_selector;
import 'package:qaul_components_widgetbook/use_cases/design_components/qaul_color_sheet.dart'
    as _qaul_components_widgetbook_use_cases_design_components_qaul_color_sheet;
import 'package:qaul_components_widgetbook/use_cases/design_components/shell/qaul_fab.dart'
    as _qaul_components_widgetbook_use_cases_design_components_shell_qaul_fab;
import 'package:qaul_components_widgetbook/use_cases/design_components/shell/qaul_navbar.dart'
    as _qaul_components_widgetbook_use_cases_design_components_shell_qaul_navbar;
import 'package:qaul_components_widgetbook/use_cases/special_forms/duplicate_username_meta_message.dart'
    as _qaul_components_widgetbook_use_cases_special_forms_duplicate_username_meta_message;
import 'package:qaul_components_widgetbook/use_cases/special_forms/group_join_meta_message.dart'
    as _qaul_components_widgetbook_use_cases_special_forms_group_join_meta_message;
import 'package:qaul_components_widgetbook/use_cases/special_forms/qaul_chat_bubble.dart'
    as _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble;
import 'package:qaul_components_widgetbook/use_cases/special_forms/room_meta_message.dart'
    as _qaul_components_widgetbook_use_cases_special_forms_room_meta_message;
import 'package:widgetbook/widgetbook.dart' as _widgetbook;

final directories = <_widgetbook.WidgetbookNode>[
  _widgetbook.WidgetbookCategory(
    name: 'design',
    children: [
      _widgetbook.WidgetbookFolder(
        name: 'account',
        children: [
          _widgetbook.WidgetbookComponent(
            name: 'QaulAccountLanding',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Account management flow',
                builder:
                    _qaul_components_widgetbook_use_cases_design_account_account_management
                        .buildInteractiveAccountFlowUseCase,
              ),
            ],
          ),
        ],
      ),
    ],
  ),
  _widgetbook.WidgetbookFolder(
    name: 'design_components',
    children: [
      _widgetbook.WidgetbookFolder(
        name: 'chat',
        children: [
          _widgetbook.WidgetbookComponent(
            name: 'ChatFooter',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Empty — attachment actions',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterEmptyClosedUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Empty — submenu open',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterEmptyOpenUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Light — empty actions',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterLightEmptyClosedUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Light — submenu open',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterLightEmptyOpenUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Light — with text',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterLightWithTextUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Long draft (multiline)',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterLongDraftUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'With text — plus and send',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_footer
                        .buildChatFooterWithTextUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'ChatHeader',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Direct — last seen',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_header
                        .buildChatHeaderDirectLastSeenUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Direct — online',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_header
                        .buildChatHeaderDirectOnlineUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Group',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_header
                        .buildChatHeaderGroupUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'ChatTimeline',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Direct chat preview',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_timeline
                        .buildChatRoomPreviewUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Group chat preview',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_chat_timeline
                        .buildGroupChatUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'DuplicateUsernameMetaMessage',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Read-only',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_duplicate_username_meta_message
                        .buildDuplicateUsernameMetaMessageReadOnlyUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Tappable action',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_duplicate_username_meta_message
                        .buildDuplicateUsernameMetaMessageUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'ForwardRecipientSelector',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Recipients — selected',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_forward_recipient_selector
                        .buildForwardRecipientSelectorSelectedUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Recipients — unselected',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_forward_recipient_selector
                        .buildForwardRecipientSelectorUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Search — open',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_chat_forward_recipient_selector
                        .buildForwardRecipientSelectorSearchUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'GroupJoinMetaMessage',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Default',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_group_join_meta_message
                        .buildGroupJoinMetaMessageUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'QaulChatBubble',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Incoming — long',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble
                        .buildIncomingLongUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Incoming — short',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble
                        .buildIncomingShortUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Outgoing — not sent',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble
                        .buildOutgoingNotSentUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Outgoing — read',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble
                        .buildOutgoingReadUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Outgoing — sent',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_qaul_chat_bubble
                        .buildOutgoingSentUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'RoomMetaMessage',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Date',
                builder:
                    _qaul_components_widgetbook_use_cases_special_forms_room_meta_message
                        .buildRoomMetaMessageDateUseCase,
              ),
            ],
          ),
        ],
      ),
      _widgetbook.WidgetbookFolder(
        name: 'shell',
        children: [
          _widgetbook.WidgetbookComponent(
            name: 'QaulFAB',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Default',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_shell_qaul_fab
                        .buildQaulFabDefaultUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Small (chat)',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_shell_qaul_fab
                        .buildQaulFabSmallUseCase,
              ),
            ],
          ),
          _widgetbook.WidgetbookComponent(
            name: 'QaulNavBar',
            useCases: [
              _widgetbook.WidgetbookUseCase(
                name: 'Horizontal (mobile)',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_shell_qaul_navbar
                        .buildNavBarHorizontalUseCase,
              ),
              _widgetbook.WidgetbookUseCase(
                name: 'Vertical (tablet/desktop)',
                builder:
                    _qaul_components_widgetbook_use_cases_design_components_shell_qaul_navbar
                        .buildNavBarVerticalUseCase,
              ),
            ],
          ),
        ],
      ),
    ],
  ),
  _widgetbook.WidgetbookFolder(
    name: 'styles',
    children: [
      _widgetbook.WidgetbookComponent(
        name: 'QaulColorSheet',
        useCases: [
          _widgetbook.WidgetbookUseCase(
            name: 'Palette',
            builder:
                _qaul_components_widgetbook_use_cases_design_components_qaul_color_sheet
                    .buildColorSheetPaletteUseCase,
          ),
        ],
      ),
    ],
  ),
];
