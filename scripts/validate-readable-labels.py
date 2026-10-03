#!/usr/bin/env python3
"""Check Linux AT-SPI readable labels before/after the value-authoring fix.

Run in a D-Bus session with DISPLAY set. Requires PyGObject, pyatspi,
xdotool and the built gallery. Pass --fixed to require complete names.
This checks exported metadata and bounds, not screen-reader speech.
"""
from pathlib import Path
import argparse, json, subprocess, time
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--fixed', action='store_true')
parser.add_argument('--binary', type=Path, default=Path('target/debug/kumo-gallery'))
parser.add_argument('--output', type=Path, default=Path('/tmp'))
args = parser.parse_args()
from gi.repository import Gio, GLib
import pyatspi
args.output.mkdir(parents=True, exist_ok=True)
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
bus.call_sync('org.a11y.Bus', '/org/a11y/bus', 'org.freedesktop.DBus.Properties', 'Set', GLib.Variant('(ssv)', ('org.a11y.Status', 'ScreenReaderEnabled', GLib.Variant('b', True))), None, Gio.DBusCallFlags.NONE, -1, None)
app = subprocess.Popen([str(args.binary.resolve())], stdout=open(args.output / 'readable-labels-gallery.log', 'w'), stderr=subprocess.STDOUT)

def walk(node):
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)
desktop = pyatspi.Registry.getDesktop(0)
expected = ['Rich inline emphasis', 'café 🦀 — long content keeps its complete accessible name', 'Healthy', 'With icon', 'Compact message', 'Learn about this update.', 'Worker Analytics', 'Long current project café 🦀 with a full accessible name', 'Standalone label café 🦀 (optional)']
try:
    time.sleep(3)
    window = subprocess.check_output(['xdotool', 'search', '--name', '^GPUI Kumo$'], text=True).strip().splitlines()[-1]
    results = []
    for theme, width in [('Light', 1040), ('Dark', 1040), ('Dark', 520), ('Light', 520)]:
        target = next((n for n in walk(desktop) if n.name == theme + ' appearance' and n.getRoleName() == 'button'))
        assert target.queryAction().doAction(0)
        subprocess.run(['xdotool', 'windowsize', window, str(width), '1000', 'windowfocus', window], check=True)
        time.sleep(2)
        nodes = list(walk(desktop))
        labels = [n for n in nodes if n.getRoleName() == 'label']
        names = [n.name for n in labels]
        missing = [name for name in expected if name not in names]
        headings = [n.name for n in nodes if n.getRoleName() == 'heading']
        assert 'Typography with heading semantics' in headings
        geometry = []
        for node in labels:
            r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            geometry.append([r.x, r.y, r.width, r.height])
        result = {'geometry': geometry, 'theme': theme, 'width': width, 'label_count': len(labels), 'unnamed_labels': sum((not n for n in names)), 'missing_samples': missing, 'heading_preserved': True}
        print('COMBINATION', json.dumps(result, ensure_ascii=False), flush=True)
        results.append(result)
        if args.fixed:
            assert not missing, missing
        else:
            assert len(missing) == len(expected), missing
    (args.output / 'readable-labels-results.json').write_text(json.dumps({'fixed': args.fixed, 'samples': expected, 'combinations': results}, ensure_ascii=False, indent=2) + '\n')
    print('PASS', json.dumps({'fixed': args.fixed, 'samples': expected, 'combinations': results}, ensure_ascii=False, indent=2), flush=True)
finally:
    app.terminate()
    try:
        app.wait(timeout=5)
    except subprocess.TimeoutExpired:
        app.kill()
        app.wait(timeout=5)
