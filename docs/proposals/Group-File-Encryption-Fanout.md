# Reducing per-recipient file encryption cost in groups

## Problem

For a chat group of `N` members, the sender today encrypts the whole file
`N − 1` times — once under each pairwise Noise session — and ships
`N − 1` full copies on the wire. A 10 MB file in a 100-person group costs
~1 GB of ciphertext.

Call sites: `services::chat::file::send_filecontainer_to_group`
(`rust/libqaul/src/services/chat/file.rs:849`) hands one payload to
`Group::send_to_remote_members`
(`rust/libqaul/src/services/group/mod.rs:158`), which loops members and
calls `Messaging::pack_and_send_message` — one full-file Noise AEAD per
member.

## Proposal: envelope encryption

For each file the sender:

1. Generates a random 32-byte **CEK** (ChaCha20-Poly1305 key).
2. AEAD-encrypts the body **once** under the CEK → content-addressable
   blob keyed by `file_id`.
3. For each remote member, AEAD-wraps the CEK under that member's Noise
   transport key and sends a small `FileKeyWrap` over the existing
   messaging path.
4. Recipients fetch the body once from any holder (sender, DTN custody
   node, or peer who already pulled it), unwrap the CEK, decrypt.

Sender cost: `1 × file_size` AEAD + `N × ~80 bytes` AEAD.

## Wire format

```proto
// Body — stored in a content-addressable blob tree, never inline on
// Noise sessions.
message FileEnvelope {
    bytes file_id = 1;          // BLAKE3(sender || created_at || nonce || len)
    bytes sender_id = 2;
    uint64 created_at = 3;
    string mime_type = 4;
    uint64 plaintext_length = 5;
    bytes ciphertext_nonce = 6; // 12 bytes
    bytes ciphertext = 7;       // AEAD(CEK, ...); CEK never stored here
}

// Per-recipient wrap, rides the existing Noise messaging frame.
message FileKeyWrap {
    bytes file_id = 1;
    bytes wrapped_cek = 2;       // AEAD under recipient's transport key
    repeated bytes available_at = 3;  // peer ids known to hold file_id
}
```

`FileKeyWrap` becomes a new variant in
`messaging::proto::common_message::Payload`, gated on a new
`Capabilities::ENVELOPE_FILE = 1 << 2` bit. Members without the bit
fall back to today's inline path — no flag-day.

## Threat model

No worse than today. The CEK is per file (not per group): a removed
member retains access to files whose wrap they already received, and
new members cannot read old files — both are unchanged from current
behaviour. No group-level forward-secrecy ratchet is introduced.

## Out of scope

- MLS / sender keys / any group key agreement. Bigger commitment,
  conflicts with DTN's offline tolerance, not warranted as a first
  step.
- Post-compromise security for files (would require re-encrypting
  bodies — exactly the cost we're avoiding).
- Chunked / streaming envelopes for very large files. Single-blob
  format above; a chunk extension can land later without changing the
  wrap path.

## Open questions

- `wrapped_cek` nonce: dedicated, or reuse the surrounding Noise frame
  counter? Draft assumes reuse.
- Signed `available_at` assertions so recipients don't waste GETs on
  liars — cheap to add later.
- Eviction: receipt-driven on the sender, size-budget on custody nodes;
  shares primitives with the receiver-confirmation rotation work.
