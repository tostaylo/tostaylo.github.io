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

I wasn't sure where to start. Reusable components? State management? At the most fundamental level my framework needed to be able to create a DOM tree. My preference was not only create a DOM tree, but do so declaratively rather than imperatively. Let's take a look at the opposing methods.

```typescript
const body = document.getElementByTagName("body");
const header = document.createElement("header");
const h1 = document.createElement("h1");
h1.innerText = "I am an h1!";
h1.id = "anH1";
header.appendChild(h1);
body.appendChild(header);
```

<figcaption>Imperative DOM manipulation</figcaption>

vs.

```typescript
App.render(
  <Header>
    <HeadingOne id="anH1" content="I am an h1!" />
  </Header>,
  document.getElementByTagName("body")
);
```

<figcaption>Declarative DOM composition</figcaption>

Do you see the difference? The former contains all the implementation details of creating the DOM line by line. The latter is an abstraction which hides the details of the work being done to create the DOM.

Let's talk about the DocumentObjectModel. It can be represented with a tree data structure.

![Image of tree data structure](/assets/images/tree.svg)

A tree is a data structure made of nodes connected by edges. The root node is the parent node at the top. Each level down the tree are children nodes. At the bottom of the tree are the leaves where no node has a descendant node.

The DOM is described well with a tree data structure. It has a root "html" node and then descendant nodes some of which have sibling nodes.

![Image of html tree](/assets/images/html_tree.svg)

I needed to be able to create an in-memory representation of the DOM and then have a procedure which builds the DOM in the browser based off this representation. I came up with this for the in-memory DOM representation.

```rust
pub struct Element {
    pub html_type: String,
    pub props: Props,
}

pub struct Props {
    pub children: Option<Vec<Element>>,
    pub text: Option<String>,
}

let div = rust_fel::Element::new(
    "div".to_owned(),
    rust_fel::Props {
      children: None,
    }
  );

let head = rust_fel::Element::new(
    "head".to_owned(),
    rust_fel::Props {
      children: None,
    }
  );

let body = rust_fel::Element::new(
    "body".to_owned(),
    rust_fel::Props {
      children: Some(vec![div]),
    }
  );

let html = rust_fel::Element::new(
  "html".to_owned(),
  rust_fel::Props {
    children: Some(vec![head, body]),
    },
  );
```

A rust_fel::Element has an html type (it's html tag name) and a children vector. Observe how "html" is the root node of the tree. On level 1 the two descendants of "html", "head" and "body", are siblings. "body" has a descendant called "div" which is on level 2 of the tree and is a leaf node.

More coming soon...
