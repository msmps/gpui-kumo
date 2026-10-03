"""Verify retained historical pixels and independent installed font metrics.

Requires Pillow/FreeType and fontconfig; run from any working directory.
Coordinates describe these recorded artifacts, not arbitrary future layouts.
"""
from pathlib import Path
import re
import subprocess
from PIL import Image, ImageFont

root = Path(__file__).resolve().parents[3]
evidence = Path(__file__).resolve().parent
old = Image.open(root / 'docs/evidence/readable-labels/text-light-1040.png').convert('RGB')
current = Image.open(evidence / 'linux/light-1040.png').convert('RGB')
reference = current.crop((57, 98, 220, 112)).tobytes()
assert old.crop((57, 436, 220, 450)).tobytes() == reference
matches = [y for y in range(530, 690)
           if current.crop((57, y, 220, y + 14)).tobytes() == reference]
assert len(matches) == 1, matches
print('Historical/gallery/explicit DejaVu Sans rich pixels match exactly:', matches)
traces = (evidence / 'linux/light-1040.log').read_text()
widths = {}
for family in ['DejaVu Sans', 'Liberation Sans', 'DejaVu Serif']:
    line = next(line for line in traces.splitlines() if line.startswith(f'FONT {family}:'))
    native = float(re.search(r'width: ([0-9.]+)px', line.split('; plain=')[1])[1])
    path = subprocess.check_output(
        ['fc-match', '-f', '%{file}', f'{family}:weight=semibold'], text=True)
    independent = ImageFont.truetype(path, 14).getlength('emphasis')
    assert abs(native - independent) < .03, (family, native, independent)
    widths[family] = native
    print(f'{family}: native={native}, FreeType={independent}, face={path}')
assert abs(widths['DejaVu Sans'] - widths['DejaVu Serif']) > .5
print('PASS: retained report pixels use the verified sans comparison, not serif')
