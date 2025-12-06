# agent-sandbox

A small example agent using adk-rust that launches an interactive assistant.

Quick start (assumes Rust is already installed):

1. Make sure you have an API key and set it in your environment or in a `.env` file:
   - `.env` at project root:
     ```text
     GOOGLE_API_KEY=your_api_key_here
     ```
   - Or set it for the session (PowerShell):
     ```powershell
     $env:GOOGLE_API_KEY = 'YOUR_KEY'
     ```

2. Build + run:
   ```powershell
   cd agent-sandbox
   cargo run
   ```

