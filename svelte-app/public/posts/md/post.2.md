What does it take to build a front-end framework? That was one of the questions on my mind during a recent development project brainstorming session. I knew I wanted to use Rust to create something new and I knew I needed to update my portfolio website. This led me to start researching how I could use Rust to build a website. At first I thought I may do a server rendered website with Rust and sprinkle some Javascript on top for the client interactions. The Rust ecosystem already contained some well know web frameworks I could use. However, I discovered I could give Rust a compilation target of Web Assembly which browsers understand! I could write the whole client in Rust and not touch Javascript myself (although under the hood JS would be needed to provide bindings for Rust).

Great! My mission had started. But one more thing occurred to me. It would be nice to have a client-side framework to write Rust with. The Rust ecosystem had one such client-side framework called Yew. Instead of choosing Yew I decided to take this opportunity and research how front-end frameworks work. Afterwords, I would try to develop one with Rust and Web Assembly. The mission just got way bigger!

## What does a front-end framework do?

Many things. Some do much more than others. Here are some characteristics I have observed.

- Provides structure to your code
- Manipulates DOM elements declaratively rather than imperatively
- Manages state
- Components
- Updates the DOM intelligently
- Security
- Routing

## What did I want out of a front-end framework?

I looked to the greats like React and Vue for inspiration.

- Reusable components
- State management at the component level
- Intelligent reactivity to state updates
- HTML-like syntax for binding state with HTML elements

## Where to start? Building the DOM

I wasn't sure where to start. Reusable components? State management? At the most fundamental level my framework needed to be able to create a DOM tree. Not only create a DOM tree, but do so declaratively rather than imperatively. Let's take a look at the opposing methods.

```typescript
const body = document.getElementByTagName('body');
const header = document.createElement('header');
const h1 = document.createElement('h1');
h1.innerText = 'I am an h1!';
h1.id = 'anH1';
header.appendChild(h1);
body.appendChild(header);
```

<figcaption>Imperative DOM manipulation</figcaption>

vs.

```typescript
App.render(
	<Header>
		<H1 id="anH1" content="I am an h1!" />
	</Header>,
	document.getElementByTagName('body')
);
```

<figcaption>Declarative DOM composition</figcaption>

The latter is an abstraction which hides the details of the work being done to render elements to the DOM.

More coming soon...
