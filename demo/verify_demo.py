"""TK19 — programmatic verification of the integration demo (REQ14).

For every layout: the page must reach SETTLED with strain exactly 0, the
canvas must show substantial rendered matter, and the settled state must
be visually frozen (two captures a second apart are identical — the
sharpened-I4 flicker-free window, on screen).
"""
import io
import struct
import sys
import zlib

from playwright.sync_api import sync_playwright

URL = "http://127.0.0.1:8321/index.html"
CASES = ["dashboard", "kanban", "gallery", "chat"]


def canvas_analysis(page):
    """Lit fraction, connected components of matter (4x downsampled),
    and whether every cell centroid sits on lit matter — the
    connectedness check the analysis/003 rejection taught us to demand:
    a cell must render as ONE piece of medium, never dots."""
    return page.evaluate(
        """() => {
            const c = document.getElementById('c');
            const x = c.getContext('2d');
            const d = x.getImageData(0, 0, c.width, c.height).data;
            const W = c.width, H = c.height, S = 4;
            const gw = Math.floor(W / S), gh = Math.floor(H / S);
            const m = new Uint8Array(gw * gh);
            let lit = 0;
            for (let gy = 0; gy < gh; gy++)
              for (let gx = 0; gx < gw; gx++) {
                const i = ((gy * S) * W + gx * S) * 4;
                const mx = Math.max(d[i], d[i+1], d[i+2]);
                const mn = Math.min(d[i], d[i+1], d[i+2]);
                if (mx > 70 && mx - mn > 25) { m[gy*gw+gx] = 1; lit++; }
              }
            // connected components over the downsampled mask
            let comps = 0;
            const lab = new Int32Array(gw * gh);
            for (let s = 0; s < gw * gh; s++) {
              if (!m[s] || lab[s]) continue;
              comps++;
              const q = [s]; lab[s] = comps;
              while (q.length) {
                const v = q.pop(), vx = v % gw, vy = (v / gw) | 0;
                for (const [dx, dy] of [[1,0],[-1,0],[0,1],[0,-1]]) {
                  const nx = vx + dx, ny = vy + dy;
                  if (nx < 0 || ny < 0 || nx >= gw || ny >= gh) continue;
                  const u = ny * gw + nx;
                  if (m[u] && !lab[u]) { lab[u] = comps; q.push(u); }
                }
              }
            }
            // every cell centroid must sit on matter
            const cells = window.__cells || [];
            let centroidsLit = 0;
            for (const cell of cells) {
              const gx = Math.min(gw - 1, Math.max(0, (cell.cx / S) | 0));
              const gy = Math.min(gh - 1, Math.max(0, (cell.cy / S) | 0));
              if (m[gy * gw + gx]) centroidsLit++;
            }
            return { litFrac: lit / (gw * gh), comps,
                     nCells: cells.length, centroidsLit };
        }"""
    )


def run():
    failures = []
    with sync_playwright() as p:
        browser = p.chromium.launch(channel="msedge")
        page = browser.new_page(viewport={"width": 1280, "height": 800})
        page.goto(URL)
        page.wait_for_timeout(1500)
        for i, name in enumerate(CASES):
            page.evaluate(
                f"document.querySelectorAll('#buttons button')[{i}].click()"
            )
            settled_ms = None
            for t in range(0, 120):
                page.wait_for_timeout(100)
                if page.eval_on_selector(
                    "#flag", "e => e.textContent"
                ) == "SETTLED":
                    settled_ms = t * 100
                    break
            strain = page.eval_on_selector("#stats", "e => e.textContent")
            an = canvas_analysis(page)
            lit = an["litFrac"]
            # flicker check on the actual pixel buffer (GPU PNG capture is
            # not byte-deterministic; the honest comparison is the raster)
            page.evaluate(
                """() => {
                    const c = document.getElementById('c');
                    window.__snap1 = c.getContext('2d')
                        .getImageData(0, 0, c.width, c.height).data.slice();
                }"""
            )
            page.wait_for_timeout(1000)
            frozen = page.evaluate(
                """() => {
                    const c = document.getElementById('c');
                    const d = c.getContext('2d')
                        .getImageData(0, 0, c.width, c.height).data;
                    const a = window.__snap1;
                    if (a.length !== d.length) return false;
                    let diff = 0;
                    for (let i = 0; i < d.length; i++)
                        if (a[i] !== d[i]) diff++;
                    return diff === 0;
                }"""
            )
            page.query_selector("#c").screenshot(
                path=f"verify_{name}.png"
            )
            connected = (
                1 <= an["comps"] <= an["nCells"]
                and an["centroidsLit"] == an["nCells"]
            )
            ok = (
                settled_ms is not None
                and "strain 0.000" in strain
                and lit > 0.02
                and frozen
                and connected
            )
            print(
                f"{name:<10} settled@{settled_ms}ms  {strain}  "
                f"matter={lit*100:.1f}%  comps={an['comps']}/"
                f"{an['nCells']} centroids={an['centroidsLit']}  "
                f"frozen={frozen}  {'PASS' if ok else 'FAIL'}"
            )
            if not ok:
                failures.append(name)
        browser.close()
    if failures:
        raise SystemExit(f"DEMO VERIFICATION FAILED: {failures}")
    print("all layouts verified: settled, strain zero, rendered, frozen")


if __name__ == "__main__":
    run()
