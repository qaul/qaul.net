// This is a generated file - do not edit.
//
// Generated from rpc/qaul_rpc.proto.

// @dart = 3.3

// ignore_for_file: annotate_overrides, camel_case_types, comment_references
// ignore_for_file: constant_identifier_names
// ignore_for_file: curly_braces_in_flow_control_structures
// ignore_for_file: deprecated_member_use_from_same_package, library_prefixes
// ignore_for_file: non_constant_identifier_names, prefer_relative_imports

import 'dart:core' as $core;

import 'package:protobuf/protobuf.dart' as $pb;

/// Identification to which module the message shall be
/// handed to.
class Modules extends $pb.ProtobufEnum {
  /// default value, when nothing is defined.
  /// drop this message
  static const Modules NONE = Modules._(0, _omitEnumNames ? '' : 'NONE');

  /// RPC related messages
  /// such as authorisation etc.
  static const Modules RPC = Modules._(1, _omitEnumNames ? '' : 'RPC');

  /// node information
  static const Modules NODE = Modules._(2, _omitEnumNames ? '' : 'NODE');

  /// user accounts on this node
  static const Modules USERACCOUNTS =
      Modules._(3, _omitEnumNames ? '' : 'USERACCOUNTS');

  /// all users in the network
  static const Modules USERS = Modules._(4, _omitEnumNames ? '' : 'USERS');

  /// routing information
  static const Modules ROUTER = Modules._(5, _omitEnumNames ? '' : 'ROUTER');

  /// feed module handling
  ///
  /// send and retrieve feed messages
  static const Modules FEED = Modules._(6, _omitEnumNames ? '' : 'FEED');

  /// connection information to other nodes
  static const Modules CONNECTIONS =
      Modules._(7, _omitEnumNames ? '' : 'CONNECTIONS');

  /// debug information & settings
  static const Modules DEBUG = Modules._(8, _omitEnumNames ? '' : 'DEBUG');

  /// chat group handling
  ///
  /// manage chat groups and group invites
  static const Modules GROUP = Modules._(9, _omitEnumNames ? '' : 'GROUP');

  /// chat module
  /// to send chat messages, get a
  /// conversation overiew and all
  /// messages within a conversation
  static const Modules CHAT = Modules._(10, _omitEnumNames ? '' : 'CHAT');

  /// all functions to send and manage
  /// files sent into a chat conversation
  static const Modules CHATFILE =
      Modules._(11, _omitEnumNames ? '' : 'CHATFILE');

  /// BLE module handling
  static const Modules BLE = Modules._(12, _omitEnumNames ? '' : 'BLE');

  /// Real Time Communication handling
  static const Modules RTC = Modules._(13, _omitEnumNames ? '' : 'RTC');

  /// Delay Tolerant Networking
  static const Modules DTN = Modules._(14, _omitEnumNames ? '' : 'DTN');

  /// Authentication
  static const Modules AUTH = Modules._(15, _omitEnumNames ? '' : 'AUTH');

  /// Event subscription
  ///
  /// Long-running RPC: the client sends one SubscribeRequest, libqaul
  /// pushes back any number of Event messages tagged with the same
  /// request_id until the client disconnects.
  static const Modules SUBSCRIBE =
      Modules._(16, _omitEnumNames ? '' : 'SUBSCRIBE');

  /// Transport management (list, enable, disable)
  static const Modules TRANSPORTS =
      Modules._(17, _omitEnumNames ? '' : 'TRANSPORTS');

  /// End-to-end crypto configuration
  ///
  /// read / write the Noise session rotation config and (future)
  /// query rotation events.
  static const Modules CRYPTO = Modules._(18, _omitEnumNames ? '' : 'CRYPTO');

  /// Account management
  ///
  /// export / delete / restore local user accounts
  static const Modules ACCOUNT_MANAGEMENT =
      Modules._(19, _omitEnumNames ? '' : 'ACCOUNT_MANAGEMENT');

  static const $core.List<Modules> values = <Modules>[
    NONE,
    RPC,
    NODE,
    USERACCOUNTS,
    USERS,
    ROUTER,
    FEED,
    CONNECTIONS,
    DEBUG,
    GROUP,
    CHAT,
    CHATFILE,
    BLE,
    RTC,
    DTN,
    AUTH,
    SUBSCRIBE,
    TRANSPORTS,
    CRYPTO,
    ACCOUNT_MANAGEMENT,
  ];

  static final $core.List<Modules?> _byValue =
      $pb.ProtobufEnum.$_initByValueList(values, 19);
  static Modules? valueOf($core.int value) =>
      value < 0 || value >= _byValue.length ? null : _byValue[value];

  const Modules._(super.value, super.name);
}

const $core.bool _omitEnumNames =
    $core.bool.fromEnvironment('protobuf.omit_enum_names');
