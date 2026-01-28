# qaul Authentication System

This document describes the authentication system used by qaul to authenticate users from UI clients (such as `qaul-cli`) towards libqaul, which manages all user resources.

## Overview

The authentication system provides secure user authentication using:
- **Optional password protection** - User accounts can optionally set a password
- **Access tokens** - Users can secure their account via persistent session tokens
- **Challenge-response protocol** - Secure authentication without transmitting passwords

## Architecture

```
┌─────────────────┐                    ┌─────────────────┐
│    qaul-cli     │                    │    libqaul      │
│   (UI Client)   │◄──── Protobuf ────►│   (Node)        │
│                 │        RPC         │                 │
│ authentication  │                    │ authentication  │
│     .rs         │                    │     .rs         │
└─────────────────┘                    └─────────────────┘
```

### Key Files

| Component | File Path | Description |
|-----------|-----------|-------------|
| UI Client | `clients/cli/src/authentication.rs` | Handles user-facing authentication flow |
| libqaul Node | `libqaul/src/rpc/authentication.rs` | Server-side authentication logic |
| Protocol | `libqaul/src/rpc/authentication.proto` | Protobuf message definitions |
| User Accounts | `clients/cli/src/user_accounts.rs` | CLI user account management and auth state |
| Configuration | `libqaul/src/storage/configuration.rs` | Stores user credentials and tokens |

## Authentication Flows

### 1. Password-Based Authentication

For accounts with a password set, a secure challenge-response mechanism is used:

```
CLI                                          libqaul
 │                                              │
 │──────── UsersRequest ───────────────────────►│
 │                                              │
 │◄─────── UsersResponse ──────────────────────│
 │         (username, user_id, salt,           │
 │          has_password)                       │
 │                                              │
 │──────── AuthRequest ────────────────────────►│
 │         (qaul_id)                            │
 │                                              │
 │◄─────── AuthChallenge ──────────────────────│
 │         (nonce, expires_at)                  │
 │                                              │
 │──────── AuthResponse ───────────────────────►│
 │         (challenge_hash)                     │
 │                                              │
 │◄─────── AuthResult ─────────────────────────│
 │         (success, error_message)             │
```

#### Challenge-Response Computation

The client computes the challenge response using double Argon2 hashing:

1. **First hash**: `hash1 = Argon2(password, user_salt)`
2. **Combine with nonce**: `combined = hash1 + nonce`
3. **Second hash**: `challenge_hash = Argon2(combined, random_salt)`

The server verifies by:
1. Retrieving the stored password hash for the user
2. Combining the stored hash with the challenge nonce
3. Verifying the received hash matches using Argon2 verification

### 2. Passwordless Authentication

For accounts without a password:

```
CLI                                          libqaul
 │                                              │
 │──────── UsersRequest ───────────────────────►│
 │                                              │
 │◄─────── UsersResponse ──────────────────────│
 │         (has_password: false)                │
 │                                              │
 │  [Client generates token and saves session]  │
```

If `has_password` is false, the client immediately authenticates and generates a session token.

## Session Management

### Session Tokens

Session tokens are generated using Argon2:

```rust
let input = format!("{}:{}", user_id, username);
let token = Argon2::hash_password(input, random_salt);
```

Tokens are:
- **Persisted** in `config.yaml` under the user's `session_token` field
- **Non-expiring** - Sessions are set to expire after 100 years
- **Restored** automatically on CLI startup

### Session Storage

Tokens are stored in the libqaul configuration file:

```yaml
user_accounts:
  - name: "username"
    id: "user_id"
    keys: "..."
    password_hash: "..."      # Optional
    password_salt: "..."      # Optional
    session_token: "..."      # Current session token
```

### Session Operations

| Operation | Description |
|-----------|-------------|
| `restore_session()` | Load token from config on startup |
| `clear_session()` | Remove token from config (logout) |
| `save_token_to_config()` | Persist new token after authentication |

## Protobuf Protocol

### Message Types

```protobuf
message AuthRpc {
  oneof message {
    AuthRequest auth_request = 1;      // Request authentication
    AuthChallenge auth_challenge = 2;  // Server challenge
    AuthResponse auth_response = 3;    // Client response to challenge
    AuthResult auth_result = 4;        // Authentication result
    UsersRequest users_request = 5;    // Request user list
    UsersResponse users_response = 6;  // User list response
  }
}

message UserInfo {
  string username = 1;
  bytes user_id = 2;
  optional string salt = 3;    // Password salt for challenge computation
  bool has_password = 4;       // Whether password authentication is required
}

message AuthChallenge {
  uint64 nonce = 1;           // Unique challenge nonce
  uint64 expires_at = 2;      // Challenge expiration timestamp
}

message AuthResponse {
  bytes challenge_hash = 1;   // Computed challenge response
}

message AuthResult {
  bool success = 1;
  string error_message = 2;
}
```

## CLI Commands

### Account Management

```bash
# Create account without password
account create {username}

# Create account with password
account create {username} -p {password}
account create {username} -p              # Prompts for password

# Login
account login {username} -p {password}
account login {username} -p               # Prompts for password

# Logout
account logout

# Check authentication status
account status

# Set/change password for current user
account password                          # Prompts for password
```

## Security Considerations

1. **Password Storage**: Passwords are never stored in plaintext. Only Argon2 hashes are stored.

2. **Challenge-Response**: The challenge-response mechanism prevents:
   - Password transmission over the wire
   - Replay attacks (unique nonces)

3. **Session Tokens**:
   - Generated with cryptographically secure random salts
   - Stored locally in the configuration file
   - Should be protected with appropriate file permissions

4. **Non-Expiring Sessions**: Currently, sessions do not expire automatically. Users should explicitly logout when needed.

## Implementation Details

### libqaul Server State

The server maintains:
- `NONCE_COUNTER`: Monotonically increasing counter for unique nonces
- `ACTIVE_CHALLENGES`: Map of pending authentication challenges by user ID
- `AUTHENTICATED_USERS`: Map of authenticated user sessions with expiration times

### Challenge Lifecycle

1. Challenge created with unique nonce
2. Stored in `ACTIVE_CHALLENGES` map
3. On successful verification, user added to `AUTHENTICATED_USERS`
4. Challenge kept for retry on verification failure
5. Expired challenges cleaned up periodically

### Error Handling

Authentication can fail due to:
- User not found
- Invalid credentials (hash mismatch)
- Challenge expired
- Invalid password hash format
- No active challenge for verification

