const {chromium}=require('/opt/codex/runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const fs=require('fs'),path=require('path');
(async()=>{
 const browser=await chromium.launch({executablePath:'/usr/bin/chromium',args:['--no-sandbox']});const results=[];
 for(const dark of[false,true])for(const width of[1040,520]){
  const page=await browser.newPage({viewport:{width,height:800}});await page.goto('http://127.0.0.1:4185');
  await page.evaluate(d=>document.documentElement.setAttribute('data-mode',d?'dark':'light'),dark);
  const name=`${dark?'dark':'light'}-${width}`,geometry={};
  for(const [suffix,title,description,labels]of[
   ['actions','Changes saved','You can undo this demo operation.',['Undo']],
   ['long','Your announcement document has been saved and is ready to share','Review the final copy with your team before publishing. Your changes are retained while you decide what to do next.',['Review changes','Later']]
  ]){
   await page.evaluate(([suffix,title,description,labels])=>window.toastManager.add({id:suffix,title,description,variant:'success',timeout:0,actions:labels.map(children=>({children}))}),[suffix,title,description,labels]);
   const toast=page.getByRole('dialog',{name:title});await toast.waitFor();await page.waitForTimeout(600);
   geometry[suffix]=await toast.boundingBox();
   for(const label of labels){const a=await toast.getByRole('button',{name:label,exact:true}).boundingBox(),r=geometry[suffix];if(a.x<r.x+16||a.x+a.width>r.x+r.width-16||a.y+a.height>r.y+r.height-16)throw Error('action outside padding');}
   await page.screenshot({path:path.join(__dirname,name+'-'+suffix+'.png')});await toast.locator('[data-kumo-part=close]').click();await toast.waitFor({state:'hidden'});
  }
  results.push({case:name,result:'PASS',geometry});await page.close();
 }
 fs.writeFileSync(path.join(__dirname,'alignment-results.json'),JSON.stringify(results,null,2)+'\n');await browser.close();
})().catch(e=>{console.error(e);process.exit(1)});
