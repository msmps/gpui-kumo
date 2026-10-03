import subprocess,time,json,sys
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toast/linux'
def children(n):
    n.clearCache();return[c for i in range(n.childCount)if(c:=n.getChildAtIndex(i))is not None]
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
    if len(sys.argv)>1 and case!=sys.argv[1]:continue
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/toast'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
      try:
        for _ in range(100):
            ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
            if ids:break
            if p.poll()is not None:raise RuntimeError('preview exited')
            time.sleep(.1)
        else:raise RuntimeError('no window')
        win=ids[-1];subprocess.run(['xdotool','windowmove',win,'0','0','windowfocus','--sync',win],check=True);time.sleep(.5)
        wait(lambda:any(x.get_process_id()==p.pid for x in children(pyatspi.Registry.getDesktop(0))))
        app=next(x for x in children(pyatspi.Registry.getDesktop(0))if x.get_process_id()==p.pid)
        def button(label):return named(app,label,pyatspi.ROLE_PUSH_BUTTON)
        def toast(title):return named(app,title,pyatspi.ROLE_DIALOG)
        def exists(title):return any(x.name==title and x.getRole()==pyatspi.ROLE_DIALOG for x in walk(app))
        def region():return next(x for x in walk(app)if x.name=='Notifications')
        producer=button('Save document');click(producer);wait(lambda:exists('Document saved'));time.sleep(.6)
        target=toast('Document saved');assert not state(target,pyatspi.STATE_MODAL)
        region_role=region().getRoleName();region_attributes=region().getAttributes()
        r=target.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);geometry={'x':r.x,'y':r.y,'width':r.width,'height':r.height}
        assert geometry=={'x':668 if width>=640 else 16,'y':692 if width>=640 else 708,'width':340 if width>=640 else width-32,'height':76},geometry
        key('F6');wait(lambda:state(region(),pyatspi.STATE_FOCUSED));key('shift+Tab');wait(lambda:state(producer,pyatspi.STATE_FOCUSED));key('F6');wait(lambda:state(region(),pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-success.png'))],check=True)
        time.sleep(5.2);assert exists('Document saved')
        click(button('Update saved toast'));wait(lambda:exists('Document ready to share'));assert sum(x.getRole()==pyatspi.ROLE_DIALOG for x in walk(app))==1
        click(button('Close'));wait(lambda:not exists('Document ready to share'))
        for label,title in [('Show default','Document updated'),('Show error','Save failed'),('Show warning','Review your changes'),('Show info','New version available')]:
            click(button(label));wait(lambda:exists(title));time.sleep(.6);subprocess.run(['import','-window',win,str(out/(case+'-'+label.split()[-1]+'.png'))],check=True);click(button('Close'));wait(lambda:not exists(title))
        click(button('Show actions'));wait(lambda:exists('Changes saved'));time.sleep(.6);click(button('Undo'));wait(lambda:any(x.name=='1 toast actions'for x in walk(app)));assert exists('Changes saved');click(button('Close'));wait(lambda:not exists('Changes saved'))
        click(producer);wait(lambda:exists('Document saved'));time.sleep(.6);target=toast('Document saved');r=target.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);subprocess.run(['xdotool','mousemove',str(r.x+40),str(r.y+40)],check=True);time.sleep(5.2);assert exists('Document saved');subprocess.run(['xdotool','mousemove','10','10'],check=True);wait(lambda:not exists('Document saved'))
        results.append({'case':case,'result':'PASS','geometry':geometry,'region_role':region_role,'region_attributes':region_attributes,'checks':['named nonmodal Dialog','source viewport edges and title/description height','F6 Notifications and reverse Tab restoration','focused timeout paused past5s','stable update preserves one root','close','five variants','action once without automatic dismissal','hover timeout paused past5s','timeout removal after hover exit']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
