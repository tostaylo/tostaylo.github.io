var app=function(){"use strict";function t(){}function n(t){return t()}function e(){return Object.create(null)}function o(t){t.forEach(n)}function r(t){return"function"==typeof t}function c(t,n){return t!=t?n==n:t!==n||t&&"object"==typeof t||"function"==typeof t}function s(t,n){t.appendChild(n)}function i(t,n,e){t.insertBefore(n,e||null)}function u(t){t.parentNode.removeChild(t)}function l(t){return document.createElement(t)}function a(t){return document.createTextNode(t)}function f(){return a(" ")}function p(){return a("")}function d(t,n,e,o){return t.addEventListener(n,e,o),()=>t.removeEventListener(n,e,o)}function m(t,n,e){null==e?t.removeAttribute(n):t.getAttribute(n)!==e&&t.setAttribute(n,e)}function h(t,n){n=""+n,t.wholeText!==n&&(t.data=n)}let $;function g(t){$=t}function v(){if(!$)throw new Error("Function called outside component initialization");return $}function y(){const t=v();return(n,e)=>{const o=t.$$.callbacks[n];if(o){const r=function(t,n){const e=document.createEvent("CustomEvent");return e.initCustomEvent(t,!1,!1,n),e}(n,e);o.slice().forEach((n=>{n.call(t,r)}))}}}const b=[],x=[],_=[],w=[],E=Promise.resolve();let S=!1;function k(t){_.push(t)}let j=!1;const P=new Set;function L(){if(!j){j=!0;do{for(let t=0;t<b.length;t+=1){const n=b[t];g(n),T(n.$$)}for(g(null),b.length=0;x.length;)x.pop()();for(let t=0;t<_.length;t+=1){const n=_[t];P.has(n)||(P.add(n),n())}_.length=0}while(b.length);for(;w.length;)w.pop()();S=!1,j=!1,P.clear()}}function T(t){if(null!==t.fragment){t.update(),o(t.before_update);const n=t.dirty;t.dirty=[-1],t.fragment&&t.fragment.p(t.ctx,n),t.after_update.forEach(k)}}const B=new Set;let C;function N(){C={r:0,c:[],p:C}}function O(){C.r||o(C.c),C=C.p}function A(t,n){t&&t.i&&(B.delete(t),t.i(n))}function M(t,n,e,o){if(t&&t.o){if(B.has(t))return;B.add(t),C.c.push((()=>{B.delete(t),o&&(e&&t.d(1),o())})),t.o(n)}}function q(t){t&&t.c()}function z(t,e,c){const{fragment:s,on_mount:i,on_destroy:u,after_update:l}=t.$$;s&&s.m(e,c),k((()=>{const e=i.map(n).filter(r);u?u.push(...e):o(e),t.$$.on_mount=[]})),l.forEach(k)}function D(t,n){const e=t.$$;null!==e.fragment&&(o(e.on_destroy),e.fragment&&e.fragment.d(n),e.on_destroy=e.fragment=null,e.ctx=[])}function H(t,n){-1===t.$$.dirty[0]&&(b.push(t),S||(S=!0,E.then(L)),t.$$.dirty.fill(0)),t.$$.dirty[n/31|0]|=1<<n%31}function I(n,r,c,s,i,l,a=[-1]){const f=$;g(n);const p=r.props||{},d=n.$$={fragment:null,ctx:null,props:l,update:t,not_equal:i,bound:e(),on_mount:[],on_destroy:[],before_update:[],after_update:[],context:new Map(f?f.$$.context:[]),callbacks:e(),dirty:a,skip_bound:!1};let m=!1;if(d.ctx=c?c(n,p,((t,e,...o)=>{const r=o.length?o[0]:e;return d.ctx&&i(d.ctx[t],d.ctx[t]=r)&&(!d.skip_bound&&d.bound[t]&&d.bound[t](r),m&&H(n,t)),e})):[],d.update(),m=!0,o(d.before_update),d.fragment=!!s&&s(d.ctx),r.target){if(r.hydrate){const t=function(t){return Array.from(t.childNodes)}(r.target);d.fragment&&d.fragment.l(t),t.forEach(u)}else d.fragment&&d.fragment.c();r.intro&&A(n.$$.fragment),z(n,r.target,r.anchor),L()}g(f)}class V{$destroy(){D(this,1),this.$destroy=t}$on(t,n){const e=this.$$.callbacks[t]||(this.$$.callbacks[t]=[]);return e.push(n),()=>{const t=e.indexOf(n);-1!==t&&e.splice(t,1)}}$set(t){var n;this.$$set&&(n=t,0!==Object.keys(n).length)&&(this.$$.skip_bound=!0,this.$$set(t),this.$$.skip_bound=!1)}}function F(n){let e,o,r,c,p,$,g,v,y,b,x,_,w,E,S,k,j,P,L=n[0].id+"",T=n[0].title+"",B=n[0].user+"",C=n[0].date+"";return{c(){e=l("article"),o=l("span"),r=a(L),c=f(),p=l("h2"),$=a(T),g=f(),v=l("p"),y=a("By\n    "),b=a(B),x=a("\n    on\n    "),_=a(C),w=f(),E=l("a"),S=a("View on Dev.to"),m(o,"class","svelte-jhbx8e"),m(p,"class","svelte-jhbx8e"),m(E,"href",k=n[0].url),m(v,"class","meta"),m(e,"class","svelte-jhbx8e")},m(t,u){i(t,e,u),s(e,o),s(o,r),s(e,c),s(e,p),s(p,$),s(e,g),s(e,v),s(v,y),s(v,b),s(v,x),s(v,_),s(v,w),s(v,E),s(E,S),j||(P=d(p,"click",n[2]),j=!0)},p(t,[n]){1&n&&L!==(L=t[0].id+"")&&h(r,L),1&n&&T!==(T=t[0].title+"")&&h($,T),1&n&&B!==(B=t[0].user+"")&&h(b,B),1&n&&C!==(C=t[0].date+"")&&h(_,C),1&n&&k!==(k=t[0].url)&&m(E,"href",k)},i:t,o:t,d(t){t&&u(e),j=!1,P()}}}function G(t,n,e){let{postSummary:o}=n;const r=y();function c(t){r("getPost",{id:t})}return t.$$set=t=>{"postSummary"in t&&e(0,o=t.postSummary)},[o,c,()=>c(o.id)]}class J extends V{constructor(t){super(),I(this,t,G,F,c,{postSummary:0})}}function K(t,n,e){const o=t.slice();return o[2]=n[e],o}function Q(t){let n,e,o=t[0],r=[];for(let n=0;n<o.length;n+=1)r[n]=R(K(t,o,n));const c=t=>M(r[t],1,1,(()=>{r[t]=null}));return{c(){for(let t=0;t<r.length;t+=1)r[t].c();n=p()},m(t,o){for(let n=0;n<r.length;n+=1)r[n].m(t,o);i(t,n,o),e=!0},p(t,e){if(1&e){let s;for(o=t[0],s=0;s<o.length;s+=1){const c=K(t,o,s);r[s]?(r[s].p(c,e),A(r[s],1)):(r[s]=R(c),r[s].c(),A(r[s],1),r[s].m(n.parentNode,n))}for(N(),s=o.length;s<r.length;s+=1)c(s);O()}},i(t){if(!e){for(let t=0;t<o.length;t+=1)A(r[t]);e=!0}},o(t){r=r.filter(Boolean);for(let t=0;t<r.length;t+=1)M(r[t]);e=!1},d(t){!function(t,n){for(let e=0;e<t.length;e+=1)t[e]&&t[e].d(n)}(r,t),t&&u(n)}}}function R(t){let n,e;return n=new J({props:{postSummary:t[2]}}),n.$on("getPost",t[1]),{c(){q(n.$$.fragment)},m(t,o){z(n,t,o),e=!0},p(t,e){const o={};1&e&&(o.postSummary=t[2]),n.$set(o)},i(t){e||(A(n.$$.fragment,t),e=!0)},o(t){M(n.$$.fragment,t),e=!1},d(t){D(n,t)}}}function U(t){let n,e,o=t[0]&&Q(t);return{c(){o&&o.c(),n=p()},m(t,r){o&&o.m(t,r),i(t,n,r),e=!0},p(t,[e]){t[0]?o?(o.p(t,e),1&e&&A(o,1)):(o=Q(t),o.c(),A(o,1),o.m(n.parentNode,n)):o&&(N(),M(o,1,1,(()=>{o=null})),O())},i(t){e||(A(o),e=!0)},o(t){M(o),e=!1},d(t){o&&o.d(t),t&&u(n)}}}function W(t,n,e){let{postSummaries:o}=n;return t.$$set=t=>{"postSummaries"in t&&e(0,o=t.postSummaries)},[o,function(n){!function(t,n){const e=t.$$.callbacks[n.type];e&&e.slice().forEach((t=>t(n)))}(t,n)}]}class X extends V{constructor(t){super(),I(this,t,W,U,c,{postSummaries:0})}}function Y(n){let e,o,r,c,p,$,g,v,y,b,x,_,w,E,S,k,j,P,L,T=n[0].title+"",B=n[0].user+"",C=n[0].date+"",N=n[0].html_content+"";return{c(){e=l("button"),e.textContent="« back",o=f(),r=l("article"),c=l("h1"),p=a(T),$=f(),g=l("p"),v=a("by\n    "),y=a(B),b=f(),x=a(C),_=f(),w=l("a"),E=a("View on Dev.to"),k=f(),j=l("p"),m(e,"class","a svelte-1f56dvt"),m(c,"class","svelte-1f56dvt"),m(w,"href",S=n[0].url),m(g,"class","meta"),m(r,"class","svelte-1f56dvt")},m(t,u){i(t,e,u),i(t,o,u),i(t,r,u),s(r,c),s(c,p),s(r,$),s(r,g),s(g,v),s(g,y),s(g,b),s(g,x),s(g,_),s(g,w),s(w,E),s(r,k),s(r,j),j.innerHTML=N,P||(L=d(e,"click",n[2]),P=!0)},p(t,[n]){1&n&&T!==(T=t[0].title+"")&&h(p,T),1&n&&B!==(B=t[0].user+"")&&h(y,B),1&n&&C!==(C=t[0].date+"")&&h(x,C),1&n&&S!==(S=t[0].url)&&m(w,"href",S),1&n&&N!==(N=t[0].html_content+"")&&(j.innerHTML=N)},i:t,o:t,d(t){t&&u(e),t&&u(o),t&&u(r),P=!1,L()}}}function Z(t,n,e){const o=y();let{post:r}=n;return t.$$set=t=>{"post"in t&&e(0,r=t.post)},[r,o,()=>o("returnToList")]}class tt extends V{constructor(t){super(),I(this,t,Z,Y,c,{post:0})}}function nt(t){let n,e;return n=new X({props:{postSummaries:t[0]}}),n.$on("getPost",t[3]),{c(){q(n.$$.fragment)},m(t,o){z(n,t,o),e=!0},p(t,e){const o={};1&e&&(o.postSummaries=t[0]),n.$set(o)},i(t){e||(A(n.$$.fragment,t),e=!0)},o(t){M(n.$$.fragment,t),e=!1},d(t){D(n,t)}}}function et(t){let n,e;return n=new tt({props:{post:t[2]}}),n.$on("returnToList",t[4]),{c(){q(n.$$.fragment)},m(t,o){z(n,t,o),e=!0},p(t,e){const o={};4&e&&(o.post=t[2]),n.$set(o)},i(t){e||(A(n.$$.fragment,t),e=!0)},o(t){M(n.$$.fragment,t),e=!1},d(t){D(n,t)}}}function ot(t){let n,e,o,r;const c=[et,nt],s=[];function a(t,n){return t[1]?t[0]&&t[1]?1:-1:0}return~(e=a(t))&&(o=s[e]=c[e](t)),{c(){n=l("main"),o&&o.c(),m(n,"class","svelte-1qzafla")},m(t,o){i(t,n,o),~e&&s[e].m(n,null),r=!0},p(t,[r]){let i=e;e=a(t),e===i?~e&&s[e].p(t,r):(o&&(N(),M(s[i],1,1,(()=>{s[i]=null})),O()),~e?(o=s[e],o?o.p(t,r):(o=s[e]=c[e](t),o.c()),A(o,1),o.m(n,null)):o=null)},i(t){r||(A(o),r=!0)},o(t){M(o),r=!1},d(t){t&&u(n),~e&&s[e].d()}}}function rt(t,n,e){var o=this&&this.__awaiter||function(t,n,e,o){return new(e||(e=Promise))((function(r,c){function s(t){try{u(o.next(t))}catch(t){c(t)}}function i(t){try{u(o.throw(t))}catch(t){c(t)}}function u(t){var n;t.done?r(t.value):(n=t.value,n instanceof e?n:new e((function(t){t(n)}))).then(s,i)}u((o=o.apply(t,n||[])).next())}))};let r,{apiPre:c}=n,s=!0,i={};var u;return u=()=>o(void 0,void 0,void 0,(function*(){!function(){o(this,void 0,void 0,(function*(){fetch(`${c}posts/posts.json`).then((t=>t.json())).then((t=>{const n=document.getElementById("loading-placeholder");n&&n.remove(),e(0,r=t)}))}))}()})),v().$$.on_mount.push(u),t.$$set=t=>{"apiPre"in t&&e(5,c=t.apiPre)},[r,s,i,function(t){return o(this,void 0,void 0,(function*(){const{id:n}=t.detail,o=yield fetch(`${c}posts/html/post.${n}.html`).then((t=>t.text()));e(2,i=Object.assign(Object.assign({},r[n-1]),{html_content:o})),e(1,s=!1)}))},function(){e(1,s=!0)},c]}return new class extends V{constructor(t){super(),I(this,t,rt,ot,c,{apiPre:5})}}({target:document.getElementById("svelte-app")||document.createElement("null"),props:{apiPre:"/svelte-app/public/"}})}();
