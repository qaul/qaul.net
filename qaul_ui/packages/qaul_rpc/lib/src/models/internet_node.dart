import 'package:equatable/equatable.dart';
import 'package:hooks_riverpod/hooks_riverpod.dart';

final connectedNodesProvider = StateProvider<List<InternetNode>>((ref) => []);

class InternetNode extends Equatable {
  const InternetNode(this.address);

  final String address;

  @override
  List<Object?> get props => [address];
}
