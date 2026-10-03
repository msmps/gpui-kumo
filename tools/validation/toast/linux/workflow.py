import subprocess,time,json
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/toast/linux/workflow'
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
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/dialog'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
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
        def dialog():return named(app,'Edit document',pyatspi.ROLE_DIALOG)
        def exists(name):return any(x.name==name and x.getRole()in(pyatspi.ROLE_DIALOG,pyatspi.ROLE_ALERT) for x in walk(app))
        def result(prefix):return any(x.name.startswith(prefix)for x in walk(app))
        opener=button('Edit document');click(opener);wait(lambda:exists('Edit document'))
        editor=named(app,'Document name',pyatspi.ROLE_ENTRY);wait(lambda:state(editor,pyatspi.STATE_FOCUSED))
        assert state(dialog(),pyatspi.STATE_MODAL)
        geometry={}
        for label,node in [('panel',dialog()),('editor',editor),('cancel',button('Cancel')),('save',button('Save changes'))]:
            r=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS);geometry[label]={'x':r.x,'y':r.y,'width':r.width,'height':r.height}
        assert geometry['panel']['width']==(512 if width>=640 else width-32)
        assert geometry['panel']['y']==(64 if width>=640 else 32)
        assert geometry['panel']['x']==(width-geometry['panel']['width'])//2
        key('Return');time.sleep(.15);assert exists('Edit document')
        key('shift+Tab');wait(lambda:state(button('Save changes'),pyatspi.STATE_FOCUSED))
        key('Tab');wait(lambda:state(editor,pyatspi.STATE_FOCUSED))
        key('Tab');wait(lambda:state(button('Cancel'),pyatspi.STATE_FOCUSED))
        key('Tab');wait(lambda:state(button('Save changes'),pyatspi.STATE_FOCUSED))
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-edit.png'))],check=True)
        click(editor);key('ctrl+a');key('BackSpace');click(button('Save changes'));wait(lambda:result('0 operations · Enter a document name'));assert exists('Edit document');assert state(button('Save changes'),pyatspi.STATE_FOCUSED)
        click(editor);text('Announcement');click(button('Save changes'));wait(lambda:result('1 operations · Saved Announcement'));wait(lambda:state(opener,pyatspi.STATE_FOCUSED));assert not exists('Edit document');wait(lambda:exists('Document saved'))
        assert not state(named(app,'Document saved',pyatspi.ROLE_DIALOG),pyatspi.STATE_MODAL)
        click(opener);wait(lambda:exists('Edit document'));key('Escape');wait(lambda:state(opener,pyatspi.STATE_FOCUSED))
        click(opener);wait(lambda:exists('Edit document'));time.sleep(.2);subprocess.run(['xdotool','mousemove','10','10','click','1'],check=True);wait(lambda:not exists('Edit document'))
        menu_trigger=button('Document options');click(menu_trigger)
        wait(lambda:any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)))
        click(named(app,'Edit document',pyatspi.ROLE_MENU_ITEM));wait(lambda:exists('Edit document'));key('Escape');wait(lambda:state(menu_trigger,pyatspi.STATE_FOCUSED))
        click(button('Delete document'));wait(lambda:exists('Delete document?'))
        alert=named(app,'Delete document?',pyatspi.ROLE_ALERT);assert state(alert,pyatspi.STATE_MODAL)
        time.sleep(.2);subprocess.run(['xdotool','mousemove','10','10','click','1'],check=True);assert exists('Delete document?')
        time.sleep(.2);subprocess.run(['import','-window',win,str(out/(case+'-alert.png'))],check=True)
        click(button('Delete'));wait(lambda:result('2 operations · Document deleted'));assert not exists('Delete document?');wait(lambda:exists('Document deleted'))
        time.sleep(.6);subprocess.run(['import','-window',win,str(out/(case+'-feedback.png'))],check=True)
        results.append({'case':case,'result':'PASS','geometry':geometry,'checks':['named modal role and bounds','explicit initial editor focus','Enter in editor does not confirm','Tab and reverse Tab wrap','rejected empty save retains dialog and focus','successful save once and retained draft','Escape and standard outside close','menu to dialog restores surviving menu trigger','alert outside press rejected','delete action once','save/delete produce nonmodal Toast feedback']});print(case,'PASS',flush=True)
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
