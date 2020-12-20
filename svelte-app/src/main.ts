import App from './App.svelte';
declare const __buildEnv__;
declare const __apiPre__;

var app = new App({
	target: __buildEnv__ === 'dev' ? document.body : document.getElementById('svelte-app'),
	props: { apiPre: __apiPre__ },
});

export default app;
