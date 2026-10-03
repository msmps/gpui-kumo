import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toolbar/editors/linux'
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
    subprocess.run(['xdotool','mousemove',str(r.x+r.width//2),str(r.y+r.height//2),'click','1'],check=True)
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
        geometry=[]
        for node in walk(primary):
            if node.getRole()in(pyatspi.ROLE_PUSH_BUTTON,pyatspi.ROLE_ENTRY):
                r=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);assert r.height==36,(node.name,r);geometry.append({'name':node.name,'bounds':str(r)})
        assert value(q)=='café 🦀';assert not state(paused,pyatspi.STATE_SENSITIVE)
        subprocess.run(['import','-window',win,str(out/(case+'-initial.png'))],check=True)
        click(before);key('Right');wait(lambda:state(q,pyatspi.STATE_FOCUSED));key('Home');key('Right');assert state(q,pyatspi.STATE_FOCUSED)
        key('End');key('shift+Left');wait(lambda:selection(q)=='🦀');key('Right');assert state(q,pyatspi.STATE_FOCUSED);key('Right');wait(lambda:state(paused,pyatspi.STATE_FOCUSED));text('blocked');key('ctrl+a');assert value(paused)=='locked';assert selection(paused)==''
        key('Right');wait(lambda:state(filter,pyatspi.STATE_FOCUSED));r=filter.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);assert r.x>=57 and r.x+r.width<=width-57,(case,r)
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-focused.png'))],check=True)
        key('Home');key('Left');wait(lambda:state(paused,pyatspi.STATE_FOCUSED));key('Left');wait(lambda:state(q,pyatspi.STATE_FOCUSED));key('Home');key('Left');wait(lambda:state(before,pyatspi.STATE_FOCUSED));key('Right');wait(lambda:state(q,pyatspi.STATE_FOCUSED))
        vertical=named(app,'Vertical editing toolbar',pyatspi.ROLE_TOOL_BAR);assert state(vertical,pyatspi.STATE_VERTICAL)
        vbefore=named(vertical,'Before',pyatspi.ROLE_PUSH_BUTTON);vq=input(vertical,'Query');vp=input(vertical,'Paused query')
        key('Tab');wait(lambda:state(vbefore,pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(q,pyatspi.STATE_FOCUSED))
        key('End');text('!');wait(lambda:value(q)=='café 🦀!');key('Return');wait(lambda:any(x.name=='1 query changes · 1 query submits'for x in walk(app)))
        key('alt+r');wait(lambda:not state(q,pyatspi.STATE_EDITABLE));text('blocked');assert value(q)=='café 🦀!';key('alt+r');wait(lambda:state(q,pyatspi.STATE_EDITABLE))
        key('alt+d');wait(lambda:not state(q,pyatspi.STATE_SENSITIVE));key('shift+Left');key('ctrl+a');text('blocked');assert value(q)=='café 🦀!';assert selection(q)=='';key('alt+d');wait(lambda:state(q,pyatspi.STATE_SENSITIVE))
        vbefore.queryComponent().grabFocus();wait(lambda:state(vbefore,pyatspi.STATE_FOCUSED));key('Down');wait(lambda:state(vq,pyatspi.STATE_FOCUSED));key('Home');key('Down');assert state(vq,pyatspi.STATE_FOCUSED);key('End');key('Down');wait(lambda:state(vp,pyatspi.STATE_FOCUSED))
        filter.queryComponent().grabFocus();wait(lambda:state(filter,pyatspi.STATE_FOCUSED));key('alt+t');wait(lambda:state(before,pyatspi.STATE_FOCUSED));time.sleep(.2);key('Tab');wait(lambda:state(named(named(app,'Vertical editing toolbar',pyatspi.ROLE_TOOL_BAR),'Paused query',pyatspi.ROLE_ENTRY),pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(before,pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-removed.png'))],check=True)
        key('alt+t');time.sleep(.2);key('Right');wait(lambda:state(input(named(app,'Editing toolbar',pyatspi.ROLE_TOOL_BAR),'Query'),pyatspi.STATE_FOCUSED));assert value(input(named(app,'Editing toolbar',pyatspi.ROLE_TOOL_BAR),'Query'))=='café 🦀!'
        results.append({'case':case,'result':'PASS','controls':geometry,'actions':['unicode selection and caret edges','disabled focus without editing/selection','skip unavailable','grouped editor narrow reveal','one Tab exit/last reentry','Change/Submit once','read-only owner update','root disabled without owner mutation','vertical caret boundaries','focused removal/traversal','retained value after reinsertion']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
