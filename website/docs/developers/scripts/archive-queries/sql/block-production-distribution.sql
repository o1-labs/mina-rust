-- Block production distribution
SELECT
    pk.value as producer,
    COUNT(*) as blocks_produced,
    ROUND(COUNT(*) * 100.0 / SUM(COUNT(*)) OVER(), 2) as percentage
FROM blocks b
JOIN public_keys pk ON b.creator_id = pk.id
GROUP BY pk.value
ORDER BY blocks_produced DESC;
