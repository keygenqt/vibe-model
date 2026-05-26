# Vibe Model

OpenAI-compatible HTTP API that routes prompts across a cluster of LLMs.

## Why

Running a single model hits limits fast. One GPU, one model, one prompt at a time.
When you have multiple machines with different models, you're stuck managing connections, load balancing, and retries yourself.
Cloud APIs solve scaling but lock you into one provider. Local models give control but lack distribution.
One API, multiple providers, zero coordination overhead.

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
