// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: services/crypto/crypto_net.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_services_2fcrypto_2fcrypto_5fnet_2eproto_2epb_2eh
#define GOOGLE_PROTOBUF_INCLUDED_services_2fcrypto_2fcrypto_5fnet_2eproto_2epb_2eh

#include <limits>
#include <string>
#include <type_traits>

#include "google/protobuf/port_def.inc"
#if PROTOBUF_VERSION < 4023000
#error "This file was generated by a newer version of protoc which is"
#error "incompatible with your Protocol Buffer headers. Please update"
#error "your headers."
#endif  // PROTOBUF_VERSION

#if 4023004 < PROTOBUF_MIN_PROTOC_VERSION
#error "This file was generated by an older version of protoc which is"
#error "incompatible with your Protocol Buffer headers. Please"
#error "regenerate this file with a newer version of protoc."
#endif  // PROTOBUF_MIN_PROTOC_VERSION
#include "google/protobuf/port_undef.inc"
#include "google/protobuf/io/coded_stream.h"
#include "google/protobuf/arena.h"
#include "google/protobuf/arenastring.h"
#include "google/protobuf/generated_message_util.h"
#include "google/protobuf/metadata_lite.h"
#include "google/protobuf/generated_message_reflection.h"
#include "google/protobuf/message.h"
#include "google/protobuf/repeated_field.h"  // IWYU pragma: export
#include "google/protobuf/extension_set.h"  // IWYU pragma: export
#include "google/protobuf/unknown_field_set.h"
// @@protoc_insertion_point(includes)

// Must be included last.
#include "google/protobuf/port_def.inc"

#define PROTOBUF_INTERNAL_EXPORT_services_2fcrypto_2fcrypto_5fnet_2eproto

PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_services_2fcrypto_2fcrypto_5fnet_2eproto {
  static const ::uint32_t offsets[];
};
extern const ::PROTOBUF_NAMESPACE_ID::internal::DescriptorTable
    descriptor_table_services_2fcrypto_2fcrypto_5fnet_2eproto;
namespace qaul {
namespace net {
namespace crypto {
class CryptoserviceContainer;
struct CryptoserviceContainerDefaultTypeInternal;
extern CryptoserviceContainerDefaultTypeInternal _CryptoserviceContainer_default_instance_;
class SecondHandshake;
struct SecondHandshakeDefaultTypeInternal;
extern SecondHandshakeDefaultTypeInternal _SecondHandshake_default_instance_;
}  // namespace crypto
}  // namespace net
}  // namespace qaul
PROTOBUF_NAMESPACE_OPEN
template <>
::qaul::net::crypto::CryptoserviceContainer* Arena::CreateMaybeMessage<::qaul::net::crypto::CryptoserviceContainer>(Arena*);
template <>
::qaul::net::crypto::SecondHandshake* Arena::CreateMaybeMessage<::qaul::net::crypto::SecondHandshake>(Arena*);
PROTOBUF_NAMESPACE_CLOSE

