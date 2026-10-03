import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toolbar/linux'
def children(n):
    n.clearCache();return[n.getChildAtIndex(i) for i in range(n.childCount)]
def walk(n):
    yield n
    for c in children(n):yield from walk(c)
def named(n,name,role):return next(x for x in walk(n) if x.name==name and x.getRole()==role)
def state(n,s):n.clearCache();return n.getState().contains(s)
def wait(test):
    for _ in range(60):
        if test():return
        time.sleep(.1)
    raise AssertionError('native state did not reach expected value')
def key(k):subprocess.run(['xdotool','key',k],check=True)
def click(n):
    r=n.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
    subprocess.run(['xdotool','mousemove',str(r.x+r.width//2),str(r.y+r.height//2),'click','1'],check=True)
results=[]
for dark in(False,True):
 for width in(1040,520):
    case=('dark' if dark else 'light')+'-'+str(width)
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/toolbar'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
      try:
        for _ in range(100):
            ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
            if ids:break
            if p.poll()is not None:raise RuntimeError('preview exited')
            time.sleep(.1)
        else:raise RuntimeError('no window')
        win=ids[-1];subprocess.run(['xdotool','windowmove',win,'0','0','windowfocus','--sync',win],check=True);time.sleep(.5)
        desktop=pyatspi.Registry.getDesktop(0);app=next(x for x in children(desktop)if x.get_process_id()==p.pid)
        primary=named(app,'Actions toolbar',pyatspi.ROLE_TOOL_BAR)
        refresh=named(primary,'Refresh',pyatspi.ROLE_PUSH_BUTTON);paused=named(primary,'Paused',pyatspi.ROLE_PUSH_BUTTON);saving=named(primary,'Saving',pyatspi.ROLE_PUSH_BUTTON);docs=named(primary,'Documentation',pyatspi.ROLE_LINK);settings=named(primary,'Settings',pyatspi.ROLE_PUSH_BUTTON);deploy=named(primary,'Deploy',pyatspi.ROLE_PUSH_BUTTON)
        geometry=[]
        for node in children(primary):
            if node.getRole()in(pyatspi.ROLE_PUSH_BUTTON,pyatspi.ROLE_LINK):
                r=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);assert r.height==36,(node.name,r);geometry.append({'name':node.name,'bounds':str(r)})
        assert not state(paused,pyatspi.STATE_SENSITIVE);assert state(saving,pyatspi.STATE_BUSY)
        subprocess.run(['import','-window',win,str(out/(case+'-initial.png'))],check=True)
        click(refresh);wait(lambda:state(refresh,pyatspi.STATE_FOCUSED));key('space');key('Right');wait(lambda:state(paused,pyatspi.STATE_FOCUSED));key('space');key('Return');key('Right');wait(lambda:state(saving,pyatspi.STATE_FOCUSED));key('Return');key('Right');wait(lambda:state(docs,pyatspi.STATE_FOCUSED));key('Return')
        wait(lambda:any(x.name=='2 activations · 1 navigation requests'for x in walk(app)))
        vertical=named(app,'Vertical navigation toolbar',pyatspi.ROLE_TOOL_BAR);assert state(vertical,pyatspi.STATE_VERTICAL);vfirst=named(vertical,'Refresh',pyatspi.ROLE_PUSH_BUTTON)
        key('Tab');wait(lambda:state(vfirst,pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(docs,pyatspi.STATE_FOCUSED))
        key('Right');wait(lambda:state(settings,pyatspi.STATE_FOCUSED));key('Return');key('Right');wait(lambda:state(deploy,pyatspi.STATE_FOCUSED));key('Return')
        time.sleep(.15);r=deploy.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);assert r.x>=57 and r.x+r.width<=width-57,(case,r)
        subprocess.run(['import','-window',win,str(out/(case+'-focused.png'))],check=True)
        key('Right');wait(lambda:state(refresh,pyatspi.STATE_FOCUSED));key('End');assert state(refresh,pyatspi.STATE_FOCUSED)
        key('Tab');wait(lambda:state(vfirst,pyatspi.STATE_FOCUSED));key('Down');wait(lambda:state(named(vertical,'Paused',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        no_loop=named(app,'Nonlooping toolbar',pyatspi.ROLE_TOOL_BAR);nfirst=named(no_loop,'Refresh',pyatspi.ROLE_PUSH_BUTTON)
        key('Tab');wait(lambda:state(nfirst,pyatspi.STATE_FOCUSED))
        for expected in('Paused','Saving','Documentation','Settings','Deploy'):
            key('Right');wait(lambda:state(named(no_loop,expected,pyatspi.ROLE_LINK if expected=='Documentation'else pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        key('Right');assert state(named(no_loop,'Deploy',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED)
        disabled=named(app,'Disabled actions toolbar',pyatspi.ROLE_TOOL_BAR);disabled_refresh=named(disabled,'Refresh',pyatspi.ROLE_PUSH_BUTTON);key('Tab');wait(lambda:state(disabled_refresh,pyatspi.STATE_FOCUSED));assert not state(disabled_refresh,pyatspi.STATE_SENSITIVE)
        for expected in('Paused','Saving','Documentation'):
            key('Right');wait(lambda:state(named(disabled,expected,pyatspi.ROLE_LINK if expected=='Documentation'else pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        disabled_docs=named(disabled,'Documentation',pyatspi.ROLE_LINK);assert state(disabled_docs,pyatspi.STATE_SENSITIVE);key('Return')
        # Restore primary to a removed item without moving focus through the owner control.
        refresh.queryComponent().grabFocus();wait(lambda:state(refresh,pyatspi.STATE_FOCUSED));
        for expected in('Paused','Saving','Documentation','Settings'):
            key('Right');wait(lambda:state(named(primary,expected,pyatspi.ROLE_LINK if expected=='Documentation'else pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        key('alt+t');wait(lambda:state(refresh,pyatspi.STATE_FOCUSED));time.sleep(.2);key('Tab');wait(lambda:state(named(named(app,'Vertical navigation toolbar',pyatspi.ROLE_TOOL_BAR),'Paused',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(named(named(app,'Actions toolbar',pyatspi.ROLE_TOOL_BAR),'Refresh',pyatspi.ROLE_PUSH_BUTTON),pyatspi.STATE_FOCUSED))
        subprocess.run(['import','-window',win,str(out/(case+'-removed.png'))],check=True)
        results.append({'case':case,'result':'PASS','controls':geometry,'actions':['pointer focus before Space','disabled focus without activation','loading Busy/inert','skip disabled','native link focus/navigation once','Tab exit/reentry','loop/nonloop','vertical navigation','End left to host','narrow reveal','group disabled buttons/active links','focused removal and subsequent traversal']});print(case,'PASS',flush=True)
      finally:
        p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
