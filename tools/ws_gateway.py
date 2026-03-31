#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""
WebSocket Gateway — thin transport bridge between browser and primal IPC.

This gateway adds NO business logic. It resolves primal sockets via biomeOS
capability.discover and passes JSON-RPC calls through. Binding construction,
narration prompting, state mapping, and game-science parameterization are
composition-level concerns handled by the client or by primals themselves.

Architecture:
  Browser <-> WebSocket <-> Gateway <-> biomeOS capability.discover
                                    <-> primal UDS (JSON-RPC pass-through)
                                    <-> Ollama HTTP (raw pass-through)

Usage:
    python3 tools/ws_gateway.py [--port 8765]
"""

import argparse
import asyncio
import glob
import json
import os
import sys
import time
import traceback
from pathlib import Path
from typing import Optional

import uvicorn
from fastapi import FastAPI, WebSocket, WebSocketDisconnect
from fastapi.responses import FileResponse, HTMLResponse

SCRIPT_DIR = Path(__file__).parent
PROJECT_ROOT = SCRIPT_DIR.parent
WEB_DIR = PROJECT_ROOT / "web"

SOCKET_DIR = f"/run/user/{os.getuid()}/biomeos"
OLLAMA_ENDPOINT = os.environ.get("OLLAMA_ENDPOINT", "http://localhost:11434")
OLLAMA_MODEL = os.environ.get("OLLAMA_MODEL", "llama3.2:3b")


# ── Transport primitives ─────────────────────────────────────────────

def find_socket(primal_name: str) -> Optional[str]:
    """Resolve a primal socket by name — env override, then glob fallback."""
    env_key = f"{primal_name.upper()}_SOCKET"
    if env_key in os.environ:
        return os.environ[env_key]
    matches = glob.glob(f"{SOCKET_DIR}/{primal_name}-*.sock")
    return matches[0] if matches else None


async def call_uds(socket_path: str, method: str, params=None, req_id=1, timeout=10.0) -> dict:
    """Send a single JSON-RPC 2.0 call over a Unix domain socket."""
    msg = json.dumps({"jsonrpc": "2.0", "method": method, "id": req_id,
                       **({"params": params} if params else {})})
    try:
        reader, writer = await asyncio.wait_for(
            asyncio.open_unix_connection(socket_path), timeout=3.0)
        writer.write((msg + "\n").encode())
        await writer.drain()
        data = await asyncio.wait_for(reader.readline(), timeout=timeout)
        writer.close()
        await writer.wait_closed()
        return json.loads(data.decode().strip()) if data else {
            "error": {"code": -1, "message": "Empty response"}}
    except asyncio.TimeoutError:
        return {"error": {"code": -2, "message": f"Timeout calling {method}"}}
    except ConnectionRefusedError:
        return {"error": {"code": -3, "message": f"Connection refused: {socket_path}"}}
    except Exception as e:
        return {"error": {"code": -4, "message": str(e)}}


async def call_ollama(prompt: str, system_prompt: str = "",
                      model: str = "", max_tokens: int = 150) -> Optional[str]:
    """Raw pass-through to Ollama — caller provides the full prompt."""
    import urllib.request
    url = f"{OLLAMA_ENDPOINT}/api/chat"
    messages = []
    if system_prompt:
        messages.append({"role": "system", "content": system_prompt})
    messages.append({"role": "user", "content": prompt})
    body = json.dumps({
        "model": model or OLLAMA_MODEL, "messages": messages, "stream": False,
        "options": {"num_predict": max_tokens, "temperature": 0.8},
    }).encode()

    def _do():
        req = urllib.request.Request(url, data=body,
                                     headers={"Content-Type": "application/json"})
        try:
            with urllib.request.urlopen(req, timeout=30) as resp:
                return json.loads(resp.read().decode()).get("message", {}).get("content", "")
        except Exception:
            return None
    return await asyncio.get_event_loop().run_in_executor(None, _do)


# ── Capability resolution via biomeOS ─────────────────────────────

_socket_cache: dict[str, str] = {}
_biomeos_sock: Optional[str] = None


def _get_biomeos_sock() -> Optional[str]:
    global _biomeos_sock
    if _biomeos_sock:
        return _biomeos_sock
    _biomeos_sock = find_socket("biomeos")
    return _biomeos_sock


async def resolve_primal(target: str) -> Optional[str]:
    """Resolve a primal's socket via biomeOS capability.discover, with local fallback."""
    if target in _socket_cache:
        return _socket_cache[target]

    bio_sock = _get_biomeos_sock()
    if bio_sock:
        resp = await call_uds(bio_sock, "capability.discover",
                              {"capability": target}, timeout=3.0)
        result = resp.get("result", {})
        endpoint = result.get("primary_endpoint") or result.get("primary_socket")
        if endpoint:
            endpoint = endpoint.removeprefix("unix://")
            _socket_cache[target] = endpoint
            return endpoint

    sock = find_socket(target)
    if sock:
        _socket_cache[target] = sock
        return sock
    return None


# ── Composition health probe ─────────────────────────────────────

COMPOSITION_PRIMALS = [
    ("biomeos", "graph.list"),
    ("beardog", "health.liveness"),
    ("songbird", "health.liveness"),
    ("esotericwebb", "webb.liveness"),
    ("ludospring", "health.check"),
    ("squirrel", "health.liveness"),
    ("petaltongue", "health.liveness"),
    ("nestgate", "health.liveness"),
]


