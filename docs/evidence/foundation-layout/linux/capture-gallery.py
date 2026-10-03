import subprocess,time,json
from pathlib import Path
from gi.repository import Gio,GLib
import pyatspi
out=Path('/workspace/gpui-kumo/docs/evidence/foundation-layout/linux')
bus=Gio.bus_get_sync(Gio.BusType.SESSION,None)
bus.call_sync('org.a11y.Bus','/org/a11y/bus','org.freedesktop.DBus.Properties','Set',GLib.Variant('(ssv)',('org.a11y.Status','ScreenReaderEnabled',GLib.Variant('b',True))),None,Gio.DBusCallFlags.NONE,-1,None)
p=subprocess.Popen(['/workspace/gpui-kumo/target/debug/kumo-gallery'],stdout=(out/'gallery.log').open('w'),stderr=subprocess.STDOUT)
def walk(n):
 n.clearCache(); yield n
 for c in n: yield from walk(c)
try:
 for _ in range(100):
  windows=subprocess.run(['xdotool','search','--onlyvisible','--name','^GPUI Kumo$'],capture_output=True,text=True).stdout.strip().splitlines()
  if windows: break
  time.sleep(.1)
 win=windows[-1]
 subprocess.run(['xdotool','windowmove',win,'0','0'],check=True)
 subprocess.run(['xdotool','windowfocus',win],check=True)
 desktop=pyatspi.Registry.getDesktop(0)
 for _ in range(100):
  nodes=list(walk(desktop))
  if any(n.name=='Dark appearance' for n in nodes):break
  time.sleep(.1)
 for dark in (False,True):
  subprocess.run(['xdotool','mousemove','--window',win,'300','700','click','--repeat','1000','--delay','1','4'],check=True)
  time.sleep(1)
  name='Dark appearance' if dark else 'Light appearance'
  node=next(n for n in walk(desktop) if n.name==name)
  r=node.queryComponent().getExtents(pyatspi.DESKTOP_COORDS)
  print('THEME',name,list(r),flush=True)
  subprocess.run(['xdotool','mousemove',str(r.x+r.width//2),str(r.y+r.height//2),'click','1'],check=True)
  time.sleep(1)
  subprocess.run(['import','-window',win,str(out/(('dark' if dark else 'light')+'-header.png'))],check=True)
  for width in (1040,520):
   subprocess.run(['xdotool','windowsize',win,str(width),'1200'],check=True)
   subprocess.run(['xdotool','mousemove','--window',win,'300','700','click','--repeat','1000','--delay','1','5'],check=True)
   time.sleep(1)
   subprocess.run(['import','-window',win,str(out/(('dark' if dark else 'light')+f'-{width}-gallery.png'))],check=True,timeout=10)
 print('Captured full-gallery foundations Light/Dark 1040/520; native AT-SPI theme action and wheel scrolling.')
finally:
 p.terminate();p.wait(timeout=10)
