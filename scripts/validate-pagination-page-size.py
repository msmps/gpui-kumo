#!/usr/bin/env python3
"""Validate native PageSize proposals, focus and popup paint in both themes/widths.

Run inside D-Bus with DISPLAY, PyGObject, pyatspi, xdotool and ImageMagick.
Native selected options and readable Info establish owner acceptance/rejection;
ComboBox string value is authored but this pinned Linux adapter may not expose
Text/Value interfaces. These checks do not establish speech or OS IME behavior.
"""
import argparse
import json
from pathlib import Path
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', type=Path, default=Path('target/debug/kumo-gallery'))
parser.add_argument('--output', type=Path, default=Path('/tmp'))
args = parser.parse_args()
from gi.repository import Gio, GLib
import pyatspi
args.output.mkdir(parents=True, exist_ok=True)
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
bus.call_sync('org.a11y.Bus', '/org/a11y/bus', 'org.freedesktop.DBus.Properties',
              'Set', GLib.Variant('(ssv)', ('org.a11y.Status', 'ScreenReaderEnabled',
                                          GLib.Variant('b', True))), None,
              Gio.DBusCallFlags.NONE, -1, None)
desktop = pyatspi.Registry.getDesktop(0)
app = subprocess.Popen([str(args.binary.resolve())],
                       stdout=open(args.output / 'page-size-gallery.log', 'w'),
                       stderr=subprocess.STDOUT)
gallery = None
window = None


def walk(node):
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)


def find(name, role=None):
    return next(n for n in walk(gallery) if n.name == name and
                (role is None or n.getRoleName() == role))


def size(index):
    return find(f'Dataset {index} page size café 🦀', 'combo box')


def bounds(node):
    r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return [r.x, r.y, r.width, r.height]


