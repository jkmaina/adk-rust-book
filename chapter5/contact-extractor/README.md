# Contact Extractor

**What:** Extract structured contact information (name, email, phone) from unstructured text using AI.

**Why:** Real applications need to parse contact info from emails, documents, and messages. This shows how to use structured output for data extraction.

## What This Example Shows

- **Data extraction**: Pulling specific information from unstructured text
- **Structured output**: Forcing consistent JSON format
- **Optional fields**: Some fields may or may not be present
- **Boolean flags**: Using `found` to indicate if contact info was detected
- **Multiple examples**: Processing different input formats

## Prerequisites

Set your Google API key in `.env` file:
```text
GOOGLE_API_KEY=your_api_key_here
```

Or in PowerShell:
```powershell
$env:GOOGLE_API_KEY = 'your_api_key_here'
```

## Run It

```bash
cargo run
```

This launches an interactive session. Try these inputs:

```
You: Contact John Doe at john@example.com or call 555-1234
Agent: {
  "name": "John Doe",
  "email": "john@example.com",
  "phone": "555-1234",
  "found": true
}

You: Reach out to Alice Smith via alice.smith@company.com
Agent: {
  "name": "Alice Smith",
  "email": "alice.smith@company.com",
  "found": true
}

You: No contact information here
Agent: {
  "name": "",
  "found": false
}
```

## Why This Matters

**Data extraction** is crucial for:
- Processing customer inquiries
- Parsing resumes and applications
- Extracting info from documents
- Building CRM systems
- Automating data entry

**Structured output ensures**:
- Consistent format every time
- Easy to parse and store in databases
- Type safety (email format, boolean flags)
- Handles missing data gracefully

## Beginner's Explanation

**The problem**: Unstructured text
```
"Contact John Doe at john@example.com or call 555-1234"
```

**The solution**: Structured JSON
```json
{
  "name": "John Doe",
  "email": "john@example.com",
  "phone": "555-1234",
  "found": true
}
```

**How the schema works**:
```json
{
  "properties": {
    "name": { "type": "string" },           // Required
    "email": { "type": "string" },          // Optional
    "phone": { "type": "string" },          // Optional
    "found": { "type": "boolean" }          // Required
  },
  "required": ["name", "found"]
}
```

- `name` and `found` are required (always present)
- `email` and `phone` are optional (may be missing)
- `found: false` when no contact info detected

**Real-world use cases**:
1. Email parser: Extract sender info from email signatures
2. Resume parser: Pull contact details from resumes
3. Form processor: Extract data from scanned documents
4. Chat bot: Capture contact info from conversations

This pattern works for any extraction task—just change the schema!

**Try it interactively**: The agent runs with Launcher, so you can test different text inputs and see the structured JSON output in real-time.
