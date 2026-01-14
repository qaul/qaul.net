# Authentication System Security Analysis

**Date**: January 2026
**Scope**: qaul authentication system (`clients/cli`, `libqaul`)
**Classification**: Security Assessment Report

---

## Executive Summary

The qaul authentication system implements a challenge-response mechanism using Argon2 password hashing for user authentication between CLI clients and the libqaul node. While the system employs modern cryptographic primitives, several security concerns were identified that warrant attention.

**Risk Rating**: **Medium**

| Finding | Severity | Status |
|---------|----------|--------|
| Non-expiring sessions | Medium | Open |
| Challenge expiration disabled | Medium | Open |
| Session token predictability | Low | Open |
| No rate limiting | Medium | Open |
| Debug logging of sensitive data | Low | Open |
| Passwordless accounts bypass | Low | By Design |

---

## 1. Architecture Analysis

### 1.1 Trust Boundaries

```
┌──────────────────────────────────────────────────────────────┐
│                    Local Machine                              │
│  ┌─────────────┐         RPC          ┌─────────────────┐    │
│  │  qaul-cli   │◄─────(protobuf)─────►│    libqaul      │    │
│  │  (Client)   │                       │    (Node)       │    │
│  └─────────────┘                       └─────────────────┘    │
│         │                                      │              │
│         ▼                                      ▼              │
│  ┌─────────────┐                       ┌─────────────────┐    │
│  │ config.yaml │                       │  config.yaml    │    │
│  │ (tokens)    │                       │  (credentials)  │    │
│  └─────────────┘                       └─────────────────┘    │
└──────────────────────────────────────────────────────────────┘
```

**Observation**: The authentication occurs locally between processes on the same machine. The primary threat model appears to be protecting user accounts on a shared node, not network-based attacks.

### 1.2 Cryptographic Primitives

| Component | Algorithm | Assessment |
|-----------|-----------|------------|
| Password Hashing | Argon2 (default params) | **Good** - Industry standard |
| Challenge Response | Double Argon2 | **Good** - Prevents replay |
| Token Generation | Argon2 with random salt | **Acceptable** |
| Nonce Generation | Monotonic counter | **Concern** - See Section 3.2 |

---

## 2. Password Security

### 2.1 Password Storage

**Location**: `libqaul/src/storage/configuration.rs:132-140`

```rust
pub struct UserAccount {
    pub password_hash: Option<String>,
    pub password_salt: Option<String>,
    pub session_token: Option<String>,
    // ...
}
```

**Assessment**:
- Passwords are stored as Argon2 hashes (not plaintext)
- Salt is stored separately and also extractable from the PHC hash format
- Stored in `config.yaml` file

**Concern**: The configuration file stores sensitive credentials. File permissions are not explicitly set or verified.

### 2.2 Password Hashing Parameters

**Location**: `clients/cli/src/authentication.rs:252`

```rust
let argon2 = Argon2::default();
```

**Assessment**: Uses default Argon2 parameters. The Rust argon2 crate defaults are reasonable (Argon2id, 19MB memory, 2 iterations, 1 parallelism), but these should be explicitly configured for security-critical applications.

---

## 3. Challenge-Response Mechanism

### 3.1 Protocol Flow

The challenge-response mechanism is well-designed:

1. Server generates unique nonce
2. Client computes: `H(H(password, salt) || nonce)`
3. Server verifies using stored hash

This prevents:
- Password transmission
- Simple replay attacks (nonce uniqueness)

### 3.2 Nonce Generation Vulnerability

**Location**: `libqaul/src/rpc/authentication.rs:57-62`

```rust
fn next_nonce() -> u64 {
    let mut counter = NONCE_COUNTER.get().write().unwrap();
    let nonce = *counter;
    *counter += 1;
    nonce
}
```

**Concern**:
- Nonces start at 1 and increment monotonically
- Nonces reset to 1 on node restart
- **Predictable nonces** could enable pre-computation attacks if an attacker can predict the next nonce

**Recommendation**: Use cryptographically random nonces:
```rust
use rand::RngCore;
fn next_nonce() -> u64 {
    rand::thread_rng().next_u64()
}
```

### 3.3 Challenge Expiration Disabled

**Location**: `libqaul/src/rpc/authentication.rs:85-86, 132-137`

```rust
expires_at: now + 9999999999 // Change to never expired(as discussed)
// ...
/*if now > challenge.expires_at {
    // ... expiration check commented out
}*/
```

**Critical Finding**: Challenge expiration has been intentionally disabled. Challenges effectively never expire.

**Risk**:
- Challenges remain valid indefinitely
- An attacker who obtains a challenge can attempt brute-force at leisure
- No protection against long-running attacks

**Recommendation**: Re-enable challenge expiration with a reasonable timeout (e.g., 5 minutes).

---

## 4. Session Management

### 4.1 Non-Expiring Sessions

**Location**: `libqaul/src/rpc/authentication.rs:183`

```rust
let expires_at = Timestamp::get_timestamp() + (86400 * 365 * 100); // 100 years
```

**Critical Finding**: Sessions are configured to expire after 100 years, effectively making them permanent.

**Risk**:
- Stolen session tokens remain valid indefinitely
- No automatic session rotation
- Compromised tokens cannot be invalidated by time

**Recommendation**: Implement reasonable session expiration (e.g., 30 days) with refresh mechanism.

### 4.2 Token Generation

**Location**: `clients/cli/src/authentication.rs:42-59`

```rust
fn generate_token(user_id: &str, username: &str) -> String {
    let input = format!("{}:{}", user_id, username);
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    match argon2.hash_password(input.as_bytes(), &salt) {
        Ok(hash) => hash.to_string(),
        Err(_) => bs58::encode(input.as_bytes()).into_string() // Fallback
    }
}
```

