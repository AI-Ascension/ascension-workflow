# Workflow context-binding catalog v1

This owner-produced catalog resolves workflow-definition `context_ref` values
for authoring validation. A binding says only that a named reference is
available for one or more workflow node kinds. It does not expose context
content, provider configuration, a runtime capture identity, or control
authority.

The harness uses a disclosed catalog to reject an unresolved reference and to
reject a reference used by an incompatible node kind. An owner that cannot yet
produce this catalog must leave it absent rather than manufacture bindings from
fixture identifiers. Studio may present the catalog as authoring metadata, but
editing a workflow definition does not create or alter an active runtime
context revision.
