import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toolbar/addon-actions/linux'
def children(n):
    n.clearCache();return[n.getChildAtIndex(i)for i in range(n.childCount)]
def walk(n):
    yield n
    for c in children(n):yield from walk(c)
def named(n,name,role):return next(x for x in walk(n)if x.name==name and x.getRole()==role)
def state(n,s):n.clearCache();return n.getState().contains(s)
def wait(test):
    for _ in range(60):
        if test():return
        time.sleep(.1)
    raise AssertionError('native state did not reach expected value')
def key(k):subprocess.run(['xdotool','key',k],check=True)
def text(s):subprocess.run(['xdotool','type','--clearmodifiers',s],check=True)
def click(n):
    r=n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    subprocess.run(['xdotool','mousemove',str(r.x+r.width//2),str(r.y+r.height//2)],check=True)
    time.sleep(.15)
    subprocess.run(['xdotool','mousedown','1'],check=True)
    time.sleep(.1)
    subprocess.run(['xdotool','mouseup','1'],check=True)
def value(n):n.clearCache();return n.queryText().getText(0,-1)
def selection(n):
    t=n.queryText();return t.getText(*t.getSelection(0))if t.getNSelections()else''
results=[]
for dark in(False,True):
 for width in(1040,520):
    case=('dark'if dark else'light')+'-'+str(width)
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/toolbar_editors'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
      try:
        for _ in range(100):
            ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
            if ids:break
            if p.poll()is not None:raise RuntimeError('preview exited')
            time.sleep(.1)
        else:raise RuntimeError('no window')
        win=ids[-1];subprocess.run(['xdotool','windowmove',win,'0','0','windowfocus','--sync',win],check=True);time.sleep(.5)
        app=next(x for x in children(pyatspi.Registry.getDesktop(0))if x.get_process_id()==p.pid)
        primary=named(app,'Editing toolbar',pyatspi.ROLE_TOOL_BAR)
        def input(n,name):return named(n,name,pyatspi.ROLE_ENTRY)
        q=input(primary,'Query');paused=input(primary,'Paused query');filter=input(primary,'Filter query');before=named(primary,'Before',pyatspi.ROLE_PUSH_BUTTON)
        clear=named(primary,'Clear',pyatspi.ROLE_PUSH_BUTTON)
        before.queryComponent().grabFocus();wait(lambda:state(before,pyatspi.STATE_FOCUSED));key('Right');wait(lambda:state(q,pyatspi.STATE_FOCUSED));key('End');key('Right');wait(lambda:state(paused,pyatspi.STATE_FOCUSED));key('Right');wait(lambda:state(filter,pyatspi.STATE_FOCUSED));key('Tab');wait(lambda:state(clear,pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-initial.png'))],check=True)
        r=clear.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);assert r.height==26,(case,r);assert r.x>=57 and r.x+r.width<=width-57,(case,r)
        click(clear);wait(lambda:state(clear,pyatspi.STATE_FOCUSED));wait(lambda:value(filter)=='');time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-cleared.png'))],check=True);wait(lambda:any(x.name=='1 filter clears'for x in walk(app)));key('space');wait(lambda:any(x.name=='2 filter clears'for x in walk(app)));key('Return');wait(lambda:any(x.name=='3 filter clears'for x in walk(app)));assert state(clear,pyatspi.STATE_FOCUSED)
        key('Right');after=named(primary,'After',pyatspi.ROLE_PUSH_BUTTON);wait(lambda:state(after,pyatspi.STATE_FOCUSED))
        key('alt+d');wait(lambda:not state(filter,pyatspi.STATE_SENSITIVE));assert state(clear,pyatspi.STATE_SENSITIVE)
        click(clear);wait(lambda:state(clear,pyatspi.STATE_FOCUSED));wait(lambda:any(x.name=='4 filter clears'for x in walk(app)));key('space');wait(lambda:any(x.name=='5 filter clears'for x in walk(app)));key('Return');wait(lambda:any(x.name=='6 filter clears'for x in walk(app)));assert state(clear,pyatspi.STATE_FOCUSED)
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-focused.png'))],check=True)
        key('alt+d');key('alt+g');wait(lambda:not state(clear,pyatspi.STATE_SENSITIVE));wait(lambda:state(after,pyatspi.STATE_FOCUSED));before.queryComponent().grabFocus();wait(lambda:state(before,pyatspi.STATE_FOCUSED))
        key('Right');wait(lambda:state(q,pyatspi.STATE_FOCUSED));key('End');key('Right');wait(lambda:state(paused,pyatspi.STATE_FOCUSED));key('Right');wait(lambda:state(after,pyatspi.STATE_FOCUSED));key('alt+g')
        filter.queryComponent().grabFocus();wait(lambda:state(filter,pyatspi.STATE_FOCUSED));key('Tab');wait(lambda:state(clear,pyatspi.STATE_FOCUSED));key('alt+t');wait(lambda:state(before,pyatspi.STATE_FOCUSED));key('Tab');vertical=named(app,'Vertical editing toolbar',pyatspi.ROLE_TOOL_BAR);wait(lambda:state(named(vertical,'Before',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-removed.png'))],check=True)
        results.append({'case':case,'result':'PASS','actionBounds':str(r),'actions':['independent Tab target','pointer focus before Space/Enter','root disabled editor leaves addon enabled and sensitive','owner-disabled group editor and action','owner-disabled group skipped','arrow from action uses remembered composite entry','focused group removal and Tab exit']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
