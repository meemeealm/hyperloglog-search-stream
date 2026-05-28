import json
import random
import time
import urllib.request

# 1. Configuration
# Change 'localhost:8080' to match whatever port the Rust app is listening on
# API_URL = "http://127.0.0.1:3000/track"

# API for Docker 
API_URL = "http://rust-engine:3000/track"

TOTAL_REQUESTS = 50000  # How many total hits to send
UNIQUE_USERS_POOL = 10000  # The actual number of unique users we will rotate through

# A list of fake search queries to randomize
QUERIES = ["rust tutorial", "how to build api", "hyperloglog explained", "databases", "data engineering"]

print(f"Starting stream simulation... Sending {TOTAL_REQUESTS} requests.")
print(f"Real unique user count should be around: {UNIQUE_USERS_POOL}\n")

# 2. Generate a fixed pool of User IDs
# This allows us to send duplicate users so we can test if your HLL ignores them!
user_pool = [f"user_{i}" for i in range(UNIQUE_USERS_POOL)]

# 3. The loop that simulates the stream
for i in range(TOTAL_REQUESTS):
    # Pick a random user from our pool and a random query
    payload = {
        "query": random.choice(QUERIES),
        "user_id": random.choice(user_pool)
    }
    
    # Convert the Python dictionary into a JSON string and encode it to bytes
    json_data = json.dumps(payload).encode('utf-8')
    
    # Prepare the HTTP POST request
    req = urllib.request.Request(
        API_URL, 
        data=json_data, 
        headers={'Content-Type': 'application/json'},
        method='POST'
    )
    
    try:
        # Send the request to your Rust app
        with urllib.request.urlopen(req) as response:
            # We don't need to do anything with the response, just let it pass
            pass
    except Exception as e:
        print(f"❌ Error sending request {i}: {e}")
        print("Is your Rust app running?")
        break

    # Print progress every 100 requests so you know it's working
    if (i + 1) % 100 == 0:
        print(f"✅ Sent {i + 1}/{TOTAL_REQUESTS} requests...")
        
    # Optional: Pause for a tiny fraction of a second so it looks like a real stream
    time.sleep(0.005) 

print("\n🏁 Stream finished! Now go check your Rust /get endpoint.")