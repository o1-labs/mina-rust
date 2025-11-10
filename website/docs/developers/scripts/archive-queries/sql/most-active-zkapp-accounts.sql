-- Most active zkApp accounts
SELECT
    pk.value as public_key,
    COUNT(*) as zkapp_transactions
FROM zkapp_commands zc
JOIN zkapp_fee_payer_body zfpb ON zc.zkapp_fee_payer_body_id = zfpb.id
JOIN public_keys pk ON zfpb.public_key_id = pk.id
GROUP BY pk.value
ORDER BY zkapp_transactions DESC
LIMIT 10;
