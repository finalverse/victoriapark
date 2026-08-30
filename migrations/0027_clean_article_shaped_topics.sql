-- Older model responses occasionally placed a whole digest in the title or
-- standfirst. They are not topic pages and should not remain in the archive.
DELETE FROM gaggles
 WHERE NOT pinned
   AND (
     char_length(title) > CASE WHEN editorial_language IN ('zh','zh-hant') THEN 48 ELSE 100 END
     OR title ~ E'[\n\r]'
     OR title LIKE '###%'
     OR standfirst LIKE '%###%'
     OR char_length(standfirst) > 700
   );

-- Make all surviving live clusters eligible for the containment merger on
-- the first fast newsroom pass after deployment.
UPDATE gaggles SET last_hot_at=now() WHERE NOT pinned;
