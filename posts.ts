function getPosts() {
	return fetch('./posts.json')
		.then((response) => response.json())
		.then((data) => data);
}

async function render() {
	const posts = await getPosts();
	const listItems = posts.map((item: { name: string }) => `<li>${item.name}</li>`).join('');
	const postUL = document.getElementById('post-titles');
	if (postUL) postUL.innerHTML = listItems;
}

render();
