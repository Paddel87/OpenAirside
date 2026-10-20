"""Evaluation of spike 3.14: measurement files and Chromium traces.

Usage:
  python3 -I analyze.py result <openairside-measure-*.json> ...
  python3 -I analyze.py trace <openairside-trace-*.json.gz> ...

`result` prints per window the latency figures and a summary of the late
answers. `trace` lists, for every late answer marked in the trace
(`oas:request` / `oas:answer`), what the renderer main thread did in between
and which slices of the GPU process overlap. Throw-away code of the spike.
"""

import gzip
import json
import statistics
import sys
from collections import Counter, defaultdict


def result(paths):
    for p in paths:
        d = json.load(open(p, encoding="utf-8"))
        s = d.get("system", {})
        r = d.get("renderer", {})
        series = d["series"]
        print(f"== {p}")
        print(
            f"   angle={s.get('angle')} fullscreen={s.get('fullscreen')} gpu={r.get('gpu')}"
            f" window={r.get('windowWidth')}x{r.get('windowHeight')} world={series['config'].get('world')}"
        )
        print(f"   traces={[(t['label'], t['atS'], round(t['bytes'] / 1e6, 1)) for t in s.get('traces', [])]}")
        print(
            "   win view        fps  frameP99 | ans late 2t afterTick | arrP99 arrMax | selfP99 selfLate"
            " | pixiP50 pixiP99 pixiMax | sendAfter p50 | core p99 eld | toCore min/p99 fromCore min/p99 | tr"
        )
        for w in series["windows"]:
            lat = w.get("latency")
            if lat is None:
                continue
            print(
                f"   {w['window']:3d} {w.get('label', w['view']):11s} {w['fps']:5.1f} {w['frameMsP99']:6.1f} |"
                f" {lat['answers']:4d} {lat['late']:4d} {lat['lateTwoTicks']:3d} {lat['afterTick']:4d} |"
                f" {lat['arriveMsP99']:6.1f} {lat['arriveMsMax']:6.1f} | {lat['selfMsP99']:6.1f} {lat['selfLate']:4d} |"
                f" {lat['pixiMsP50']:5.1f} {lat['pixiMsP99']:6.1f} {lat['pixiMsMax']:6.1f} |"
                f" {lat['sendAfterTickMsP50']:5.1f} | {lat['coreMsP99']:5.1f} {lat['coreEldMsMax']:5.1f} |"
                f" {lat['toCoreMsMin']:6.1f}/{lat['toCoreMsP99']:6.1f} {lat['fromCoreMsMin']:6.1f}/{lat['fromCoreMsP99']:6.1f}"
                f" | {'T' if lat['traced'] else ''}"
            )
        late = series.get("late", [])
        total = sum(w.get("latency", {}).get("answers", 0) for w in series["windows"])
        n_late = sum(w.get("latency", {}).get("late", 0) for w in series["windows"])
        print(f"   late answers: {n_late} of {total} ({100 * n_late / max(total, 1):.2f} %), listed {len(late)}")
        if late:
            self_late = sum(1 for x in late if x["selfMs"] > 12)
            ticks = Counter(x["ticks"] for x in late)
            print(f"   self message also late: {self_late} of {len(late)}; ticks between: {dict(ticks)}")
            for key in ("arriveMs", "selfMs", "pixiMs", "coreMs", "eldMs", "toCoreMs", "fromCoreMs"):
                vals = [x[key] for x in late if isinstance(x[key], (int, float))]
                if vals:
                    print(f"   {key:10s} median {statistics.median(vals):6.1f}  max {max(vals):6.1f}")


def load_trace(path):
    opener = gzip.open if path.endswith(".gz") else open
    with opener(path, "rt", encoding="utf-8") as f:
        data = json.load(f)
    return data["traceEvents"] if isinstance(data, dict) else data


def trace(paths):
    for p in paths:
        events = load_trace(p)
        names = {}
        threads = {}
        for e in events:
            if e.get("ph") == "M" and e.get("name") == "process_name":
                names[e["pid"]] = e["args"].get("name")
            if e.get("ph") == "M" and e.get("name") == "thread_name":
                threads[(e["pid"], e["tid"])] = e["args"].get("name")
        marks = sorted(
            (e["ts"], e["name"], e["pid"], e["tid"])
            for e in events
            if e.get("name") in ("oas:request", "oas:answer") and e.get("ph") in ("R", "I", "i", "n", "b", "e")
        )
        print(f"== {p}: {len(events)} events, processes {sorted(set(names.values()))}")
        if not marks:
            print("   no oas marks found")
            continue
        rpid, rtid = marks[0][2], marks[0][3]
        main_slices = sorted(
            (e["ts"], e.get("dur", 0), e["name"])
            for e in events
            if e.get("ph") == "X" and e.get("pid") == rpid and e.get("tid") == rtid
        )
        gpu_pids = [pid for pid, n in names.items() if n and "GPU" in n]
        gpu_slices = sorted(
            (e["ts"], e.get("dur", 0), e["name"], threads.get((e["pid"], e["tid"]), ""))
            for e in events
            if e.get("ph") == "X" and e.get("pid") in gpu_pids and e.get("dur", 0) >= 2000
        )
        pairs = []
        last_req = None
        for ts, name, _pid, _tid in marks:
            if name == "oas:request":
                last_req = ts
            elif last_req is not None:
                pairs.append((last_req, ts))
                last_req = None
        late = [(a, b) for a, b in pairs if b - a > 12_000]
        print(f"   requests answered: {len(pairs)}, late: {len(late)}")
        busy = defaultdict(float)
        for a, b in late:
            for ts, dur, name in main_slices:
                if ts + dur < a or ts > b:
                    continue
                busy[name] += (min(ts + dur, b) - max(ts, a)) / 1000
        print("   renderer main thread between request and late answer (ms summed over late answers):")
        for name, ms in sorted(busy.items(), key=lambda kv: -kv[1])[:25]:
            print(f"     {ms:9.1f}  {name}")
        for a, b in late[:5]:
            print(f"   late answer after {(b - a) / 1000:.1f} ms at {a / 1000:.1f} ms:")
            for ts, dur, name in main_slices:
                if ts + dur >= a and ts <= b and dur >= 1000:
                    print(f"     main {(ts - a) / 1000:7.1f} +{dur / 1000:6.1f}  {name}")
            for ts, dur, name, th in gpu_slices:
                if ts + dur >= a and ts <= b:
                    print(f"     gpu  {(ts - a) / 1000:7.1f} +{dur / 1000:6.1f}  {name} [{th}]")


if __name__ == "__main__":
    {"result": result, "trace": trace}[sys.argv[1]](sys.argv[2:])
