// This is a generated file - do not edit.
//
// Generated from services/crypto/crypto_net.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

export 'package:protobuf/protobuf.dart' show GeneratedMessageGenericExtensions;

enum CryptoserviceContainer_Message { secondHandshake, notSet }

/// Cryptoservice sending container
class CryptoserviceContainer extends $pb.GeneratedMessage {
  factory CryptoserviceContainer({
    SecondHandshake? secondHandshake,
  }) {
    final result = create();
    if (secondHandshake != null) result.secondHandshake = secondHandshake;
    return result;
  }

  CryptoserviceContainer._();

  factory CryptoserviceContainer.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory CryptoserviceContainer.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static const $core.Map<$core.int, CryptoserviceContainer_Message>
      _CryptoserviceContainer_MessageByTag = {
    1: CryptoserviceContainer_Message.secondHandshake,
    0: CryptoserviceContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'CryptoserviceContainer',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.crypto'),
      createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<SecondHandshake>(1, _omitFieldNames ? '' : 'secondHandshake',
        subBuilder: SecondHandshake.create)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CryptoserviceContainer clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  CryptoserviceContainer copyWith(
          void Function(CryptoserviceContainer) updates) =>
      super.copyWith((message) => updates(message as CryptoserviceContainer))
          as CryptoserviceContainer;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CryptoserviceContainer create() => CryptoserviceContainer._();
  @$core.override
  CryptoserviceContainer createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static CryptoserviceContainer getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<CryptoserviceContainer>(create);
  static CryptoserviceContainer? _defaultInstance;

  @$pb.TagNumber(1)
  CryptoserviceContainer_Message whichMessage() =>
      _CryptoserviceContainer_MessageByTag[$_whichOneof(0)]!;
  @$pb.TagNumber(1)
  void clearMessage() => $_clearField($_whichOneof(0));

  /// Second Handshake Message
  @$pb.TagNumber(1)
  SecondHandshake get secondHandshake => $_getN(0);
  @$pb.TagNumber(1)
  set secondHandshake(SecondHandshake value) => $_setField(1, value);
  @$pb.TagNumber(1)
  $core.bool hasSecondHandshake() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecondHandshake() => $_clearField(1);
  @$pb.TagNumber(1)
  SecondHandshake ensureSecondHandshake() => $_ensure(0);
}

/// Second Handshake Message
class SecondHandshake extends $pb.GeneratedMessage {
  factory SecondHandshake({
    $core.List<$core.int>? signature,
    $fixnum.Int64? receivedAt,
  }) {
    final result = create();
    if (signature != null) result.signature = signature;
    if (receivedAt != null) result.receivedAt = receivedAt;
    return result;
  }

  SecondHandshake._();

  factory SecondHandshake.fromBuffer($core.List<$core.int> data,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromBuffer(data, registry);
  factory SecondHandshake.fromJson($core.String json,
          [$pb.ExtensionRegistry registry = $pb.ExtensionRegistry.EMPTY]) =>
      create()..mergeFromJson(json, registry);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(
      _omitMessageNames ? '' : 'SecondHandshake',
      package:
          const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.crypto'),
      createEmptyInstance: create)
    ..a<$core.List<$core.int>>(
        1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(
        2, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6,
        defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false;

  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecondHandshake clone() => deepCopy();
  @$core.Deprecated('See https://github.com/google/protobuf.dart/issues/998.')
  SecondHandshake copyWith(void Function(SecondHandshake) updates) =>
      super.copyWith((message) => updates(message as SecondHandshake))
          as SecondHandshake;

  @$core.override
  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecondHandshake create() => SecondHandshake._();
  @$core.override
  SecondHandshake createEmptyInstance() => create();
  @$core.pragma('dart2js:noInline')
  static SecondHandshake getDefault() => _defaultInstance ??=
      $pb.GeneratedMessage.$_defaultFor<SecondHandshake>(create);
  static SecondHandshake? _defaultInstance;

  /// confirm message ID of first handshake message
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> value) => $_setBytes(0, value);
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => $_clearField(1);

  /// received at timestamp
  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 value) => $_setInt64(1, value);
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => $_clearField(2);
}

const $core.bool _omitFieldNames =
    $core.bool.fromEnvironment('protobuf.omit_field_names');
const $core.bool _omitMessageNames =
    $core.bool.fromEnvironment('protobuf.omit_message_names');
