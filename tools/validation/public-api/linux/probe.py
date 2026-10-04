"""Linux public-API regression; evidence stays in --output (outside the checkout).
Run in an accessibility-enabled D-Bus session with DISPLAY and a supported GPUI GPU.
"""
import argparse
import json
import subprocess
import time
from pathlib import Path
import pyatspi

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--binary', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
args = parser.parse_args()
args.output.mkdir(parents=True, exist_ok=True)


def children(node):
    node.clearCache()
    return [child for child in node if child is not None]


def walk(node):
    yield node
    for child in children(node):
        yield from walk(child)


def wait(predicate):
    for _ in range(80):
        try:
            if predicate():
                return
        except (StopIteration, RuntimeError):
            pass
        time.sleep(.1)
    raise AssertionError('native state did not reach the expected value')


def key(keys):
    subprocess.run(['xdotool', 'key', '--clearmodifiers', keys], check=True)


def click(node):
    rect = node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    subprocess.run(['xdotool', 'mousemove', str(rect.x + rect.width // 2),
                    str(rect.y + rect.height // 2)], check=True)
    time.sleep(.1)
    subprocess.run(['xdotool', 'click', '1'], check=True)


def focused(node):
    node.clearCache()
    return node.getState().contains(pyatspi.STATE_FOCUSED)


results = []
for dark in (False, True):
    for width in (1040, 520):
        case = ('dark' if dark else 'light') + '-' + str(width)
        with (args.output / (case + '.log')).open('w') as log:
            process = subprocess.Popen([str(args.binary.resolve()), f'--width={width}'] +
                                       (['--dark'] if dark else []), stdout=log, stderr=log)
            try:
                wait(lambda: any(app.get_process_id() == process.pid
                                 for app in children(pyatspi.Registry.getDesktop(0))))
                app = next(app for app in children(pyatspi.Registry.getDesktop(0))
                           if app.get_process_id() == process.pid)
                ids = subprocess.check_output(['xdotool', 'search', '--onlyvisible',
                                               '--pid', str(process.pid)], text=True).splitlines()
                win = ids[-1]
                subprocess.run(['xdotool', 'windowmove', win, '0', '0',
                                'windowfocus', '--sync', win], check=True)

                def node(name, role=None):
                    return next(n for n in walk(app) if n.name == name and
                                (role is None or n.getRole() == role))

                def exists(name, role=None):
                    return any(n.name == name and (role is None or n.getRole() == role)
                               for n in walk(app))

                def button(name):
                    return node(name, pyatspi.ROLE_PUSH_BUTTON)

                def count(number):
                    return exists(f'Activations: {number}')

                wait(lambda: exists('Project name', pyatspi.ROLE_ENTRY))
                for name, role in [('Delete project Foo', pyatspi.ROLE_PUSH_BUTTON),
                                   ('Open settings', pyatspi.ROLE_PUSH_BUTTON),
                                   ('Endpoint', pyatspi.ROLE_ENTRY),
                                   ('Notes', pyatspi.ROLE_ENTRY),
                                   ('API key', pyatspi.ROLE_PASSWORD_TEXT),
                                   ('Region', pyatspi.ROLE_COMBO_BOX)]:
                    assert exists(name, role), name
                assert exists('Layout preference', pyatspi.ROLE_PANEL)
                assert exists('Compact layout', pyatspi.ROLE_RADIO_BUTTON)
                geometry = {}
                for name in ('Save', 'Delete project Foo', 'Open settings'):
                    r = button(name).queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                    geometry[name] = dict(x=r.x, y=r.y, width=r.width, height=r.height)
                assert len({r['y'] + r['height'] / 2 for r in geometry.values()}) == 1
                subprocess.run(['import', '-window', win,
                                str(args.output / (case + '-default.png'))], check=True)
                click(button('Save'))
                wait(lambda: count(1) and focused(button('Save')))
                key('space')
                wait(lambda: count(2))
                key('Return')
                wait(lambda: count(3))
                button('Save').queryAction().doAction(0)
                wait(lambda: count(4))
                # Label pointer association uses the retained control focus target.
                click(node('Endpoint', pyatspi.ROLE_LABEL))
                wait(lambda: focused(node('Endpoint', pyatspi.ROLE_ENTRY)))
                editor = node('Project name', pyatspi.ROLE_ENTRY)
                click(editor)
                subprocess.run(['xdotool', 'type', '--clearmodifiers', 'draft'], check=True)
                wait(lambda: editor.queryText().getText(0, -1) == 'draft')
                key('ctrl+a')
                key('F6')
                wait(lambda: exists('Nom du projet', pyatspi.ROLE_ENTRY))
                editor = node('Nom du projet', pyatspi.ROLE_ENTRY)
                assert focused(editor)
                assert editor.queryText().getText(0, -1) == 'draft'
                assert editor.queryText().getSelection(0) == (0, 5)
                for name, role in [('Point de terminaison', pyatspi.ROLE_ENTRY),
                                   ('Remarques', pyatspi.ROLE_ENTRY),
                                   ('Clé API', pyatspi.ROLE_PASSWORD_TEXT),
                                   ('Région', pyatspi.ROLE_COMBO_BOX)]:
                    assert exists(name, role), name
                key('ctrl+z')
                wait(lambda: editor.queryText().getText(0, -1) == '')
                key('F8')
                wait(lambda: not exists('Nom du projet (optional)', pyatspi.ROLE_LABEL))
                assert exists('Nom du projet', pyatspi.ROLE_ENTRY)
                assert exists('Point de terminaison', pyatspi.ROLE_ENTRY)
                key('F8')
                wait(lambda: exists('Point de terminaison', pyatspi.ROLE_LABEL))
                # Open Select keeps its popup and current value while renaming.
                click(node('Région', pyatspi.ROLE_COMBO_BOX))
                wait(lambda: exists('Europe'))
                key('F6')
                wait(lambda: exists('Region', pyatspi.ROLE_COMBO_BOX))
                assert exists('Europe')
                key('Escape')
                # Named overlays refresh native surface names without remounting.
                click(button('Open popover'))
                wait(lambda: exists('Project settings'))
                key('F6')
                wait(lambda: exists('Paramètres du projet'))
                assert exists('Retained popup content')
                subprocess.run(['import', '-window', win,
                                str(args.output / (case + '-popover.png'))], check=True)
                click(button('Close popover'))
                wait(lambda: not exists('Paramètres du projet'))
                click(button('Open dialog'))
                wait(lambda: exists('Modifier le projet', pyatspi.ROLE_DIALOG))
                click(button('Rename record'))
                wait(lambda: exists('Edit project', pyatspi.ROLE_DIALOG))
                assert focused(button('Rename record'))
                subprocess.run(['import', '-window', win,
                                str(args.output / (case + '-dialog.png'))], check=True)
                key('Escape')
                wait(lambda: not exists('Edit project', pyatspi.ROLE_DIALOG))
                click(button('Actions'))
                wait(lambda: exists('Project actions', pyatspi.ROLE_MENU))
                assert exists('Archive current project', pyatspi.ROLE_MENU_ITEM)
                key('F6')
                wait(lambda: exists('Actions du projet', pyatspi.ROLE_MENU))
                assert exists('Archive current project', pyatspi.ROLE_MENU_ITEM)
                key('Escape')
                wait(lambda: not exists('Actions du projet', pyatspi.ROLE_MENU))
                key('F6')
                wait(lambda: exists('Endpoint', pyatspi.ROLE_ENTRY))
                click(button('Save'))
                wait(lambda: count(5))
                key('F10')
                wait(lambda: not button('Save').getState().contains(pyatspi.STATE_FOCUSABLE))
                click(button('Save'))
                key('space')
                key('Return')
                assert button('Delete project Foo').getState().contains(pyatspi.STATE_BUSY)
                click(button('Delete project Foo'))
                key('space')
                key('Return')
                try:
                    assert button('Save').queryAction().nActions == 0
                except NotImplementedError:
                    pass  # Adapter may omit the Action interface entirely.
                time.sleep(.2)
                assert count(5)
                # Restore focus inside the fixture after the disabled press blurred it.
                click(button('Open settings'))
                key('F10')
                wait(lambda: button('Save').getState().contains(pyatspi.STATE_FOCUSABLE))
                click(button('Save'))
                wait(lambda: count(6) and focused(button('Save')))
                key('Tab')
                wait(lambda: any(focused(n) for n in walk(app)
                                 if n.getRole() in (pyatspi.ROLE_PUSH_BUTTON, pyatspi.ROLE_ENTRY)))

                click(button('Check endpoint'))
                wait(lambda: exists('Checks: 1') and focused(button('Check endpoint')))
                key('space')
                wait(lambda: exists('Checks: 2') and focused(button('Check endpoint')))
                key('Return')
                wait(lambda: exists('Checks: 3'))
                button('Check endpoint').queryAction().doAction(0)
                wait(lambda: exists('Checks: 4'))
                key('F11')
                wait(lambda: not exists('Check endpoint') and
                     focused(node('Endpoint', pyatspi.ROLE_ENTRY)))
                key('Tab')
                wait(lambda: focused(node('Notes', pyatspi.ROLE_ENTRY)))
                subprocess.run(['xdotool', 'windowsize', win, '680', '850'], check=True)
                time.sleep(.4)
                key('F9')
                time.sleep(.4)
                subprocess.run(['import', '-window', win,
                                str(args.output / (case + '-resized-opposite.png'))], check=True)
                results.append(dict(case=case, result='PASS', geometry=geometry,
                                    checks=['native default/override/icon-only/hidden names',
                                            'Field pointer focus association',
                                            'retained Unicode names, value, selection and undo',
                                            'Select/Dropdown/Popover/Dialog remain open through naming',
                                            'pointer/Space/Enter/AT-SPI activation exactly once',
                                            'disabled actions guarded and traversal resumes',
                                            'floating action pointer focus, keyboard/AT-SPI and focused removal/Tab exit',
                                            'both-theme wide/narrow/resized screenshots']))
                print(case, 'PASS', flush=True)
            finally:
                process.terminate()
                process.wait(timeout=10)
(args.output / 'results.json').write_text(json.dumps(results, indent=2) + '\n')
