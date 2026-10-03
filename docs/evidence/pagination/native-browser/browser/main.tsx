import React,{useState} from 'react';
import {createRoot} from 'react-dom/client';
import {Pagination} from '/workspace/.kumo-reference/packages/kumo/src/components/pagination/pagination';
function App(){
 const [page,setPage]=useState(3),[size,setSize]=useState(10),[mode,setMode]=useState<'input'|'dropdown'>('dropdown'),[total,setTotal]=useState(95),[proposals,setProposals]=useState(0);
 return <main><button id="theme" onClick={()=>document.documentElement.dataset.mode=document.documentElement.dataset.mode==='dark'?'light':'dark'}>Theme</button><button id="mode" onClick={()=>setMode(mode==='input'?'dropdown':'input')}>Mode</button><button id="total" onClick={()=>{setTotal(25);setPage(1)}}>Smaller total</button><button id="reset" onClick={()=>{setPage(3);setSize(10);setTotal(95);setMode('dropdown')}}>Reset</button>
 <section id="accepted"><Pagination page={page} setPage={p=>{setProposals(n=>n+1);setPage(p)}} perPage={size} totalCount={total}><Pagination.Info/><Pagination.Controls pageSelector={mode}/></Pagination></section>
 <section id="sized"><Pagination page={page} setPage={setPage} perPage={size} totalCount={total} labels={{pageNumber:'Sized page',pageSize:'Page size'}}><Pagination.Info/><Pagination.PageSize value={size} onChange={s=>{setSize(s);setPage(1)}}/><Pagination.Controls pageSelector="dropdown"/></Pagination></section>
 <section id="rejected"><Pagination page={3} setPage={()=>setProposals(n=>n+1)} perPage={10} totalCount={95} labels={{pageNumber:'Rejected page'}}><Pagination.Info/><Pagination.Controls pageSelector="dropdown"/></Pagination></section>
 <section id="input"><Pagination page={3} setPage={()=>{}} perPage={10} totalCount={95} labels={{pageNumber:'Input page'}}><Pagination.Info/><Pagination.Controls/></Pagination></section>
 <section id="simple"><Pagination page={3} setPage={()=>{}} perPage={10} totalCount={95}><Pagination.Info/><Pagination.Controls controls="simple"/></Pagination></section>
 <section id="unknown"><Pagination page={3} setPage={()=>{}} hasNextPage><Pagination.Info/><Pagination.Controls pageSelector="dropdown"/></Pagination></section>
 <output id="proposals">{proposals}</output><button id="outside">Outside</button>
 </main>
}
createRoot(document.getElementById('root')!).render(<App/>);
