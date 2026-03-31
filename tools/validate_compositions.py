#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""
Composition Subsystem Validator — tests each C1-C7 composition independently.

Probes live primal sockets to verify each subsystem can operate in isolation.
Outputs a structured pass/fail report per composition.

Usage:
    python3 tools/validate_compositions.py
"""

import glob
import json
import os
import socket
import sys
import time

SOCKET_DIR = f"/run/user/{os.getuid()}/biomeos"


def find_sock(name: str) -> str | None:
    if name == "petaltongue":
        ipc = f"{SOCKET_DIR}/petaltongue-ipc.sock"
        if os.path.exists(ipc):
            return ipc
    if name in ("biomeos", "neural-api"):
        matches = glob.glob(f"{SOCKET_DIR}/neural-api-*.sock")
    else:
        matches = glob.glob(f"{SOCKET_DIR}/{name}-*.sock")
    if not matches:
        return None
    for sock in matches:
        if _probe_alive(sock):
            return sock
    return matches[0]


def _probe_alive(sock_path: str) -> bool:
    """Quick check if a socket accepts a connection."""
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(1.0)
        s.connect(sock_path)
        s.close()
        return True
    except Exception:
        return False


def call_uds(sock_path: str, method: str, params=None, timeout=5.0) -> dict:
    msg = json.dumps({"jsonrpc": "2.0", "method": method, "id": 1,
                       **({"params": params} if params else {})})
    try:
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.settimeout(timeout)
        s.connect(sock_path)
        s.sendall((msg + "\n").encode())
        data = b""
        while True:
            chunk = s.recv(4096)
            if not chunk:
                break
            data += chunk
            if b"\n" in data:
                break
        s.close()
        return json.loads(data.decode().strip().split("\n")[0])
    except Exception as e:
        return {"error": {"code": -1, "message": str(e)}}


class Result:
    def __init__(self, name: str):
        self.name = name
        self.checks: list[tuple[str, bool, str]] = []

    def check(self, label: str, passed: bool, detail: str = ""):
        self.checks.append((label, passed, detail))
        mark = "\033[32mPASS\033[0m" if passed else "\033[31mFAIL\033[0m"
        print(f"  [{mark}] {label}" + (f" — {detail}" if detail else ""))

    def summary(self) -> tuple[int, int]:
        p = sum(1 for _, ok, _ in self.checks if ok)
        return p, len(self.checks)


def validate_c1_render() -> Result:
    """C1: Render (petalTongue standalone)"""
    r = Result("C1: Render")
    sock = find_sock("petaltongue")
    r.check("petaltongue socket found", sock is not None, sock or "")
    if not sock:
        return r

    resp = call_uds(sock, "health.liveness")
    r.check("health.liveness", "result" in resp)

    resp = call_uds(sock, "visualization.render.dashboard", {
        "session_id": "c1-test", "title": "C1 Validation",
        "bindings": [{"channel_type": "gauge", "id": "test", "label": "Test",
                       "value": 42.0, "min": 0.0, "max": 100.0,
                       "unit": "", "normal_range": [0, 100], "warning_range": [0, 100]}],
        "modality": "svg",
    })
    r.check("visualization.render.dashboard", "result" in resp,
            f"keys: {list(resp.get('result', {}).keys()) if 'result' in resp else resp.get('error', {}).get('message', '')}")

    resp = call_uds(sock, "visualization.export", {"session_id": "c1-test", "format": "svg"})
    svg = resp.get("result", {}).get("content", "")
    r.check("visualization.export (SVG)", bool(svg), f"{len(svg)} bytes" if svg else "empty")

    identity_tf = {"a": 1.0, "b": 0.0, "tx": 0.0, "c": 0.0, "d": 1.0, "ty": 0.0}
    scene_graph = {
        "nodes": {
            "root": {
                "id": "root", "transform": identity_tf,
                "primitives": [], "children": ["test"],
                "visible": True, "opacity": 1.0,
                "label": "Root", "data_source": None,
            },
            "test": {
                "id": "test", "transform": identity_tf,
                "primitives": [], "children": [],
                "visible": True, "opacity": 1.0,
                "label": "Validation scene", "data_source": None,
            },
        },
        "root_id": "root",
    }
    resp = call_uds(sock, "visualization.render.scene", {
        "session_id": "c1-scene", "scene": scene_graph,
    })
    r.check("visualization.render.scene", "result" in resp,
            resp.get("error", {}).get("message", "") if "error" in resp else "")

    resp = call_uds(sock, "visualization.session.list")
    r.check("visualization.session.list", "result" in resp,
            f"sessions: {resp.get('result', {})}")

    return r


def validate_c2_narration() -> Result:
    """C2: Narration (Squirrel AI standalone)"""
    r = Result("C2: Narration")
    sock = find_sock("squirrel")
    r.check("squirrel socket found", sock is not None, sock or "NOT FOUND — expected gap")
    if not sock:
        r.check("ai.query", False, "Squirrel not running (gap SQ-01: AiRouter + Ollama)")
        r.check("ai.list_providers", False, "Squirrel not running")
        return r

    resp = call_uds(sock, "health.liveness")
    r.check("health.liveness", "result" in resp)

    resp = call_uds(sock, "ai.query", {"prompt": "Say hello in one word."}, timeout=15)
    r.check("ai.query", "result" in resp)

    resp = call_uds(sock, "ai.list_providers")
    r.check("ai.list_providers", "result" in resp)

    return r


def validate_c3_session() -> Result:
    """C3: Session (esotericWebb standalone)"""
    r = Result("C3: Session")
    sock = find_sock("esotericwebb")
    r.check("esotericwebb socket found", sock is not None, sock or "")
    if not sock:
        return r

    resp = call_uds(sock, "webb.liveness")
    r.check("webb.liveness", "result" in resp)

    resp = call_uds(sock, "session.start", {"content": "default"})
    has_session = "result" in resp
    r.check("session.start", has_session)

    resp = call_uds(sock, "session.state")
    state = resp.get("result", {})
    has_turn = "turn_count" in state or "turn" in state
    r.check("session.state (has turn)", has_turn, f"turn={state.get('turn_count', state.get('turn', '?'))}")

    has_trust = "trust" in state and isinstance(state.get("trust"), dict)
    r.check("session.state (has trust)", has_trust)

    resp = call_uds(sock, "session.actions")
    result = resp.get("result", {})
    actions = result.get("actions", result) if isinstance(result, dict) else result
    has_actions = isinstance(actions, list) and len(actions) > 0
    r.check("session.actions (non-empty)", has_actions, f"{len(actions) if isinstance(actions, list) else 0} actions")

    if has_actions and isinstance(actions, list) and len(actions) > 0:
        a = actions[0]
        kind = a.get("kind", a.get("action_kind", ""))
        act_id = a.get("id", a.get("detail", ""))
        resp = call_uds(sock, "session.act", {"action_kind": kind, "detail": act_id})
        acted = "result" in resp
        if not acted:
            resp = call_uds(sock, "session.act", {"kind": kind, "id": act_id})
            acted = "result" in resp
        r.check("session.act", acted,
                resp.get("error", {}).get("message", "") if not acted else "")
    else:
        r.check("session.act", False, "no actions to test")

    resp = call_uds(sock, "session.graph")
    has_graph = "result" in resp
    r.check("session.graph", has_graph)

    return r


def validate_c4_game_science() -> Result:
    """C4: Game Science (ludoSpring standalone)"""
    r = Result("C4: Game Science")
    sock = find_sock("ludospring")
    r.check("ludospring socket found", sock is not None, sock or "")
    if not sock:
        return r

    resp = call_uds(sock, "health.check")
    r.check("health.check", "result" in resp)

    resp = call_uds(sock, "game.evaluate_flow", {"skill": 0.5, "challenge": 0.5, "time_pressure": 0.3})
    result = resp.get("result", {})
    has_flow = "flow_score" in result or "score" in result or "state" in result
    r.check("game.evaluate_flow", has_flow,
            f"state={result.get('state', result.get('flow_score', '?'))}")

    resp = call_uds(sock, "game.fitts_cost", {"distance": 100.0, "target_width": 50.0})
    r.check("game.fitts_cost", "result" in resp,
            f"ID={resp.get('result', {}).get('index_of_difficulty', '?')}" if "result" in resp else
            resp.get("error", {}).get("message", ""))

    resp = call_uds(sock, "game.wfc_step", {"n_tiles": 4, "width": 4, "height": 4})
    r.check("game.wfc_step", "result" in resp,
            resp.get("error", {}).get("message", "") if "error" in resp else "")

    resp = call_uds(sock, "game.engagement", {
        "skill": 0.6, "challenge": 0.5, "session_duration_s": 120,
        "action_count": 10, "exploration_breadth": 3,
        "challenge_seeking": 2, "retry_count": 1, "deliberate_pauses": 1,
    })
    r.check("game.engagement", "result" in resp,
            resp.get("error", {}).get("message", "") if "error" in resp else "")

    return r


def validate_c5_persistence() -> Result:
    """C5: Persistence (NestGate standalone)"""
    r = Result("C5: Persistence")
    sock = find_sock("nestgate")
    r.check("nestgate socket found", sock is not None, sock or "")
    if not sock:
        return r

    resp = call_uds(sock, "health.liveness")
    alive = "result" in resp
    r.check("health.liveness", alive,
            "NOT RESPONDING — NestGate process may be stopped" if not alive else "")

    if not alive:
        r.check("storage.store", False, "NestGate not responding (gap NG-01: in-memory KV)")
        r.check("storage.retrieve", False, "NestGate not responding")
        r.check("storage.list", False, "NestGate not responding")
        return r

    test_key = f"_c5_validate_{int(time.time())}"
    resp = call_uds(sock, "storage.store", {"key": test_key, "value": "validation_ping"})
    r.check("storage.store", "result" in resp)

    resp = call_uds(sock, "storage.retrieve", {"key": test_key})
    val = resp.get("result", {}).get("value", "")
    r.check("storage.retrieve (round-trip)", val == "validation_ping",
            f"got: {val!r}")

    resp = call_uds(sock, "storage.list")
    r.check("storage.list", "result" in resp)

    return r


def validate_c6_proprioception() -> Result:
    """C6: Proprioception (petalTongue interaction loop)"""
    r = Result("C6: Proprioception")
    sock = find_sock("petaltongue")
    r.check("petaltongue socket found", sock is not None, sock or "")
    if not sock:
        return r

    resp = call_uds(sock, "interaction.subscribe", {
        "subscriber_id": "c6-validator",
        "events": ["select", "navigate"],
    })
    r.check("interaction.subscribe", "result" in resp,
            f"subscriber: {resp.get('result', {})}")

    resp = call_uds(sock, "visualization.interact.apply", {
        "intent": "select", "targets": ["c6-test-element"],
    })
    r.check("visualization.interact.apply", "result" in resp)

    resp = call_uds(sock, "interaction.poll", {"subscriber_id": "c6-validator"})
    events = resp.get("result", {}).get("events", [])
    r.check("interaction.poll (events)", "result" in resp,
            f"{len(events)} events")

    resp = call_uds(sock, "visualization.showing")
    r.check("visualization.showing", "result" in resp,
            f"showing: {resp.get('result', {})}")

    return r


def validate_c7_interactive() -> Result:
    """C7: Full Interactive Product (cross-subsystem)"""
    r = Result("C7: Full Interactive")

    bio_sock = find_sock("biomeos")
    r.check("biomeOS Neural API found", bio_sock is not None, bio_sock or "")
    if bio_sock:
        resp = call_uds(bio_sock, "graph.list")
        r.check("biomeOS graph.list", "result" in resp,
                f"{len(resp.get('result', []))} graphs" if "result" in resp else
                resp.get("error", {}).get("message", ""))

    bd_sock = find_sock("beardog")
    sb_sock = find_sock("songbird")
    if bd_sock:
        bd_resp = call_uds(bd_sock, "health.liveness")
        r.check("Tower: BearDog alive", "result" in bd_resp, bd_sock)
    else:
        r.check("Tower: BearDog alive", False, "socket not found")
    if sb_sock:
        sb_resp = call_uds(sb_sock, "health.liveness")
        r.check("Tower: Songbird alive", "result" in sb_resp, sb_sock)
    else:
        r.check("Tower: Songbird alive", False, "socket not found")

    webb_sock = find_sock("esotericwebb")
    if webb_sock:
        resp = call_uds(webb_sock, "session.start", {"content": "default"})
        has_session = "result" in resp
        r.check("C3→C7: session.start", has_session)

        if has_session:
            state_resp = call_uds(webb_sock, "session.state")
            state = state_resp.get("result", {})

            pt_sock = find_sock("petaltongue")
            if pt_sock and state:
                turn = state.get("turn_count", state.get("turn", 0))
                bindings = [{"channel_type": "gauge", "id": "turn", "label": "Turn",
                             "value": float(turn), "min": 0.0, "max": 100.0,
                             "unit": "", "normal_range": [0,100], "warning_range": [0,100]}]
                resp = call_uds(pt_sock, "visualization.render.dashboard", {
                    "session_id": "c7-test", "title": "C7 Cross-System",
                    "bindings": bindings, "modality": "svg",
                })
                r.check("C3→C1: state→DataBinding→render", "result" in resp)

                resp = call_uds(pt_sock, "visualization.export", {"session_id": "c7-test", "format": "svg"})
                svg = resp.get("result", {}).get("content", "")
                r.check("C1→C7: render→export SVG", bool(svg), f"{len(svg)} bytes")
            else:
                r.check("C3→C1: state→DataBinding→render", False, "petalTongue or state missing")
                r.check("C1→C7: render→export SVG", False, "")
    else:
        r.check("C3→C7: session.start", False, "esotericWebb not found")

    ludo_sock = find_sock("ludospring")
    if ludo_sock:
        resp = call_uds(ludo_sock, "game.evaluate_flow", {"skill": 0.5, "challenge": 0.5, "time_pressure": 0.3})
        r.check("C4→C7: game.evaluate_flow", "result" in resp)
    else:
        r.check("C4→C7: game.evaluate_flow", False, "ludoSpring not found")

    r.check("C2→C7: Squirrel AI", False, "Squirrel not running (expected gap SQ-01)")

    ng_sock = find_sock("nestgate")
    if ng_sock:
        resp = call_uds(ng_sock, "health.liveness")
        r.check("C5→C7: NestGate alive", "result" in resp)
    else:
        r.check("C5→C7: NestGate alive", False, "NestGate not responding")

    return r


def main():
    print("=" * 60)
    print("ecoPrimals Composition Subsystem Validation")
    print("=" * 60)
    print()

    compositions = [
        ("C1: Render (petalTongue)", validate_c1_render),
        ("C2: Narration (Squirrel AI)", validate_c2_narration),
        ("C3: Session (esotericWebb)", validate_c3_session),
        ("C4: Game Science (ludoSpring)", validate_c4_game_science),
        ("C5: Persistence (NestGate)", validate_c5_persistence),
        ("C6: Proprioception (petalTongue)", validate_c6_proprioception),
        ("C7: Full Interactive", validate_c7_interactive),
    ]

    all_results = []
    for label, fn in compositions:
        print(f"\n{'─' * 50}")
        print(f"  {label}")
        print(f"{'─' * 50}")
        result = fn()
        all_results.append(result)

    print(f"\n{'=' * 60}")
    print("SUMMARY")
    print(f"{'=' * 60}")
    total_pass, total_checks = 0, 0
    for result in all_results:
        p, t = result.summary()
        total_pass += p
        total_checks += t
        status = "\033[32mPASS\033[0m" if p == t else ("\033[33mPARTIAL\033[0m" if p > 0 else "\033[31mFAIL\033[0m")
        print(f"  {result.name:35s}  {p}/{t}  {status}")

    print(f"\n  {'TOTAL':35s}  {total_pass}/{total_checks}")
    pct = (total_pass / total_checks * 100) if total_checks > 0 else 0
    print(f"  Pass rate: {pct:.0f}%")

    if total_pass < total_checks:
        print(f"\n  Known gaps (expected failures):")
        print(f"    - C2: Squirrel not running (gap SQ-01)")
        print(f"    - C5: NestGate process stopped (gap NG-01)")

    return 0 if total_pass == total_checks else 1


if __name__ == "__main__":
    sys.exit(main())
