// DO NOT EDIT.
// swift-format-ignore-file
//
// Generated by the Swift generator plugin for the protocol buffer compiler.
// Source: services/group/group_net.proto
//
// For information on using the generated types, please see the documentation:
//   https://github.com/apple/swift-protobuf/

import Foundation
import SwiftProtobuf

// If the compiler emits an error on this type, it is because this file
// was generated by a version of the `protoc` Swift plug-in that is
// incompatible with the version of SwiftProtobuf to which you are linking.
// Please ensure that you are building against the same version of the API
// that was used to generate this file.
fileprivate struct _GeneratedWithProtocGenSwiftVersion: SwiftProtobuf.ProtobufAPIVersionCheck {
  struct _2: SwiftProtobuf.ProtobufAPIVersion_2 {}
  typealias Version = _2
}

/// Group network message container
struct Qaul_Net_Group_GroupContainer {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  var message: Qaul_Net_Group_GroupContainer.OneOf_Message? = nil

  /// group invite
  var inviteMember: Qaul_Net_Group_InviteMember {
    get {
      if case .inviteMember(let v)? = message {return v}
      return Qaul_Net_Group_InviteMember()
    }
    set {message = .inviteMember(newValue)}
  }

  /// reply invite
  var replyInvite: Qaul_Net_Group_ReplyInvite {
    get {
      if case .replyInvite(let v)? = message {return v}
      return Qaul_Net_Group_ReplyInvite()
    }
    set {message = .replyInvite(newValue)}
  }

  /// group notify
  var notify: Qaul_Net_Group_GroupNotify {
    get {
      if case .notify(let v)? = message {return v}
      return Qaul_Net_Group_GroupNotify()
    }
    set {message = .notify(newValue)}
  }

  /// member removed
  var removed: Qaul_Net_Group_RemovedMember {
    get {
      if case .removed(let v)? = message {return v}
      return Qaul_Net_Group_RemovedMember()
    }
    set {message = .removed(newValue)}
  }

  /// group chat message
  var groupMessage: Qaul_Net_Group_GroupMessage {
    get {
      if case .groupMessage(let v)? = message {return v}
      return Qaul_Net_Group_GroupMessage()
    }
    set {message = .groupMessage(newValue)}
  }

  var unknownFields = SwiftProtobuf.UnknownStorage()

  enum OneOf_Message: Equatable {
    /// group invite
    case inviteMember(Qaul_Net_Group_InviteMember)
    /// reply invite
    case replyInvite(Qaul_Net_Group_ReplyInvite)
    /// group notify
    case notify(Qaul_Net_Group_GroupNotify)
    /// member removed
    case removed(Qaul_Net_Group_RemovedMember)
    /// group chat message
    case groupMessage(Qaul_Net_Group_GroupMessage)

