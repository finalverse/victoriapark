-- A named person is already a precise dossier signal. Requiring a second
-- sector keyword excluded ordinary headlines such as "Trump addresses..." or
-- "Musk says..." even though the subject was explicit. Conflict and trade
-- watches intentionally keep the stricter anchor-plus-signal rule to avoid
-- pulling every story that merely mentions a country.
UPDATE gaggles
SET keywords = ARRAY(
        SELECT DISTINCT term
        FROM unnest(keywords || anchor_terms) AS term
        WHERE btrim(term) <> ''
    ),
    last_searched_at = NULL
WHERE topic IN (
    'tracked:donald-trump',
    'tracked:elon-musk',
    'tracked:jensen-huang'
);
