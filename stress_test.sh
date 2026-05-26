#!/bin/bash
echo "Generating high-throughput pipeline traffic..."

# Fire 100 fast requests to the ingestion channel using 10 unique users
for i in {1..100}
do
  # Clean assignment right under the loop declaration
  vicar_user=$((i % 10)) 
  
  curl -s -X POST http://127.0.0.1:3000/track \
    -H "Content-Type: application/json" \
    -d "{\"query\": \"rust performance\", \"user_id\": \"user_${vicar_user}\"}" > /dev/null &
done

wait
echo "✅ 100 events sent. Querying stream aggregations..."
sleep 1 

echo "------------------------------------------------"
echo "Server Response Metrics:"
curl -s http://127.0.0.1:3000/analytics/rust%20performance
echo ""
echo "------------------------------------------------"

echo ""
read -p "Press [Enter] key to close this test window..."