import App from './App.svelte';
declare const __buildEnv__: string;
declare const __apiPre__: string;

var app = new App({
	target:
		__buildEnv__ === 'dev' ? document.body : document.getElementById('svelte-app') || document.createElement('null'),
	props: { apiPre: __apiPre__ },
});

export default app;