def show(node):
    for _ in range(25):
        x, y, w, h = bounds(node)
        if 200 <= y <= 500:
            return
        clicks = max(1, min(100, abs(y - 320) // 42))
        subprocess.run(['xdotool', 'mousemove', '200', '700', 'click', '--repeat',
                        str(clicks), '--delay', '10', '5' if y > 500 else '4'], check=True)
        time.sleep(.5)
    raise AssertionError(('not visible', node.name, bounds(node)))


def click(node):
    action = node.queryAction()
    i = next(i for i in range(action.nActions) if action.getName(i) == 'click')
    assert action.doAction(i)
    time.sleep(.7)


def pointer(node):
    x, y, w, h = bounds(node)
    subprocess.run(['xdotool', 'windowfocus', window, 'mousemove',
                    str(x + w // 2), str(y + h // 2), 'click', '1'], check=True)
    time.sleep(.7)


def key(name):
    subprocess.run(['xdotool', 'key', name], check=True)
    time.sleep(.7)


def counter():
    return int(next(n.name.removeprefix('Page size proposals: ') for n in walk(gallery)
                    if n.getRoleName() == 'label' and n.name.startswith('Page size proposals: ')))


def is_open():
    return any(n.getRoleName() == 'list item' and n.name in ('10', '20', '25', '50', '100', '250') for n in walk(gallery))


def selected(value):
    assert find(str(value), 'list item').getState().contains(pyatspi.STATE_SELECTED)


def has_click(node):
    try:
        action = node.queryAction()
        return any(action.getName(i) == 'click' for i in range(action.nActions))
    except NotImplementedError:
        return False


def capture(theme, width, stage):
    subprocess.run(['timeout', '5', 'import', '-window', window,
                    str(args.output / f'page-size-{theme.lower()}-{width}-{stage}.png')], check=True)


try:
    time.sleep(3)
    gallery = next(n for n in desktop if n.get_process_id() == app.pid)
    window = subprocess.check_output(['xdotool', 'search', '--pid', str(app.pid),
                                     '--name', '^GPUI Kumo$'], text=True).strip().splitlines()[-1]
    subprocess.run(['xdotool', 'windowmove', window, '0', '0', 'windowfocus', window], check=True)
    results = []
    for theme, width in [('Light', 1040), ('Dark', 1040), ('Dark', 520), ('Light', 520)]:
        subprocess.run(['xdotool', 'mousemove', '200', '700', 'click', '--repeat',
                        '420', '--delay', '4', '4'], check=True)
        time.sleep(.7)
        click(find(theme + ' appearance', 'button'))
        subprocess.run(['xdotool', 'windowsize', window, str(width), '1000'], check=True)
        time.sleep(.7)
        show(find('Reset size datasets', 'button'))
        click(find('Reset size datasets', 'button'))
        show(size(5))
        original = counter()
        pointer(size(5))
        assert is_open()
        selected(25)
        for value in (25, 50, 100, 250):
            assert bounds(find(str(value), 'list item'))[3] == 32, (value, bounds(find(str(value), 'list item')))
        capture(theme, width, 'open')
        pointer(find('50', 'list item'))
        assert counter() == original + 1
        assert size(5).getState().contains(pyatspi.STATE_FOCUSED)
        assert not is_open()
        assert find('Dataset 5 page number', 'entry').queryText().getText(0, -1) == '1'
        find('Showing 1-50 of 500', 'label')
        key('space')
        selected(50)
        key('Down')
        key('Return')
        assert counter() == original + 2
        find('Showing 1-100 of 500', 'label')
        key('Return')
        selected(100)
        key('Return')  # Confirming current owner size is a no-op.
        assert counter() == original + 2
        key('space')
        key('Escape')
        assert size(5).getState().contains(pyatspi.STATE_FOCUSED)
        key('space')
        key('Tab')
        assert not is_open()
        assert not size(5).getState().contains(pyatspi.STATE_FOCUSED)
        capture(theme, width, 'accepted')
        show(size(6))
        pointer(size(6))
        selected(10)
        pointer(find('20', 'list item'))
        assert counter() == original + 3
        assert size(6).getState().contains(pyatspi.STATE_FOCUSED)
        key('Return')
        selected(10)  # Rejected value was never copied into durable selection.
        key('Escape')
        capture(theme, width, 'rejected')
        show(size(7))
        pointer(size(7))
        selected(10)
        key('Down')
        key('Return')
        assert counter() == original + 4
        assert find('Dataset 7 page number', 'entry').queryText().getText(0, -1) == '2'
        find('Showing 21-40 of 200', 'label')
        show(find('Toggle size availability', 'button'))
        click(find('Toggle size availability', 'button'))
        disabled = size(5)
        assert not disabled.getState().contains(pyatspi.STATE_ENABLED)
        assert not disabled.getState().contains(pyatspi.STATE_SENSITIVE)
        assert not has_click(disabled)
        show(disabled)
        pointer(disabled)
        key('space')
        key('Return')
        assert not is_open()
        assert counter() == original + 4
        capture(theme, width, 'disabled')
        show(find('Toggle size availability', 'button'))
        click(find('Toggle size availability', 'button'))
        assert size(5).getState().contains(pyatspi.STATE_ENABLED)
        assert size(5).getState().contains(pyatspi.STATE_SENSITIVE)
        assert has_click(size(5))
        rectangles = [bounds(size(i)) for i in (5, 6, 7)]
        for x, y, w, h in rectangles:
            assert x >= 57 and x + w <= width - 57 and h == 36, (width, rectangles)
        result = {'theme': theme, 'width': width, 'pointer_keyboard_once': True,
                  'owner_reset_to_first': True, 'controlled_rejection_selected_10': True,
                  'hidden_label_keeps_page': True, 'escape_tab_focus': True,
                  'disabled_guard_and_restoration': True, 'bounds': rectangles,
                  'combo_interfaces': pyatspi.listInterfaces(size(5)),
                  'expanded_export': 'not mapped by pinned adapter; actual popup queried'}
        results.append(result)
        print('COMBINATION PASS', json.dumps(result), flush=True)
    (args.output / 'page-size-results.json').write_text(json.dumps(results, indent=2) + '\n')
    print('PASS', flush=True)
except Exception:
    if window:
        capture('failure', 0, 'failure')
    raise
finally:
    app.terminate()
    try:
        app.wait(timeout=5)
    except subprocess.TimeoutExpired:
        app.kill()
        app.wait(timeout=5)
