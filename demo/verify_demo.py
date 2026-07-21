"""TK19 — programmatic verification of the integration demo (REQ14),
upgraded to the framework standard (paper §9): the settled screen must
MEAN what the program said.

Per layout: settled with strain exactly 0; the vessel substantially
claimed (E_fill); sizes SOLVED from material (soft main larger than stiff
satellites); bound cells packed adjacent (E_cohesion); phenotypes
genuinely expressed through the registry; raster frozen at rest; and a
vessel reshape re-settles with NO further program call.
"""
from playwright.sync_api import sync_playwright

URL = "http://127.0.0.1:8321/index.html"

# name, cells, expected kinds, big cell, satellites, bound chains
CASES = [
    ("dashboard", ["main", "k1", "k2", "k3"],
     {"main": "chart", "k1": "chart", "k2": "chart", "k3": "chart"},
     "main", ["k1", "k2", "k3"], [("k1", "k2"), ("k2", "k3")]),
    ("kanban", ["head", "todo", "doing", "done"],
     {"head": "reader", "todo": "reader", "doing": "reader",
      "done": "reader"},
     None, [], [("todo", "doing"), ("doing", "done")]),
    ("gallery", ["main", "t1", "t2", "t3", "t4"],
     {"main": "player", "t1": "image", "t2": "image", "t3": "image",
      "t4": "image"},
     "main", ["t1", "t2", "t3", "t4"],
     [("t1", "t2"), ("t2", "t3"), ("t3", "t4")]),
    ("chat", ["sidebar", "pane", "composer"],
     {"sidebar": "reader", "pane": "reader", "composer": "reader"},
     "pane", ["sidebar", "composer"], []),
]

CELL_INFO_JS = """() => (window.__cells || []).map(c => {
    let area = 0;
    const p = c.pts;
    for (let i = 0; i < p.length; i++) {
        const [x1, y1] = p[i], [x2, y2] = p[(i + 1) % p.length];
        area += x1 * y2 - x2 * y1;
    }
    const xs = p.map(q => q[0]), ys = p.map(q => q[1]);
    return { name: c.name, kind: c.kind, settled: c.settled,
             area: Math.abs(area) / 2,
             minx: Math.min(...xs), maxx: Math.max(...xs),
             miny: Math.min(...ys), maxy: Math.max(...ys) };
})"""


def lit_fraction(page):
    return page.evaluate(
        """() => {
            const c = document.getElementById('c');
            const d = c.getContext('2d')
                .getImageData(0, 0, c.width, c.height).data;
            let lit = 0;
            for (let i = 0; i < d.length; i += 16) {
                const mx = Math.max(d[i], d[i+1], d[i+2]);
                const mn = Math.min(d[i], d[i+1], d[i+2]);
                if (mx > 70 && mx - mn > 25) lit++;
            }
            return lit / (d.length / 16);
        }"""
    )


def raster_frozen(page, ms=1000):
    page.evaluate(
        """() => { const c = document.getElementById('c');
           window.__snap1 = c.getContext('2d')
             .getImageData(0, 0, c.width, c.height).data.slice(); }"""
    )
    page.wait_for_timeout(ms)
    return page.evaluate(
        """() => { const c = document.getElementById('c');
           const d = c.getContext('2d')
             .getImageData(0, 0, c.width, c.height).data;
           const a = window.__snap1;
           if (a.length !== d.length) return false;
           for (let i = 0; i < d.length; i++)
             if (a[i] !== d[i]) return false;
           return true; }"""
    )


def wait_settled(page, timeout_s=30):
    for t in range(timeout_s * 10):
        page.wait_for_timeout(100)
        if page.eval_on_selector("#flag", "e => e.textContent") == "SETTLED":
            return t * 100
    return None


def gap(a, b):
    dx = max(0.0, max(a["minx"], b["minx"]) - min(a["maxx"], b["maxx"]))
    dy = max(0.0, max(a["miny"], b["miny"]) - min(a["maxy"], b["maxy"]))
    return max(dx, dy)


