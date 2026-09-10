# Definition and store migration policy

Workflow definitions and accepted plans are immutable once admitted. Phase 1
does not support editing a live definition, moving a cursor, changing a pinned
version, lowering a hard policy, or migrating a pending effect to a new payload.

To publish a change, create a new version and validate it against the current
capability manifest. A new run pins the new semantic digest; existing runs keep
their old digest and journal. If the new version is disabled, stop new
admissions and keep status, replay and recovery visibility for existing runs.

Store schema changes require an explicit producer migration with a tested backup
and recovery path. A newer schema, failed migration, integrity mismatch or
unknown event is a safe failure. Destructive downgrade and silent event rewrite
are prohibited.
