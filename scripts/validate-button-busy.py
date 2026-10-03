#!/usr/bin/env python3
"""Check actual Linux loading-button owner updates in both themes and widths.

Run in a D-Bus session with DISPLAY set; requires PyGObject, pyatspi,
xdotool, ImageMagick and the native gallery. Checks metadata/paint,
not screen-reader speech or IME behavior.
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
                       stdout=open(args.output / 'button-busy-gallery.log', 'w'),
                       stderr=subprocess.STDOUT)

def walk(node):
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)

gallery = None

def button(name):
    return next(n for n in walk(gallery) if n.name == name and n.getRoleName() == 'button')

def click(name):
    node = button(name)
    action = node.queryAction()
    index = next(i for i in range(action.nActions) if action.getName(i) == 'click')
    assert action.doAction(index)
    time.sleep(1.2)

def has_click(node):
    try:
        action = node.queryAction()
        return any(action.getName(i) == 'click' for i in range(action.nActions))
    except NotImplementedError:
        return False

def show(name):
    for _ in range(20):
        r = button(name).queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        if 180 <= r.y <= 420:
            return
        clicks = max(1, min(80, abs(r.y - 300) // 42))
        subprocess.run(['xdotool', 'mousemove', '200', '500', 'click', '--repeat',
                        str(clicks), '--delay', '12', '5' if r.y > 420 else '4'], check=True)
        time.sleep(.5)
    raise AssertionError((name, 'not visible', r.y))

def counter():
    node = next(n for n in walk(gallery) if n.getRoleName() == 'label' and n.name.startswith('Activations: '))
    return int(node.name.removeprefix('Activations: '))

def pointer_save():
    r = button('Save changes').queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    subprocess.run(['xdotool', 'windowfocus', window, 'mousemove', str(r.x + r.width // 2),
                    str(r.y + r.height // 2), 'click', '1'], check=True)
    time.sleep(1)


def availability(loading):
    node = button('Save changes')
    state = node.getState()
    assert state.contains(pyatspi.STATE_BUSY) == loading
    assert state.contains(pyatspi.STATE_ENABLED) != loading
    assert state.contains(pyatspi.STATE_SENSITIVE) != loading
    assert has_click(node) != loading
    assert node.description == ('Loading' if loading else '')
    return {'busy': loading, 'enabled': not loading, 'sensitive': not loading,
            'click': not loading, 'description': node.description}

try:
    time.sleep(3)
    gallery = next(n for n in desktop if n.get_process_id() == app.pid)
    window = subprocess.check_output(['xdotool', 'search', '--pid', str(app.pid), '--name', '^GPUI Kumo$'],
                                     text=True).strip().splitlines()[-1]
    subprocess.run(['xdotool', 'windowmove', window, '0', '0', 'windowfocus', window], check=True)
    results = []
    for theme, width in [('Light', 1040), ('Dark', 1040), ('Dark', 520), ('Light', 520)]:
        # Theme handlers must be visible when activated; verify actual painted themes.
        subprocess.run(['xdotool', 'mousemove', '200', '500', 'click', '--repeat',
                        '420', '--delay', '4', '4'], check=True)
        time.sleep(1)
        click(theme + ' appearance')
        subprocess.run(['xdotool', 'windowsize', window, str(width), '1000'], check=True)
        time.sleep(1)
        show('Save changes')
        initial = availability(False)
        count = counter()
        pointer_save()
        assert button('Save changes').getState().contains(pyatspi.STATE_FOCUSED)
        count += 1
        assert counter() == count
        for key in ['space', 'Return']:
            subprocess.run(['xdotool', 'key', key], check=True)
            time.sleep(1)
            count += 1
            assert counter() == count
        click('Start loading')
        loading = availability(True)
        pointer_save()
        assert counter() == count
        subprocess.run(['timeout', '5', 'import', '-window', window,
                        str(args.output / f'button-{theme.lower()}-{width}-loading.png')], check=True)
        click('Stop loading')
        restored = availability(False)
        subprocess.run(['timeout', '5', 'import', '-window', window,
                        str(args.output / f'button-{theme.lower()}-{width}-restored.png')], check=True)
        assert counter() == count
        long_controls = [n for n in walk(gallery) if n.getRoleName() == 'button' and n.name == 'Secondary destructive']
        assert len(long_controls) == 3
        bounds = []
        for node in long_controls:
            r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            assert r.x >= 57 and r.x + r.width <= width - 57, (width, r.x, r.width)
            bounds.append([r.x, r.y, r.width, r.height])
        long_name = 'Create a project with a longer descriptive name'
        r = button(long_name).queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        assert r.x >= 57 and r.x + r.width <= width - 57, (width, long_name, r.x, r.width)
        long_example = [r.x, r.y, r.width, r.height]
        show('Secondary destructive')
        subprocess.run(['timeout', '5', 'import', '-window', window,
                        str(args.output / f'button-{theme.lower()}-{width}-variants.png')], check=True)
        show(long_name)
        subprocess.run(['timeout', '5', 'import', '-window', window,
                        str(args.output / f'button-{theme.lower()}-{width}-sizes.png')], check=True)
        result = {'theme': theme, 'width': width, 'initial': initial,
                  'long_example_inside_panel': long_example,
                  'pointer_space_enter_once': True, 'unavailable_pointer_guarded': True,
                  'long_controls_inside_panel': bounds,
                  'loading': loading, 'restored': restored}
        results.append(result)
        print('COMBINATION PASS', json.dumps(result), flush=True)
    (args.output / 'button-busy-results.json').write_text(json.dumps(results, indent=2) + '\n')
    print('PASS', flush=True)
finally:
    app.terminate()
    try:
        app.wait(timeout=5)
    except subprocess.TimeoutExpired:
        app.kill()
        app.wait(timeout=5)
