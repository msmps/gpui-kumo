#!/usr/bin/env python3
"""Validate native page Dropdown proposals, focus, modes and source paint.

Run inside D-Bus with DISPLAY, PyGObject, pyatspi, xdotool and ImageMagick.
Native selected options and readable Info establish owner acceptance/rejection;
ComboBox string value is authored but this pinned Linux adapter may not expose
Text/Value interfaces. These checks do not establish speech or OS IME behavior.
"""
import argparse
import json
import hashlib
from pathlib import Path
import subprocess
import time

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', type=Path, default=Path('target/debug/kumo-gallery'))
parser.add_argument('--output', type=Path, default=Path('/tmp'))
parser.add_argument('--placement-only', action='store_true', help='Run collision, moving-anchor and constrained scroll checks only')
parser.add_argument('--reduce-motion', action='store_true', help='Request the gallery native reduced-motion preference')
parser.add_argument('--case', choices=['light-1040', 'dark-1040', 'dark-520', 'light-520'])
parser.add_argument('--require-expansion-state', action='store_true', help='Require authored Expandable/Expanded export (#41)')
args = parser.parse_args()
from gi.repository import Gio, GLib
import pyatspi
args.output.mkdir(parents=True, exist_ok=True)
(args.output / 'probe-environment.json').write_text(json.dumps({
    'binary': str(args.binary.resolve()),
    'sha256': hashlib.sha256(args.binary.read_bytes()).hexdigest(),
    'reduce_motion': args.reduce_motion,
    'head': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'working_tree': subprocess.check_output(['git', 'status', '--short'], text=True),
}, indent=2) + '\n')
bus = Gio.bus_get_sync(Gio.BusType.SESSION, None)
bus.call_sync('org.a11y.Bus', '/org/a11y/bus', 'org.freedesktop.DBus.Properties',
              'Set', GLib.Variant('(ssv)', ('org.a11y.Status', 'ScreenReaderEnabled',
                                          GLib.Variant('b', True))), None,
              Gio.DBusCallFlags.NONE, -1, None)
desktop = pyatspi.Registry.getDesktop(0)
app = subprocess.Popen([str(args.binary.resolve())] + (['--reduce-motion'] if args.reduce_motion else []),
                       stdout=open(args.output / 'dropdown-gallery.log', 'w'),
                       stderr=subprocess.STDOUT)
gallery = None
window = None


def wait_for(read, message):
    deadline = time.monotonic() + 5
    while True:
        if read():
            return
        if time.monotonic() >= deadline:
            raise AssertionError(message)
        time.sleep(.1)


def stable_read(read):
    # Owner/mode updates replace native accessible identities mid traversal.
    # Retry transport/lookup races only; never retry a contract assertion.
    def retry(*arguments):
        for attempt in range(10):
            try:
                return read(*arguments)
            except (GLib.GError, StopIteration):
                if attempt == 9:
                    raise
                time.sleep(.1)
        raise RuntimeError('unreachable')
    return retry


def expect_counter(expected):
    wait_for(lambda: counter() == expected, ('proposal count', expected, counter()))


def walk(node):
    node.clearCache()
    yield node
    for child in node:
        yield from walk(child)


@stable_read
def find(name, role=None):
    return next(n for n in walk(gallery) if n.name == name and
                (role is None or n.getRoleName() == role))


def page_number(index):
    return find(f'Dataset {index} page number', 'combo box')


def bounds(node):
    # Reacquire current identities after scroll/remount rather than retaining an
    # Accessible proxy whose bounds can refer to an earlier rendered subtree.
    node = find(node.name, node.getRoleName())
    r = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    return [r.x, r.y, r.width, r.height]


def show(node):
    for _ in range(25):
        x, y, w, h = bounds(node)
        if 200 <= y <= 500:
            time.sleep(.8)
            if 200 <= bounds(node)[1] <= 500:
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
    time.sleep(1.2)


