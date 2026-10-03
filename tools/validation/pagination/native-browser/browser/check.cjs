const {chromium}=require('/opt/codex/runtimes/codex-primary-runtime/dependencies/node/node_modules/playwright');
const assert=require('node:assert/strict'),fs=require('fs'),path=require('path');
(async()=>{
 const out=path.join(__dirname,'evidence');fs.mkdirSync(out,{recursive:true});
 const browser=await chromium.launch({executablePath:'/usr/bin/chromium',headless:true,args:['--no-sandbox']});
 const results=[];
 for(const theme of ['light','dark'])for(const width of [1040,520]){
  const page=await browser.newPage({viewport:{width,height:1000}});page.on('pageerror',e=>{throw e});
  await page.goto('http://127.0.0.1:4174');await page.evaluate(theme=>document.documentElement.dataset.mode=theme,theme);
  const trigger=page.getByRole('combobox',{name:'Page number',exact:true});
  const geometry=await page.locator('#accepted [data-slot="input-group"]').evaluate(el=>Array.from(el.children).map(child=>{const r=child.getBoundingClientRect(),s=getComputedStyle(child);return {tag:child.tagName,x:r.x,width:r.width,height:r.height,background:s.backgroundColor,shadow:s.boxShadow,gap:s.gap}}));
  const input=await page.locator('#input [data-slot="input-group"]').boundingBox(),simple=await page.locator('#simple [data-slot="input-group"]').boundingBox();
  assert.equal(input.width,214);assert.equal(simple.width,83);
  for(const el of geometry.filter(x=>x.tag==='BUTTON'&&x.width===42))assert.equal(el.height,36);
  const closed=await trigger.evaluate(e=>({bg:getComputedStyle(e).backgroundColor,attributes:e.getAttributeNames()}));
  await trigger.hover();const hover=await trigger.evaluate(e=>getComputedStyle(e).backgroundColor);
  await trigger.click();await page.getByRole('option').first().waitFor();await page.waitForTimeout(200);
  const rows=await page.getByRole('option').evaluateAll(nodes=>nodes.map(n=>({value:n.textContent,height:n.getBoundingClientRect().height,selected:n.getAttribute('aria-selected'),checks:n.querySelectorAll('svg').length})));
  assert.equal(rows.length,10);assert(rows.every(x=>x.height===33));assert.equal(rows.filter(x=>x.selected==='true').length,1);assert.equal(rows.filter(x=>x.checks>0).length,1);
  const open=await trigger.evaluate(e=>({bg:getComputedStyle(e).backgroundColor,popupOpen:e.hasAttribute('data-popup-open'),state:e.getAttribute('data-state')}));
  assert(open.popupOpen);assert.equal(open.state,null);assert.equal(open.bg,hover);
  await page.screenshot({path:path.join(out,`${theme}-${width}-open.png`)});
  await page.getByRole('option',{name:'4',exact:true}).click();await assert.equal(await page.locator('#proposals').textContent(),'1');
  await assert.equal(await trigger.textContent(),'4');assert(await trigger.evaluate(e=>document.activeElement===e));
  await page.keyboard.press('Space');await page.keyboard.press('ArrowDown');await page.keyboard.press('Enter');assert.equal(await page.locator('#proposals').textContent(),'2');assert.equal(await trigger.textContent(),'5');
  await page.keyboard.press('Space');await page.keyboard.press('Escape');assert(await trigger.evaluate(e=>document.activeElement===e));
  await page.keyboard.press('Space');await page.keyboard.press('Tab');await page.waitForTimeout(100);assert.equal(await trigger.getAttribute('aria-expanded'),'false');assert(!(await trigger.evaluate(e=>document.activeElement===e)));
  const rejected=page.getByRole('combobox',{name:'Rejected page',exact:true});await rejected.click();await page.getByRole('option',{name:'4',exact:true}).click();assert.equal(await rejected.textContent(),'3');assert.equal(await page.locator('#proposals').textContent(),'3');
  await page.locator('#mode').click();assert.equal(await page.locator('#accepted [role="combobox"]').count(),0);assert.equal((await page.locator('#accepted [data-slot="input-group"]').boundingBox()).width,214);
  await page.locator('#mode').click();await page.locator('#total').click();await trigger.click();await page.getByRole('option').first().waitFor();assert.equal(await page.getByRole('option').count(),3);await page.keyboard.press('Escape');
  await page.locator('#reset').click();const pageSize=page.getByRole('combobox',{name:'Page size',exact:true});await pageSize.click();await page.getByRole('option',{name:'25',exact:true}).click();assert.equal(await pageSize.textContent(),'25');
  const sizedPage=page.getByRole('combobox',{name:'Sized page',exact:true});await sizedPage.click();await page.getByRole('option').first().waitFor();assert.equal(await page.getByRole('option').count(),4);await page.keyboard.press('Escape');
  await page.screenshot({path:path.join(out,`${theme}-${width}-settled.png`)});
  results.push({theme,width,geometry,input,simple,closed,hover,open,rows,checks:'pointer and keyboard controlled proposals, rejection, Escape/Tab focus, mode/total changes, PageSize'});await page.close();
 }
 fs.writeFileSync(path.join(out,'results.json'),JSON.stringify({chromium:browser.version(),results},null,2)+'\n');await browser.close();console.log(JSON.stringify({cases:results.length,checks:'all source assertions passed'}));
})().catch(e=>{console.error(e);process.exit(1)});
