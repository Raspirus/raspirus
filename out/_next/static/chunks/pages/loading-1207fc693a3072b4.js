(self.webpackChunk_N_E=self.webpackChunk_N_E||[]).push([[619],{7989:function(n,e,i){(window.__NEXT_P=window.__NEXT_P||[]).push(["/loading",function(){return i(6566)}])},6566:function(n,e,i){"use strict";i.r(e),i.d(e,{default:function(){return l}});var t=i(5893),o=i(9008),a=i.n(o),r=i(982),s=i.n(r),d=i(1163),_=i(7294),c=i(5436);function l(){let n=(0,d.useRouter)(),{query:{scan_path:e}}=n;function i(){(0,c.dw)("start_scanner",{path:e,update:!1,dbfile:""}).then(e=>{console.log("Message: ",e),"None"!=e?n.push("/infected"):n.push("/clean")}).catch(e=>{console.error(e),n.push({pathname:"/",query:{scanner_error:e}})}),console.log("Finished scanning")}return(0,_.useEffect)(()=>{setTimeout(i,0)},[]),(0,t.jsxs)(t.Fragment,{children:[(0,t.jsx)(a(),{children:(0,t.jsx)("title",{children:"Loading..."})}),(0,t.jsxs)("main",{className:"flex flex-col items-center justify-center h-full",children:[(0,t.jsx)("h1",{className:"text-2xl font-bold text-center",children:"Loading... Please wait"}),(0,t.jsxs)("div",{className:"flex flex-row m-10",children:[(0,t.jsx)("div",{className:[s().main_div,s().zero_div].join(" ")}),(0,t.jsx)("div",{className:[s().main_div,s().first_div].join(" ")}),(0,t.jsx)("div",{className:[s().main_div,s().second_div].join(" ")}),(0,t.jsx)("div",{className:[s().main_div,s().third_div].join(" ")}),(0,t.jsx)("div",{className:[s().main_div,s().fourth_div].join(" ")})]})]})]})}},982:function(n){n.exports={main_body:"animation_main_body__TRJaI",main_div:"animation_main_div__jJO9o",scaling:"animation_scaling__x9tPQ",zero_div:"animation_zero_div___BqU7",first_div:"animation_first_div__iPdLo",second_div:"animation_second_div____udD",third_div:"animation_third_div__2XUy5",fourth_div:"animation_fourth_div__Q_EA_"}},5436:function(n,e,i){"use strict";i.d(e,{dw:function(){return a}});var t=Object.defineProperty;function o(n,e=!1){let i=window.crypto.getRandomValues(new Uint32Array(1))[0],t=`_${i}`;return Object.defineProperty(window,t,{value:i=>(e&&Reflect.deleteProperty(window,t),null==n?void 0:n(i)),writable:!1,configurable:!0}),i}async function a(n,e={}){return new Promise((i,t)=>{let a=o(n=>{i(n),Reflect.deleteProperty(window,`_${r}`)},!0),r=o(n=>{t(n),Reflect.deleteProperty(window,`_${a}`)},!0);window.__TAURI_IPC__({cmd:n,callback:a,error:r,...e})})}function r(n,e="asset"){let i=encodeURIComponent(n);return navigator.userAgent.includes("Windows")?`https://${e}.localhost/${i}`:`${e}://localhost/${i}`}((n,e)=>{for(var i in e)t(n,i,{get:e[i],enumerable:!0})})({},{convertFileSrc:()=>r,invoke:()=>a,transformCallback:()=>o})}},function(n){n.O(0,[774,888,179],function(){return n(n.s=7989)}),_N_E=n.O()}]);