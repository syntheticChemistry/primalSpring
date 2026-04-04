#!/usr/bin/env python3
# SPDX-License-Identifier: AGPL-3.0-or-later
"""
Composition Subsystem Validator — tests primal-layer readiness for C1-C7.

Probes live primal sockets to verify each subsystem the primal layer owns.
C1/C2/C5/C6: primalSpring owns these primals and validates them directly.
C3/C4/C7: primalSpring sketches these for downstream (esotericWebb) and
parallel (ludoSpring) systems. Validation checks primal-layer readiness
(substrate, Tower, capability routing) — NOT downstream binaries.

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
    candidates = []
    if name == "petaltongue":
        candidates.append(f"{SOCKET_DIR}/petaltongue-ipc.sock")
    candidates.append(f"{SOCKET_DIR}/{name}.sock")
    xdg = os.environ.get("XDG_RUNTIME_DIR", f"/run/user/{os.getuid()}")
    candidates.append(f"{xdg}/{name}/{name}.sock")
    for c in candidates:
        if os.path.exists(c) and _probe_alive(c):
            return c
    if name in ("biomeos", "neural-api"):
        matches = glob.glob(f"{SOCKET_DIR}/neural-api-*.sock")
    else:
        matches = glob.glob(f"{SOCKET_DIR}/{name}-*.sock")
    if not matches:
        return None
    for sock in matches:
        if _probe_alive(sock):
            return sock
    return None


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
    r.check("squirrel socket found", sock is not None, sock or "NOT FOUND")
    if not sock:
        r.check("ai.query", False, "Squirrel not running")
        r.check("ai.list_providers", False, "Squirrel not running")
        return r

    resp = call_uds(sock, "health.liveness")
    r.check("health.liveness", "result" in resp)

    resp = call_uds(sock, "ai.query", {"prompt": "Say hello in one word."}, timeout=15)
    r.check("ai.query", "result" in resp)

    resp = call_uds(sock, "ai.list_providers")
    r.check("ai.list_providers", "result" in resp)

    return r


def validate_c3_session_readiness() -> Result:
    """C3: Session Readiness — primal layer can support session composition.

    esotericWebb is downstream and owns its own session validation.
    primalSpring validates that the substrate is ready for it.
    """
    r = Result("C3: Session Readiness")

    bio_sock = find_sock("biomeos")
    r.check("biomeOS substrate alive", bio_sock is not None, bio_sock or "")
    if not bio_sock:
        r.check("Tower: BearDog alive", False, "no substrate")
        r.check("Tower: Songbird alive", False, "no substrate")
        r.check("capability routing available", False, "no substrate")
        return r

    bd_sock = find_sock("beardog")
    if bd_sock:
        bd_resp = call_uds(bd_sock, "health.liveness")
        r.check("Tower: BearDog alive", "result" in bd_resp, bd_sock)
    else:
        r.check("Tower: BearDog alive", False, "socket not found")

    sb_sock = find_sock("songbird")
    if sb_sock:
        sb_resp = call_uds(sb_sock, "health.liveness")
        r.check("Tower: Songbird alive", "result" in sb_resp, sb_sock)
    else:
        r.check("Tower: Songbird alive", False, "socket not found")

    resp = call_uds(bio_sock, "capability.list")
    result = resp.get("result", {})
    caps = result if isinstance(result, list) else result.get("capabilities", [])
    r.check("capability routing available", "result" in resp,
            f"{len(caps) if isinstance(caps, list) else '?'} capabilities registered")

    has_narrative_ready = any(
        "session" in str(c).lower() or "narrative" in str(c).lower()
        for c in (caps if isinstance(caps, list) else [])
    )
    r.check("narrative/session domain routable",
            has_narrative_ready,
            "domain registered" if has_narrative_ready else
            "not yet registered — esotericWebb not running (expected)")

    return r


def validate_c4_game_readiness() -> Result:
    """C4: Game Science Readiness — primal layer can support game composition.

    ludoSpring is a parallel peer and owns its own game.* validation.
    primalSpring validates that the substrate is ready for it.
    """
    r = Result("C4: Game Science Readiness")

    bio_sock = find_sock("biomeos")
    r.check("biomeOS substrate alive", bio_sock is not None, bio_sock or "")
    if not bio_sock:
        r.check("Tower: BearDog alive", False, "no substrate")
        r.check("Tower: Songbird alive", False, "no substrate")
        r.check("capability routing available", False, "no substrate")
        return r

    bd_sock = find_sock("beardog")
    if bd_sock:
        bd_resp = call_uds(bd_sock, "health.liveness")
        r.check("Tower: BearDog alive", "result" in bd_resp, bd_sock)
    else:
        r.check("Tower: BearDog alive", False, "socket not found")

    sb_sock = find_sock("songbird")
    if sb_sock:
        sb_resp = call_uds(sb_sock, "health.liveness")
        r.check("Tower: Songbird alive", "result" in sb_resp, sb_sock)
    else:
        r.check("Tower: Songbird alive", False, "socket not found")

    resp = call_uds(bio_sock, "capability.list")
    result = resp.get("result", {})
    caps = result if isinstance(result, list) else result.get("capabilities", [])
    r.check("capability routing available", "result" in resp,
            f"{len(caps) if isinstance(caps, list) else '?'} capabilities registered")

    has_game_ready = any(
        "game" in str(c).lower()
        for c in (caps if isinstance(caps, list) else [])
    )
    r.check("game.* domain routable",
            has_game_ready,
            "domain registered" if has_game_ready else
            "not yet registered — ludoSpring not running (expected)")

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

    resp = call_uds(sock, "storage.list", {"family_id": "validation"})
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


def validate_c7_product_readiness() -> Result:
    """C7: Full Product Readiness — all primal capability domains healthy.

    Validates the complete primal stack is ready for esotericWebb to
    compose on. Does NOT require downstream (esotericWebb) or parallel
    (ludoSpring) binaries. Those systems validate themselves.
    """
    r = Result("C7: Product Readiness")

    bio_sock = find_sock("biomeos")
    r.check("biomeOS Neural API alive", bio_sock is not None, bio_sock or "")
    if not bio_sock:
        r.check("biomeOS graph.list", False, "no substrate")
        r.check("Tower: BearDog alive", False, "no substrate")
        r.check("Tower: Songbird alive", False, "no substrate")
        return r

    resp = call_uds(bio_sock, "graph.list")
    r.check("biomeOS graph.list", "result" in resp,
            f"{len(resp.get('result', []))} graphs" if "result" in resp else
            resp.get("error", {}).get("message", ""))

    bd_sock = find_sock("beardog")
    if bd_sock:
        bd_resp = call_uds(bd_sock, "health.liveness")
        r.check("Tower: BearDog alive", "result" in bd_resp, bd_sock)
    else:
        r.check("Tower: BearDog alive", False, "socket not found")

    sb_sock = find_sock("songbird")
    if sb_sock:
        sb_resp = call_uds(sb_sock, "health.liveness")
        r.check("Tower: Songbird alive", "result" in sb_resp, sb_sock)
    else:
        r.check("Tower: Songbird alive", False, "socket not found")

    sq_sock = find_sock("squirrel")
    if sq_sock:
        sq_resp = call_uds(sq_sock, "health.liveness")
        r.check("AI layer: Squirrel alive", "result" in sq_resp)
    else:
        r.check("AI layer: Squirrel alive", False, "Squirrel not running (optional)")

    pt_sock = find_sock("petaltongue")
    if pt_sock:
        pt_resp = call_uds(pt_sock, "health.liveness")
        r.check("Viz layer: petalTongue alive", "result" in pt_resp)
    else:
        r.check("Viz layer: petalTongue alive", False, "petalTongue not running (optional)")

    ng_sock = find_sock("nestgate")
    if ng_sock:
        ng_resp = call_uds(ng_sock, "health.liveness")
        r.check("Persistence: NestGate alive", "result" in ng_resp)
    else:
        r.check("Persistence: NestGate alive", False, "NestGate not running (optional)")

    resp = call_uds(bio_sock, "capability.list")
    result = resp.get("result", {})
    caps = result if isinstance(result, list) else result.get("capabilities", [])
    r.check("biomeOS capability routing", "result" in resp,
            f"{len(caps) if isinstance(caps, list) else '?'} capabilities registered")

    return r


def main():
    print("=" * 60)
    print("ecoPrimals Composition Subsystem Validation")
    print("=" * 60)
    print()

    compositions = [
        ("C1: Render (petalTongue)", validate_c1_render),
        ("C2: Narration (Squirrel AI)", validate_c2_narration),
        ("C3: Session Readiness (substrate)", validate_c3_session_readiness),
        ("C4: Game Science Readiness (substrate)", validate_c4_game_readiness),
        ("C5: Persistence (NestGate)", validate_c5_persistence),
        ("C6: Proprioception (petalTongue)", validate_c6_proprioception),
        ("C7: Product Readiness (full stack)", validate_c7_product_readiness),
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
        print(f"    - C3/C4 domain routing: downstream not running (expected)")
        print(f"    - C5: NestGate storage.list may need family_id (gap NG-01)")

    return 0 if total_pass == total_checks else 1


if __name__ == "__main__":
    sys.exit(main())
