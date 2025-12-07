# my-first-agent

A small verification tool that checks your adk-rust setup and (optionally) makes a tiny LLM call.

What it does
- Loads environment variables (via `dotenv`)
- Checks `GOOGLE_API_KEY` is present
- Creates a GeminiModel and builds an agent
- Optionally performs a small LLM connectivity test when run with `--test-api` (this makes a real API request)

Prerequisites
- Rust installed (rustup + cargo)
- A valid `GOOGLE_API_KEY` (export to your shell or put it in a `.env` file)

Run
```powershell
Set-Location 'C:\Projects\adk-learning\my-first-agent'

# Default (no API call)
cargo run

# Run the optional live model test (will use your GOOGLE_API_KEY)
cargo run -- --test-api
```

Notes
- The live test sends a very small prompt ("Say 'Hello' in one word") and prints the model's short response.
- If the API key is missing the program will fail with a helpful message; set your environment variable for your operating system or put it in a `.env` file before running.