namespace qaul {
namespace net {
namespace crypto {

// ===================================================================


// -------------------------------------------------------------------

class CryptoserviceContainer final :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:qaul.net.crypto.CryptoserviceContainer) */ {
 public:
  inline CryptoserviceContainer() : CryptoserviceContainer(nullptr) {}
  ~CryptoserviceContainer() override;
  template<typename = void>
  explicit PROTOBUF_CONSTEXPR CryptoserviceContainer(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  CryptoserviceContainer(const CryptoserviceContainer& from);
  CryptoserviceContainer(CryptoserviceContainer&& from) noexcept
    : CryptoserviceContainer() {
    *this = ::std::move(from);
  }

  inline CryptoserviceContainer& operator=(const CryptoserviceContainer& from) {
    CopyFrom(from);
    return *this;
  }
  inline CryptoserviceContainer& operator=(CryptoserviceContainer&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  inline const ::PROTOBUF_NAMESPACE_ID::UnknownFieldSet& unknown_fields() const {
    return _internal_metadata_.unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(::PROTOBUF_NAMESPACE_ID::UnknownFieldSet::default_instance);
  }
  inline ::PROTOBUF_NAMESPACE_ID::UnknownFieldSet* mutable_unknown_fields() {
    return _internal_metadata_.mutable_unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>();
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return default_instance().GetMetadata().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return default_instance().GetMetadata().reflection;
  }
  static const CryptoserviceContainer& default_instance() {
    return *internal_default_instance();
  }
  enum MessageCase {
    kSecondHandshake = 1,
    MESSAGE_NOT_SET = 0,
  };

  static inline const CryptoserviceContainer* internal_default_instance() {
    return reinterpret_cast<const CryptoserviceContainer*>(
               &_CryptoserviceContainer_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(CryptoserviceContainer& a, CryptoserviceContainer& b) {
    a.Swap(&b);
  }
  inline void Swap(CryptoserviceContainer* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(CryptoserviceContainer* other) {
    if (other == this) return;
    ABSL_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  CryptoserviceContainer* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<CryptoserviceContainer>(arena);
  }
  using ::PROTOBUF_NAMESPACE_ID::Message::CopyFrom;
  void CopyFrom(const CryptoserviceContainer& from);
  using ::PROTOBUF_NAMESPACE_ID::Message::MergeFrom;
  void MergeFrom( const CryptoserviceContainer& from) {
    CryptoserviceContainer::MergeImpl(*this, from);
  }
  private:
  static void MergeImpl(::PROTOBUF_NAMESPACE_ID::Message& to_msg, const ::PROTOBUF_NAMESPACE_ID::Message& from_msg);
  public:
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  ::size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  ::uint8_t* _InternalSerialize(
      ::uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _impl_._cached_size_.Get(); }

  private:
  void SharedCtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(CryptoserviceContainer* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::absl::string_view FullMessageName() {
    return "qaul.net.crypto.CryptoserviceContainer";
  }
  protected:
  explicit CryptoserviceContainer(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  static const ClassData _class_data_;
  const ::PROTOBUF_NAMESPACE_ID::Message::ClassData*GetClassData() const final;

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kSecondHandshakeFieldNumber = 1,
  };
  // .qaul.net.crypto.SecondHandshake second_handshake = 1;
  bool has_second_handshake() const;
  private:
  bool _internal_has_second_handshake() const;

  public:
  void clear_second_handshake() ;
  const ::qaul::net::crypto::SecondHandshake& second_handshake() const;
  PROTOBUF_NODISCARD ::qaul::net::crypto::SecondHandshake* release_second_handshake();
  ::qaul::net::crypto::SecondHandshake* mutable_second_handshake();
  void set_allocated_second_handshake(::qaul::net::crypto::SecondHandshake* second_handshake);
  private:
  const ::qaul::net::crypto::SecondHandshake& _internal_second_handshake() const;
  ::qaul::net::crypto::SecondHandshake* _internal_mutable_second_handshake();
  public:
  void unsafe_arena_set_allocated_second_handshake(
      ::qaul::net::crypto::SecondHandshake* second_handshake);
  ::qaul::net::crypto::SecondHandshake* unsafe_arena_release_second_handshake();
  void clear_message();
  MessageCase message_case() const;
  // @@protoc_insertion_point(class_scope:qaul.net.crypto.CryptoserviceContainer)
 private:
  class _Internal;
  void set_has_second_handshake();

  inline bool has_message() const;
  inline void clear_has_message();

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  struct Impl_ {
    union MessageUnion {
      constexpr MessageUnion() : _constinit_{} {}
        ::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized _constinit_;
      ::qaul::net::crypto::SecondHandshake* second_handshake_;
    } message_;
    mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
    ::uint32_t _oneof_case_[1];

  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_services_2fcrypto_2fcrypto_5fnet_2eproto;
};// -------------------------------------------------------------------

class SecondHandshake final :
    public ::PROTOBUF_NAMESPACE_ID::Message /* @@protoc_insertion_point(class_definition:qaul.net.crypto.SecondHandshake) */ {
 public:
  inline SecondHandshake() : SecondHandshake(nullptr) {}
  ~SecondHandshake() override;
  template<typename = void>
  explicit PROTOBUF_CONSTEXPR SecondHandshake(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  SecondHandshake(const SecondHandshake& from);
  SecondHandshake(SecondHandshake&& from) noexcept
    : SecondHandshake() {
    *this = ::std::move(from);
  }

  inline SecondHandshake& operator=(const SecondHandshake& from) {
    CopyFrom(from);
    return *this;
  }
  inline SecondHandshake& operator=(SecondHandshake&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  inline const ::PROTOBUF_NAMESPACE_ID::UnknownFieldSet& unknown_fields() const {
    return _internal_metadata_.unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>(::PROTOBUF_NAMESPACE_ID::UnknownFieldSet::default_instance);
  }
  inline ::PROTOBUF_NAMESPACE_ID::UnknownFieldSet* mutable_unknown_fields() {
    return _internal_metadata_.mutable_unknown_fields<::PROTOBUF_NAMESPACE_ID::UnknownFieldSet>();
  }

  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* descriptor() {
    return GetDescriptor();
  }
  static const ::PROTOBUF_NAMESPACE_ID::Descriptor* GetDescriptor() {
    return default_instance().GetMetadata().descriptor;
  }
  static const ::PROTOBUF_NAMESPACE_ID::Reflection* GetReflection() {
    return default_instance().GetMetadata().reflection;
  }
  static const SecondHandshake& default_instance() {
    return *internal_default_instance();
  }
  static inline const SecondHandshake* internal_default_instance() {
    return reinterpret_cast<const SecondHandshake*>(
               &_SecondHandshake_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    1;

  friend void swap(SecondHandshake& a, SecondHandshake& b) {
    a.Swap(&b);
  }
  inline void Swap(SecondHandshake* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(SecondHandshake* other) {
    if (other == this) return;
    ABSL_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  SecondHandshake* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<SecondHandshake>(arena);
  }
  using ::PROTOBUF_NAMESPACE_ID::Message::CopyFrom;
  void CopyFrom(const SecondHandshake& from);
  using ::PROTOBUF_NAMESPACE_ID::Message::MergeFrom;
  void MergeFrom( const SecondHandshake& from) {
    SecondHandshake::MergeImpl(*this, from);
  }
  private:
  static void MergeImpl(::PROTOBUF_NAMESPACE_ID::Message& to_msg, const ::PROTOBUF_NAMESPACE_ID::Message& from_msg);
  public:
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  ::size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  ::uint8_t* _InternalSerialize(
      ::uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _impl_._cached_size_.Get(); }

  private:
  void SharedCtor(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  void SharedDtor();
  void SetCachedSize(int size) const final;
  void InternalSwap(SecondHandshake* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::absl::string_view FullMessageName() {
    return "qaul.net.crypto.SecondHandshake";
  }
  protected:
  explicit SecondHandshake(::PROTOBUF_NAMESPACE_ID::Arena* arena);
  public:

  static const ClassData _class_data_;
  const ::PROTOBUF_NAMESPACE_ID::Message::ClassData*GetClassData() const final;

  ::PROTOBUF_NAMESPACE_ID::Metadata GetMetadata() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kSignatureFieldNumber = 1,
    kReceivedAtFieldNumber = 2,
  };
  // bytes signature = 1;
  void clear_signature() ;
  const std::string& signature() const;




  template <typename Arg_ = const std::string&, typename... Args_>
  void set_signature(Arg_&& arg, Args_... args);
  std::string* mutable_signature();
  PROTOBUF_NODISCARD std::string* release_signature();
  void set_allocated_signature(std::string* ptr);

  private:
  const std::string& _internal_signature() const;
  inline PROTOBUF_ALWAYS_INLINE void _internal_set_signature(
      const std::string& value);
  std::string* _internal_mutable_signature();

  public:
  // uint64 received_at = 2;
  void clear_received_at() ;
  ::uint64_t received_at() const;
  void set_received_at(::uint64_t value);

  private:
  ::uint64_t _internal_received_at() const;
  void _internal_set_received_at(::uint64_t value);

  public:
  // @@protoc_insertion_point(class_scope:qaul.net.crypto.SecondHandshake)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  struct Impl_ {
    ::PROTOBUF_NAMESPACE_ID::internal::ArenaStringPtr signature_;
    ::uint64_t received_at_;
    mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_services_2fcrypto_2fcrypto_5fnet_2eproto;
};

// ===================================================================




// ===================================================================


#ifdef __GNUC__
#pragma GCC diagnostic push
#pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// -------------------------------------------------------------------

// CryptoserviceContainer

// .qaul.net.crypto.SecondHandshake second_handshake = 1;
inline bool CryptoserviceContainer::has_second_handshake() const {
  return message_case() == kSecondHandshake;
}
inline bool CryptoserviceContainer::_internal_has_second_handshake() const {
  return message_case() == kSecondHandshake;
}
inline void CryptoserviceContainer::set_has_second_handshake() {
  _impl_._oneof_case_[0] = kSecondHandshake;
}
inline void CryptoserviceContainer::clear_second_handshake() {
  if (message_case() == kSecondHandshake) {
    if (GetArenaForAllocation() == nullptr) {
      delete _impl_.message_.second_handshake_;
    }
    clear_has_message();
  }
}
inline ::qaul::net::crypto::SecondHandshake* CryptoserviceContainer::release_second_handshake() {
  // @@protoc_insertion_point(field_release:qaul.net.crypto.CryptoserviceContainer.second_handshake)
  if (message_case() == kSecondHandshake) {
    clear_has_message();
    ::qaul::net::crypto::SecondHandshake* temp = _impl_.message_.second_handshake_;
    if (GetArenaForAllocation() != nullptr) {
      temp = ::PROTOBUF_NAMESPACE_ID::internal::DuplicateIfNonNull(temp);
    }
    _impl_.message_.second_handshake_ = nullptr;
    return temp;
  } else {
    return nullptr;
  }
}
inline const ::qaul::net::crypto::SecondHandshake& CryptoserviceContainer::_internal_second_handshake() const {
  return message_case() == kSecondHandshake
      ? *_impl_.message_.second_handshake_
      : reinterpret_cast<::qaul::net::crypto::SecondHandshake&>(::qaul::net::crypto::_SecondHandshake_default_instance_);
}
inline const ::qaul::net::crypto::SecondHandshake& CryptoserviceContainer::second_handshake() const {
  // @@protoc_insertion_point(field_get:qaul.net.crypto.CryptoserviceContainer.second_handshake)
  return _internal_second_handshake();
}
inline ::qaul::net::crypto::SecondHandshake* CryptoserviceContainer::unsafe_arena_release_second_handshake() {
  // @@protoc_insertion_point(field_unsafe_arena_release:qaul.net.crypto.CryptoserviceContainer.second_handshake)
  if (message_case() == kSecondHandshake) {
    clear_has_message();
    ::qaul::net::crypto::SecondHandshake* temp = _impl_.message_.second_handshake_;
    _impl_.message_.second_handshake_ = nullptr;
    return temp;
  } else {
    return nullptr;
  }
}
inline void CryptoserviceContainer::unsafe_arena_set_allocated_second_handshake(::qaul::net::crypto::SecondHandshake* second_handshake) {
  clear_message();
  if (second_handshake) {
    set_has_second_handshake();
    _impl_.message_.second_handshake_ = second_handshake;
  }
  // @@protoc_insertion_point(field_unsafe_arena_set_allocated:qaul.net.crypto.CryptoserviceContainer.second_handshake)
}
inline ::qaul::net::crypto::SecondHandshake* CryptoserviceContainer::_internal_mutable_second_handshake() {
  if (message_case() != kSecondHandshake) {
    clear_message();
    set_has_second_handshake();
    _impl_.message_.second_handshake_ = CreateMaybeMessage< ::qaul::net::crypto::SecondHandshake >(GetArenaForAllocation());
  }
  return _impl_.message_.second_handshake_;
}
inline ::qaul::net::crypto::SecondHandshake* CryptoserviceContainer::mutable_second_handshake() {
  ::qaul::net::crypto::SecondHandshake* _msg = _internal_mutable_second_handshake();
  // @@protoc_insertion_point(field_mutable:qaul.net.crypto.CryptoserviceContainer.second_handshake)
  return _msg;
}

inline bool CryptoserviceContainer::has_message() const {
  return message_case() != MESSAGE_NOT_SET;
}
inline void CryptoserviceContainer::clear_has_message() {
  _impl_._oneof_case_[0] = MESSAGE_NOT_SET;
}
inline CryptoserviceContainer::MessageCase CryptoserviceContainer::message_case() const {
  return CryptoserviceContainer::MessageCase(_impl_._oneof_case_[0]);
}
// -------------------------------------------------------------------

// SecondHandshake

// bytes signature = 1;
inline void SecondHandshake::clear_signature() {
  _impl_.signature_.ClearToEmpty();
}
inline const std::string& SecondHandshake::signature() const {
  // @@protoc_insertion_point(field_get:qaul.net.crypto.SecondHandshake.signature)
  return _internal_signature();
}
template <typename Arg_, typename... Args_>
inline PROTOBUF_ALWAYS_INLINE void SecondHandshake::set_signature(Arg_&& arg,
                                                     Args_... args) {
  ;
  _impl_.signature_.SetBytes(static_cast<Arg_&&>(arg), args..., GetArenaForAllocation());
  // @@protoc_insertion_point(field_set:qaul.net.crypto.SecondHandshake.signature)
}
inline std::string* SecondHandshake::mutable_signature() {
  std::string* _s = _internal_mutable_signature();
  // @@protoc_insertion_point(field_mutable:qaul.net.crypto.SecondHandshake.signature)
  return _s;
}
inline const std::string& SecondHandshake::_internal_signature() const {
  return _impl_.signature_.Get();
}
inline void SecondHandshake::_internal_set_signature(const std::string& value) {
  ;


  _impl_.signature_.Set(value, GetArenaForAllocation());
}
inline std::string* SecondHandshake::_internal_mutable_signature() {
  ;
  return _impl_.signature_.Mutable( GetArenaForAllocation());
}
inline std::string* SecondHandshake::release_signature() {
  // @@protoc_insertion_point(field_release:qaul.net.crypto.SecondHandshake.signature)
  return _impl_.signature_.Release();
}
inline void SecondHandshake::set_allocated_signature(std::string* value) {
  _impl_.signature_.SetAllocated(value, GetArenaForAllocation());
  #ifdef PROTOBUF_FORCE_COPY_DEFAULT_STRING
        if (_impl_.signature_.IsDefault()) {
          _impl_.signature_.Set("", GetArenaForAllocation());
        }
  #endif  // PROTOBUF_FORCE_COPY_DEFAULT_STRING
  // @@protoc_insertion_point(field_set_allocated:qaul.net.crypto.SecondHandshake.signature)
}

// uint64 received_at = 2;
inline void SecondHandshake::clear_received_at() {
  _impl_.received_at_ = ::uint64_t{0u};
}
inline ::uint64_t SecondHandshake::received_at() const {
  // @@protoc_insertion_point(field_get:qaul.net.crypto.SecondHandshake.received_at)
  return _internal_received_at();
}
inline void SecondHandshake::set_received_at(::uint64_t value) {
  _internal_set_received_at(value);
  // @@protoc_insertion_point(field_set:qaul.net.crypto.SecondHandshake.received_at)
}
inline ::uint64_t SecondHandshake::_internal_received_at() const {
  return _impl_.received_at_;
}
inline void SecondHandshake::_internal_set_received_at(::uint64_t value) {
  ;
  _impl_.received_at_ = value;
}

#ifdef __GNUC__
#pragma GCC diagnostic pop
#endif  // __GNUC__

// @@protoc_insertion_point(namespace_scope)
}  // namespace crypto
}  // namespace net
}  // namespace qaul


// @@protoc_insertion_point(global_scope)

#include "google/protobuf/port_undef.inc"

#endif  // GOOGLE_PROTOBUF_INCLUDED_services_2fcrypto_2fcrypto_5fnet_2eproto_2epb_2eh
