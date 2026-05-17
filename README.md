# ADK-Rust Book Companion Examples

This repository is the official runnable companion workspace for the ADK-Rust book.

## Baseline

- ADK-Rust crates: `0.8.2`
- Rust: `1.92+`
- Edition: `2024`

The root workspace lets you compile the chapter crates together while still keeping each example in its own directory.

## What Is Included

- `chapter1/` through `chapter16/`: runnable example crates grouped by chapter
- `scripts/check-examples.sh`: offline workspace validation
- `scripts/check-drift.py`: edition and dependency drift checks
- `scripts/smoke-examples.sh`: opt-in live smoke runs for networked examples

## Repository Layout

- [chapter1](chapter1): Rust foundations examples
- [chapter2](chapter2): architecture and composition examples
- [chapter3](chapter3): first runnable ADK agents
- [chapter4](chapter4): output control and first production levers
- [chapter5](chapter5): session-backed agent behavior
- [chapter6](chapter6): function tools and agent-as-tool patterns
- [chapter7](chapter7): sequential, parallel, loop, and escalation workflows
- [chapter8](chapter8): session backends and persistence
- [chapter9](chapter9): callbacks, plugins, and guardrails
- [chapter10](chapter10): memory, artifacts, and RAG
- [chapter11](chapter11): routing and supervisor patterns
- [chapter12](chapter12): streaming, realtime, reasoning, and launcher examples
- [chapter13](chapter13): validation and evaluation examples
- [chapter14](chapter14): deployment and A2A packaging examples
- [chapter15](chapter15): telemetry and observability examples
- [chapter16](chapter16): auth, guardrails, audit, and retrieval governance

## Concepts Covered

- **Rust foundations for agent systems**: readers learn the Rust ideas that show up repeatedly in ADK-Rust code, including ownership, borrowing, traits, async execution, `Arc`, and `Result`. The goal is not generic language theory; it is learning how Rust’s type system, concurrency model, and error handling make production agent runtimes safer and easier to reason about.
- **ADK-Rust architecture**: readers learn how ADK-Rust is organized into service, agent, runner, and application layers. This gives them a mental model for where models, sessions, tools, plugins, memory, and deployment surfaces belong, so later examples feel like one coherent runtime instead of a pile of unrelated APIs.
- **First-agent construction**: readers learn the smallest honest runnable flow, including model setup, session creation, runner wiring, content construction, and streamed response handling. This matters because it teaches what actually happens during execution instead of hiding the runtime behind a convenience wrapper too early.
- **Instruction design and session-backed template personalization**: readers learn how instructions shape behavior and how session state can inject user-specific context such as name, language, expertise, or role. This is where agents stop feeling generic and start behaving like systems that respond differently for different users.
- **Structured output and schema-shaped responses**: readers learn how to move from conversational prose to responses that another system can consume predictably. The examples show how output schemas improve downstream integration, while also making clear that structural validity is not the same thing as factual correctness.
- **Session history, keyed state, and persistent session backends**: readers learn the difference between conversation history and explicit state, and how that model extends from in-memory development to PostgreSQL, MongoDB, and Neo4j-backed persistence. This is the foundation for multi-turn behavior, continuity, and long-running agent systems.
- **Function tools, multi-tool selection, and agent-as-tool delegation**: readers learn how agents move beyond language-only behavior by calling typed tools, choosing between multiple capabilities, and delegating to specialist sub-agents. This section shows how to design narrow capability boundaries that are easier to trust, test, and extend.
- **Sequential, parallel, loop, and escalation workflows**: readers learn when to choose ordered pipelines, concurrent specialist analysis, bounded refinement loops, or escalation flows. The emphasis is on workflow shape as an engineering decision: when process matters, orchestration often matters more than a single clever prompt.
- **Callbacks, plugins, and runtime guardrails**: readers learn the difference between local agent interception and runner-wide lifecycle interception, and how those surfaces are used for logging, early rejection, safety controls, and reusable runtime policy. This helps them place cross-cutting logic in the right layer instead of burying everything inside the agent.
- **Memory, artifacts, embedding pipelines, and retrieval-augmented generation**: readers learn how ADK-Rust separates short-term conversation state from longer-term recall, persistent artifacts, and external knowledge retrieval. The examples show when to store generated outputs, when to retrieve memory deliberately, and how embedding-based retrieval fits into a larger application architecture.
- **Conditional routing, supervisor patterns, and graph-based coordination**: readers learn how to move beyond static workflows into rule-based routing, model-driven routing, and explicit graph coordination. This matters for systems with multiple specialists, branching control flow, or stateful orchestration that has to remain inspectable.
- **Streaming, realtime interaction, reasoning traces, and launcher-based app wiring**: readers learn the difference between standard streamed responses, specialist realtime transports, and reasoning-aware execution surfaces. They also see how launcher-based wiring helps connect ADK logic to CLI, server, or app-level entry points without changing the underlying runtime model.
- **Validation, smoke testing, and evaluation workflows**: readers learn how to validate example crates offline, run selective live smokes when credentials are available, and think about evaluation as a separate discipline from compilation or unit-style checks. This section is about keeping agents correct over time, not just getting them to compile once.
- **Deployment packaging, A2A servers, and A2A clients**: readers learn how to package agent systems for real execution surfaces, including launcher-based entrypoints, HTTP serving, and Agent-to-Agent protocol scenarios. The key lesson is that deployment is not an afterthought; it is part of how the runtime gets exposed safely and predictably.
- **Telemetry, spans, usage tracking, and observability patterns**: readers learn how to instrument an agent system so operators can understand latency, usage, failure modes, and runtime behavior. This includes structured logs, spans, token usage recording, and the operational patterns needed when an agent becomes part of a production service.
- **Typed identity, RBAC, SSO, audit trails, and retrieval governance**: readers learn how security becomes concrete in an agent system through typed identifiers, runtime authorization, audit logging, claims mapping, and scoped retrieval boundaries. The focus is on building systems where unsafe behavior is harder to perform and easier to explain when it is blocked.

