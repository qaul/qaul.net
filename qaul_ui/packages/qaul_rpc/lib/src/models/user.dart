import 'package:equatable/equatable.dart';

class User extends Equatable {
  const User({
    required this.name,
    required this.idBase58,
  });

  final String name;
  final String idBase58;

  @override
  List<Object?> get props => [name, idBase58];
}
