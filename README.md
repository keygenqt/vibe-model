# Vibe Model

OpenAI-compatible HTTP API that routes prompts across a cluster of LLMs.

## Features

- Routes prompts to the best available model in the cluster

## CLI

```bash
vibe-model serve start                      # Start server
vibe-model serve stop                       # Stop server
vibe-model serve status                     # Show server status
vibe-model ping prompt "your test prompt"   # Test cluster
vibe-model ping status                      # Show cluster models
```
