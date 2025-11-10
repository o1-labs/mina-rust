-- Most active accounts
SELECT
    pk.value as public_key,
    COUNT(*) as transaction_count
FROM user_commands uc
JOIN public_keys pk ON uc.source_id = pk.id
GROUP BY pk.value
ORDER BY transaction_count DESC
LIMIT 10;
