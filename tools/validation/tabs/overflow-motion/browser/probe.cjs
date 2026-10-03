const {chromium}=require('/opt/codex/runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const path=require('path'),fs=require('fs');
(async()=>{const browser=await chromium.launch({executablePath:'/usr/bin/chromium',args:['--no-sandbox']});const results=[];
for(const dark of [false,true])for(const width of [1040,520]){
const page=await browser.newPage({viewport:{width,height:1000}});await page.goto('http://127.0.0.1:4175');await page.evaluate(d=>document.documentElement.setAttribute('data-mode',d?'dark':'light'),dark);await page.waitForTimeout(250);
const geometry=await page.evaluate(()=>Array.from(document.querySelectorAll('section:not(#dynamic)')).map(s=>{const list=s.querySelector('[role=tablist]'),tab=s.querySelector('[role=tab]'),style=getComputedStyle(tab);return{id:s.id,listHeight:list.getBoundingClientRect().height,tabHeight:tab.getBoundingClientRect().height,radius:style.borderRadius,fontSize:style.fontSize};}));
for(const row of geometry){const expected={'segmented-base':[36,32],'segmented-sm':[26,22],'underline-base':[30,24],'underline-sm':[26,20]}[row.id];if(row.listHeight!==expected[0]||row.tabHeight!==expected[1])throw Error(JSON.stringify(row));}
const dynamic=page.locator('#dynamic'),right=dynamic.getByRole('button',{name:/Scroll.*right/i});
await right.click();await page.waitForTimeout(350);await right.press('Space');await page.waitForTimeout(350);
const before=await dynamic.locator('[role=tablist]').evaluate(e=>e.scrollLeft);if(before<=0)throw Error('scroll action failed');if(await dynamic.getByRole('tab').first().getAttribute('aria-selected')!=='true')throw Error('scroll selected a tab');
await page.screenshot({path:path.join(__dirname,`${dark?'dark':'light'}-${width}-overflow.png`)});
const box=await dynamic.locator('[role=tablist]').boundingBox();await page.mouse.move(box.x+125,box.y+box.height/2);await page.mouse.down();await page.mouse.move(box.x+95,box.y+box.height/2,{steps:4});await page.mouse.up();await page.waitForTimeout(100);if(await dynamic.locator('[role=tablist]').evaluate(e=>e.scrollLeft)<=before)throw Error('drag did not scroll');if(await dynamic.getByRole('tab').first().getAttribute('aria-selected')!=='true')throw Error('drag release selected a tab');
await page.locator('#toggle').click();await page.waitForTimeout(250);if(await dynamic.getByRole('tab').count()!==2)throw Error('owner shrink');
if(await dynamic.locator('[role=tablist]').evaluate(e=>e.scrollWidth>e.clientWidth))throw Error('shrink overflow');
results.push({theme:dark?'dark':'light',width,result:'PASS',geometry,scrollOffset:before,actions:['pointer edge scroll','Space edge scroll','no selection proposal','drag release suppresses activation','owner shrink removes overflow']});await page.close();}
fs.writeFileSync(path.join(__dirname,'results.json'),JSON.stringify(results,null,2)+'\n');await browser.close();})().catch(e=>{console.error(e);process.exit(1)});