## Real-World AI Agent Systems You Can Build

- [customer-support-agent](customer-support-agent): a customer support agent that looks up order status, escalates refunds, and enforces role boundaries
- [internal-knowledge-assistant](internal-knowledge-assistant): an internal knowledge assistant with scoped RAG collections for engineering docs, HR policies, and operations playbooks
- [multilingual-user-assistant](multilingual-user-assistant): a multilingual assistant that adapts language, tone, and explanation depth from session-backed user state
- [structured-intake-extractor](structured-intake-extractor): a structured data extractor for contacts, tickets, or intake forms that returns schema-shaped output for downstream systems
- [research-writing-workflow](research-writing-workflow): a research-to-writing workflow that separates research, drafting, and editorial refinement into explicit stages
- [parallel-review-system](parallel-review-system): a parallel review system that gathers technical, product, and user-experience perspectives before producing one response
- [iterative-content-refiner](iterative-content-refiner): an iterative content refiner that improves drafts until a quality condition is met or a safe iteration limit is reached
- [guarded-agent-surface](guarded-agent-surface): a guarded agent surface that redacts PII, blocks unsafe requests, and records audit events for allowed and denied actions
- [role-aware-operations-assistant](role-aware-operations-assistant): a role-aware operations assistant that can search, summarize, and execute tool actions only when runtime permissions allow them
- [deployment-ready-agent-service](deployment-ready-agent-service): a deployment-ready agent service exposed through CLI, server, or A2A interfaces using the same underlying runtime model
- [telemetry-aware-production-agent](telemetry-aware-production-agent): a telemetry-aware production agent that emits logs, spans, and usage data for latency, cost, and failure analysis
- [realtime-voice-assistant](realtime-voice-assistant): a realtime voice or streaming assistant that handles incremental responses instead of waiting for one final blocking reply
- [realtime-voice-assistant-gemini](realtime-voice-assistant-gemini): a Gemini Live variant that shows the same event-driven realtime model with Google-native voice and transcript streaming

