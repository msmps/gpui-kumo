#!/usr/bin/env python3
"""Exercise Pagination in the native Linux X11 gallery through AT-SPI and keys.

Run inside a D-Bus session with DISPLAY set. Requires PyGObject, pyatspi,
Tkinter, xdotool, ImageMagick and the built gallery. This establishes metadata,
activation, editing and captures; it does not establish spoken or IME behavior.
"""
import argparse
from pathlib import Path
import subprocess
import time
import json
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', type=Path, default=Path('target/debug/kumo-gallery'))
parser.add_argument('--output', type=Path, default=Path('/tmp'))
parser.add_argument('--require-disabled-state', action='store_true', help='Require corrected disabled-state export from the pinned #36 patch')
args = parser.parse_args()

import tkinter
from gi.repository import Gio, GLib
import pyatspi

args.output.mkdir(parents=True, exist_ok=True)
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
bus.call_sync('org.a11y.Bus', '/org/a11y/bus', 'org.freedesktop.DBus.Properties', 'Set', GLib.Variant('(ssv)', ('org.a11y.Status', 'ScreenReaderEnabled', GLib.Variant('b', True))), None, Gio.DBusCallFlags.NONE, -1, None)
desktop = pyatspi.Registry.getDesktop(0)
app = subprocess.Popen([str(args.binary.resolve())], stdout=open(args.output / 'pagination-atspi-gallery.log', 'w'), stderr=subprocess.STDOUT)

def walk(node):
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)

def find(name, role=None, root=None):
    return next((n for n in walk(root or desktop) if n.name == name and (role is None or n.getRoleName() == role)))

def click(node):
    action = node.queryAction()
    i = next((i for i in range(action.nActions) if action.getName(i) == 'click'))
    assert action.doAction(i)
    time.sleep(1.2)

# Disabled Base Buttons remove the Action interface entirely.
def has_click(node):
    try:
        action = node.queryAction()
        return any((action.getName(i) == 'click' for i in range(action.nActions)))
    except NotImplementedError:
        return False

def key(k):
    subprocess.run(['xdotool', 'windowfocus', window, 'key', k], check=True)
    time.sleep(2)

def text(node):
    context = GLib.MainContext.default()
    while context.pending():
        context.iteration(False)
    node.clearCache()
    return node.queryText().getText(0, -1)

def nav(index):
    return find(f'Dataset {index} pages', 'landmark')

# Group and Entry share a name; select the native text entry.
def input(index=0):
    return find(f'Dataset {index} page number', 'entry')

window = None

def capture(name):
    if window is None:
        return
    time.sleep(0.8)
    subprocess.run(['import', '-window', window, str(args.output / ('pagination-' + name + '.png'))], check=True)

# Wait for actual input processing after a synthesized key burst.
def await_text(value, seconds=20):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        actual = text(input())
        if actual == value:
            return
        time.sleep(0.2)
    raise AssertionError(('settled draft', actual, value))

