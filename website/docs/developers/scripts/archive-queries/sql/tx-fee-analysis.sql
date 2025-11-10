-- Transaction fee analysis
SELECT
    percentile_cont(0.5) WITHIN GROUP (ORDER BY uc.fee::bigint) as median_fee,
    percentile_cont(0.75) WITHIN GROUP (ORDER BY uc.fee::bigint) as p75_fee,
    percentile_cont(0.95) WITHIN GROUP (ORDER BY uc.fee::bigint) as p95_fee,
    AVG(uc.fee::bigint) as avg_fee,
    MIN(uc.fee::bigint) as min_fee,
    MAX(uc.fee::bigint) as max_fee
FROM user_commands uc
WHERE uc.command_type = 'payment';
