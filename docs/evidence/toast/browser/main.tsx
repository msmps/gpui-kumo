import React from'react';import{createRoot}from'react-dom/client';
import{Toasty,useKumoToastManager,createKumoToastManager}from'/workspace/.kumo-reference/packages/kumo/src/components/toast/toast';
import{Button}from'/workspace/.kumo-reference/packages/kumo/src/components/button/button';
const external=createKumoToastManager();
function App(){const m=useKumoToastManager();(window as any).toastManager=m;(window as any).externalToastManager=external;return <main><Button onClick={()=>m.add({id:'saved',title:'Document saved',description:'Your changes are ready to share.',variant:'success',timeout:0})}>Save document</Button><Button>After toast</Button></main>};
createRoot(document.getElementById('root')!).render(<Toasty toastManager={external}><App/></Toasty>);