def draft(value):
    r = input().queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    subprocess.run(['xdotool', 'windowfocus', window, 'mousemove', str(r.x + r.width // 2), str(r.y + r.height // 2)], check=True)
    time.sleep(0.5)
    subprocess.run(['xdotool', 'click', '1'], check=True)
    time.sleep(1.2)
    key('ctrl+a')
    subprocess.run(['xdotool', 'type', '--delay', '120', value], check=True)
    await_text(value)
    print('DRAFT PASS', repr(value), flush=True)
try:
    for _ in range(60):
        time.sleep(0.2)
        try:
            nav(0)
            break
        except StopIteration:
            pass
    window = subprocess.check_output(['xdotool', 'search', '--name', '^GPUI Kumo$'], text=True).strip().splitlines()[-1]
    subprocess.run(['xdotool', 'windowmove', window, '0', '0', 'windowsize', window, '1040', '1000', 'windowfocus', window, 'mousemove', '10', '90'], check=True)
    time.sleep(1.5)
    results = []
    for theme, width in [('Light', 1040), ('Dark', 1040), ('Dark', 520), ('Light', 520)]:
        click(find(theme + ' appearance', 'button'))
        subprocess.run(['xdotool', 'windowsize', window, str(width), '1000'], check=True)
        time.sleep(1.5)
        click(find('Reset full dataset', 'button'))
        assert text(input()) == '5'
        next_button = find('Next page', 'button', nav(0))
        if args.require_disabled_state:
            assert next_button.getState().contains(pyatspi.STATE_ENABLED)
            assert next_button.getState().contains(pyatspi.STATE_SENSITIVE)
            loading = [n for n in walk(desktop) if n.getRoleName() == 'button' and n.description == 'Loading']
            assert {n.name for n in loading} >= {'Primary', 'Secondary', 'Ghost', 'Destructive', 'Secondary destructive', 'Outline'}
            for node in loading:
                assert not node.getState().contains(pyatspi.STATE_ENABLED), node.name
                assert not node.getState().contains(pyatspi.STATE_SENSITIVE), node.name
                assert not has_click(node), node.name
        r = next_button.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        subprocess.run(['xdotool', 'mousemove', str(r.x + r.width // 2), str(r.y + r.height // 2)], check=True)
        time.sleep(0.4)
        subprocess.run(['xdotool', 'click', '1'], check=True)
        time.sleep(1.2)
        assert text(input()) == '6'
        assert pyatspi.STATE_FOCUSED in find('Next page', 'button', nav(0)).getState().getStates()
        key('space')
        assert text(input()) == '7', ('Space actual', text(input()), find('Next page', 'button', nav(0)).getState().getStates())
        key('Return')
        assert text(input()) == '8'
        capture(theme.lower() + '-' + str(width) + '-keyboard')
        draft('999999999999999999999999')
        key('Return')
        assert text(input()) == '10'
        disabled = find('Next page', 'button', nav(0))
        assert not has_click(disabled)
        states = disabled.getState().getStates()
        print('DISABLED EXPORT', states, flush=True)
        if args.require_disabled_state:
            assert pyatspi.STATE_ENABLED not in states and pyatspi.STATE_SENSITIVE not in states
        r = disabled.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
        subprocess.run(['xdotool', 'mousemove', str(r.x + r.width // 2), str(r.y + r.height // 2), 'click', '1'], check=True)
        time.sleep(1)
        key('space')
        key('Return')
        assert text(input()) == '10'
        capture(theme.lower() + '-' + str(width) + '-last')
        key('Tab')
        assert text(input()) == '10'
        draft('bad')
        key('Return')
        assert text(input()) == '10'
        draft('3')
        key('Tab')
        assert text(input()) == '3'
        assert input().queryComponent().grabFocus()
        time.sleep(0.5)
        key('ctrl+a')
        key('ctrl+c')
        clip = tkinter.Tk()
        clip.withdraw()
        payload = clip.clipboard_get()
        clip.destroy()
        assert payload == '3'
        print('COMBINATION PASS', theme, width, flush=True)
        results.append({'theme': theme, 'width': width, 'pointer_space_enter_pages': [6, 7, 8], 'huge_draft_clamped': 10, 'malformed_restored': 10, 'blur_committed': 3, 'native_clipboard': payload, 'landmark': nav(0).name, 'disabled_states': [str(state) for state in states], 'enabled_restored': args.require_disabled_state, 'busy_variants_guarded': len(loading) if args.require_disabled_state else None, 'loading_busy_exported': all(n.getState().contains(pyatspi.STATE_BUSY) for n in loading) if args.require_disabled_state else None})
    click(find('Next page', 'button', nav(3)))
    assert text(input(3)) == '1'
    for page in (2, 3):
        click(find('Next page', 'button', nav(2)))
    assert not has_click(find('Next page', 'button', nav(2)))
    assert any((n.name == 'Page 3 · café 🦀' for n in walk(desktop)))
    assert not any((n.name in ('First page', 'Last page') for n in walk(nav(2))))
    result = {'combinations': results, 'controlled_rejection': True, 'unknown_server_boundary': True, 'readable_info': True, 'disabled_button_state_export': 'passed' if args.require_disabled_state else 'not checked (pass --require-disabled-state)'}
    (args.output / 'pagination-native-results.json').write_text(json.dumps(result, indent=2) + '\n')
    print('PASS', json.dumps(result, indent=2), flush=True)
except Exception:
    capture('failure')
    raise
finally:
    app.terminate()
    try:
        app.wait(timeout=5)
    except subprocess.TimeoutExpired:
        app.kill()
        app.wait()
