import psycopg2
import pandas as pd
import matplotlib.pyplot as plt

# Connect to archive database
conn = psycopg2.connect(
    host="localhost",
    port=5432,
    database="archive",
    user="postgres",
    password="mina"
)

# Analyze block production over time
query = """
SELECT
    DATE(to_timestamp(timestamp::bigint / 1000)) as date,
    COUNT(*) as blocks_per_day,
    COUNT(DISTINCT creator_id) as unique_producers
FROM blocks
WHERE timestamp > extract(epoch from now() - interval '30 days') * 1000
GROUP BY date
ORDER BY date;
"""

df = pd.read_sql_query(query, conn)
print(df)

# Plot block production
df.plot(x='date', y='blocks_per_day', kind='line')
plt.title('Daily Block Production')
plt.show()