## Run Agents Online

Use the public playground to run the registered examples without cloning this repo:

1. Open [playground.adk-rust.com](https://playground.adk-rust.com/).
2. Pick an example from the sidebar or open a deep link directly.
3. Click `Run` to execute the example in the hosted playground.
4. Inspect streamed output, traces, timing, token usage, and cost estimates in the UI.

Deep-link format: `https://playground.adk-rust.com/#<example_id>`

## Playground Agent Catalog

The public playground currently registers **78** deep-linkable examples. Each link below opens the example directly in the online playground.

### Getting Started

- [Quickstart](https://playground.adk-rust.com/#quickstart): Basic LLM agent with Gemini — the simplest ADK program ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/quickstart.rs))
- [Instruction Templates](https://playground.adk-rust.com/#template): Dynamic instructions with session state placeholders ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/template.rs))
- [Structured Output](https://playground.adk-rust.com/#structured_output): Force JSON responses matching a schema ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/structured_output.rs))

### Function Tools

- [Basic Function Tools](https://playground.adk-rust.com/#function_tool): Agent with typed function tools and schema validation ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/function_tool.rs))
- [Multiple Tools](https://playground.adk-rust.com/#multi_tools): Agent with weather, calculator, and unit converter tools ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/multi_tools.rs))
- [Multi-Turn Conversation](https://playground.adk-rust.com/#multi_turn): Shopping assistant with cart — tool context preserved across 3 turns ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/multi_turn.rs))

### Agents

- [Agent-as-Tool](https://playground.adk-rust.com/#agent_tool): Wrap specialist agents as callable tools for a coordinator ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/agent_tool.rs))
- [Customer Service](https://playground.adk-rust.com/#customer_service): Billing issue → agent escalation → manager approval — full resolution flow ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/customer_service.rs))
- [LLM Conditional Router](https://playground.adk-rust.com/#conditional_router): LLM classifies queries and routes to specialist agents ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/conditional_router.rs))

### Callbacks

- [Logging Callbacks](https://playground.adk-rust.com/#callbacks_logging): Before/after callbacks for logging agent interactions ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/callbacks_logging.rs))
- [Input Guardrails](https://playground.adk-rust.com/#callbacks_guardrails): Block inappropriate content with before_callback guardrails ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/callbacks_guardrails.rs))

### Workflows

- [Sequential Pipeline](https://playground.adk-rust.com/#sequential): Chain agents in a multi-step pipeline (research → write → edit) ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/sequential.rs))
- [Parallel Analysis](https://playground.adk-rust.com/#parallel): Run multiple agents concurrently and merge results ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/parallel.rs))
- [Iterative Loop](https://playground.adk-rust.com/#loop_workflow): Refine content in a loop until quality threshold is met ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/loop_workflow.rs))

### Graph

- [Graph Pipeline](https://playground.adk-rust.com/#graph_workflow): Analyst → Writer → Editor agents in a sequential graph with deterministic data prep nodes ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/graph_workflow.rs))
- [Conditional Routing](https://playground.adk-rust.com/#graph_conditional): LLM classifier routes support tickets to specialist agents via conditional edges ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/graph_conditional.rs))
- [ReAct Pattern](https://playground.adk-rust.com/#react_pattern): Iterative reasoning with tools in a graph cycle ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/react_pattern.rs))
- [Supervisor Routing](https://playground.adk-rust.com/#supervisor_routing): Supervisor delegates tasks to specialist agent nodes ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/supervisor_routing.rs))

### Sessions

- [Session & State](https://playground.adk-rust.com/#session_state): Manage conversation sessions with Runner and state ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/session_state.rs))
- [PostgreSQL Sessions](https://playground.adk-rust.com/#postgres_sessions): ACID-compliant session persistence with PostgreSQL — three-tier state, JSONB, advisory-lock migrations ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/postgres_sessions.rs))
- [MongoDB Sessions](https://playground.adk-rust.com/#mongodb_sessions): Schema-flexible document sessions with MongoDB — nested state, arrays, TTL indexes ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/mongodb_sessions.rs))
- [Neo4j Sessions](https://playground.adk-rust.com/#neo4j_sessions): Graph-powered session relationships with Neo4j — nodes, constraints, Cypher queries ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/neo4j_sessions.rs))

### Providers

- [OpenAI](https://playground.adk-rust.com/#openai_quickstart): Responses API — o4-mini reasoning with configurable effort + tool use ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/openai_quickstart.rs))
- [Anthropic](https://playground.adk-rust.com/#anthropic_quickstart): Claude Sonnet 4.5 with extended thinking (10K budget) + code review ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_quickstart.rs))
- [DeepSeek](https://playground.adk-rust.com/#deepseek_quickstart): DeepSeek Reasoner with chain-of-thought for math & logic ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/deepseek_quickstart.rs))
- [Mistral](https://playground.adk-rust.com/#mistral_quickstart): Mistral Medium — multilingual translation + sentiment tools ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/mistral_quickstart.rs))
- [xAI (Grok)](https://playground.adk-rust.com/#xai_quickstart): Grok-3-mini-fast debugging assistant with tool use ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/xai_quickstart.rs))
- [Azure AI](https://playground.adk-rust.com/#azure_ai_quickstart): Azure AI Inference endpoint — text classification + summarization with Llama/Mistral ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/azure_ai_quickstart.rs))
- [AWS Bedrock](https://playground.adk-rust.com/#bedrock_quickstart): Amazon Bedrock with Claude — cloud architecture design + threat modeling via IAM auth ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/bedrock_quickstart.rs))
- [OpenRouter](https://playground.adk-rust.com/#openrouter_quickstart): Multi-provider AI gateway — 200+ models with tool use, provider routing, and automatic fallback ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/openrouter_quickstart.rs))

### Audio

- [Poem → Speech](https://playground.adk-rust.com/#poem_tts): LLM writes a random poem, Gemini TTS synthesizes it to audio ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/poem_tts.rs))
- [Realtime Voice](https://playground.adk-rust.com/#realtime_audio): OpenAI Realtime API — text prompt to expressive voice audio via WebSocket ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/realtime_audio.rs))
- [Realtime Session Update](https://playground.adk-rust.com/#realtime_session_update): Mid-session persona switch — general assistant → travel agent with swapped tools, no reconnect ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/realtime_session_update.rs))
- [Realtime Tools](https://playground.adk-rust.com/#realtime_tools): Function calling in voice — weather, calculator, and time tools over a single WebSocket ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/realtime_tools.rs))
- [Gemini Live Tools](https://playground.adk-rust.com/#gemini_live_tools): Gemini Live voice agent with weather + time tools — native audio, tool call → response loop ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/gemini_live_tools.rs))
- [Gemini Live Context Switch](https://playground.adk-rust.com/#gemini_live_context): Mid-session persona switch via session resumption — tech support → billing agent with swapped tools ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/gemini_live_context.rs))

### Extensions

- [Skill Discovery](https://playground.adk-rust.com/#skill_discovery): Discover, parse, score, and inject agentskills.io skill files into prompts ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/skill_discovery.rs))
- [Plugin System](https://playground.adk-rust.com/#plugin_system): Lifecycle hooks for agents — message, model, tool, and run callbacks ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/plugin_system.rs))

### Coding

- [Code Execution](https://playground.adk-rust.com/#code_execution): Typed sandbox with truthful capability model — policy validation and CodeTool ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/code_execution.rs))
- [CLI Launcher](https://playground.adk-rust.com/#cli_launcher): Deploy agents as interactive REPL or HTTP server with streaming ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/cli_launcher.rs))

### RAG

- [Multi-Collection RAG](https://playground.adk-rust.com/#rag_multi_collection): Domain-isolated knowledge bases with cross-collection search ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/rag_multi_collection.rs))
- [Custom Embedder](https://playground.adk-rust.com/#rag_custom_embedder): Implement EmbeddingProvider trait — TF-IDF example with cosine similarity ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/rag_custom_embedder.rs))

### Thinking

- [Reasoning Effort (OpenAI)](https://playground.adk-rust.com/#thinking_openai): Responses API with o4-mini — Low/Medium/High reasoning effort + detailed summaries ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/thinking_openai.rs))
- [Extended Thinking (Anthropic)](https://playground.adk-rust.com/#thinking_anthropic): Claude's internal reasoning with 10K token budget — deep systems design analysis ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/thinking_anthropic.rs))
- [Chain-of-Thought (DeepSeek)](https://playground.adk-rust.com/#thinking_deepseek): Visible chain-of-thought reasoning — watch the model think through math problems ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/thinking_deepseek.rs))
- [Grok Thinking (xAI)](https://playground.adk-rust.com/#thinking_xai): OpenAI-compatible reasoning — Grok-3-mini thinks through Fermi estimation with tools ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/thinking_xai.rs))
- [Thought Signatures (Gemini)](https://playground.adk-rust.com/#thinking_gemini): Native thinking traces + thought_signature on tool calls — multi-turn with preserved context ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/thinking_gemini.rs))

### Advanced

- [Artifact Storage](https://playground.adk-rust.com/#artifact_agent): Agent with versioned file storage — save, load, and list artifacts mid-conversation ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/artifact_agent.rs))
- [Long-Term Memory](https://playground.adk-rust.com/#memory_agent): Cross-session memory recall — agent remembers past conversations and preferences ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/memory_agent.rs))
- [Advanced Guardrails](https://playground.adk-rust.com/#guardrails_advanced): PII redaction, content filtering, and GuardrailSet with LLM agent integration ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/guardrails_advanced.rs))
- [RBAC Access Control](https://playground.adk-rust.com/#auth_rbac): Role-based tool permissions — analyst can search but not delete, admin gets full access ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/auth_rbac.rs))

### Security

- [Typed Identity](https://playground.adk-rust.com/#auth_identity): Injection-proof identity system — validated IDs, null-byte rejection, multi-tenant session isolation ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/auth_identity.rs))
- [Audit Trail](https://playground.adk-rust.com/#auth_audit): Tamper-evident access logging — RBAC permission matrix, AuditSink, AuthMiddleware tool protection ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/auth_audit.rs))
- [SSO & JWT](https://playground.adk-rust.com/#auth_sso): Enterprise identity — Google/Azure/Okta SSO, JWT validation, OIDC discovery, claims-to-RBAC mapping ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/auth_sso.rs))

### Built-in Tools

- [Google Search (Gemini)](https://playground.adk-rust.com/#builtin_gemini): GoogleSearchTool wrapper — server-side search, grounding metadata, and thought signatures across multi-turn tool use ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/builtin_gemini.rs))
- [Web Search (Anthropic)](https://playground.adk-rust.com/#builtin_anthropic): WebSearchTool wrapper for Claude — server-side search with local function tools across multiple turns ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/builtin_anthropic.rs))
- [Web Search (OpenAI)](https://playground.adk-rust.com/#builtin_openai): OpenAIWebSearchTool wrapper — hosted search with local function tools across multiple turns ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/builtin_openai.rs))

### Payments

- [Checkout Agent](https://playground.adk-rust.com/#payments_checkout): AI-driven checkout lifecycle — create session, select fulfillment, authorize payment, verify status with evidence trail ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/payments_checkout.rs))
- [Payment Guardrails](https://playground.adk-rust.com/#payments_guardrails): Amount thresholds, merchant allowlists, policy sets, card/PII redaction, evidence references ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/payments_guardrails.rs))
- [Shopping Agent](https://playground.adk-rust.com/#payments_agent): LLM agent with checkout tools — browse, cart, guardrail-enforced payment, masked transaction status ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/payments_agent.rs))

### Competitive

- [Auto-Provider + Encryption](https://playground.adk-rust.com/#competitive_auto_provider): provider_from_env() auto-detects API keys + EncryptedSession with AES-256-GCM at-rest encryption ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/competitive_auto_provider.rs))
- [Durable Graph Resume](https://playground.adk-rust.com/#competitive_graph_resume): MemoryCheckpointer saves graph state — resume from checkpoint after crash, skip completed nodes ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/competitive_graph_resume.rs))
- [Tool Search Filter](https://playground.adk-rust.com/#competitive_tool_search): ToolSearchConfig regex filtering — hide dangerous tools from the LLM while keeping them registered ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/competitive_tool_search.rs))

### v0.7+ Features

- [DeepSeek V4 Thinking](https://playground.adk-rust.com/#deepseek_v4_thinking): ThinkingMode, ReasoningEffort, strict tool mode — V4 Flash/Pro with visible chain-of-thought ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/deepseek_v4_thinking.rs))
- [Project-Scoped Memory](https://playground.adk-rust.com/#project_scoped_memory): project_id dimension isolates memories per project — same user, different projects, zero leakage ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/project_scoped_memory.rs))
- [Bounded Execution](https://playground.adk-rust.com/#bounded_execution): RunConfig with history_max_events and max_tool_concurrency — production safety bounds for agents ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/bounded_execution.rs))
- [AWP Agent Discovery](https://playground.adk-rust.com/#awp_discovery): Agentic Web Protocol — capability manifests, trust levels, rate limits, and agent-to-agent discovery ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/awp_discovery.rs))
- [Minimal Tier Agent](https://playground.adk-rust.com/#minimal_agent): Lightest possible agent using v0.8 minimal feature tier — 32% smaller builds, ~50s compile ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/minimal_agent.rs))

### Anthropic

- [Prompt Caching](https://playground.adk-rust.com/#anthropic_caching): Multi-turn agent with prompt caching — cache creation (25% surcharge) then cache hit (90% discount) ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_caching.rs))
- [Vision Agent](https://playground.adk-rust.com/#anthropic_vision): Image analysis agent — Claude sees images via URL and logs structured observations with tools ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_vision.rs))
- [Structured Extraction](https://playground.adk-rust.com/#anthropic_structured): Typed JSON extraction from unstructured text — tool schema forces structured output ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_structured.rs))
- [Streaming + Tools](https://playground.adk-rust.com/#anthropic_streaming): Real-time streaming with mid-stream tool calls — time-to-first-token metrics ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_streaming.rs))
- [Token Counting & Models](https://playground.adk-rust.com/#anthropic_token_counting): Model discovery, pre-flight token counting, cost estimation, and rate limit info ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_token_counting.rs))
- [Multi-Tool Agent](https://playground.adk-rust.com/#anthropic_multi_tool): Travel assistant with weather, calculator, and unit converter — parallel tool orchestration ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_multi_tool.rs))
- [Thinking Graph](https://playground.adk-rust.com/#anthropic_thinking_graph): Extended thinking (10K budget) in a StateGraph — deep thinker → concise summarizer pipeline ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/anthropic_thinking_graph.rs))

### Action Nodes

- [Data Enrichment](https://playground.adk-rust.com/#action_set_transform): SET + TRANSFORM action nodes prep data, then an LLM agent writes personalized outreach ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/action_set_transform.rs))
- [Smart Ticket Router](https://playground.adk-rust.com/#action_switch_loop): LLM classifier + deterministic SWITCH routing + specialist agents handle support tickets ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/action_switch_loop.rs))
- [Content Pipeline](https://playground.adk-rust.com/#action_workflow): SET → Research Agent → TRANSFORM → Writer Agent → SWITCH → Editor Agent — full content pipeline ([source](https://github.com/zavora-ai/adk-playground/blob/main/playground/backend/examples/action_workflow.rs))

## Quick Start

```bash
cargo check --workspace
cargo run -p chapter3-quickstart
```

Examples that call live providers require credentials such as `GOOGLE_API_KEY` or `OPENAI_API_KEY`. The per-example READMEs call out those requirements.

## Validation

Offline validation:

```bash
./scripts/check-examples.sh
```

Opt-in live smoke validation:

```bash
BOOK_RUN_LIVE_SMOKE=1 ./scripts/smoke-examples.sh
```
