-- Get block statistics by creator
SELECT
    pk.value as creator,
    COUNT(*) as blocks_produced,
    MIN(b.height) as first_block,
    MAX(b.height) as latest_block
FROM blocks b
JOIN public_keys pk ON b.creator_id = pk.id
GROUP BY pk.value
ORDER BY blocks_produced DESC;
