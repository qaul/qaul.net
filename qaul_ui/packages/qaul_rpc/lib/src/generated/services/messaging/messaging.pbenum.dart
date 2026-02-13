// This is a generated file - do not edit.
//
// Generated from services/messaging/messaging.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// state of the crypto session
class CryptoState extends $pb.ProtobufEnum {
  /// no crypto at all
  static const CryptoState NONE =
      CryptoState._(0, _omitEnumNames ? '' : 'NONE');

  /// crypto session is in handshake state
  static const CryptoState HANDSHAKE =
      CryptoState._(1, _omitEnumNames ? '' : 'HANDSHAKE');

  /// crypto session is in transport state
  static const CryptoState TRANSPORT =
      CryptoState._(2, _omitEnumNames ? '' : 'TRANSPORT');

  static const $core.List<CryptoState> values = <CryptoState>[
    NONE,
    HANDSHAKE,
    TRANSPORT,
  ];

  static final $core.List<CryptoState?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 2);
  static CryptoState? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const CryptoState._(super.value, super.name);
}

/// the enum definition of the type
class DtnResponse_ResponseType extends $pb.ProtobufEnum {
  /// the message was accepted for storage
  static const DtnResponse_ResponseType ACCEPTED =
      DtnResponse_ResponseType._(0, _omitEnumNames ? '' : 'ACCEPTED');

  /// the message was rejected
  static const DtnResponse_ResponseType REJECTED =
      DtnResponse_ResponseType._(1, _omitEnumNames ? '' : 'REJECTED');

  static const $core.List<DtnResponse_ResponseType> values =
      <DtnResponse_ResponseType>[
    ACCEPTED,
    REJECTED,
  ];

  static final $core.List<DtnResponse_ResponseType?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 1);
  static DtnResponse_ResponseType? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const DtnResponse_ResponseType._(super.value, super.name);
}

/// the enum definition of the rejection reason
class DtnResponse_Reason extends $pb.ProtobufEnum {
  /// none
  static const DtnResponse_Reason NONE =
      DtnResponse_Reason._(0, _omitEnumNames ? '' : 'NONE');

  /// this user is not accepted
  static const DtnResponse_Reason USER_NOT_ACCEPTED =
      DtnResponse_Reason._(1, _omitEnumNames ? '' : 'USER_NOT_ACCEPTED');

  /// overall quota reached
  static const DtnResponse_Reason OVERALL_QUOTA =
      DtnResponse_Reason._(2, _omitEnumNames ? '' : 'OVERALL_QUOTA');

  /// user quota reached
  static const DtnResponse_Reason USER_QUOTA =
      DtnResponse_Reason._(3, _omitEnumNames ? '' : 'USER_QUOTA');

  static const $core.List<DtnResponse_Reason> values = <DtnResponse_Reason>[
    NONE,
    USER_NOT_ACCEPTED,
    OVERALL_QUOTA,
    USER_QUOTA,
  ];

  static final $core.List<DtnResponse_Reason?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 3);
  static DtnResponse_Reason? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const DtnResponse_Reason._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
