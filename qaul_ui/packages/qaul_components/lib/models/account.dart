enum QaulAccountSessionState { noLocalAccount, signedOut, signedIn }

enum QaulAccountAction {
  create,
  restore,
  login,
  export,
  logout,
  delete,
  password,
  learnMore,
}

class QaulAccountSummary {
  const QaulAccountSummary({
    required this.id,
    required this.name,
    this.publicKey,
    this.hasPassword = false,
  });

  final String id;
  final String name;
  final String? publicKey;
  final bool hasPassword;
}
