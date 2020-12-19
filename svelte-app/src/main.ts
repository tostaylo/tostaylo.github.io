import App from './App.svelte';

var app = new App({
	target: __buildEnv__ === 'dev' ? document.body : document.getElementById('svelte-app'),
});

export default app;
