import 'package:flutter/material.dart';
import 'package:qaul_components/qaul_components.dart';
import 'package:qaul_rpc/qaul_rpc.dart';

class UserDetailsHeading extends StatelessWidget {
  const UserDetailsHeading(this.user, {super.key});
  final User user;

  @override
  Widget build(BuildContext context) {
    return Column(
      mainAxisSize: MainAxisSize.min,
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        QaulAccountHeading(
          account: QaulAccountSummary(
            id: user.idBase58,
            name: user.name,
            publicKey: user.keyBase58,
            hasPassword: user.hasPassword,
          ),
        ),
        const SizedBox(height: 60),
      ],
    );
  }
}
