//
//  Generated code. Do not modify.
//  source: services/messaging/messaging.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

class CryptoState extends $pb.ProtobufEnum {
  static const CryptoState NONE = CryptoState._(0, _omitEnumNames ? '' : 'NONE');
  static const CryptoState HANDSHAKE = CryptoState._(1, _omitEnumNames ? '' : 'HANDSHAKE');
  static const CryptoState TRANSPORT = CryptoState._(2, _omitEnumNames ? '' : 'TRANSPORT');

  static const $core.List<CryptoState> values = <CryptoState> [
    NONE,
    HANDSHAKE,
    TRANSPORT,
  ];

  static final $core.Map<$core.int, CryptoState> _byValue = $pb.ProtobufEnum.initByValue(values);
  static CryptoState? valueOf($core.int value) => _byValue[value];

  const CryptoState._($core.int v, $core.String n) : super(v, n);
}

class DtnResponse_ResponseType extends $pb.ProtobufEnum {
  static const DtnResponse_ResponseType ACCEPTED = DtnResponse_ResponseType._(0, _omitEnumNames ? '' : 'ACCEPTED');
  static const DtnResponse_ResponseType REJECTED = DtnResponse_ResponseType._(1, _omitEnumNames ? '' : 'REJECTED');

  static const $core.List<DtnResponse_ResponseType> values = <DtnResponse_ResponseType> [
    ACCEPTED,
    REJECTED,
  ];

  static final $core.Map<$core.int, DtnResponse_ResponseType> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DtnResponse_ResponseType? valueOf($core.int value) => _byValue[value];

  const DtnResponse_ResponseType._($core.int v, $core.String n) : super(v, n);
}

class DtnResponse_Reason extends $pb.ProtobufEnum {
  static const DtnResponse_Reason NONE = DtnResponse_Reason._(0, _omitEnumNames ? '' : 'NONE');
  static const DtnResponse_Reason USER_NOT_ACCEPTED = DtnResponse_Reason._(1, _omitEnumNames ? '' : 'USER_NOT_ACCEPTED');
  static const DtnResponse_Reason OVERALL_QUOTA = DtnResponse_Reason._(2, _omitEnumNames ? '' : 'OVERALL_QUOTA');
  static const DtnResponse_Reason USER_QUOTA = DtnResponse_Reason._(3, _omitEnumNames ? '' : 'USER_QUOTA');

  static const $core.List<DtnResponse_Reason> values = <DtnResponse_Reason> [
    NONE,
    USER_NOT_ACCEPTED,
    OVERALL_QUOTA,
    USER_QUOTA,
  ];

  static final $core.Map<$core.int, DtnResponse_Reason> _byValue = $pb.ProtobufEnum.initByValue(values);
  static DtnResponse_Reason? valueOf($core.int value) => _byValue[value];

  const DtnResponse_Reason._($core.int v, $core.String n) : super(v, n);
}


const _omitEnumNames = $core.bool.fromEnvironment('protobuf.omit_enum_names');
