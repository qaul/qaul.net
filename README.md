# Alexandria

An experimental data persistence module. Handles both key-value stores
and blobs on disk, in user-defined namespaces and scopes that differentiate
in storage attributes. Supports pubkey crypto and provides some easy
utilities to get your data back!

## The problem

The problem that `alexandria` solves isn't one that nobody solved before,
rather it's one that everyone has to solve and aims to do it better.
Fundamentally there's two types of payloads: `KV` are key-value encoded
structures that are internally represented as `json`, and then `Blobs`,
which are literally just binary large objects, that are not parsed further
and passed through.

Every file is contained in a scope, which is optionally contained in
a namespace. `lib:spacekookie/messages` is the scope `messages` in
namespace `spacekookie`, while `lib:/messages` is the scope `messages`
in the root namespace.

A scope can have scope attributes such as "auth_required", "encrypted"
and a storage offset. Under the hood not every data entry in a scope
might yield in a new file (lots of smaller ones might be stored together
unless they are marked "fast", indicating that their contents cycle quickly),
but fundamentally all files in a `Scope` are stored somewhere near each other.


