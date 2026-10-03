import os, subprocess, time, json
from pathlib import Path
root=Path('/workspace/gpui-kumo')
out=root/'docs/evidence/foundation-layout/linux'
out.mkdir(parents=True,exist_ok=True)
results=[]
for dark in (False,True):
    for width in (1040,520):
        name=('dark' if dark else 'light')+'-'+str(width)
        with (out/(name+'.log')).open('w') as log:
            p=subprocess.Popen([str(root/'target/debug/examples/foundations'),f'--width={width}']+(['--dark'] if dark else []),stdout=log,stderr=subprocess.STDOUT)
            try:
                for _ in range(100):
                    if p.poll() is not None: raise RuntimeError(f'preview exited: {p.returncode}')
                    ids=subprocess.run(['xdotool','search','--onlyvisible','--pid',str(p.pid)],capture_output=True,text=True).stdout.strip().splitlines()
                    if ids: break
                    time.sleep(.1)
                else: raise RuntimeError('no window')
                win=ids[-1]
                subprocess.run(['xdotool','windowfocus','--sync',win],check=True)
                time.sleep(1)
                geometry=subprocess.check_output(['xdotool','getwindowgeometry','--shell',win],text=True)
                for attempt in range(20):
                    subprocess.run(['import','-window',win,str(out/(name+'.png'))],check=True,timeout=10)
                    mean=float(subprocess.check_output(['identify','-format','%[fx:mean]',str(out/(name+'.png'))],text=True))
                    if mean > .005: break
                    time.sleep(.5)
                else: raise RuntimeError('no painted frame')
                results.append({'case':name,'geometry':geometry,'pid':p.pid})
                if width==1040:
                    for resized in (737,738,520,1040):
                        subprocess.run(['xdotool','windowsize',win,str(resized),'1200'],check=True)
                        time.sleep(.5)
                        subprocess.run(['import','-window',win,str(out/(name+f'-resize-{resized}.png'))],check=True,timeout=10)
            finally:
                p.terminate(); p.wait(timeout=10)
(out/'captures.json').write_text(json.dumps(results,indent=2)+'\n')
print(json.dumps(results,indent=2))
