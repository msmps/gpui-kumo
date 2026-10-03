import os, subprocess, time, json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo')
out=root/'docs/evidence/tabs/overflow-motion/linux'
def children(node):
    node.clearCache()
    return [node.getChildAtIndex(i) for i in range(node.childCount)]
def walk(node):
    yield node
    for child in children(node):
        yield from walk(child)
def named(node, name, role):
    return next(x for x in walk(node) if x.name==name and x.getRole()==role)
def state(node, value):
    node.clearCache(); return node.getState().contains(value)
def wait(test):
    for _ in range(50):
        if test(): return
        time.sleep(.1)
    raise AssertionError('native state did not reach expected value')
results=[]
for dark in (False,True):
    for width in (1040,520):
        name=('dark' if dark else 'light')+'-'+str(width)
        with (out/(name+'.log')).open('w') as log:
            p=subprocess.Popen([str(root/'target/debug/examples/tabs'),f'--width={width}']+(['--dark'] if dark else []),stdout=log,stderr=subprocess.STDOUT)
            try:
                for _ in range(100):
                    if p.poll() is not None: raise RuntimeError(f'preview exited: {p.returncode}')
                    ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
                    if ids: break
                    time.sleep(.1)
                else: raise RuntimeError('no window')
                win=ids[-1]
                subprocess.run(['xdotool','windowmove',win,'0','0'],check=True)
                subprocess.run(['xdotool','windowfocus','--sync',win],check=True)
                time.sleep(1)
                desktop=pyatspi.Registry.getDesktop(0)
                app=next(x for x in children(desktop) if x.get_process_id()==p.pid)
                first=named(app,'Project sections',pyatspi.ROLE_PAGE_TAB_LIST)
                compact=named(app,'Compact project sections',pyatspi.ROLE_PAGE_TAB_LIST)
                overview=named(first,'Overview',pyatspi.ROLE_PAGE_TAB)
                metrics=named(first,'Metrics',pyatspi.ROLE_PAGE_TAB)
                disabled=named(first,'Unavailable',pyatspi.ROLE_PAGE_TAB)
                assert not state(disabled,pyatspi.STATE_SENSITIVE)
                rect=overview.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                print(name, 'pointer', rect, subprocess.check_output(['xdotool','getwindowgeometry','--shell',win],text=True), flush=True)
                subprocess.run(['import','-window',win,str(out/(name+'-before.png'))],check=True)
                subprocess.run(['xdotool','mousemove',str(rect.x+rect.width//2),str(rect.y+rect.height//2),'click','1'],check=True)
                wait(lambda: state(overview,pyatspi.STATE_FOCUSED))
                subprocess.run(['import','-window',win,str(out/(name+'-initial.png'))],check=True)
                subprocess.run(['xdotool','key','Right'],check=True)
                wait(lambda: state(metrics,pyatspi.STATE_FOCUSED))
                assert state(overview,pyatspi.STATE_SELECTED)
                assert not state(metrics,pyatspi.STATE_SELECTED)
                subprocess.run(['xdotool','key','space'],check=True)
                wait(lambda: state(metrics,pyatspi.STATE_SELECTED))
                subprocess.run(['xdotool','key','Tab'],check=True)
                compact_overview=named(compact,'Overview',pyatspi.ROLE_PAGE_TAB)
                wait(lambda: state(compact_overview,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','shift+Tab'],check=True)
                wait(lambda: state(metrics,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','End'],check=True)
                settings=named(first,'Settings café 🦀',pyatspi.ROLE_PAGE_TAB)
                wait(lambda: state(settings,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','Return'],check=True)
                wait(lambda: state(settings,pyatspi.STATE_SELECTED))
                row_bounds=[]
                for title, height in [('Compact project sections',26),('Underlined project sections',30),('Compact underlined sections',26)]:
                    row=named(app,title,pyatspi.ROLE_PAGE_TAB_LIST)
                    tab=named(row,'Overview',pyatspi.ROLE_PAGE_TAB)
                    rect=tab.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                    subprocess.run(['xdotool','mousemove',str(rect.x+rect.width//2),str(rect.y+rect.height//2),'click','1'],check=True)
                    wait(lambda: state(tab,pyatspi.STATE_FOCUSED))
                    subprocess.run(['xdotool','key','End'],check=True)
                    selected=named(row,'Settings café 🦀',pyatspi.ROLE_PAGE_TAB)
                    wait(lambda: state(selected,pyatspi.STATE_FOCUSED))
                    subprocess.run(['xdotool','key','Return'],check=True)
                    wait(lambda: state(selected,pyatspi.STATE_SELECTED))
                    a=row.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                    b=selected.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                    assert a.height==height, (title,a)
                    assert b.x>=a.x and b.x+b.width<=a.x+a.width, (title,a,b)
                    row_bounds.append({'name':title,'list':str(a),'selected':str(b)})
                subprocess.run(['import','-window',win,str(out/(name+'-selected.png'))],check=True)
                overflow=named(app,'Overflow project sections',pyatspi.ROLE_PAGE_TAB_LIST)
                edge=named(app,'Scroll tabs right',pyatspi.ROLE_PUSH_BUTTON)
                rect=edge.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                assert rect.width==40, rect
                subprocess.run(['xdotool','mousemove',str(rect.x+rect.width//2),str(rect.y+rect.height//2),'click','1'],check=True)
                wait(lambda: state(named(app,'Scroll tabs right',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
                time.sleep(.3)
                overview_overflow=named(overflow,'Overview',pyatspi.ROLE_PAGE_TAB)
                assert state(overview_overflow,pyatspi.STATE_SELECTED)
                subprocess.run(['xdotool','key','space'],check=True)
                time.sleep(.3)
                assert state(named(app,'Scroll tabs right',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED)
                assert state(overview_overflow,pyatspi.STATE_SELECTED)
                viewport=overflow.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                tabs_now=[x for x in children(overflow) if x.getRole()==pyatspi.ROLE_PAGE_TAB]
                marker=tabs_now[4]
                start=marker.queryComponent().getExtents(pyatspi.DESKTOP_COORDS).x
                subprocess.run(['xdotool','mousemove',str(viewport.x+125),str(viewport.y+viewport.height//2),'mousedown','1','mousemove',str(viewport.x+95),str(viewport.y+viewport.height//2)],check=True)
                time.sleep(.1)
                subprocess.run(['xdotool','mouseup','1'],check=True)
                time.sleep(.1)
                assert marker.queryComponent().getExtents(pyatspi.DESKTOP_COORDS).x < start
                assert state(overview_overflow,pyatspi.STATE_SELECTED)
                # Refocus the action before testing owner removal.
                edge=named(app,'Scroll tabs right',pyatspi.ROLE_PUSH_BUTTON)
                edge.queryComponent().grabFocus()
                wait(lambda: state(edge,pyatspi.STATE_FOCUSED))
                subprocess.run(['import','-window',win,str(out/(name+'-overflow.png'))],check=True)
                subprocess.run(['xdotool','key','alt+t'],check=True)
                wait(lambda: state(named(overflow,'Overview',pyatspi.ROLE_PAGE_TAB),pyatspi.STATE_FOCUSED))
                assert len(children(overflow))==2, len(children(overflow))
                subprocess.run(['xdotool','key','Tab'],check=True)
                toggle=named(app,'Toggle extra tabs',pyatspi.ROLE_PUSH_BUTTON)
                wait(lambda: state(toggle,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','Tab'],check=True)
                after=named(app,'After tabs',pyatspi.ROLE_PUSH_BUTTON)
                wait(lambda: state(after,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','shift+Tab'],check=True)
                wait(lambda: state(toggle,pyatspi.STATE_FOCUSED))
                subprocess.run(['xdotool','key','shift+Tab'],check=True)
                wait(lambda: state(named(overflow,'Overview',pyatspi.ROLE_PAGE_TAB),pyatspi.STATE_FOCUSED))
                results.append({'case':name,'result':'PASS','list_bounds':str(first.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)), 'actions':['pointer focus','manual Right','Space select','Tab exit','ShiftTab return','End skip disabled/reveal','Enter select','edge pointer focus','edge Space scroll without selection','native drag release suppresses selection','focused edge owner removal recovery','Tab exit after removal'],'disabled_sensitive':False, 'other_rows':row_bounds})
                print(name,'PASS',flush=True)
            finally:
                if p.poll() is None:
                    subprocess.run(['import','-window',win,str(out/(name+'-last.png'))],check=False)
                p.terminate(); p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
