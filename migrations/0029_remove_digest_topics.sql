-- A special topic is one event, not a generic roundup that happens to contain
-- many articles. Remove legacy mixed-digest pages; the runtime validator now
-- rejects the same language before insertion.
DELETE FROM gaggles
 WHERE NOT pinned
   AND lower(title) ~ '(汇总|彙總|概览|概覽|热点回顾|熱點回顧|多领域|多領域|多起事件|动态更新|動態更新|news roundup|weekly roundup)';