  #if !swift(>=4.1)
    static func ==(lhs: Qaul_Net_Group_GroupContainer.OneOf_Message, rhs: Qaul_Net_Group_GroupContainer.OneOf_Message) -> Bool {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch (lhs, rhs) {
      case (.inviteMember, .inviteMember): return {
        guard case .inviteMember(let l) = lhs, case .inviteMember(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.replyInvite, .replyInvite): return {
        guard case .replyInvite(let l) = lhs, case .replyInvite(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.notify, .notify): return {
        guard case .notify(let l) = lhs, case .notify(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.removed, .removed): return {
        guard case .removed(let l) = lhs, case .removed(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      case (.groupMessage, .groupMessage): return {
        guard case .groupMessage(let l) = lhs, case .groupMessage(let r) = rhs else { preconditionFailure() }
        return l == r
      }()
      default: return false
      }
    }
  #endif
  }

  init() {}
}

/// Invite member
struct Qaul_Net_Group_InviteMember {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  /// group id
  var groupID: Data = Data()

  /// group name
  var groupName: String = String()

  /// group admin id
  var adminID: Data = Data()

  /// group created at
  var createdAt: UInt64 = 0

  /// group member count
  var membersCount: UInt32 = 0

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

/// Group member
struct Qaul_Net_Group_Member {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  ///user id
  var userID: Data = Data()

  ///role
  var role: UInt32 = 0

  ///joined at
  var joinedAt: UInt64 = 0

  ///state 
  var state: UInt32 = 0

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

/// Group Notify
struct Qaul_Net_Group_GroupNotify {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  /// group id
  var groupID: Data = Data()

  /// group name
  var groupName: String = String()

  ///created at
  var createdAt: UInt64 = 0

  /// creator id
  var creatorID: Data = Data()

  /// updated members
  var members: [Qaul_Net_Group_Member] = []

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

/// Accept Invite
struct Qaul_Net_Group_ReplyInvite {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  /// group id
  var groupID: Data = Data()

  /// accept true : accept, false: decline
  var accept: Bool = false

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

/// Removed member 
struct Qaul_Net_Group_RemovedMember {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  /// group id
  var groupID: Data = Data()

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

///Group chat message
struct Qaul_Net_Group_GroupMessage {
  // SwiftProtobuf.Message conformance is added in an extension below. See the
  // `Message` and `Message+*Additions` files in the SwiftProtobuf library for
  // methods supported on all messages.

  /// group id
  var groupID: Data = Data()

  /// content
  var content: String = String()

  /// sent at
  var sentAt: UInt64 = 0

  var unknownFields = SwiftProtobuf.UnknownStorage()

  init() {}
}

// MARK: - Code below here is support for the SwiftProtobuf runtime.

fileprivate let _protobuf_package = "qaul.net.group"

extension Qaul_Net_Group_GroupContainer: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".GroupContainer"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "invite_member"),
    2: .standard(proto: "reply_invite"),
    3: .same(proto: "notify"),
    4: .same(proto: "removed"),
    5: .standard(proto: "group_message"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try {
        var v: Qaul_Net_Group_InviteMember?
        var hadOneofValue = false
        if let current = self.message {
          hadOneofValue = true
          if case .inviteMember(let m) = current {v = m}
        }
        try decoder.decodeSingularMessageField(value: &v)
        if let v = v {
          if hadOneofValue {try decoder.handleConflictingOneOf()}
          self.message = .inviteMember(v)
        }
      }()
      case 2: try {
        var v: Qaul_Net_Group_ReplyInvite?
        var hadOneofValue = false
        if let current = self.message {
          hadOneofValue = true
          if case .replyInvite(let m) = current {v = m}
        }
        try decoder.decodeSingularMessageField(value: &v)
        if let v = v {
          if hadOneofValue {try decoder.handleConflictingOneOf()}
          self.message = .replyInvite(v)
        }
      }()
      case 3: try {
        var v: Qaul_Net_Group_GroupNotify?
        var hadOneofValue = false
        if let current = self.message {
          hadOneofValue = true
          if case .notify(let m) = current {v = m}
        }
        try decoder.decodeSingularMessageField(value: &v)
        if let v = v {
          if hadOneofValue {try decoder.handleConflictingOneOf()}
          self.message = .notify(v)
        }
      }()
      case 4: try {
        var v: Qaul_Net_Group_RemovedMember?
        var hadOneofValue = false
        if let current = self.message {
          hadOneofValue = true
          if case .removed(let m) = current {v = m}
        }
        try decoder.decodeSingularMessageField(value: &v)
        if let v = v {
          if hadOneofValue {try decoder.handleConflictingOneOf()}
          self.message = .removed(v)
        }
      }()
      case 5: try {
        var v: Qaul_Net_Group_GroupMessage?
        var hadOneofValue = false
        if let current = self.message {
          hadOneofValue = true
          if case .groupMessage(let m) = current {v = m}
        }
        try decoder.decodeSingularMessageField(value: &v)
        if let v = v {
          if hadOneofValue {try decoder.handleConflictingOneOf()}
          self.message = .groupMessage(v)
        }
      }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    // The use of inline closures is to circumvent an issue where the compiler
    // allocates stack space for every if/case branch local when no optimizations
    // are enabled. https://github.com/apple/swift-protobuf/issues/1034 and
    // https://github.com/apple/swift-protobuf/issues/1182
    switch self.message {
    case .inviteMember?: try {
      guard case .inviteMember(let v)? = self.message else { preconditionFailure() }
      try visitor.visitSingularMessageField(value: v, fieldNumber: 1)
    }()
    case .replyInvite?: try {
      guard case .replyInvite(let v)? = self.message else { preconditionFailure() }
      try visitor.visitSingularMessageField(value: v, fieldNumber: 2)
    }()
    case .notify?: try {
      guard case .notify(let v)? = self.message else { preconditionFailure() }
      try visitor.visitSingularMessageField(value: v, fieldNumber: 3)
    }()
    case .removed?: try {
      guard case .removed(let v)? = self.message else { preconditionFailure() }
      try visitor.visitSingularMessageField(value: v, fieldNumber: 4)
    }()
    case .groupMessage?: try {
      guard case .groupMessage(let v)? = self.message else { preconditionFailure() }
      try visitor.visitSingularMessageField(value: v, fieldNumber: 5)
    }()
    case nil: break
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_GroupContainer, rhs: Qaul_Net_Group_GroupContainer) -> Bool {
    if lhs.message != rhs.message {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_InviteMember: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".InviteMember"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "group_id"),
    2: .standard(proto: "group_name"),
    3: .standard(proto: "admin_id"),
    4: .standard(proto: "created_at"),
    5: .standard(proto: "members_count"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.groupID) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.groupName) }()
      case 3: try { try decoder.decodeSingularBytesField(value: &self.adminID) }()
      case 4: try { try decoder.decodeSingularUInt64Field(value: &self.createdAt) }()
      case 5: try { try decoder.decodeSingularUInt32Field(value: &self.membersCount) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.groupID.isEmpty {
      try visitor.visitSingularBytesField(value: self.groupID, fieldNumber: 1)
    }
    if !self.groupName.isEmpty {
      try visitor.visitSingularStringField(value: self.groupName, fieldNumber: 2)
    }
    if !self.adminID.isEmpty {
      try visitor.visitSingularBytesField(value: self.adminID, fieldNumber: 3)
    }
    if self.createdAt != 0 {
      try visitor.visitSingularUInt64Field(value: self.createdAt, fieldNumber: 4)
    }
    if self.membersCount != 0 {
      try visitor.visitSingularUInt32Field(value: self.membersCount, fieldNumber: 5)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_InviteMember, rhs: Qaul_Net_Group_InviteMember) -> Bool {
    if lhs.groupID != rhs.groupID {return false}
    if lhs.groupName != rhs.groupName {return false}
    if lhs.adminID != rhs.adminID {return false}
    if lhs.createdAt != rhs.createdAt {return false}
    if lhs.membersCount != rhs.membersCount {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_Member: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".Member"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "user_id"),
    2: .same(proto: "role"),
    3: .standard(proto: "joined_at"),
    4: .same(proto: "state"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.userID) }()
      case 2: try { try decoder.decodeSingularUInt32Field(value: &self.role) }()
      case 3: try { try decoder.decodeSingularUInt64Field(value: &self.joinedAt) }()
      case 4: try { try decoder.decodeSingularUInt32Field(value: &self.state) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.userID.isEmpty {
      try visitor.visitSingularBytesField(value: self.userID, fieldNumber: 1)
    }
    if self.role != 0 {
      try visitor.visitSingularUInt32Field(value: self.role, fieldNumber: 2)
    }
    if self.joinedAt != 0 {
      try visitor.visitSingularUInt64Field(value: self.joinedAt, fieldNumber: 3)
    }
    if self.state != 0 {
      try visitor.visitSingularUInt32Field(value: self.state, fieldNumber: 4)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_Member, rhs: Qaul_Net_Group_Member) -> Bool {
    if lhs.userID != rhs.userID {return false}
    if lhs.role != rhs.role {return false}
    if lhs.joinedAt != rhs.joinedAt {return false}
    if lhs.state != rhs.state {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_GroupNotify: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".GroupNotify"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "group_id"),
    2: .standard(proto: "group_name"),
    3: .standard(proto: "created_at"),
    4: .standard(proto: "creator_id"),
    5: .same(proto: "members"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.groupID) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.groupName) }()
      case 3: try { try decoder.decodeSingularUInt64Field(value: &self.createdAt) }()
      case 4: try { try decoder.decodeSingularBytesField(value: &self.creatorID) }()
      case 5: try { try decoder.decodeRepeatedMessageField(value: &self.members) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.groupID.isEmpty {
      try visitor.visitSingularBytesField(value: self.groupID, fieldNumber: 1)
    }
    if !self.groupName.isEmpty {
      try visitor.visitSingularStringField(value: self.groupName, fieldNumber: 2)
    }
    if self.createdAt != 0 {
      try visitor.visitSingularUInt64Field(value: self.createdAt, fieldNumber: 3)
    }
    if !self.creatorID.isEmpty {
      try visitor.visitSingularBytesField(value: self.creatorID, fieldNumber: 4)
    }
    if !self.members.isEmpty {
      try visitor.visitRepeatedMessageField(value: self.members, fieldNumber: 5)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_GroupNotify, rhs: Qaul_Net_Group_GroupNotify) -> Bool {
    if lhs.groupID != rhs.groupID {return false}
    if lhs.groupName != rhs.groupName {return false}
    if lhs.createdAt != rhs.createdAt {return false}
    if lhs.creatorID != rhs.creatorID {return false}
    if lhs.members != rhs.members {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_ReplyInvite: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".ReplyInvite"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "group_id"),
    2: .same(proto: "accept"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.groupID) }()
      case 2: try { try decoder.decodeSingularBoolField(value: &self.accept) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.groupID.isEmpty {
      try visitor.visitSingularBytesField(value: self.groupID, fieldNumber: 1)
    }
    if self.accept != false {
      try visitor.visitSingularBoolField(value: self.accept, fieldNumber: 2)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_ReplyInvite, rhs: Qaul_Net_Group_ReplyInvite) -> Bool {
    if lhs.groupID != rhs.groupID {return false}
    if lhs.accept != rhs.accept {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_RemovedMember: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".RemovedMember"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "group_id"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.groupID) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.groupID.isEmpty {
      try visitor.visitSingularBytesField(value: self.groupID, fieldNumber: 1)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_RemovedMember, rhs: Qaul_Net_Group_RemovedMember) -> Bool {
    if lhs.groupID != rhs.groupID {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}

extension Qaul_Net_Group_GroupMessage: SwiftProtobuf.Message, SwiftProtobuf._MessageImplementationBase, SwiftProtobuf._ProtoNameProviding {
  static let protoMessageName: String = _protobuf_package + ".GroupMessage"
  static let _protobuf_nameMap: SwiftProtobuf._NameMap = [
    1: .standard(proto: "group_id"),
    2: .same(proto: "content"),
    3: .standard(proto: "sent_at"),
  ]

  mutating func decodeMessage<D: SwiftProtobuf.Decoder>(decoder: inout D) throws {
    while let fieldNumber = try decoder.nextFieldNumber() {
      // The use of inline closures is to circumvent an issue where the compiler
      // allocates stack space for every case branch when no optimizations are
      // enabled. https://github.com/apple/swift-protobuf/issues/1034
      switch fieldNumber {
      case 1: try { try decoder.decodeSingularBytesField(value: &self.groupID) }()
      case 2: try { try decoder.decodeSingularStringField(value: &self.content) }()
      case 3: try { try decoder.decodeSingularUInt64Field(value: &self.sentAt) }()
      default: break
      }
    }
  }

  func traverse<V: SwiftProtobuf.Visitor>(visitor: inout V) throws {
    if !self.groupID.isEmpty {
      try visitor.visitSingularBytesField(value: self.groupID, fieldNumber: 1)
    }
    if !self.content.isEmpty {
      try visitor.visitSingularStringField(value: self.content, fieldNumber: 2)
    }
    if self.sentAt != 0 {
      try visitor.visitSingularUInt64Field(value: self.sentAt, fieldNumber: 3)
    }
    try unknownFields.traverse(visitor: &visitor)
  }

  static func ==(lhs: Qaul_Net_Group_GroupMessage, rhs: Qaul_Net_Group_GroupMessage) -> Bool {
    if lhs.groupID != rhs.groupID {return false}
    if lhs.content != rhs.content {return false}
    if lhs.sentAt != rhs.sentAt {return false}
    if lhs.unknownFields != rhs.unknownFields {return false}
    return true
  }
}
