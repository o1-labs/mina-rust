-- Account balance history (requires account state tracking)
SELECT
    aa.account_identifier_id,
    pk.value as public_key,
    aa.balance,
    b.height,
    b.timestamp
FROM accounts_accessed aa
JOIN public_keys pk ON aa.account_identifier_id = pk.id
JOIN blocks b ON aa.block_id = b.id
WHERE pk.value = 'YOUR_PUBLIC_KEY_HERE'
ORDER BY b.height DESC;
