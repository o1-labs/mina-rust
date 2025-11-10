-- Get recent blocks
SELECT
    b.height,
    b.state_hash,
    b.parent_hash,
    pk.value as creator,
    b.timestamp
FROM blocks b
JOIN public_keys pk ON b.creator_id = pk.id
ORDER BY b.height DESC
LIMIT 10;
