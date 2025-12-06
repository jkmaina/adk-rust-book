# my-minimal-agent

A small demo crate that contains three beginner-friendly binaries demonstrating a minimal agent, an interactive CLI and a search-enabled agent.

Quick prerequisites
- Rust installed (rustup + cargo)
- A `GOOGLE_API_KEY` environment variable or `.env` file for the GeminiModel (used by the interactive/search binaries)
-- Ensure your Rust toolchain is installed and any native build tools required for your platform are available (e.g., a system C/C++ toolchain if native crates require compilation).

Available binaries
- minimal  — create the agent and exit (no CLI)
- interactive — interactive CLI assistant (launches a simple launcher)
- search — interactive assistant that includes a `GoogleSearchTool` (this project includes a small stub tool so examples work without a real web search API)
- custom_tools — a tiny interactive agent with a simple calculator FunctionTool (good for learning)

How to run
```powershell
Set-Location 'C:\Projects\adk-learning\my-minimal-agent'

# Build all binaries
cargo build --bins

# Run 'minimal'
cargo run --bin minimal

# Run the interactive CLI
cargo run --bin interactive

# Run the search-enabled assistant (uses the bundled stub GoogleSearchTool if you don't have an external search API)
cargo run --bin search

# Run the simple tools demo (calculator)
```powershell
# run the interactive calculator agent
cargo run --bin custom_tools
```

How the calculator tool works (short & beginner-friendly)
- When you run `custom_tools` you'll get an interactive launcher where you can ask math questions.
- The calculator tool is intentionally small and flexible — examples the model might call internally:

1) Pass a simple expression string
```json
{"expression": "1+1"}
```

2) Pass numbers and an operation
```json
{"a": 3, "b": 4, "operation": "multiply"}
```

3) The tool also accepts a JSON string (some LLMs pass a stringified JSON) or nested wrappers like
```json
{"arguments": {"a":"2","b":"5","operation":"add"}}
```

The tool accepts numbers as numbers or strings (e.g. `"a": "2"`) and supports `add`, `subtract`, `multiply`, and `divide`.
If the tool can't parse the input it returns a small error object such as `{ "error": "could not parse expression" }`.

This is a compact, learning-friendly example to help you experiment with how agents call tools and how function-call arguments can be shaped.
```

Using an API key
- Add a `.env` file to the project root:
  ```text
  GOOGLE_API_KEY=your_api_key_here
  ```
  Or set the variable in PowerShell for the current session:
  ```powershell
  $env:GOOGLE_API_KEY = 'your_api_key_here'
  ```

