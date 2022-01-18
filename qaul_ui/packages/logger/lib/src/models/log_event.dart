import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';

@immutable
abstract class LogEvent extends Equatable {
  const LogEvent({required this.name, this.parameters});

  final String name;
  final Map<String, Object?>? parameters;

  @override
  List<Object> get props => [name];
}
