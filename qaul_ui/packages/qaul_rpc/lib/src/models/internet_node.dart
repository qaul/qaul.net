import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

class InternetNode extends Equatable {
  const InternetNode(this.address, {required this.isActive});

  final String address;
  final bool isActive;

  @override
  List<Object?> get props => [address];

}
