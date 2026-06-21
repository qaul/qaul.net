// Copyright (c) 2026 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # Group membership & metadata CRDT
//!
//! Replaces the monotonic `Group::revision` last-writer-wins merge with
//! a conflict-free replicated data type so two partitioned members can
//! concurrently invite / remove members and edit metadata, and every
//! replica converges to the same state deterministically.
//!
//! See `docs/proposals/Group-State-CRDT.md`.
//!
//! ## Model
//!
//! The state is a **grow-only set of signed operations** (`GroupOp`),
//! deduplicated by `op_id`. Merge is the set union — unconditional and
//! commutative/associative/idempotent, so it is a proper CRDT.
//!
//! The membership / metadata *view* is **derived** from the op set by a
//! deterministic fold in total `(lamport, actor_id, op_id)` order:
//!
//! - **Membership** is an Observed-Remove Set (OR-Set) keyed by
//!   `member_id`: every `Add` carries a unique `op_id`; a `Remove`
//!   tombstones the `op_id`s it observed. A member is present iff it has
//!   at least one `Add` whose `op_id` is not tombstoned, so a concurrent
//!   re-add (fresh `op_id`) survives a remove — **add-wins** on a
//!   concurrent add-vs-remove of the same member (the right default for
//!   mesh invites; flagged for product review in the proposal).
//! - **Metadata** (name, avatar) is a per-field last-writer-wins
//!   register keyed by `(lamport, actor_id)`.
//!
//! ## Authorization is checked at *apply* time, not merge time
//!
//! Every op merges regardless of whether its actor was allowed to issue
//! it; the fold simply skips ops that fail the authorization check
//! against the view built so far. Because the view is re-derived from
//! the whole op set, an op that was unauthorized when first received can
//! become authorized later — e.g. once an admin-promotion with a lower
//! lamport is merged, an op that depended on that admin status applies
//! retroactively, with no rebroadcast.
//!
//! This core module is deliberately free of protobuf / signature /
//! storage concerns so the merge + authorization logic can be unit
//! tested in isolation. Ops handed to [`GroupCrdt::merge_op`] are
//! assumed already signature-verified by the caller.

use std::collections::{BTreeMap, BTreeSet};

/// 16-byte random operation id (UUID).
pub type OpId = [u8; 16];

/// Role byte: `0` = member, `255` = admin (matches the wire `role`).
pub const ROLE_MEMBER: i32 = 0;
pub const ROLE_ADMIN: i32 = 255;

/// A single signed group operation. `signature` and decoding live in
/// the wire layer; this core treats an op as already-verified data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GroupOp {
    pub op_id: OpId,
    /// PeerId bytes of the signer.
    pub actor_id: Vec<u8>,
    /// Group-local lamport clock value of this op.
    pub lamport: u64,
    /// Wall-clock ms at creation — display/diagnostics only; never a
    /// merge or authorization input.
    pub created_at: u64,
    pub kind: OpKind,
}

/// The operation payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpKind {
    /// Add (or re-add, or promote) a member.
    Add { member_id: Vec<u8>, role: i32 },
    /// Remove a member, tombstoning the add `op_id`s it observed.
    Remove {
        member_id: Vec<u8>,
        observed_adds: Vec<OpId>,
    },
    /// Update metadata fields (only the `Some` fields are written).
    UpdateMetadata {
        name: Option<String>,
        avatar: Option<Vec<u8>>,
    },
    /// Compaction barrier: ops with `lamport < below` are no longer
    /// accepted once this epoch is in effect (tombstone GC). See
    /// [`GroupCrdt::merge_op`].
    Compact { epoch: u64, below: u64 },
}

/// A member as seen in the derived view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberView {
    pub role: i32,
}

impl MemberView {
    pub fn is_admin(&self) -> bool {
        self.role == ROLE_ADMIN
    }
}

/// The derived, converged view of a group's membership and metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct GroupView {
    /// Present members, keyed by member_id, with their effective role.
    pub members: BTreeMap<Vec<u8>, MemberView>,
    pub name: Option<String>,
    pub avatar: Option<Vec<u8>>,
    /// Highest lamport observed across applied ops (for the next tick).
    pub max_lamport: u64,
}

