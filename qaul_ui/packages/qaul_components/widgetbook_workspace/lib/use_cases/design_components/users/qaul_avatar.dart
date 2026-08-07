import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:widgetbook_annotation/widgetbook_annotation.dart' as widgetbook;

@widgetbook.UseCase(
  name: 'Qaul Avatar',
  type: QaulAvatar,
  path: 'design_components/users',
)
Widget buildQaulAvatarUseCase(BuildContext context) {
  const users = [
    (
      name: 'Gustavo Silva',
      id: '12D3KooWG78qQyC7QLdzpHjFq9UbqZFChkw6MM8XiNncCRhmdpKU',
    ),
    (
      name: 'Ana Maria',
      id: '12D3KooWGty7UmMpt1KZmEAJdGE38QXq3DQ4wGyTRW3S9xfce9vg',
    ),
    (
      name: 'lokopkpo 😘',
      id: '12D3KooWLSWqK6vhW2LuRMpGqQcd2migeh12f8n497T27AUn2faN',
    ),
  ];

  return Center(
    child: Wrap(
      spacing: 32,
      runSpacing: 32,
      crossAxisAlignment: WrapCrossAlignment.center,
      children: [
        for (final user in users) ...[
          QaulAvatar(
            name: user.name,
            id: user.id,
            size: QaulAvatarSize.tiny,
          ),
          QaulAvatar(
            name: user.name,
            id: user.id,
            size: QaulAvatarSize.small,
          ),
          QaulAvatarBadge(
            size: QaulAvatarSize.small,
            child: QaulAvatar(
              name: user.name,
              id: user.id,
              size: QaulAvatarSize.small,
            ),
          ),
          QaulAvatar(
            name: user.name,
            id: user.id,
            size: QaulAvatarSize.large,
          ),
        ],
        const QaulAvatar.group(size: QaulAvatarSize.small),
        const QaulAvatar.group(size: QaulAvatarSize.large),
        const QaulAvatar.blank(size: QaulAvatarSize.large),
      ],
    ),
  );
}
