"""
Parse a Strava feed HTML file and extract feed entry data to CSV.

Usage:
    python scripts/parse_feed.py [input.html] [output.csv]

Defaults:
    input:  feed.html
    output: feed_entries.csv
"""
import re, csv, io, sys
from pathlib import Path

input_path  = Path(sys.argv[1]) if len(sys.argv) > 1 else Path(__file__).parent.parent / "feed.html"
output_path = Path(sys.argv[2]) if len(sys.argv) > 2 else Path(__file__).parent.parent / "feed_entries.csv"

with open(input_path, "r", encoding="utf-8") as f:
    html = f.read()

entry_blocks = re.split(r'(?=id="feed-entry-\d+"\s)', html)

rows = []
for block in entry_blocks:
    entry_id_m = re.search(r'id="feed-entry-(\d+)"', block)
    if not entry_id_m:
        continue
    entry_id = entry_id_m.group(1)

    name_m = re.search(r'data-testid="owners-name"[^>]*>([\s\S]*?)</a>', block)
    athlete = re.sub(r'<[^>]+>', '', name_m.group(1)).strip() if name_m else ""
    athlete = ' '.join(athlete.split())

    date_m = re.search(r'data-testid="date_at_time">([^<]+)</time>', block)
    date_val = ' '.join(date_m.group(1).split()) if date_m else ""
    date_val = re.sub(r'\s+at\s+\d+:\d+\s*[AP]M', '', date_val).strip()

    dist_m = re.search(r'Distance</span>\s*<div[^>]*>([\d.]+)<abbr[^>]*title="([^"]+)"', block)
    distance = dist_m.group(1) if dist_m else ""
    unit     = dist_m.group(2) if dist_m else ""

    rows.append({
        "feed_entry_id": entry_id,
        "athlete_name":  athlete,
        "distance":      distance,
        "unit":          unit,
        "date_at_time":  date_val,
    })

with io.open(output_path, "w", newline="", encoding="utf-8-sig") as f:
    w = csv.DictWriter(f, fieldnames=["feed_entry_id", "athlete_name", "distance", "unit", "date_at_time"])
    w.writeheader()
    w.writerows(rows)

print(f"Parsed {len(rows)} entries -> {output_path}")