impl GroupView {
    pub fn is_member(&self, id: &[u8]) -> bool {
        self.members.contains_key(id)
    }
    pub fn is_admin(&self, id: &[u8]) -> bool {
        self.members.get(id).is_some_and(|m| m.is_admin())
    }
}

/// CRDT state for one group: the founder, the grow-only op set, and the
/// compaction horizon.
#[derive(Debug, Clone)]
pub struct GroupCrdt {
    /// The group creator. Bootstrap admin, always present, can never be
    /// removed, and is the only actor allowed to remove an admin.
    founder: Vec<u8>,
    /// When true, only admins may `Add` members (invite-only); when
    /// false, any member may add (open invite). Apply-time policy only —
    /// the same op wire shape is used either way.
    invite_only: bool,
    /// All merged ops, deduplicated by `op_id`.
    ops: BTreeMap<OpId, GroupOp>,
    /// Active compaction floor: ops with `lamport < compaction_below`
    /// are rejected by `merge_op` (a peer resurfacing beyond the
    /// horizon cannot reintroduce collapsed state). 0 = no compaction.
    compaction_below: u64,
    /// Highest compaction epoch applied.
    epoch: u64,
}

impl GroupCrdt {
    /// Create CRDT state for a group founded by `founder`.
    pub fn new(founder: Vec<u8>) -> Self {
        GroupCrdt {
            founder,
            invite_only: false,
            ops: BTreeMap::new(),
            compaction_below: 0,
            epoch: 0,
        }
    }

    /// Set invite-only policy (admins-only adds). Apply-time only.
    pub fn set_invite_only(&mut self, invite_only: bool) {
        self.invite_only = invite_only;
    }

    pub fn founder(&self) -> &[u8] {
        &self.founder
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }

    /// Number of ops currently retained (for tests / metrics).
    pub fn op_count(&self) -> usize {
        self.ops.len()
    }

    /// All `Add` op_ids currently in the set for `member_id`. Used to
    /// build the `observed_adds` of a `Remove` so it tombstones every
    /// add this replica has seen for that member (OR-Set semantics).
    pub fn add_op_ids_for(&self, member_id: &[u8]) -> Vec<OpId> {
        self.ops
            .values()
            .filter_map(|op| match &op.kind {
                OpKind::Add { member_id: m, .. } if m.as_slice() == member_id => Some(op.op_id),
                _ => None,
            })
            .collect()
    }

    /// Merge one op into the set. Idempotent (dedup by `op_id`) and
    /// order-independent. Returns `true` if the op was newly stored.
    ///
    /// Ops below the active compaction floor are **rejected, not
    /// merged** — a peer that resurfaces beyond the compaction horizon
    /// cannot reintroduce state that was collapsed away. A `Compact` op
    /// raises the floor (and, being itself an op, also advances the
    /// lamport clock and prunes now-stale ops).
    pub fn merge_op(&mut self, op: GroupOp) -> bool {
        // reject ops below the compaction floor (except Compact itself,
        // which establishes the floor)
        if op.lamport < self.compaction_below && !matches!(op.kind, OpKind::Compact { .. }) {
            return false;
        }
        if self.ops.contains_key(&op.op_id) {
            return false;
        }

        // A Compact op raises the floor for everyone and prunes ops
        // that are now below it. Authorization for Compact is enforced
        // in the fold (admin-only); but the floor is structural so it
        // applies regardless — an unauthorized Compact still merges as
        // an op and is simply ignored by the view if it didn't come
        // from an admin. To avoid an unauthorized peer collapsing
        // state, only honour the floor for Compacts that the view
        // accepts; see `recompute_compaction`.
        let newly = self.ops.insert(op.op_id, op).is_none();
        if newly {
            self.recompute_compaction();
        }
        newly
    }

    /// Re-derive the compaction floor from authorized `Compact` ops and
    /// prune ops that fall below it.
    fn recompute_compaction(&mut self) {
        // Derive the view first (authorization needs it), then find the
        // highest authorized Compact.
        let view = self.view();
        let mut floor = 0u64;
        let mut epoch = 0u64;
        for op in self.ops.values() {
            if let OpKind::Compact { epoch: e, below } = op.kind {
                // Only admins may compact.
                if view.is_admin(&op.actor_id) || op.actor_id == self.founder {
                    if e > epoch {
                        epoch = e;
                        floor = below;
                    }
                }
            }
        }
        self.compaction_below = floor;
        self.epoch = epoch;
        if floor > 0 {
            // prune ops strictly below the floor (keep Compact ops so
            // the floor stays derivable)
            self.ops.retain(|_, op| {
                op.lamport >= floor || matches!(op.kind, OpKind::Compact { .. })
            });
        }
    }

