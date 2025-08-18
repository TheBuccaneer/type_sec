import json, sys
from pathlib import Path

# Walk common criterion locations (workspace crates and root)
roots = []
p = Path("target/criterion")
if p.exists():
    roots.append(p)
for cr in Path("crates").glob("*/target/criterion"):
    roots.append(cr)

rows = []
for root in roots:
    for est in root.rglob("estimates.json"):
        try:
            data = json.loads(est.read_text())
        except Exception:
            continue
        median = data.get("median", {}).get("point_estimate")
        parts = est.parts
        try:
            i = parts.index("criterion")
        except ValueError:
            continue
        bench = parts[i+1] if len(parts) > i+1 else ""
        group = parts[i+2] if len(parts) > i+2 else ""
        param = parts[i+3] if len(parts) > i+3 else ""
        rows.append((str(root), bench, group, param, median, str(est)))
print("root,bench,group,param,median_ns,estimates_path")
for r in rows:
    root, bench, group, param, med, path = r
    med_str = "" if med is None else f"{med:.6f}"
    print(f"{root},{bench},{group},{param},{med_str},{path}")
