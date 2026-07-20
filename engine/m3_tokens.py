"""M3 token measurement (cl100k_base, same tokenizer as R3).

Counts the exact program text the LLM emitted for each novel intent,
against the R3 baseline. R3's <80 bar applied to its own 3-4 cell
layouts; the scale-fair figure is tokens per cell, reported alongside.
"""
import tiktoken

ENC = tiktoken.get_encoding("cl100k_base")

CASES = [
    ("kanban", "layouts/m3_kanban.nnui", 4),
    ("gallery", "layouts/m3_gallery.nnui", 5),
    ("chat", "layouts/m3_chat.nnui", 3),
]

R3 = [("dashboard", 68, 4), ("article", 47, 3), ("settings", 52, 3)]

print("M3 token economy (cl100k_base), novel LLM-emitted layouts:")
print(f"  {'layout':<10}{'tokens':>7}{'cells':>7}{'tok/cell':>10}")
total, total_cells = 0, 0
for name, path, cells in CASES:
    src = open(path, encoding="utf-8").read()
    n = len(ENC.encode(src))
    total += n
    total_cells += cells
    print(f"  {name:<10}{n:>7}{cells:>7}{n / cells:>10.1f}")
print(f"  {'TOTAL':<10}{total:>7}{total_cells:>7}{total / total_cells:>10.1f}")
print()
print("R3 baseline:")
print(f"  {'layout':<10}{'tokens':>7}{'cells':>7}{'tok/cell':>10}")
r3_total, r3_cells = 0, 0
for name, n, cells in R3:
    r3_total += n
    r3_cells += cells
    print(f"  {name:<10}{n:>7}{cells:>7}{n / cells:>10.1f}")
print(f"  {'TOTAL':<10}{r3_total:>7}{r3_cells:>7}{r3_total / r3_cells:>10.1f}")
