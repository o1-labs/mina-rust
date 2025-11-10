-- Create indexes for common queries
CREATE INDEX CONCURRENTLY idx_blocks_height ON blocks(height);
CREATE INDEX CONCURRENTLY idx_blocks_timestamp ON blocks(timestamp);
CREATE INDEX CONCURRENTLY idx_user_commands_source ON user_commands(source_id);
CREATE INDEX CONCURRENTLY idx_user_commands_receiver ON user_commands(receiver_id);
