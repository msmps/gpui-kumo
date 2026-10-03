import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/dropdown/linux'
def children(n):
    n.clearCache();return[n.getChildAtIndex(i)for i in range(n.childCount)]
def walk(n):
    yield n
    for c in children(n):yield from walk(c)
def named(n,name,role):return next(x for x in walk(n)if x.name==name and x.getRole()==role)
def state(n,s):n.clearCache();return n.getState().contains(s)
def wait(test):
    for _ in range(60):
        try:
            if test():return
        except StopIteration:pass
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
results=[]
for dark in(False,True):
 for width in(1040,520):
    case=('dark'if dark else'light')+'-'+str(width)
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/dropdown'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
      try:
        for _ in range(100):
            ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
            if ids:break
            if p.poll()is not None:raise RuntimeError('preview exited')
            time.sleep(.1)
        else:raise RuntimeError('no window')
        win=ids[-1];subprocess.run(['xdotool','windowmove',win,'0','0','windowfocus','--sync',win],check=True);time.sleep(.5)
        app=next(x for x in children(pyatspi.Registry.getDesktop(0))if x.get_process_id()==p.pid)
        trigger=named(app,'Save options',pyatspi.ROLE_PUSH_BUTTON)
        before=named(app,'Before menu',pyatspi.ROLE_PUSH_BUTTON);after=named(app,'After menu',pyatspi.ROLE_PUSH_BUTTON)
        def menu():return named(app,'Save actions',pyatspi.ROLE_MENU)
        def item(label):return named(menu(),label,pyatspi.ROLE_MENU_ITEM)
        def focused(label):return state(item(label),pyatspi.STATE_FOCUSED)
        def result(n,id):return any(x.name==f'{n} actions · last: {id}'for x in walk(app))
        trigger.queryComponent().grabFocus();wait(lambda:state(trigger,pyatspi.STATE_FOCUSED));key('Down');wait(lambda:focused('Save'))
        key('Down');wait(lambda:focused('Save as draft'));key('Down');wait(lambda:focused('Publish (unavailable)'));key('Return');assert not result(1,'paused');key('Down');wait(lambda:focused('Duplicate'));key('End');wait(lambda:focused('Delete'))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-keyboard.png'))],check=True)
        key('Down');wait(lambda:focused('Save'));key('space');wait(lambda:result(1,'save'));wait(lambda:state(trigger,pyatspi.STATE_FOCUSED))
        click(trigger);wait(lambda:any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)));click(item('Delete'));wait(lambda:result(2,'delete'));wait(lambda:state(trigger,pyatspi.STATE_FOCUSED))
        key('Down');wait(lambda:focused('Save'));key('d');wait(lambda:focused('Duplicate'));key('d');wait(lambda:focused('Delete'));key('Escape');wait(lambda:state(trigger,pyatspi.STATE_FOCUSED))
        key('Up');wait(lambda:focused('Delete'));key('Tab');wait(lambda:state(after,pyatspi.STATE_FOCUSED))
        trigger.queryComponent().grabFocus();wait(lambda:state(trigger,pyatspi.STATE_FOCUSED));key('Down');wait(lambda:focused('Save'));key('shift+Tab');wait(lambda:state(trigger,pyatspi.STATE_FOCUSED))
        click(trigger);wait(lambda:any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)));click(before);wait(lambda:not any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)))
        click(trigger);wait(lambda:any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)))
        paused=item('Publish (unavailable)');assert not state(paused,pyatspi.STATE_SENSITIVE)
        target=item('Save as draft');r=target.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);subprocess.run(['xdotool','mousemove',str(r.x+r.width//2),str(r.y+r.height//2)],check=True);wait(lambda:focused('Save as draft'))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-pointer.png'))],check=True)
        geometry={}
        for label in['Save','Save as draft','Publish (unavailable)','Duplicate','Delete']:
            r=item(label).queryComponent().getExtents(pyatspi.DESKTOP_COORDS);geometry[label]={'x':r.x,'y':r.y,'width':r.width,'height':r.height}
        results.append({'case':case,'result':'PASS','geometry':geometry,'checks':['menu and menuitem native roles','disabled Sensitive false','arrows focus disabled and wrap; disabled activation guarded','End','Space and pointer once','repeated typeahead','Escape restores trigger','Tab/reverse Tab exits','outside dismissal','pointer hover focuses row']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
