import 'package:equatable/equatable.dart';

class InternetNode extends Equatable {
  const InternetNode(this.address);

  final String address;

  @override
  List<Object?> get props => [address];
}