async def probe_composition_health() -> dict:
    """Probe all known composition primals and return health map."""
    results = {}
    for name, method in COMPOSITION_PRIMALS:
        sock = await resolve_primal(name)
        if not sock:
            results[name] = {"status": "not_found", "socket": None}
            continue
        resp = await call_uds(sock, method, timeout=3.0)
        alive = "result" in resp
        results[name] = {"status": "alive" if alive else "error",
                         "socket": sock, "response": resp}
    return results


# ── FastAPI app ───────────────────────────────────────────────────

app = FastAPI(title="ecoPrimals Gateway (thin bridge)")


@app.get("/")
async def index():
    play_html = WEB_DIR / "play.html"
    if play_html.exists():
        return FileResponse(play_html, media_type="text/html")
    return HTMLResponse("<h1>ecoPrimals Gateway</h1><p>web/play.html not found</p>")


@app.get("/health")
async def health():
    return await probe_composition_health()


@app.websocket("/ws")
async def websocket_endpoint(ws: WebSocket):
    """
    Generic WebSocket-to-IPC bridge.

    The client sends structured messages. The gateway routes them to
    the appropriate primal socket and returns the response. The gateway
    adds no business logic — it is a transport bridge only.

    Message types:
      {"type": "rpc", "target": "<primal>", "method": "<method>", "params": {...}}
        → resolves primal socket, calls method, returns result

      {"type": "ollama", "prompt": "...", "system_prompt": "...", "model": "...", "max_tokens": N}
        → passes prompt to Ollama, returns response

      {"type": "health"}
        → returns composition health for all known primals

      {"type": "discover", "capability": "<cap>"}
        → resolves primal by capability via biomeOS
    """
    await ws.accept()
    client_id = f"client-{int(time.time() * 1000) % 100000}"

    try:
        while True:
            raw = await ws.receive_text()
            try:
                msg = json.loads(raw)
            except json.JSONDecodeError:
                await ws.send_json({"type": "error", "message": "Invalid JSON"})
                continue

            msg_type = msg.get("type", "")
            req_id = msg.get("id", int(time.time() * 1000) % 100000)

            if msg_type == "rpc":
                target = msg.get("target", "esotericwebb")
                method = msg.get("method", "")
                params = msg.get("params")

                sock = await resolve_primal(target)
                if not sock:
                    await ws.send_json({
                        "type": "rpc_result", "id": req_id, "target": target,
                        "method": method,
                        "error": {"code": -5, "message": f"Cannot resolve primal: {target}"},
                    })
                    continue

                resp = await call_uds(sock, method, params, req_id)
                await ws.send_json({
                    "type": "rpc_result", "id": req_id, "target": target,
                    "method": method, "data": resp,
                })

            elif msg_type == "ollama":
                prompt = msg.get("prompt", "")
                system_prompt = msg.get("system_prompt", "")
                model = msg.get("model", "")
                max_tokens = msg.get("max_tokens", 150)
                text = await call_ollama(prompt, system_prompt, model, max_tokens)
                await ws.send_json({
                    "type": "ollama_result", "id": req_id,
                    "text": text or "(no response)",
                })

            elif msg_type == "health":
                health_data = await probe_composition_health()
                await ws.send_json({
                    "type": "health_result", "id": req_id, "data": health_data,
                })

            elif msg_type == "discover":
                capability = msg.get("capability", "")
                sock = await resolve_primal(capability)
                await ws.send_json({
                    "type": "discover_result", "id": req_id,
                    "capability": capability,
                    "socket": sock,
                    "resolved": sock is not None,
                })

            elif msg_type == "batch":
                calls = msg.get("calls", [])
                results = []
                for call in calls:
                    target = call.get("target", "esotericwebb")
                    method = call.get("method", "")
                    params = call.get("params")
                    call_id = call.get("id", req_id)
                    sock = await resolve_primal(target)
                    if sock:
                        resp = await call_uds(sock, method, params, call_id)
                        results.append({"target": target, "method": method,
                                        "id": call_id, "data": resp})
                    else:
                        results.append({"target": target, "method": method,
                                        "id": call_id,
                                        "error": {"code": -5,
                                                  "message": f"Cannot resolve: {target}"}})
                await ws.send_json({
                    "type": "batch_result", "id": req_id, "results": results,
                })

            else:
                await ws.send_json({
                    "type": "error", "message": f"Unknown type: {msg_type}",
                })

    except WebSocketDisconnect:
        pass
    except Exception as e:
        print(f"[gateway] WebSocket error: {e}", file=sys.stderr)
        traceback.print_exc()


def main():
    parser = argparse.ArgumentParser(description="ecoPrimals Gateway (thin bridge)")
    parser.add_argument("--host", default="0.0.0.0")
    parser.add_argument("--port", type=int, default=8765)
    args = parser.parse_args()

    print(f"[gateway] biomeOS socket:    {find_socket('biomeos')}")
    print(f"[gateway] Ollama:            {OLLAMA_ENDPOINT} ({OLLAMA_MODEL})")
    print(f"[gateway] Serving:           http://localhost:{args.port}/")
    print(f"[gateway] WebSocket bridge:  ws://localhost:{args.port}/ws")
    print(f"[gateway] Health:            http://localhost:{args.port}/health")

    uvicorn.run(app, host=args.host, port=args.port, log_level="info")


if __name__ == "__main__":
    main()
