import subprocess,time,json,sys
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toast/linux/alignment'
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


out.mkdir(exist_ok=True)
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
        geometry={}
        for label,title,suffix in [('Show actions','Changes saved','actions'),('Show long toast','Your announcement document has been saved and is ready to share','long')]:
            click(button(label));wait(lambda:exists(title));time.sleep(.6)
            target=toast(title)
            r=target.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
            geometry[suffix]={'x':r.x,'y':r.y,'width':r.width,'height':r.height}
            assert r.width==(340 if width>=640 else width-32)
            for action in (['Undo']if suffix=='actions'else['Review changes','Later']):
                button_node=button(action);a=button_node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
                assert a.x>=r.x+16 and a.x+a.width<=r.x+r.width-16
                assert a.y>=r.y+16 and a.y+a.height<=r.y+r.height-16
            subprocess.run(['import','-window',win,str(out/(case+'-'+suffix+'.png'))],check=True)
            click(button('Close'));wait(lambda:not exists(title))
        results.append({'case':case,'result':'PASS','geometry':geometry,'checks':['two-action and long wrapped content within surface padding','source viewport widths']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