def pointer(node):
    x, y, w, h = bounds(node)
    print('POINTER', node.name, [x, y, w, h], flush=True)
    subprocess.run(['xdotool', 'windowfocus', '--sync', window, 'mousemove', '--sync',
                    str(x + w // 2), str(y + h // 2)], check=True)
    # Allow hover-driven rendering to install the current hitboxes before down.
    time.sleep(.3)
    subprocess.run(['xdotool', 'click', '1'], check=True)
    time.sleep(.7)


def key(name):
    subprocess.run(['xdotool', 'key', name], check=True)
    time.sleep(.7)


@stable_read
def counter():
    return int(next(n.name.removeprefix('Page proposals: ') for n in walk(gallery)
                    if n.getRoleName() == 'label' and n.name.startswith('Page proposals: ')))


@stable_read
def is_open():
    return any(n.getRoleName() == 'list item' for n in walk(gallery))


def expansion(node, opened):
    node = find(node.name, node.getRoleName())
    node.clearCache()
    state = node.getState()
    actual = {'expandable': state.contains(pyatspi.STATE_EXPANDABLE),
              'expanded': state.contains(pyatspi.STATE_EXPANDED)}
    if args.require_expansion_state:
        wait_for(lambda: find(node.name, node.getRoleName()).getState().contains(pyatspi.STATE_EXPANDED) == opened, ('expansion publication', node.name, opened))
        node = find(node.name, node.getRoleName())
        node.clearCache()
        state = node.getState()
        actual = {'expandable': state.contains(pyatspi.STATE_EXPANDABLE), 'expanded': state.contains(pyatspi.STATE_EXPANDED)}
        assert actual == {'expandable': True, 'expanded': opened}, (node.name, actual, opened)
    return actual


@stable_read
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
                    str(args.output / f'dropdown-{theme.lower()}-{width}-{stage}.png')], check=True)



@stable_read
def focused(name):
    return any(n.name == name and n.getState().contains(pyatspi.STATE_FOCUSED)
               for n in walk(gallery))


@stable_read
def options():
    return [n.name for n in walk(gallery) if n.getRoleName() == 'list item']


def put_anchor(target):
    for _ in range(25):
        y = bounds(page_number(0))[1]
        if abs(y - target) <= 25:
            return
        subprocess.run(['xdotool', 'mousemove', '200', '300', 'click', '--repeat',
                        str(max(1, min(100, abs(y - target) // 42))), '--delay', '5',
                        '5' if y > target else '4'], check=True)
        time.sleep(.4)
    raise AssertionError(('anchor position', target, bounds(page_number(0))))


def placement_probe(theme, width):
    # The gallery starts with Pagination, so its first anchor cannot be moved
    # below the top-of-document position. Constrain the actual native viewport.
    subprocess.run(['xdotool', 'windowsize', window, str(width), '400'], check=True)
    time.sleep(1)
    put_anchor(210)
    trigger = bounds(page_number(0))
    original = counter()
    pointer(page_number(0))
    expansion(page_number(0), True)
    menu = bounds(find('Dataset 0 page number', 'list box'))
    assert menu[1] >= 8 and menu[1] + menu[3] <= 392, menu
    assert menu[1] < trigger[1], ('must flip above near lower edge', trigger, menu)
    capture(theme, width, 'collision')
    subprocess.run(['xdotool', 'mousemove', '200', '300', 'click', '--repeat', '2', '5'], check=True)
    time.sleep(1)
    moved = bounds(page_number(0))
    moved_menu = bounds(find('Dataset 0 page number', 'list box'))
    assert moved[1] != trigger[1], ('gallery must scroll', trigger, moved)
    above_gap = moved[1] - moved_menu[1] - moved_menu[3]
    below_gap = moved_menu[1] - moved[1] - moved[3]
    assert min(abs(above_gap - 11), abs(below_gap - 11)) <= 1, ('popup must follow anchor, allowing a side flip', trigger, menu, moved, moved_menu)
    assert moved_menu[1] >= 8 and moved_menu[1] + moved_menu[3] <= 392, moved_menu
    capture(theme, width, 'moving-anchor')
    key('End')
    last = bounds(find('10', 'list item'))
    assert 8 <= last[1] and last[1] + last[3] <= 392, ('last row must scroll into viewport', last)
    capture(theme, width, 'constrained-scroll')
    key('Return')
    expect_counter(original + 1)
    assert not is_open()
    subprocess.run(['xdotool', 'windowsize', window, str(width), '1000'], check=True)
    time.sleep(1)
    result = {'theme': theme, 'width': width, 'trigger': trigger, 'menu': menu, 'moved': moved, 'moved_menu': moved_menu, 'last': last}
    print('PLACEMENT PASS', json.dumps(result), flush=True)
    return result


try:
    time.sleep(3)
    gallery = next(n for n in desktop if n.get_process_id() == app.pid)
    window = subprocess.check_output(['xdotool', 'search', '--pid', str(app.pid),
                                     '--name', '^GPUI Kumo$'], text=True).strip().splitlines()[-1]
    subprocess.run(['xdotool', 'windowmove', window, '0', '0', 'windowfocus', window], check=True)
    results = []
    for theme, width in [('Light', 1040), ('Dark', 1040), ('Dark', 520), ('Light', 520)]:
        if args.case and args.case != f'{theme.lower()}-{width}':
            continue
        subprocess.run(['xdotool', 'mousemove', '200', '700', 'click', '--repeat',
                        '420', '--delay', '4', '4'], check=True)
        time.sleep(1)
        click(find(theme + ' appearance', 'button'))
        subprocess.run(['xdotool', 'windowsize', window, str(width), '1000'], check=True)
        time.sleep(1)
        show(find('Reset full dataset', 'button'))
        click(find('Reset full dataset', 'button'))
        if not any(n.name == 'Dataset 0 page number' and n.getRoleName() == 'combo box'
                   for n in walk(gallery)):
            show(find('Toggle input/dropdown', 'button'))
            click(find('Toggle input/dropdown', 'button'))
        if args.placement_only:
            results.append(placement_probe(theme, width))
            continue
        show(find('Dataset 8 page size café 🦀', 'combo box'))
        pointer(find('Dataset 8 page size café 🦀', 'combo box'))
        pointer(find('10', 'list item'))
        show(page_number(8))
        pointer(page_number(8))
        pointer(find('3', 'list item'))
        show(page_number(0))
        original = counter()
        pointer(page_number(0))
        selected(5)
        rows = {value: bounds(find(value, 'list item')) for value in options()}
        assert len(rows) == 10
        assert all(rect[3] == 33 for rect in rows.values()), rows
        assert bounds(page_number(0))[3] == 36
        assert expansion(page_number(0), True) == {'expandable': True, 'expanded': True}
        capture(theme, width, 'open-hover')
        pointer(find('3', 'list item'))
        expect_counter(original + 1)
        assert counter() == original + 1
        assert focused('Dataset 0 page number')
        key('space')
        selected(3)
        key('Down')
        key('Return')
        expect_counter(original + 2)
        assert counter() == original + 2
        key('Return')
        selected(4)
        key('Return')
        expect_counter(original + 2)
        assert counter() == original + 2
        key('space')
        key('Escape')
        assert focused('Dataset 0 page number')
        key('space')
        key('Tab')
        assert not is_open()
        assert focused('Next page')
        pointer(page_number(0))
        key('shift+Tab')
        assert not is_open()
        assert focused('Previous page')
        pointer(page_number(0))
        show(find('Toggle input/dropdown', 'button'))
        click(find('Toggle input/dropdown', 'button'))
        assert not is_open()
        assert find('Dataset 0 page number', 'entry').queryText().getText(0, -1) == '4'
        show(find('Toggle input/dropdown', 'button'))
        click(find('Toggle input/dropdown', 'button'))
        show(page_number(0))
        pointer(page_number(0))
        selected(4)
        key('Escape')
        show(find('Toggle full availability', 'button'))
        click(find('Toggle full availability', 'button'))
        disabled = page_number(0)
        assert not disabled.getState().contains(pyatspi.STATE_ENABLED)
        assert not disabled.getState().contains(pyatspi.STATE_SENSITIVE)
        assert not has_click(disabled)
        show(disabled)
        pointer(disabled)
        key('space')
        key('Return')
        assert not is_open()
        expect_counter(original + 2)
        assert counter() == original + 2
        show(find('Toggle full availability', 'button'))
        click(find('Toggle full availability', 'button'))
        show(page_number(0))
        wait_for(lambda: page_number(0).getState().contains(pyatspi.STATE_ENABLED), 'restored trigger publication')
        assert page_number(0).getState().contains(pyatspi.STATE_SENSITIVE)
        assert has_click(page_number(0))
        show(page_number(8))
        pointer(page_number(8))
        selected(3)
        pointer(find('4', 'list item'))
        expect_counter(original + 3)
        accepted = counter()
        assert accepted == original + 3, (original, accepted)
        assert focused('Dataset 8 page number')
        show(page_number(9))
        pointer(page_number(9))
        selected(3)
        pointer(find('4', 'list item'))
        expect_counter(accepted + 1)
        assert counter() == accepted + 1
        pointer(page_number(9))
        selected(3)
        key('Escape')
        capture(theme, width, 'rejected')
        size = find('Dataset 8 page size café 🦀', 'combo box')
        show(size)
        pointer(size)
        pointer(find('20', 'list item'))
        show(page_number(8))
        pointer(page_number(8))
        assert options() == ['1', '2', '3', '4', '5'], options()
        key('Escape')
        pointer(find('Dataset 8 page size café 🦀', 'combo box'))
        pointer(find('50', 'list item'))
        pointer(page_number(8))
        assert options() == ['1', '2'], options()
        selected(2)
        key('Escape')
        capture(theme, width, 'page-size')
        results.append({'theme': theme, 'width': width, 'rows': rows,
                        'proposals': counter(), 'checks': 'pointer/Space/Enter once, same-value no-op, rejection, Escape/Tab/Shift-Tab focus, mode/availability, PageSize option changes'})
        print(json.dumps(results[-1]), flush=True)
        placement_probe(theme, width)
    (args.output / ('placement-results.json' if args.placement_only else 'dropdown-results.json')).write_text(json.dumps(results, indent=2) + '\n')
except Exception:
    if window:
        capture('failure', 0, 'failure')
    raise
finally:
    app.terminate()
    app.wait(timeout=10)
