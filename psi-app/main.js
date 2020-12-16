function init() {
	document.getElementById('psi-script')?.remove();
	const head = document.querySelector('head');
	const script = document.createElement('script');
	script.src = `./psi-app/index.js?${Date.now()}`;
	script.type = 'module';
	script.id = 'psi-script';
	head.appendChild(script);
}

init();