**Concerns**:
1. **Fallback is insecure**: If Argon2 fails, falls back to base58 encoding (not cryptographic)
2. **Deterministic input**: Token is derived from `user_id:username`, not random
3. **Salt provides entropy**: The random salt makes tokens unpredictable, but the base input is known

**Recommendation**: Generate tokens using pure random bytes:
```rust
fn generate_token() -> String {
    let mut bytes = [0u8; 32];
    OsRng.fill_bytes(&mut bytes);
    bs58::encode(bytes).into_string()
}
```

### 4.3 Token Storage

**Location**: `libqaul/src/storage/configuration.rs`

Tokens are stored in `config.yaml` in plaintext.

**Risk**: Any process with read access to the config file can steal session tokens.

**Recommendation**:
- Set restrictive file permissions (0600)
- Consider encrypting tokens at rest
- Store tokens in a dedicated secure storage

---

## 5. Authentication Bypass Scenarios

### 5.1 Passwordless Accounts

**Location**: `libqaul/src/rpc/authentication.rs:141-150`

```rust
let stored_hash = match user.password_hash {
    Some(hash) => hash,
    None => {
        // No password - immediately authenticate
        Self::mark_authenticated(qaul_id);
        return Ok(true);
    }
};
```

**Observation**: Accounts without passwords are automatically authenticated upon request. This is by design but creates risk if users don't understand the security implications.

**Recommendation**: Warn users during account creation about passwordless account risks.

### 5.2 No Rate Limiting

**Location**: All authentication handlers

**Finding**: No rate limiting on authentication attempts.

**Risk**:
- Brute-force attacks on passwords
- Denial of service through authentication flooding

**Recommendation**: Implement:
- Per-user rate limiting (e.g., 5 attempts per minute)
- Exponential backoff after failed attempts
- Account lockout after N failures

---

## 6. Information Disclosure

### 6.1 Debug Logging

**Location**: `libqaul/src/rpc/authentication.rs:66-69, 108-125`

```rust
println!("LIBQAUL: Creating challenge for qaul_id: {:?}", qaul_id.to_bytes());
// ...
log::info!("Storing challenge with key: {:?}", qaul_id_bytes);
log::info!("Looking for challenge with key: {:?}", qaul_id_bytes);
```

**Concern**: Extensive debug logging of user IDs and authentication state could leak sensitive information in production logs.

**Recommendation**:
- Remove `println!` statements from production code
- Use appropriate log levels (debug/trace, not info)
- Avoid logging raw user identifiers

### 6.2 User Enumeration

**Location**: `clients/cli/src/authentication.rs:233-240`

```rust
println!("User '{}' not found", pending_username);
println!("Available users:");
for user in &response.users {
    println!("  - {}", user.username);
}
```

**Concern**: Failed login reveals all available usernames, enabling user enumeration.

**Recommendation**: Return generic "Invalid credentials" message without listing users.

---

## 7. Protocol Security

### 7.1 Salt Transmission

**Location**: `libqaul/src/rpc/authentication.rs:324-335`

```rust
let salt = if let Some(ref s) = user_config.password_salt {
    Some(s.clone())
} else if let Some(ref hash) = user_config.password_hash {
    // Extract salt from hash
    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() >= 5 {
        Some(parts[4].to_string())
    }
    // ...
}
```

**Observation**: Password salt is sent to the client during `UsersResponse`. This is necessary for the client-side challenge computation but exposes the salt.

**Risk**: Known salt reduces password security margin slightly (attacker can pre-compute rainbow tables for specific salts).

**Mitigating Factor**: Argon2 with proper parameters is resistant to pre-computation attacks.

### 7.2 No Transport Security

**Observation**: The RPC communication uses protobuf over an unspecified transport. If this runs over network (not just local IPC), credentials could be exposed.

**Recommendation**: Ensure RPC communication is:
- Local only (Unix sockets), or
- Encrypted (TLS) if network-accessible

---

## 8. Summary of Recommendations

### High Priority

1. **Re-enable challenge expiration** - Set reasonable timeout (5 minutes)
2. **Implement session expiration** - 30-day maximum with refresh
3. **Add rate limiting** - Prevent brute-force attacks
4. **Use cryptographic random nonces** - Replace monotonic counter

### Medium Priority

5. **Fix token fallback** - Remove insecure base58 fallback
6. **Remove debug logging** - Clean up sensitive data logging
7. **Fix user enumeration** - Don't list users on failed login

### Low Priority

8. **Secure config file permissions** - Enforce 0600 permissions
9. **Document security model** - Clarify passwordless account risks
10. **Explicit Argon2 parameters** - Don't rely on defaults

---

## 9. Positive Findings

The system does implement several security best practices:

1. **Argon2 password hashing** - Modern, secure algorithm
2. **Challenge-response protocol** - Prevents password transmission
3. **No plaintext password storage** - Only hashes stored
4. **Salt per user** - Prevents rainbow table attacks
5. **Nonce in challenge** - Prevents simple replay attacks

---

## Appendix: Files Reviewed

| File | Lines | Purpose |
|------|-------|---------|
| `clients/cli/src/authentication.rs` | 332 | CLI authentication handler |
| `libqaul/src/rpc/authentication.rs` | 364 | Server authentication logic |
| `libqaul/src/rpc/authentication.proto` | 47 | Protocol definitions |
| `libqaul/src/storage/configuration.rs` | 372 | Credential storage |
| `clients/cli/src/user_accounts.rs` | 448 | User account management |