    /// The next lamport value this replica should stamp on a new op:
    /// one above the highest lamport seen.
    pub fn next_lamport(&self) -> u64 {
        self.ops.values().map(|o| o.lamport).max().unwrap_or(0) + 1
    }

    /// Derive the converged membership / metadata view by folding the
    /// op set in total `(lamport, actor_id, op_id)` order, enforcing
    /// authorization against the view built so far.
    pub fn view(&self) -> GroupView {
        // total order: lamport, then actor, then op_id — deterministic
        // across replicas regardless of merge/arrival order.
        let mut ordered: Vec<&GroupOp> = self.ops.values().collect();
        ordered.sort_by(|a, b| {
            a.lamport
                .cmp(&b.lamport)
                .then_with(|| a.actor_id.cmp(&b.actor_id))
                .then_with(|| a.op_id.cmp(&b.op_id))
        });

        // OR-Set working state: per member, the live (non-tombstoned)
        // add op_ids with their role + ordering key.
        let mut live_adds: BTreeMap<Vec<u8>, BTreeMap<OpId, AddTag>> = BTreeMap::new();
        let mut tombstoned: BTreeSet<OpId> = BTreeSet::new();

        // founder is a synthetic, un-removable admin add at the very
        // bottom of the order.
        live_adds
            .entry(self.founder.clone())
            .or_default()
            .insert(
                [0u8; 16],
                AddTag {
                    role: ROLE_ADMIN,
                    lamport: 0,
                    actor: self.founder.clone(),
                },
            );

        // metadata LWW registers
        let mut name_reg: Option<LwwReg<String>> = None;
        let mut avatar_reg: Option<LwwReg<Vec<u8>>> = None;
        let mut max_lamport = 0u64;

        // helper: is `id` an admin given current live_adds?
        let is_admin = |live: &BTreeMap<Vec<u8>, BTreeMap<OpId, AddTag>>, id: &[u8], founder: &[u8]| -> bool {
            if id == founder {
                return true;
            }
            match live.get(id) {
                Some(adds) if !adds.is_empty() => effective_role(adds) == ROLE_ADMIN,
                _ => false,
            }
        };
        let is_member = |live: &BTreeMap<Vec<u8>, BTreeMap<OpId, AddTag>>, id: &[u8], founder: &[u8]| -> bool {
            id == founder || live.get(id).is_some_and(|a| !a.is_empty())
        };

        for op in ordered {
            max_lamport = max_lamport.max(op.lamport);
            match &op.kind {
                OpKind::Add { member_id, role } => {
                    // tombstoned adds never come alive
                    if tombstoned.contains(&op.op_id) {
                        continue;
                    }
                    // authorization: promotion to admin requires actor
                    // be an admin; a plain add requires actor be a
                    // member (or admin if invite-only).
                    let authorized = if *role == ROLE_ADMIN {
                        is_admin(&live_adds, &op.actor_id, &self.founder)
                    } else if self.invite_only {
                        is_admin(&live_adds, &op.actor_id, &self.founder)
                    } else {
                        is_member(&live_adds, &op.actor_id, &self.founder)
                    };
                    if !authorized {
                        continue;
                    }
                    live_adds.entry(member_id.clone()).or_default().insert(
                        op.op_id,
                        AddTag {
                            role: *role,
                            lamport: op.lamport,
                            actor: op.actor_id.clone(),
                        },
                    );
                }
                OpKind::Remove {
                    member_id,
                    observed_adds,
                } => {
                    // the founder can never be removed
                    if member_id.as_slice() == self.founder.as_slice() {
                        continue;
                    }
                    // actor must be an admin...
                    if !is_admin(&live_adds, &op.actor_id, &self.founder) {
                        continue;
                    }
                    // ...and removing an admin requires the founder.
                    let target_is_admin = is_admin(&live_adds, member_id, &self.founder);
                    if target_is_admin && op.actor_id.as_slice() != self.founder.as_slice() {
                        continue;
                    }
                    // tombstone observed adds (OR-Set semantics)
                    for add_id in observed_adds {
                        tombstoned.insert(*add_id);
                        if let Some(adds) = live_adds.get_mut(member_id) {
                            adds.remove(add_id);
                        }
                    }
                }
                OpKind::UpdateMetadata { name, avatar } => {
                    if !is_admin(&live_adds, &op.actor_id, &self.founder) {
                        continue;
                    }
                    let key = LwwKey {
                        lamport: op.lamport,
                        actor: op.actor_id.clone(),
                    };
                    if let Some(n) = name {
                        if LwwReg::should_write(&name_reg, &key) {
                            name_reg = Some(LwwReg {
                                key: key.clone(),
                                value: n.clone(),
                            });
                        }
                    }
                    if let Some(a) = avatar {
                        if LwwReg::should_write(&avatar_reg, &key) {
                            avatar_reg = Some(LwwReg {
                                key: key.clone(),
                                value: a.clone(),
                            });
                        }
                    }
                }
                OpKind::Compact { .. } => {
                    // structural; handled in recompute_compaction
                }
            }
        }

        // build member view: a member is present iff it has ≥1 live add
        let mut members = BTreeMap::new();
        for (id, adds) in &live_adds {
            if adds.is_empty() {
                continue;
            }
            members.insert(
                id.clone(),
                MemberView {
                    role: effective_role(adds),
                },
            );
        }

        GroupView {
            members,
            name: name_reg.map(|r| r.value),
            avatar: avatar_reg.map(|r| r.value),
            max_lamport,
        }
    }
}

