#!/usr/bin/env bash
# SPDX-License-Identifier: AGPL-3.0-or-later
#
# desktop_session.sh — Base desktop composition session for ecoPrimals
#
# Provides a generic interactive loop on top of a running Desktop NUCLEUS.
# Springs extend this by overriding hooks:
#   on_start    — called once after capabilities discovered
#   on_tick     — called every POLL_INTERVAL
#   on_click    — called with click event data
#   on_keypress — called with key event data
#   on_ai       — called with AI response text
#   on_stop     — called before teardown
#
# Usage:
#   COMPOSITION_NAME=myspring source tools/desktop_session.sh
#   # Override hooks, then call: desktop_session_run
#
# Or standalone for testing:
#   ./tools/desktop_session.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

COMPOSITION_NAME="${COMPOSITION_NAME:-desktop-session}"
FAMILY_ID="${FAMILY_ID:-desktop-nucleus}"
REQUIRED_CAPS="${REQUIRED_CAPS:-visualization security}"
OPTIONAL_CAPS="${OPTIONAL_CAPS:-compute tensor dag ledger attribution ai discovery}"
POLL_INTERVAL="${POLL_INTERVAL:-0.5}"

source "$SCRIPT_DIR/nucleus_composition_lib.sh"

SESSION_RUNNING=false
SUBSCRIBER_ID="${COMPOSITION_NAME}-input"

# ── Default hooks (override these in your spring) ─────────────────

on_start()    { log "Desktop session started. Override on_start() for your domain."; }
on_tick()     { :; }
on_click()    { log "click: $*"; }
on_keypress() { log "key: $*"; }
on_ai()       { log "ai: $*"; }
on_stop()     { log "Desktop session stopping."; }

# ── Scene Helpers ─────────────────────────────────────────────────

_session_push_status() {
    local title="$1"
    local subtitle="${2:-}"
    local body="${3:-}"

    local nodes
    nodes=$(python3 -c "
import json
identity = {'a':1,'b':0,'tx':0,'c':0,'d':1,'ty':0}
white = {'r':1,'g':1,'b':1,'a':1}
green = {'r':0.2,'g':0.95,'b':0.4,'a':1}
grey = {'r':0.5,'g':0.5,'b':0.6,'a':1}
dark = {'r':0.05,'g':0.05,'b':0.12,'a':1}

def tnode(nid, x, y, text, size, color, bold=False):
    return [nid, {
        'id': nid, 'transform': identity, 'children': [], 'visible': True, 'opacity': 1.0,
        'label': None, 'data_source': None,
        'primitives': [{'Text': {'x':x,'y':y,'content':text,'font_size':size,'color':color,
                        'anchor':'TopLeft','bold':bold,'italic':False,'data_id':None}}]
    }]

children = ['title_node']
entries = [tnode('title_node', 50, 40, '''$title''', 24, green, True)]

if '''$subtitle''':
    children.append('sub_node')
    entries.append(tnode('sub_node', 50, 72, '''$subtitle''', 14, grey))
if '''$body''':
    children.append('body_node')
    entries.append(tnode('body_node', 50, 110, '''$body''', 16, white))

nodes = {
    'root': {
        'id': 'root', 'transform': identity, 'children': children,
        'visible': True, 'opacity': 1.0, 'label': None, 'data_source': None,
        'primitives': [{'Rect': {'x':0,'y':0,'width':800,'height':500,
                        'fill':dark,'stroke':None,'corner_radius':0,'data_id':None}}]
    }
}
for nid, n in entries:
    nodes[nid] = n

print(json.dumps({'nodes': nodes, 'root_id': 'root'}))
" 2>/dev/null)
    push_scene "$COMPOSITION_NAME" "$nodes"
}

# ── Core Loop ─────────────────────────────────────────────────────

desktop_session_run() {
    log "═══════════════════════════════════════"
    log "  Desktop Session: $COMPOSITION_NAME"
    log "═══════════════════════════════════════"

    discover_capabilities
    composition_startup "$COMPOSITION_NAME — Desktop Session"

    local viz_sock
    viz_sock="$(resolve_capability visualization petaltongue)"

    if [[ -n "$viz_sock" ]]; then
        subscribe_interactions "$SUBSCRIBER_ID"
        subscribe_sensor_stream "$SUBSCRIBER_ID"
    fi

    SESSION_RUNNING=true
    on_start

    trap '_session_cleanup' EXIT INT TERM

    while $SESSION_RUNNING; do
        on_tick

        if [[ -n "$viz_sock" ]]; then
            local events
            events=$(poll_interaction "$SUBSCRIBER_ID" 2>/dev/null || echo "")
            if [[ -n "$events" ]] && [[ "$events" != "null" ]] && [[ "$events" != "[]" ]]; then
                _process_events "$events"
            fi

            local sensor
            sensor=$(poll_sensor_stream "$SUBSCRIBER_ID" 2>/dev/null || echo "")
            if [[ -n "$sensor" ]] && [[ "$sensor" != "null" ]] && [[ "$sensor" != "[]" ]]; then
                process_sensor_batch "$sensor" 2>/dev/null || true
            fi
        fi

        sleep "$POLL_INTERVAL"
    done
}

_process_events() {
    local events_json="$1"
    python3 -c "
import json, sys
events = json.loads('''$events_json''')
if isinstance(events, list):
    for ev in events:
        etype = ev.get('event_type', ev.get('type', 'unknown'))
        print(f'{etype}|{json.dumps(ev)}')
elif isinstance(events, dict) and 'events' in events:
    for ev in events['events']:
        etype = ev.get('event_type', ev.get('type', 'unknown'))
        print(f'{etype}|{json.dumps(ev)}')
" 2>/dev/null | while IFS='|' read -r etype edata; do
        case "$etype" in
            click|Click)     on_click "$edata" ;;
            keypress|Key*)   on_keypress "$edata" ;;
            *)               log "event: $etype" ;;
        esac
    done
}

_session_cleanup() {
    SESSION_RUNNING=false
    on_stop
    if cap_available visualization; then
        unsubscribe_sensor_stream "$SUBSCRIBER_ID" 2>/dev/null || true
    fi
    composition_teardown
}

desktop_session_stop() {
    SESSION_RUNNING=false
}

# ── Standalone mode ───────────────────────────────────────────────

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    on_start() {
        _session_push_status \
            "ecoPrimals Desktop Session" \
            "$COMPOSITION_NAME | NUCLEUS: $FAMILY_ID" \
            "Polling for interactions... (Ctrl+C to stop)"
        if ai_available; then
            log "AI available via Songbird->Ollama bridge"
            local greeting
            greeting=$(ai_complete "You are the ecoPrimals desktop assistant. Say hello in one sentence." 2>/dev/null)
            if [[ -n "$greeting" ]]; then
                _session_push_status \
                    "ecoPrimals Desktop Session" \
                    "AI: $AI_MODEL" \
                    "$greeting"
            fi
        fi
    }

    on_click() {
        log "Click event: $1"
    }

    on_keypress() {
        log "Key event: $1"
    }

    desktop_session_run
fi
