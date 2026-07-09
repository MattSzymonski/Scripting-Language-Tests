# 🧠 Graphify — Complete Guide

> **Repository:** [Graphify-Labs/graphify](https://github.com/Graphify-Labs/graphify)  
> **Stars:** 80.8k | **Forks:** 8k | **License:** MIT  
> **PyPI:** `graphifyy` | **CLI:** `graphify` | **Version installed:** 0.9.11

---

## What Is Graphify?

Graphify maps your entire project (code, docs, PDFs, images, videos) into a **knowledge graph** — a web of interconnected concepts you can query, traverse, and explore. Instead of `grep`-ing through files, you ask questions and trace connections.

### Key Features

| Feature                  | Description                                                                   |
| ------------------------ | ----------------------------------------------------------------------------- |
| **God nodes**            | The most-connected concepts — see what everything flows through               |
| **Community detection**  | Graph split into subsystems via Leiden algorithm, with LLM-free labels        |
| **Cross-file links**     | Calls, imports, inheritance resolved across ~40 languages via tree-sitter AST |
| **Query, path, explain** | Ask a question, trace the path between two things, or explain one concept     |
| **Rationale + doc refs** | `# NOTE:` / `# WHY:` comments and ADR/RFC citations become first-class nodes  |
| **Beyond code**          | Docs, PDFs, images, and video/audio all map into the same graph               |
| **Local-first**          | Code is parsed locally with tree-sitter (no LLM, nothing leaves your machine) |

### What It Outputs

Three files in `graphify-out/`:

| File              | Purpose                                                                             |
| ----------------- | ----------------------------------------------------------------------------------- |
| `graph.html`      | Interactive force-directed graph — open in any browser, click nodes, filter, search |
| `GRAPH_REPORT.md` | Highlights: god nodes, surprising connections, suggested questions                  |
| `graph.json`      | The full graph — query it anytime without re-reading files                          |

---

## How It Works Under the Hood

```mermaid
flowchart TD
    A[Source Files] --> B{File Type?}
    B -->|Code| C[tree-sitter AST Parser]
    B -->|Docs/PDFs/Images/Video| D[LLM Semantic Extraction]
    C --> E[Extract nodes + edges]
    D --> E
    E --> F[Build Knowledge Graph]
    F --> G[Leiden Community Detection]
    G --> H[Generate Outputs]
    H --> I[graph.html]
    H --> J[GRAPH_REPORT.md]
    H --> K[graph.json]
```

### Extraction Pipeline

1. **Code** → parsed locally with **tree-sitter AST** (deterministic, no LLM, nothing leaves your machine). Extracts functions, classes, variables, and their relationships across **~40 languages** (Python, Rust, C#, JS, Go, Java, C/C++, Ruby, Swift, Kotlin, Scala, PHP, Lua, Zig, Elixir, Julia, Dart, Groovy, Fortran, Verilog, Pascal, SQL, Bash, PowerShell, and more).

2. **Docs/PDFs/Images/Video** → sent to an LLM for semantic extraction (uses your IDE's model or a configured API key).

3. **Graph construction** → nodes (concepts) + edges (relationships), each tagged:
   - `EXTRACTED` — explicit in source code (e.g., `import X`, `class Y extends Z`)
   - `INFERRED` — resolved by graphify's analysis (e.g., cross-file call resolution)
   - `AMBIGUOUS` — uncertain, needs human judgment

4. **Community detection** → Leiden algorithm groups related concepts into subsystems, with auto-generated labels.

5. **Query layer** → `query`, `path`, `explain` commands let you interrogate the graph without re-reading files.

### Confidence Tags

Every edge carries a confidence tag so you always know what was found vs guessed:

- **`EXTRACTED`** — explicit in the source
- **`INFERRED`** — derived by resolution
- **`AMBIGUOUS`** — uncertain

---

## Supported File Types

### Code (36+ tree-sitter grammars)

`.py` `.ts` `.mts` `.cts` `.js` `.jsx` `.tsx` `.mjs` `.go` `.rs` `.java` `.c` `.cpp` `.cc` `.cxx` `.h` `.hpp` `.cu` `.cuh` `.metal` `.rb` `.cs` `.kt` `.kts` `.scala` `.php` `.swift` `.lua` `.luau` `.toc` `.zig` `.ps1` `.psm1` `.psd1` `.ex` `.exs` `.m` `.mm` `.jl` `.vue` `.svelte` `.astro` `.groovy` `.gradle` `.dart` `.v` `.sv` `.svh` `.sql` `.f` `.f90` `.f95` `.f03` `.f08` `.pas` `.pp` `.dpr` `.dpk` `.lpr` `.inc` `.dfm` `.lfm` `.lpk` `.sh` `.bash` `.json` `.dm` `.dme` `.dmi` `.dmm` `.dmf` `.sln` `.slnx` `.csproj` `.fsproj` `.vbproj` `.xaml` `.razor` `.cshtml`

### Other Formats

| Category              | Formats                                                                   |
| --------------------- | ------------------------------------------------------------------------- |
| **Docs**              | `.md` `.mdx` `.qmd` `.html` `.txt` `.rst` `.yaml` `.yml`                  |
| **Office**            | `.docx` `.xlsx` (requires `uv tool install graphifyy[office]`)            |
| **PDFs**              | `.pdf`                                                                    |
| **Images**            | `.png` `.jpg` `.webp` `.gif`                                              |
| **Video/Audio**       | `.mp4` `.mov` `.mp3` `.wav` (requires `uv tool install graphifyy[video]`) |
| **YouTube/URLs**      | Any video URL (requires `uv tool install graphifyy[video]`)               |
| **Salesforce Apex**   | `.cls` `.trigger`                                                         |
| **Terraform/HCL**     | `.tf` `.tfvars` `.hcl` (requires `uv tool install graphifyy[terraform]`)  |
| **Package Manifests** | `pyproject.toml` `go.mod` `pom.xml` `apm.yml`                             |
| **MCP Configs**       | `.mcp.json` `mcp.json` `mcp_servers.json` `claude_desktop_config.json`    |
| **Google Workspace**  | `.gdoc` `.gsheet` `.gslides` (opt-in; requires gws auth)                  |

---

## Installation

### Prerequisites

| Requirement      | Version | Check              |
| ---------------- | ------- | ------------------ |
| Python           | 3.10+   | `python --version` |
| uv (recommended) | any     | `uv --version`     |

### Step 1 — Install uv (if not already)

**Windows (PowerShell):**
```powershell
powershell -ExecutionPolicy ByPass -Command "irm https://astral.sh/uv/install.ps1 | iex"
```

**macOS (Homebrew):**
```bash
brew install python@3.12 uv
```

**Ubuntu/Debian:**
```bash
sudo apt install python3.12 python3-pip pipx
```

### Step 2 — Install graphify

```bash
# Recommended (isolated environment)
uv tool install graphifyy

# Update shell PATH if command not found
uv tool update-shell

# Alternative: pipx
pipx install graphifyy

# Alternative: pip (not recommended on Mac/Windows)
pip install graphifyy
```

> **⚠️ Important:** The PyPI package is `graphifyy` (double-y). Other `graphify*` packages on PyPI are not affiliated. The CLI command is still `graphify`.

### Step 3 — Register the skill with your AI assistant

```bash
graphify install
```

This installs the skill file to `~/.claude/skills/graphify/SKILL.md`.

For project-scoped installs (committable to git):
```bash
graphify install --project
```

### Platform-specific setup

Make your assistant always use the graph:

```bash
graphify claude install      # Claude Code (CLAUDE.md + PreToolUse hook)
graphify cursor install      # Cursor (.cursor/rules/graphify.mdc)
graphify copilot install     # GitHub Copilot CLI
graphify vscode install      # VS Code Copilot Chat
graphify codex install       # Codex (AGENTS.md + hook)
graphify gemini install      # Gemini CLI (GEMINI.md + BeforeTool hook)
graphify codebuddy install   # CodeBuddy (CODEBUDDY.md + hook)
graphify opencode install    # OpenCode (AGENTS.md + plugin)
graphify kilo install        # Kilo Code (native skill + command + plugin)
graphify aider install       # Aider (AGENTS.md)
graphify agents install      # Cross-framework Agent Skills
graphify trae install        # Trae
graphify kiro install        # Kiro IDE/CLI
graphify pi install          # Pi coding agent
graphify devin install       # Devin CLI
graphify antigravity install # Google Antigravity
```

Remove from all platforms:
```bash
graphify uninstall           # Remove from all platforms
graphify uninstall --purge   # Also delete graphify-out/
```

---

## 101 Guide — Getting Started

### Step 1: Build Your First Graph

Navigate to any project folder and run:

```bash
graphify .
```

> **⚠️ PowerShell Note:** Use `graphify .` — NOT `/graphify .` — the leading slash is a path separator in PowerShell.

Or inside your AI coding assistant, just type:
```
/graphify .
```

### Step 2: Explore the Output

Open the generated files:

- **`graphify-out/graph.html`** — Open in any browser. Click nodes, filter, search. A force-directed interactive graph with community colors.
- **`graphify-out/GRAPH_REPORT.md`** — Read the highlights: god nodes, surprising connections, suggested questions.
- **`graphify-out/graph.json`** — The full graph data for programmatic access.

### Step 3: Query the Graph

Once built, query instead of reading files:

```bash
# Ask a natural-language question
graphify query "what connects auth to the database?"

# Trace the shortest path between two concepts
graphify path "UserService" "DatabasePool"

# Get a detailed breakdown of one concept and all its connections
graphify explain "RateLimiter"

# Use a specific graph file
graphify query "show the auth flow" --graph graphify-out/graph.json

# Deeper search with budget control
graphify query "..." --dfs --budget 1500
```

### Step 4: Use Inside Your AI Assistant

Once the graph is built and the skill is installed, your AI assistant will automatically consult it for codebase questions. The assistant prefers scoped queries like `graphify query "<question>"` over reading the full report or grepping raw files.

`GRAPH_REPORT.md` is still available for broad architecture review.

---

## Common Commands Cheat Sheet

### Build & Update

| Command                                       | Description                                 |
| --------------------------------------------- | ------------------------------------------- |
| `graphify .`                                  | Build graph for current directory           |
| `graphify ./src`                              | Build graph for a specific folder           |
| `graphify . --update`                         | Re-extract only changed files               |
| `graphify . --mode deep`                      | More aggressive relationship extraction     |
| `graphify . --no-viz`                         | Skip HTML visualization, just report + JSON |
| `graphify . --cluster-only`                   | Rerun clustering without re-extracting      |
| `graphify . --cluster-only --resolution 1.5`  | More granular communities                   |
| `graphify . --cluster-only --exclude-hubs 99` | Suppress utility super-hubs                 |
| `graphify . --force`                          | Overwrite even if new graph has fewer nodes |
| `graphify . --directed`                       | Preserve edge direction                     |
| `graphify . --watch`                          | Auto-rebuild as files change                |

### Query

| Command                                    | Description                                   |
| ------------------------------------------ | --------------------------------------------- |
| `graphify query "..."`                     | Ask a natural-language question               |
| `graphify query "..." --dfs --budget 1500` | Deep search with node budget                  |
| `graphify path "A" "B"`                    | Find shortest path between two concepts       |
| `graphify explain "X"`                     | Deep-dive on one node and all its connections |

### Export & Integrate

| Command                                         | Description                         |
| ----------------------------------------------- | ----------------------------------- |
| `graphify . --obsidian`                         | Generate Obsidian vault             |
| `graphify . --wiki`                             | Build agent-crawlable markdown wiki |
| `graphify . --svg`                              | Export graph as SVG                 |
| `graphify . --graphml`                          | Export for Gephi / yEd              |
| `graphify . --neo4j`                            | Generate cypher.txt for Neo4j       |
| `graphify . --neo4j-push bolt://localhost:7687` | Push directly to Neo4j              |
| `graphify . --falkordb`                         | Generate cypher.txt for FalkorDB    |
| `graphify export callflow-html`                 | Mermaid architecture/call-flow HTML |

### Add External Content

| Command                                         | Description                |
| ----------------------------------------------- | -------------------------- |
| `graphify add https://arxiv.org/abs/1706.03762` | Fetch a paper and add it   |
| `graphify add <youtube-url>`                    | Transcribe and add a video |
| `graphify add https://... --author "Name"`      | Add with attribution       |

### Git Integration

| Command                                       | Description                |
| --------------------------------------------- | -------------------------- |
| `graphify hook install`                       | Auto-rebuild on git commit |
| `graphify hook uninstall`                     | Remove git hooks           |
| `graphify hook status`                        | Check hook status          |
| `graphify clone https://github.com/user/repo` | Clone and build graph      |

### PR Management

| Command                    | Description                                      |
| -------------------------- | ------------------------------------------------ |
| `graphify prs`             | PR dashboard: CI, review, worktree, graph impact |
| `graphify prs 42`          | Deep dive on PR #42                              |
| `graphify prs --triage`    | AI ranks your review queue                       |
| `graphify prs --conflicts` | PRs sharing graph communities (merge-order risk) |
| `graphify prs --base main` | Filter to PRs targeting a specific base branch   |

### Graph Merging & Global Graph

| Command                                                   | Description                              |
| --------------------------------------------------------- | ---------------------------------------- |
| `graphify merge-graphs a.json b.json`                     | Combine two graphs                       |
| `graphify global add graphify-out/graph.json --as myrepo` | Register into cross-project global graph |
| `graphify global list`                                    | Show all registered repos                |
| `graphify global remove myrepo`                           | Remove a project from global graph       |

### MCP Server

```bash
# Expose as MCP server (stdio)
python -m graphify.serve graphify-out/graph.json

# Expose over HTTP (team-wide)
python -m graphify.serve graphify-out/graph.json --transport http --port 8080

# With API key protection
python -m graphify.serve graphify-out/graph.json --transport http --host 0.0.0.0 --api-key "$SECRET"

# Docker
docker build -t graphify .
docker run -p 8080:8080 -v "$(pwd)/graphify-out:/data" graphify \
  /data/graph.json --transport http --host 0.0.0.0 --api-key "$SECRET"
```

### Headless CI Extraction

```bash
# LLM extraction for docs/PDFs (code is always local)
graphify extract ./docs --backend gemini
graphify extract ./docs --backend claude
graphify extract ./docs --backend openai
graphify extract ./docs --backend ollama    # Fully local
graphify extract ./docs --backend bedrock   # AWS Bedrock via IAM
graphify extract ./docs --backend azure     # Azure OpenAI
graphify extract ./docs --backend claude-cli # Via Claude Code CLI subscription

# PostgreSQL schema introspection
graphify extract --postgres "postgresql://user:pass@host/db"

# Cargo workspace dependencies
graphify extract ./my-workspace --cargo

# Deep extraction with timing
graphify extract ./docs --mode deep --timing

# Parallelism control
graphify extract ./docs --max-workers 16
graphify extract ./docs --max-concurrency 2
```

---

## Ignoring Files

Create a `.graphifyignore` in your project root — same syntax as `.gitignore`, including `!` negation.

`.gitignore` is respected automatically. If a `.graphifyignore` is also present, the two are merged — `.graphifyignore` patterns win on conflicts.

```gitignore
# .graphifyignore
node_modules/
dist/
*.generated.py

# Only index src/, ignore everything else
*
!src/
!src/**
```

---

## Team Setup

`graphify-out/` is meant to be committed to git so everyone on the team starts with a map.

**Recommended `.gitignore` additions:**

```gitignore
graphify-out/cost.json        # local only
# graphify-out/cache/         # optional: commit for speed, skip to keep repo small
```

**Workflow:**

1. One person runs `graphify .` and commits `graphify-out/`.
2. Everyone pulls — their assistant reads the graph immediately.
3. Run `graphify hook install` to auto-rebuild after each commit (AST only, no API cost). This also sets up a git merge driver so `graph.json` is never left with conflict markers — two devs committing in parallel get their graphs union-merged automatically.
4. When docs or papers change, run `graphify . --update` to refresh those nodes.

---

## Privacy

| Data Type              | Processing                | Network                               |
| ---------------------- | ------------------------- | ------------------------------------- |
| **Code**               | tree-sitter AST (local)   | ❌ Nothing leaves your machine         |
| **Video/Audio**        | faster-whisper (local)    | ❌ Nothing leaves your machine         |
| **Docs, PDFs, Images** | Your AI assistant's model | ✅ API call (only if you configure it) |

- **No telemetry**, no usage tracking, no analytics.
- Query logging is local-only (`~/.cache/graphify-queries.log`). Disable with `GRAPHIFY_QUERY_LOG_DISABLE=1`.
- For fully-local setup: `--backend ollama` keeps everything on-device.

---

## Environment Variables

| Variable                            | Purpose                             | Required For         |
| ----------------------------------- | ----------------------------------- | -------------------- |
| `ANTHROPIC_API_KEY`                 | Claude (Anthropic) backend          | `--backend claude`   |
| `OPENAI_API_KEY`                    | OpenAI or compatible APIs           | `--backend openai`   |
| `GEMINI_API_KEY` / `GOOGLE_API_KEY` | Google Gemini backend               | `--backend gemini`   |
| `DEEPSEEK_API_KEY`                  | DeepSeek backend                    | `--backend deepseek` |
| `MOONSHOT_API_KEY`                  | Kimi Code backend                   | `--backend kimi`     |
| `OLLAMA_BASE_URL`                   | Ollama local inference URL          | `--backend ollama`   |
| `OLLAMA_MODEL`                      | Ollama model name                   | `--backend ollama`   |
| `AZURE_OPENAI_API_KEY`              | Azure OpenAI Service                | `--backend azure`    |
| `AZURE_OPENAI_ENDPOINT`             | Azure resource endpoint URL         | `--backend azure`    |
| `GRAPHIFY_MAX_WORKERS`              | AST parallelism thread count        | Large codebases      |
| `GRAPHIFY_MAX_OUTPUT_TOKENS`        | Raise output cap for dense corpora  | Large files          |
| `GRAPHIFY_FORCE`                    | Force graph rebuild                 | After refactors      |
| `GRAPHIFY_GOOGLE_WORKSPACE`         | Auto-enable Google Workspace export | Google docs          |
| `GRAPHIFY_QUERY_LOG_DISABLE`        | Disable query logging entirely      | Privacy              |

---

## Troubleshooting

### `graphify: command not found`

The CLI's bin directory isn't on your `PATH`:

- **uv:** Run `uv tool update-shell`, then open a new terminal.
- **pipx:** Run `pipx ensurepath`, then open a new terminal.
- **pip:** Add `~/.local/bin` (Linux) or `~/Library/Python/3.x/bin` (Mac) to your `PATH`.

### `uvx graphify …` fails to resolve

The PyPI package is `graphifyy`, not `graphify`. Use:
```bash
uvx --from graphifyy graphify install
```

### `/graphify .` causes "path not recognized" in PowerShell

PowerShell treats `/` as a path separator. Use `graphify .` (no leading slash) on Windows.

### Graph has fewer nodes after `--update` or rebuild

Old nodes from deleted files linger. Pass `--force` to overwrite:
```bash
graphify extract . --force
```

### Extraction returns empty for docs/PDFs

Docs, PDFs, and images require an LLM call. Check your API key:
```bash
ANTHROPIC_API_KEY=sk-... graphify extract ./docs --backend claude
```

### Graph HTML too large (>5000 nodes)

Skip HTML and use JSON directly:
```bash
graphify cluster-only ./my-project --no-viz
graphify query "..."
```

### Claude Code prompt cache invalidated

Add to `.claudeignore`:
```gitignore
graph.json
graphify-out/
```

---

## Benchmarks

| Benchmark                        | Graphify Score | Comparison                      |
| -------------------------------- | -------------- | ------------------------------- |
| LOCOMO (n=300) recall@10         | **0.497**      | mem0: 0.048, supermemory: 0.149 |
| LOCOMO (n=300) QA accuracy       | 45.3%          | supermemory: 49.7%, mem0: 27.3% |
| LongMemEval-S (n=50) QA accuracy | **76%**        | Tied with dense RAG             |
| Graph build LLM credits          | **0**          | Per-token for most systems      |

Benchmarked on the same harness with the same model and budgets, scored by a judge blind-validated against a second judge (90.6% agreement, Cohen's kappa 0.81).

---

## Learn More

- [How It Works](https://github.com/Graphify-Labs/graphify/blob/v8/docs/how-it-works.md) — extraction pipeline, community detection, confidence scoring
- [ARCHITECTURE.md](https://github.com/Graphify-Labs/graphify/blob/v8/ARCHITECTURE.md) — module breakdown, how to add a language
- [BENCHMARKS.md](https://github.com/Graphify-Labs/graphify/blob/v8/BENCHMARKS.md) — full per-system tables and reproduction commands
- [Docker MCP + SQLite](https://github.com/Graphify-Labs/graphify/blob/v8/docs/docker-mcp-sqlite.md) — optional integrations
- [The Memory Layer](https://safishamsi.gumroad.com/l/qetvlo) — the book on the ideas behind graphify

### Community

- [Discord](https://discord.gg/598Ad9zQZ)
- [X (Twitter)](https://x.com/graphifyy)
- [GitHub Sponsors](https://github.com/sponsors/safishamsi)
- [Website](https://graphifylabs.ai/)
