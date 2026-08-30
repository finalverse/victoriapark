-- Final legacy variants of mixed-event roundup framing. A phrase signalling
-- several unrelated events cannot name one event-level special topic.
DELETE FROM gaggles
 WHERE NOT pinned
   AND lower(title) ~ '(多起|等引发关注|等引發關注|风云再起|風雲再起|等焦点|等焦點)';
