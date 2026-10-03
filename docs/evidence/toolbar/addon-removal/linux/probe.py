import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toolbar/addon-removal/linux'
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
        filter.queryComponent().grabFocus();wait(lambda:state(filter,pyatspi.STATE_FOCUSED));key('Tab');wait(lambda:state(clear,pyatspi.STATE_FOCUSED));time.sleep(.2)
        click(clear);wait(lambda:state(clear,pyatspi.STATE_FOCUSED));wait(lambda:value(filter)=='');wait(lambda:any(x.name=='1 filter clears'for x in walk(app)))
        key('space');wait(lambda:any(x.name=='2 filter clears'for x in walk(app)));key('Return');wait(lambda:any(x.name=='3 filter clears'for x in walk(app)))
        filter.queryComponent().grabFocus();wait(lambda:state(filter,pyatspi.STATE_FOCUSED));subprocess.run(['xclip','-selection','clipboard'],input='retained 🦀'.encode(),check=True);key('ctrl+v');wait(lambda:value(filter)=='retained 🦀');key('End');key('shift+Left');wait(lambda:selection(filter)=='🦀');key('Tab');wait(lambda:state(clear,pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-focused.png'))],check=True)
        key('alt+c');wait(lambda:state(filter,pyatspi.STATE_FOCUSED));assert value(filter)=='retained 🦀';assert selection(filter)=='🦀';assert not any(x.name=='Clear'for x in walk(primary))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-removed.png'))],check=True)
        vertical=named(app,'Vertical editing toolbar',pyatspi.ROLE_TOOL_BAR);vbefore=named(vertical,'Before',pyatspi.ROLE_PUSH_BUTTON)
        key('Tab');wait(lambda:state(vbefore,pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(filter,pyatspi.STATE_FOCUSED));assert selection(filter)=='🦀'
        key('alt+c');wait(lambda:any(x.name=='Clear'for x in walk(primary)));clear=named(primary,'Clear',pyatspi.ROLE_PUSH_BUTTON);key('Tab');wait(lambda:state(clear,pyatspi.STATE_FOCUSED));key('space');wait(lambda:any(x.name=='4 filter clears'for x in walk(app)));wait(lambda:value(filter)=='')
        key('alt+d');wait(lambda:not state(filter,pyatspi.STATE_SENSITIVE));key('alt+c');wait(lambda:state(filter,pyatspi.STATE_FOCUSED));text('blocked');assert value(filter)=='';assert not any(x.name=='Clear'for x in walk(primary))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-disabled-removed.png'))],check=True)
        key('Tab');wait(lambda:state(vbefore,pyatspi.STATE_FOCUSED))
        results.append({'case':case,'result':'PASS','removedFocus':'retained Filter query','removedSelection':'🦀','removalTab':'Vertical toolbar Before','disabledRemovedFocus':'retained unavailable Filter query','actions':['pointer focus before once-only Space/Enter','Unicode selection/value retained','independent action removal and editor recovery','Tab exit/remembered editor reentry','reinserted keyboard activation once','root-disabled editor recovery/edit guard']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
