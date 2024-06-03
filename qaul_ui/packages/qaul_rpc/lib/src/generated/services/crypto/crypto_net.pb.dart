//
//  Generated code. Do not modify.
//  source: services/crypto/crypto_net.proto
//
// @dart = 2.12

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_final_fields
// ignore_for_file: unnecessary_import, unnecessary_this, unused_import

import 'dart:core' as $core;

import 'package:fixnum/fixnum.dart' as $fixnum;
import 'package:protobuf/protobuf.dart' as $pb;

enum CryptoserviceContainer_Message {
  secondHandshake, 
  notSet
}

/// Cryptoservice sending container
class CryptoserviceContainer extends $pb.GeneratedMessage {
  factory CryptoserviceContainer({
    SecondHandshake? secondHandshake,
  }) {
    final $result = create();
    if (secondHandshake != null) {
      $result.secondHandshake = secondHandshake;
    }
    return $result;
  }
  CryptoserviceContainer._() : super();
  factory CryptoserviceContainer.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory CryptoserviceContainer.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static const $core.Map<$core.int, CryptoserviceContainer_Message> _CryptoserviceContainer_MessageByTag = {
    1 : CryptoserviceContainer_Message.secondHandshake,
    0 : CryptoserviceContainer_Message.notSet
  };
  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'CryptoserviceContainer', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.crypto'), createEmptyInstance: create)
    ..oo(0, [1])
    ..aOM<SecondHandshake>(1, _omitFieldNames ? '' : 'secondHandshake', subBuilder: SecondHandshake.create)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  CryptoserviceContainer clone() => CryptoserviceContainer()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  CryptoserviceContainer copyWith(void Function(CryptoserviceContainer) updates) => super.copyWith((message) => updates(message as CryptoserviceContainer)) as CryptoserviceContainer;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static CryptoserviceContainer create() => CryptoserviceContainer._();
  CryptoserviceContainer createEmptyInstance() => create();
  static $pb.PbList<CryptoserviceContainer> createRepeated() => $pb.PbList<CryptoserviceContainer>();
  @$core.pragma('dart2js:noInline')
  static CryptoserviceContainer getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<CryptoserviceContainer>(create);
  static CryptoserviceContainer? _defaultInstance;

  CryptoserviceContainer_Message whichMessage() => _CryptoserviceContainer_MessageByTag[$_whichOneof(0)]!;
  void clearMessage() => clearField($_whichOneof(0));

  /// Second Handshake Message
  @$pb.TagNumber(1)
  SecondHandshake get secondHandshake => $_getN(0);
  @$pb.TagNumber(1)
  set secondHandshake(SecondHandshake v) { setField(1, v); }
  @$pb.TagNumber(1)
  $core.bool hasSecondHandshake() => $_has(0);
  @$pb.TagNumber(1)
  void clearSecondHandshake() => clearField(1);
  @$pb.TagNumber(1)
  SecondHandshake ensureSecondHandshake() => $_ensure(0);
}

/// Second Handshake Message
class SecondHandshake extends $pb.GeneratedMessage {
  factory SecondHandshake({
    $core.List<$core.int>? signature,
    $fixnum.Int64? receivedAt,
  }) {
    final $result = create();
    if (signature != null) {
      $result.signature = signature;
    }
    if (receivedAt != null) {
      $result.receivedAt = receivedAt;
    }
    return $result;
  }
  SecondHandshake._() : super();
  factory SecondHandshake.fromBuffer($core.List<$core.int> i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromBuffer(i, r);
  factory SecondHandshake.fromJson($core.String i, [$pb.ExtensionRegistry r = $pb.ExtensionRegistry.EMPTY]) => create()..mergeFromJson(i, r);

  static final $pb.BuilderInfo _i = $pb.BuilderInfo(_omitMessageNames ? '' : 'SecondHandshake', package: const $pb.PackageName(_omitMessageNames ? '' : 'qaul.net.crypto'), createEmptyInstance: create)
    ..a<$core.List<$core.int>>(1, _omitFieldNames ? '' : 'signature', $pb.PbFieldType.OY)
    ..a<$fixnum.Int64>(2, _omitFieldNames ? '' : 'receivedAt', $pb.PbFieldType.OU6, defaultOrMaker: $fixnum.Int64.ZERO)
    ..hasRequiredFields = false
  ;

  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.deepCopy] instead. '
  'Will be removed in next major version')
  SecondHandshake clone() => SecondHandshake()..mergeFromMessage(this);
  @$core.Deprecated(
  'Using this can add significant overhead to your binary. '
  'Use [GeneratedMessageGenericExtensions.rebuild] instead. '
  'Will be removed in next major version')
  SecondHandshake copyWith(void Function(SecondHandshake) updates) => super.copyWith((message) => updates(message as SecondHandshake)) as SecondHandshake;

  $pb.BuilderInfo get info_ => _i;

  @$core.pragma('dart2js:noInline')
  static SecondHandshake create() => SecondHandshake._();
  SecondHandshake createEmptyInstance() => create();
  static $pb.PbList<SecondHandshake> createRepeated() => $pb.PbList<SecondHandshake>();
  @$core.pragma('dart2js:noInline')
  static SecondHandshake getDefault() => _defaultInstance ??= $pb.GeneratedMessage.$_defaultFor<SecondHandshake>(create);
  static SecondHandshake? _defaultInstance;

  /// confirm message ID of first handshake message
  @$pb.TagNumber(1)
  $core.List<$core.int> get signature => $_getN(0);
  @$pb.TagNumber(1)
  set signature($core.List<$core.int> v) { $_setBytes(0, v); }
  @$pb.TagNumber(1)
  $core.bool hasSignature() => $_has(0);
  @$pb.TagNumber(1)
  void clearSignature() => clearField(1);

  /// received at timestamp
  @$pb.TagNumber(2)
  $fixnum.Int64 get receivedAt => $_getI64(1);
  @$pb.TagNumber(2)
  set receivedAt($fixnum.Int64 v) { $_setInt64(1, v); }
  @$pb.TagNumber(2)
  $core.bool hasReceivedAt() => $_has(1);
  @$pb.TagNumber(2)
  void clearReceivedAt() => clearField(2);
}


const _omitFieldNames = $core.bool.fromEnvironment('protobuf.omit_field_names');
const _omitMessageNames = $core.bool.fromEnvironment('protobuf.omit_message_names');
