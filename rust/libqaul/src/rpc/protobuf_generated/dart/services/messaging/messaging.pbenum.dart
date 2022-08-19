///
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12
// ignore_for_file: annotate_overrides,camel_case_types,constant_identifier_names,directives_ordering,library_prefixes,non_constant_identifier_names,prefer_final_fields,return_of_invalid_type,unnecessary_const,unnecessary_import,unnecessary_this,unused_import,unused_shown_name

// ignore_for_file: UNDEFINED_SHOWN_NAME
import 'dart:core' as $core;
import 'package:protobuf/protobuf.dart' as $pb;

class DtnResponse_Type extends $pb.ProtobufEnum {
  static const DtnResponse_Type ACCEPTED = DtnResponse_Type._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'ACCEPTED');
  static const DtnResponse_Type REJECTED = DtnResponse_Type._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'REJECTED');

  static const $core.List<DtnResponse_Type> values = <DtnResponse_Type> [
    ACCEPTED,
    REJECTED,
  ];

  static final $core.Map<$core.int, DtnResponse_Type> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DtnResponse_Type? valueOf($core.int value) => _byValue[value];

  const DtnResponse_Type._($core.int v, $core.String n) : super(v, n);
}

class DtnResponse_Reason extends $pb.ProtobufEnum {
  static const DtnResponse_Reason USER_NOT_ACCEPTED = DtnResponse_Reason._(0, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USER_NOT_ACCEPTED');
  static const DtnResponse_Reason OVERALL_QUOTA = DtnResponse_Reason._(1, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'OVERALL_QUOTA');
  static const DtnResponse_Reason USER_QUOTA = DtnResponse_Reason._(2, const $core.bool.fromEnvironment('protobuf.omit_enum_names') ? '' : 'USER_QUOTA');

  static const $core.List<DtnResponse_Reason> values = <DtnResponse_Reason> [
    USER_NOT_ACCEPTED,
    OVERALL_QUOTA,
    USER_QUOTA,
  ];

  static final $core.Map<$core.int, DtnResponse_Reason> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DtnResponse_Reason? valueOf($core.int value) => _byValue[value];

  const DtnResponse_Reason._($core.int v, $core.String n) : super(v, n);
}

