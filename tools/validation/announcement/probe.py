import subprocess,time,json,sys
from pathlib import Path
import pyatspi
root=Path('/workspace/gpui-kumo');out=root/'docs/evidence/announcement/linux'
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


out.mkdir(parents=True,exist_ok=True)
results=[]
for dark in(False,True):
 for width in(1040,520):
    case=('dark'if dark else'light')+'-'+str(width)
    if len(sys.argv)>1 and case!=sys.argv[1]:continue
    with(out/(case+'.log')).open('w')as log:
      p=subprocess.Popen([str(root/'target/debug/examples/announcement'),f'--width={width}']+(['--dark']if dark else[]),stdout=log,stderr=subprocess.STDOUT)
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
        recorder=None
        if case=='light-1040':
            movie=Path('/workspace/artifacts/gpui-kumo-announcement-rehearsal.mp4');movie.parent.mkdir(exist_ok=True)
            recorder=subprocess.Popen(['ffmpeg','-y','-f','x11grab','-framerate','30','-video_size','1040x800','-i',':100+0,0','-c:v','libx264','-preset','veryfast','-crf','20','-pix_fmt','yuv420p',str(movie)],stdin=subprocess.PIPE,stdout=subprocess.DEVNULL,stderr=(out/'recording.log').open('w'))
        def screenshot(suffix):
            time.sleep(.4);path=out/(case+'-'+suffix+'.png');subprocess.run(['import','-window',win,str(path)],check=True);return path
        try:
            screenshot('idle');time.sleep(.8)
            trigger=button('Document options');click(trigger);wait(lambda:any(x.getRole()==pyatspi.ROLE_MENU for x in walk(app)));screenshot('menu');time.sleep(.8)
            click(named(app,'Edit document',pyatspi.ROLE_MENU_ITEM));wait(lambda:exists('Edit document'))
            editor=named(app,'Document name',pyatspi.ROLE_ENTRY);wait(lambda:state(editor,pyatspi.STATE_FOCUSED));assert state(toast('Edit document'),pyatspi.STATE_MODAL)
            key('ctrl+a');text('Kumo for GPUI launch');screenshot('edit');time.sleep(.8)
            click(button('Save changes'));wait(lambda:exists('Document saved'));wait(lambda:state(trigger,pyatspi.STATE_FOCUSED));assert not state(toast('Document saved'),pyatspi.STATE_MODAL)
            screenshot('saved');click(button('Switch theme'));wait(lambda:exists('Document saved'));path=screenshot('theme-feedback')
            intensity=float(subprocess.run(['convert',str(path),'-crop','1x1+0+0','-format','%[fx:mean]','info:'],capture_output=True,text=True,check=True).stdout)
            assert (intensity>.8 if dark else intensity<.2),intensity
            time.sleep(.8);click(button('Delete document'));wait(lambda:any(x.name=='Delete document?'and x.getRole()==pyatspi.ROLE_ALERT for x in walk(app)))
            alert=named(app,'Delete document?',pyatspi.ROLE_ALERT);assert state(alert,pyatspi.STATE_MODAL);screenshot('delete');time.sleep(.8)
            click(button('Delete'));wait(lambda:exists('Document deleted'));assert not any(x.name=='Delete document?'and x.getRole()==pyatspi.ROLE_ALERT for x in walk(app));screenshot('deleted');time.sleep(1.2)
            results.append({'case':case,'result':'PASS','checks':['menu pointer opens modal editor with input focus','save restores menu trigger and produces nonmodal feedback','live theme switch retains toast','alert confirmation and delete feedback']});print(case,'PASS',flush=True)
        finally:
            if recorder:
                recorder.communicate(input=b'q\n',timeout=10);assert recorder.returncode==0
      finally:p.terminate();p.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results,indent=2)+'\n')
