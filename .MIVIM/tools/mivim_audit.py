"""MIVIM link-completeness audit (ONBOARDING section 6 deep-audit).

Builds a definitions map and a body-reference map (outside fenced code),
reports body definitions, zero placeholders, duplicates, missing and unused
definitions, and emits the VALIDATE_WORKFLOW.json row shape.
"""
import json
import re
import sys

def audit(path, display_name):
    text = open(path, encoding="utf-8").read()
    lines = text.split("\n")

    # Strip fenced code blocks (fence closes only at >= opening length).
    active, fence_len = [], 0
    for ln in lines:
        m = re.match(r"^\s*(`{3,}|~{3,})", ln)
        if m:
            if fence_len == 0:
                fence_len = len(m.group(1))
            elif len(m.group(1)) >= fence_len:
                fence_len = 0
            active.append("")
            continue
        active.append("" if fence_len else ln)

    defs, def_lines = [], {}
    body_defs = []
    reg_start = None
    for i, ln in enumerate(active):
        if re.match(r"^## \(Z\) Context-Citations", ln):
            reg_start = i
        m = re.match(r"^\[([A-Za-z0-9]+)\]:", ln)
        if m:
            label = m.group(1).lower()
            defs.append(label)
            def_lines.setdefault(label, []).append(i + 1)
            if reg_start is None:
                body_defs.append(f"line {i+1}: [{m.group(1)}]:")
        if re.match(r"^>\s*\[[A-Za-z0-9]+\]:", ln):
            body_defs.append(f"line {i+1}: blockquote definition")

    zero = []
    for i, ln in enumerate(active):
        for m in re.finditer(r"\[([A-Z]{1,6}00)\]", ln):
            zero.append(f"line {i+1}: [{m.group(1)}]")

    # References: [ID] not followed by ':' or '(' (inline links excluded),
    # not preceded by ']' and only outside the registry definition lines.
    refs = set()
    for i, ln in enumerate(active):
        if re.match(r"^\[([A-Za-z0-9]+)\]:", ln):
            ln = re.sub(r"^\[[A-Za-z0-9]+\]:", "", ln)  # keep rest of line
        # GFM task-list checkboxes and inline code spans are not markers.
        ln = re.sub(r"^(\s*[-*] )\[[ xX]\] ", r"\1", ln)
        ln = re.sub(r"`[^`]*`", "", ln)
        for m in re.finditer(r"\[([A-Za-z0-9]+)\](?![:(\]])", ln):
            # exclude markdown inline link text [text](url)
            refs.add(m.group(1).lower())

    dset = set(defs)
    dupes = sorted({d for d in dset if len(def_lines[d]) > 1})
    missing = sorted(refs - dset)
    unused = sorted(dset - refs)

    row = {
        "file": display_name,
        "definitions": len(dset),
        "unique_active_references": len(refs & dset),
        "body_definitions": body_defs,
        "zero_placeholders": zero,
        "duplicate_definitions": dupes,
        "missing_definitions": missing,
        "unused_definitions": unused,
    }
    return row

if __name__ == "__main__":
    rows = [audit(p, n) for p, n in
            [(sys.argv[i], sys.argv[i + 1]) for i in range(1, len(sys.argv), 2)]]
    print(json.dumps(rows, indent=2))