def check_layout(page, name, cells, kinds, big, sats, chains):
    problems = []
    settled_ms = wait_settled(page)
    if settled_ms is None:
        return [f"{name}: never settled"]
    strain = page.eval_on_selector("#stats", "e => e.textContent")
    if "strain 0.000" not in strain:
        problems.append(f"strain not zero: {strain}")
    info = {c["name"]: c for c in page.evaluate(CELL_INFO_JS)}
    if sorted(info) != sorted(cells):
        problems.append(f"cells mismatch: {sorted(info)}")
        return [f"{name}: {p}" for p in problems]
    for cn, kind in kinds.items():
        if info[cn]["kind"] != kind:
            problems.append(
                f"{cn} expressed {info[cn]['kind']}, wanted {kind}")
    if big:
        mean_sat = sum(info[s]["area"] for s in sats) / len(sats)
        ratio = info[big]["area"] / mean_sat
        if ratio < 1.5:
            problems.append(
                f"size not solved: {big} only {ratio:.2f}x satellites")
    for a, b in chains:
        g = gap(info[a], info[b])
        if g > 90:
            problems.append(f"bound pair {a}-{b} not adjacent (gap {g:.0f}px)")
    lit = lit_fraction(page)
    if lit < 0.80:
        problems.append(f"tessellation short: lit {lit*100:.0f}%")
    corners = page.evaluate(
        """() => {
            const c = document.getElementById('c');
            const x = c.getContext('2d');
            const box = 50; let claimed = 0;
            for (const [ox, oy] of [[0,0],[c.width-box,0],
                                    [0,c.height-box],
                                    [c.width-box,c.height-box]]) {
                const d = x.getImageData(ox, oy, box, box).data;
                let lit = 0;
                for (let i = 0; i < d.length; i += 4) {
                    const mx = Math.max(d[i], d[i+1], d[i+2]);
                    const mn = Math.min(d[i], d[i+1], d[i+2]);
                    if (mx > 70 && mx - mn > 25) lit++;
                }
                if (lit > box * box * 0.3) claimed++;
            }
            return claimed;
        }"""
    )
    if corners < 4:
        problems.append(f"corners unclaimed: {corners}/4")
    if not raster_frozen(page):
        problems.append("raster not frozen at rest")
    page.query_selector("#c").screenshot(path=f"verify_{name}.png")
    extra = (f" size={info[big]['area']/ (sum(info[s]['area'] for s in sats)/len(sats)):.2f}x"
             if big else "")
    print(f"{name:<10} settled@{settled_ms}ms lit={lit*100:.0f}%{extra} "
          f"{'PASS' if not problems else 'FAIL ' + '; '.join(problems)}")
    return [f"{name}: {p}" for p in problems]


def run():
    failures = []
    with sync_playwright() as p:
        browser = p.chromium.launch(channel="msedge")
        page = browser.new_page(viewport={"width": 1380, "height": 860})
        page.goto(URL)
        page.wait_for_timeout(1500)
        for i, (name, cells, kinds, big, sats, chains) in enumerate(CASES):
            page.evaluate(
                f"document.querySelectorAll('#buttons button')[{i}].click()")
            failures += check_layout(page, name, cells, kinds, big, sats,
                                     chains)
        # §9: vessel reshape re-settles with NO program call
        page.evaluate(
            "document.querySelectorAll('#buttons button')[0].click()")
        wait_settled(page)
        prog_before = page.eval_on_selector("#program", "e => e.textContent")
        page.evaluate("document.getElementById('v-tall').click()")
        page.wait_for_timeout(300)
        t = wait_settled(page)
        prog_after = page.eval_on_selector("#program", "e => e.textContent")
        lit = lit_fraction(page)
        ok = (t is not None and prog_before == prog_after and lit > 0.80)
        page.query_selector("#c").screenshot(path="verify_resize.png")
        print(f"{'resize':<10} resettled@{t}ms lit={lit*100:.0f}% "
              f"program-unchanged={prog_before == prog_after} "
              f"{'PASS' if ok else 'FAIL'}")
        if not ok:
            failures.append("resize: re-settle without program call failed")
        browser.close()
    if failures:
        raise SystemExit("DEMO VERIFICATION FAILED:\n  " +
                         "\n  ".join(failures))
    print("framework verified: fill, solved sizes, cohesion, phenotypes, "
          "frozen rest, zero-cost resize")


if __name__ == "__main__":
    run()
