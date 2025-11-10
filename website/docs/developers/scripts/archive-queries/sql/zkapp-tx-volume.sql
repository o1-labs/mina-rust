-- zkApp transaction volume
SELECT
    DATE(to_timestamp(b.timestamp::bigint / 1000)) as date,
    COUNT(zc.id) as zkapp_count
FROM zkapp_commands zc
JOIN blocks_zkapp_commands bzc ON zc.id = bzc.zkapp_command_id
JOIN blocks b ON bzc.block_id = b.id
GROUP BY DATE(to_timestamp(b.timestamp::bigint / 1000))
ORDER BY date DESC;