/// A live add's role plus its ordering key.
#[derive(Debug, Clone)]
struct AddTag {
    role: i32,
    lamport: u64,
    actor: Vec<u8>,
}

/// The effective role for a member is taken from its live add with the
/// highest `(lamport, actor)` — so the most recent promotion/demotion
/// among concurrent adds wins deterministically.
fn effective_role(adds: &BTreeMap<OpId, AddTag>) -> i32 {
    adds.values()
        .max_by(|a, b| {
            a.lamport
                .cmp(&b.lamport)
                .then_with(|| a.actor.cmp(&b.actor))
        })
        .map(|t| t.role)
        .unwrap_or(ROLE_MEMBER)
}

/// Ordering key for an LWW register write.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LwwKey {
    lamport: u64,
    actor: Vec<u8>,
}

impl LwwKey {
    fn gt(&self, other: &LwwKey) -> bool {
        (self.lamport, &self.actor) > (other.lamport, &other.actor)
    }
}

struct LwwReg<T> {
    key: LwwKey,
    value: T,
}

impl<T> LwwReg<T> {
    /// A write with `key` wins over the current register iff the
    /// register is empty or `key` is strictly greater.
    fn should_write(current: &Option<LwwReg<T>>, key: &LwwKey) -> bool {
        match current {
            None => true,
            Some(reg) => key.gt(&reg.key),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn id(b: u8) -> Vec<u8> {
        vec![b; 4]
    }
    fn oid(b: u8) -> OpId {
        let mut o = [0u8; 16];
        o[0] = b;
        o
    }

    fn add(op_id: u8, actor: u8, lamport: u64, member: u8, role: i32) -> GroupOp {
        GroupOp {
            op_id: oid(op_id),
            actor_id: id(actor),
            lamport,
            created_at: 0,
            kind: OpKind::Add { member_id: id(member), role },
        }
    }
    fn remove(op_id: u8, actor: u8, lamport: u64, member: u8, observed: &[u8]) -> GroupOp {
        GroupOp {
            op_id: oid(op_id),
            actor_id: id(actor),
            lamport,
            created_at: 0,
            kind: OpKind::Remove {
                member_id: id(member),
                observed_adds: observed.iter().map(|b| oid(*b)).collect(),
            },
        }
    }
    fn meta(op_id: u8, actor: u8, lamport: u64, name: Option<&str>) -> GroupOp {
        GroupOp {
            op_id: oid(op_id),
            actor_id: id(actor),
            lamport,
            created_at: 0,
            kind: OpKind::UpdateMetadata { name: name.map(|s| s.to_string()), avatar: None },
        }
    }

    // founder (id 1) is always an admin member with no ops.
    #[test]
    fn founder_is_bootstrap_admin() {
        let c = GroupCrdt::new(id(1));
        let v = c.view();
        assert!(v.is_member(&id(1)));
        assert!(v.is_admin(&id(1)));
        assert_eq!(v.members.len(), 1);
    }

    // founder adds a member (open invite); member is present, not admin.
    #[test]
    fn open_invite_add() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER));
        let v = c.view();
        assert!(v.is_member(&id(2)));
        assert!(!v.is_admin(&id(2)));
    }

    // a non-member cannot add under open invite (not a member yet).
    #[test]
    fn non_member_cannot_add() {
        let mut c = GroupCrdt::new(id(1));
        // actor 9 is not a member
        c.merge_op(add(10, 9, 1, 2, ROLE_MEMBER));
        assert!(!c.view().is_member(&id(2)), "add by non-member is a no-op");
    }

    // remove tombstones the observed add → member gone.
    #[test]
    fn remove_member() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER));
        // founder removes member 2, observing add op 10
        c.merge_op(remove(11, 1, 2, 2, &[10]));
        assert!(!c.view().is_member(&id(2)));
    }

    // re-add after a remove works (fresh op_id not tombstoned).
    #[test]
    fn re_add_after_remove() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER));
        c.merge_op(remove(11, 1, 2, 2, &[10]));
        c.merge_op(add(12, 1, 3, 2, ROLE_MEMBER)); // fresh add
        assert!(c.view().is_member(&id(2)), "re-add resurrects the member");
    }

    // concurrent add-vs-remove of the same member: the remove only
    // tombstones the add it observed; a concurrent fresh add survives
    // (ADD-WINS).
    #[test]
    fn concurrent_add_wins_over_remove() {
        let mut c = GroupCrdt::new(id(1));
        // founder promotes 2 to admin so 2 can add; and adds 3 originally
        c.merge_op(add(10, 1, 1, 2, ROLE_ADMIN));
        c.merge_op(add(11, 1, 2, 3, ROLE_MEMBER)); // original add of 3 (op 11)
        // partition: admin 2 removes 3 observing op 11; concurrently
        // founder re-adds 3 with a fresh op 12 it has NOT observed.
        c.merge_op(remove(20, 2, 5, 3, &[11]));
        c.merge_op(add(12, 1, 5, 3, ROLE_MEMBER));
        assert!(c.view().is_member(&id(3)), "concurrent fresh add wins");
    }

    // merge order does not affect the converged view (commutativity).
    #[test]
    fn merge_order_independent() {
        let ops = vec![
            add(10, 1, 1, 2, ROLE_ADMIN),
            add(11, 2, 2, 3, ROLE_MEMBER),
            remove(12, 1, 3, 3, &[11]),
            add(13, 1, 4, 3, ROLE_MEMBER),
            meta(14, 2, 5, Some("hello")),
        ];
        let mut a = GroupCrdt::new(id(1));
        for op in &ops {
            a.merge_op(op.clone());
        }
        let mut b = GroupCrdt::new(id(1));
        for op in ops.iter().rev() {
            b.merge_op(op.clone());
        }
        assert_eq!(a.view(), b.view(), "view is independent of merge order");
    }

    // an op that needs admin rights but arrives before the promotion
    // that grants them is retroactively legitimised once the promotion
    // (with a lower lamport) is merged.
    #[test]
    fn retroactive_legitimization() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)); // 2 is a plain member
        c.merge_op(add(11, 1, 2, 3, ROLE_MEMBER)); // 3 is a member
        // 2 (not admin) removes 3 at lamport 5 → unauthorized, no-op
        c.merge_op(remove(20, 2, 5, 3, &[11]));
        assert!(c.view().is_member(&id(3)), "remove by non-admin is a no-op");
        // founder promotes 2 to admin at lamport 3 (< 5)
        c.merge_op(add(21, 1, 3, 2, ROLE_ADMIN));
        // now, re-derived in lamport order, 2 is admin by the time its
        // remove at L5 is folded → the remove applies retroactively
        assert!(!c.view().is_member(&id(3)), "remove now legitimised");
    }

    // only admins may remove; and only the founder may remove an admin.
    #[test]
    fn only_founder_removes_admin() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_ADMIN)); // 2 admin
        c.merge_op(add(11, 1, 2, 3, ROLE_ADMIN)); // 3 admin
        // admin 2 tries to remove admin 3 → not allowed (not founder)
        c.merge_op(remove(20, 2, 5, 3, &[11]));
        assert!(c.view().is_admin(&id(3)), "admin cannot remove another admin");
        // founder removes admin 3 → allowed
        c.merge_op(remove(21, 1, 6, 3, &[11]));
        assert!(!c.view().is_member(&id(3)), "founder can remove an admin");
    }

    // an admin CAN remove a plain member.
    #[test]
    fn admin_removes_plain_member() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_ADMIN)); // 2 admin
        c.merge_op(add(11, 1, 2, 3, ROLE_MEMBER)); // 3 member
        c.merge_op(remove(20, 2, 5, 3, &[11])); // admin 2 removes member 3
        assert!(!c.view().is_member(&id(3)));
    }

    // the founder can never be removed, even by themselves.
    #[test]
    fn founder_cannot_be_removed() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(remove(20, 1, 5, 1, &[]));
        assert!(c.view().is_member(&id(1)), "founder is un-removable");
        assert!(c.view().is_admin(&id(1)));
    }

    // invite-only flips add authorization to admins-only.
    #[test]
    fn invite_only_requires_admin() {
        let mut c = GroupCrdt::new(id(1));
        c.set_invite_only(true);
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)); // founder(admin) adds 2: ok
        c.merge_op(add(11, 2, 2, 3, ROLE_MEMBER)); // member 2 adds 3: NOT ok
        let v = c.view();
        assert!(v.is_member(&id(2)));
        assert!(!v.is_member(&id(3)), "non-admin add blocked in invite-only");
    }

    // metadata LWW: the write with the higher (lamport, actor) wins.
    #[test]
    fn metadata_lww() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_ADMIN)); // 2 admin so it can edit
        c.merge_op(meta(20, 1, 5, Some("from-founder")));
        c.merge_op(meta(21, 2, 7, Some("from-admin-2"))); // higher lamport wins
        assert_eq!(c.view().name.as_deref(), Some("from-admin-2"));
        // a lower-lamport write does not override
        c.merge_op(meta(22, 1, 3, Some("stale")));
        assert_eq!(c.view().name.as_deref(), Some("from-admin-2"));
    }

    // non-admin metadata edit is a no-op.
    #[test]
    fn metadata_requires_admin() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)); // 2 plain member
        c.merge_op(meta(20, 2, 5, Some("nope")));
        assert_eq!(c.view().name, None, "non-admin metadata edit ignored");
    }

    // merging the same op twice is idempotent.
    #[test]
    fn idempotent_merge() {
        let mut c = GroupCrdt::new(id(1));
        assert!(c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)));
        assert!(!c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)), "dup op rejected");
        assert_eq!(c.op_count(), 1);
    }

    // compaction raises the floor; ops below it are rejected on merge.
    #[test]
    fn compaction_rejects_stale_ops() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_ADMIN));
        c.merge_op(add(11, 1, 10, 3, ROLE_MEMBER));
        // founder compacts everything below lamport 8
        c.merge_op(GroupOp {
            op_id: oid(50),
            actor_id: id(1),
            lamport: 11,
            created_at: 0,
            kind: OpKind::Compact { epoch: 1, below: 8 },
        });
        assert_eq!(c.epoch(), 1);
        // a late op from a resurfacing peer below the floor is rejected
        let accepted = c.merge_op(remove(60, 1, 5, 3, &[11]));
        assert!(!accepted, "op below compaction floor is rejected");
        assert!(c.view().is_member(&id(3)), "stale remove never applied");
    }

    // a non-admin Compact does not move the floor.
    #[test]
    fn compaction_requires_admin() {
        let mut c = GroupCrdt::new(id(1));
        c.merge_op(add(10, 1, 1, 2, ROLE_MEMBER)); // 2 plain member
        c.merge_op(GroupOp {
            op_id: oid(50),
            actor_id: id(2),
            lamport: 5,
            created_at: 0,
            kind: OpKind::Compact { epoch: 1, below: 3 },
        });
        assert_eq!(c.epoch(), 0, "non-admin compact ignored");
    }
}
