-- Transaction volume analysis
SELECT
    DATE(to_timestamp(b.timestamp::bigint / 1000)) as date,
    COUNT(uc.id) as tx_count,
    SUM(uc.amount::bigint) as total_volume,
    AVG(uc.fee::bigint) as avg_fee
FROM user_commands uc
JOIN blocks_user_commands buc ON uc.id = buc.user_command_id
JOIN blocks b ON buc.block_id = b.id
WHERE uc.command_type = 'payment'
GROUP BY DATE(to_timestamp(b.timestamp::bigint / 1000))
ORDER BY date DESC;
