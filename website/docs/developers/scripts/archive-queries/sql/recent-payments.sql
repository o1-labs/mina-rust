-- Get recent payments
SELECT
    b.height,
    b.timestamp,
    pk_source.value as source,
    pk_receiver.value as receiver,
    uc.amount,
    uc.fee
FROM user_commands uc
JOIN blocks_user_commands buc ON uc.id = buc.user_command_id
JOIN blocks b ON buc.block_id = b.id
JOIN public_keys pk_source ON uc.source_id = pk_source.id
JOIN public_keys pk_receiver ON uc.receiver_id = pk_receiver.id
WHERE uc.command_type = 'payment'
ORDER BY b.height DESC
LIMIT 20;
