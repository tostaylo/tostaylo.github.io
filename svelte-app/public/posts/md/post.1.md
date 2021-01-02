Is your website fast? Fast can mean many things in the context of a website. When does a user first see a page element appear on page-load? Do elements move around on the page during the browser's render? When a user clicks a button does it function immediately and as intended? When a user scrolls the page is it a smooth experience without [jank](https://developer.mozilla.org/en-US/docs/Glossary/Jank)? This criteria matters not only on page-load but also for the duration a user is interacting with your site.

There are many tools out there to help us measure user-centric metrics occurring on page-load. [Lighthouse](https://developers.google.com/web/tools/lighthouse), [Pingdom](https://www.pingdom.com/), and [WebPageTest](https://www.webpagetest.org/) to name a few (many use [Lighthouse](https://developers.google.com/web/tools/lighthouse) under the hood). We see less tooling allowing us to universally measure the performance of user interactions on our site after page-load, probably due to how each site has its own unique requirements and user interactions. Therefore, the developer (you!) must tailor post-page-load, user-centric performance testing, based on what your individual site requirements are.

I've been dabbling with building a testing framework that automates measuring the duration of certain user interactions on my website. My hope is this post provides a starting point to anyone who is considering doing the same with their own site.

## What to Test

Possible areas to prioritize testing focus.

**Changing routes in a SPA**

- Navigating from a landing page to the page the user will likely visit next

**Priority Interactions**

- Login
- View Cart
- Open Navigation Menu
- Add a Like or Favorite

## Define Baselines

After reading the article [The Psychology of Web Performance](https://blog.uptrends.com/web-performance/the-psychology-of-web-performance/) and it's excerpt from [Usability Engineering](https://www.nngroup.com/books/usability-engineering/) there are "[3 important limits](https://www.nngroup.com/articles/response-times-3-important-limits/) to keep in mind when optimizing web and application performance."

- A user feels a response is instantaneous at a .01 second.
- A user experiences uninterrupted flow with 1 second response times.
- Once response times exceed 10 seconds, user attention suffers breaking flow, and frustration rises.

When defining baselines to determine the success or failure of a test keep these limits in mind.

## What Tools Will We Use?

[Puppeteer](https://github.com/puppeteer/puppeteer) - a headless browser Node.js library.

- Navigates to web pages
- Enables communication with Chrome Developer Tools Protocol
- Interacts with our pages as a user would

[Chrome Developer Tools Performance Timeline](https://developers.google.com/web/tools/chrome-devtools/evaluate-performance/reference) - included with the [Chrome](https://www.google.com/chrome/) web browser

- Enables recording of events occurring in the browser on page load or user interactions

## Process

Assuming you've already installed Node and Puppeteer, let's look at a starting point for your Node.js script taken from [this blog post](https://addyosmani.com/blog/puppeteer-recipes/) by Addy Osmani.

```javascript
const puppeteer = require('puppeteer');

(async () => {
	const browser = await puppeteer.launch();
	const page = await browser.newPage();
	const navigationPromise = page.waitForNavigation();
	await page.goto('https://pptr.dev/#?product=Puppeteer&version=v2.1.1&show=outline');
	await page.setViewport({ width: 1440, height: 714 });
	await navigationPromise;
	const selector = 'body > sidebar-component > sidebar-item:nth-child(3) > .pptr-sidebar-item';
	await page.waitForSelector(selector);
	await page.tracing.start({ path: 'trace.json', screenshots: true });
	await page.click(selector);
	await page.tracing.stop();

	await browser.close();
})();
```

What you see above is the basic idea.

1. Start a new browser instance

2. Navigate to a URL

3. Start a trace (utilizes the [Chrome Dev Tools Protocol](https://chromedevtools.github.io/devtools-protocol/) to interact with the Performance Timeline tool)

4. Select an element on the page and click it

5. Stop the trace

6. Close the browser

The output of the trace file generated will look something like this:

```json
{
	"traceEvents": [
		{
			"args": { "data": { "type": "click" } },
			"cat": "devtools.timeline",
			"dur": 7761,
			"name": "EventDispatch",
			"ph": "X",
			"pid": 61589,
			"tdur": 7706,
			"tid": 775,
			"ts": 161565081204,
			"tts": 119022
		},
		{
			"args": {
				"cat": "devtools.timeline,rail",
				"dur": 18,
				"name": "Paint",
				"ph": "X",
				"pid": 61589,
				"tdur": 18,
				"tid": 775,
				"ts": 161565160795,
				"tts": 196604
			}
		}
	]
}
```

<figcaption>Here is the <a href="https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/edit">documentation</a> on the trace file and what the key-value pairs represent.</figcaption>

The trace file contains a `traceEvents` array. If we analyze these events we can identify the browser rendering events of "click", "Layout", "UpdateLayoutTree", "Paint", and "CompositeLayers". Each event has the values of

```
ts = time start;
dur = duration;
```

```
ts: The tracing clock timestamp of the event. The timestamps are provided at microsecond granularity.
There is an extra parameter dur to specify the tracing clock duration of complete events in microseconds.
All other parameters are the same as in duration events. The ts parameter indicate the time of the start of the complete event.
Unlike duration events, the timestamps of complete events can be in any order.
An optional parameter tdur specifies the thread clock duration of complete events in microseconds.
```

In order to accurately measure how long the user interaction has taken we need to look at the trace file and identify the browser rendering events. [This article](https://developers.google.com/web/fundamentals/performance/rendering) by Paul Irish explains the browser's rendering process as the "Pixel Pipeline."

- User Interaction Event (often JavaScript or CSS Animations)
- Style
- Layout
- Paint
- Composite

If we follow the order of events of the "Pixel Pipeline" we should be able to determine the total duration of the rendering process initiated from the user interaction by subtracting the "User Interaction" event start time ("click" event in my example) from the sum of the start time and the duration of the final "Composite" event.

```typescript
const totalDuration = finalCompositeStartTime + finalCompositeDuration - clickStartTime;
```

<figcaption>What your calculation may look like.</figcaption>

![Alt Text](https://dev-to-uploads.s3.amazonaws.com/i/2otnht0qsjllu42ghc0l.png)

<figcaption>If we upload the trace event file into the Chrome Developer Tools Performance Timeline tool we can view the events graphically.</figcaption>

![Alt Text](https://dev-to-uploads.s3.amazonaws.com/i/4n38r31xg6cdae4rtpqu.png)

<figcaption>Zoom in at the end of the timeline trace to find the "Composite Layers" event.</figcaption>

Of course, there are other modifications you can make to simulate users more realistically. Notably, simulating users on poor performing networks and CPU's. Puppeteer allows configuration of network and CPU throttling during the performance timeline traces.

```typescript
// Connect to Chrome DevTools
const client = await page.target().createCDPSession();

//Set Network Throttling property
await client.send('Network.emulateNetworkConditions', {
	offline: false,
	downloadThroughput: (200 * 1024) / 8,
	uploadThroughput: (200 * 1024) / 8,
	latency: 20,
});

// Set  CPU Throttling property
await client.send('Emulation.setCPUThrottlingRate', { rate: 4 });
```

<figcaption>Credit to <a href="https://michaljanaszek.com/blog/test-website-performance-with-puppeteer/#emulateSlowNetworkAndCPU" >this blog post</a> by Michal Janaszek for the code snippet.</figcaption>

That's all there is to it! Utilizing this tooling and knowledge of the browser's rendering process we can create baselines, execute user interactions, and analyze the resulting timings of user interactions.

Part 2 of this series will be an example project I've created which follows the methodology in this post.
