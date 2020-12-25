Is your website fast? Fast can mean many things in the context of a website. Metrics. Website speed is often measured on page load. During page load, when does a user see a page element? Does that element move? When the user clicks a button does it function? These things matter not only on page load but also for the duration a user is interacting with your site.

There are many tools out there to help us measure user-centric metrics which occur on page load. Lighthouse, PageSpeedInsights, and WebpageSpeedTest to name a few. We see less of tooling which allows us to universally measure all user interactions on our site. This is due to how each site has its own unique requirements and user interactions. It is up to us to tailor post-load, user-centric performance testing based on what our site requirements are.

## What to Test

- Changing routes in a SPA
- Common User Flows
- Priority Interactions

## Define Baselines

After reading the article [The Psychology of Web Performance](https://blog.uptrends.com/web-performance/the-psychology-of-web-performance/) and it's excerpt from [Usability Engineering](https://www.nngroup.com/books/usability-engineering/) there are " [3 important limits](https://www.nngroup.com/articles/response-times-3-important-limits/) to keep in mind when optimizing web and application performance."

- A user feels a response is instantaneous at a .01 second.
- A user experiences uninterrupted flow with 1 second response times.
- Once response times exceed 10 seconds, user attention suffers breaking flow and frustration rises.

When defining baselines to determine success or failures keep these limits in mind.

## What Tools Will We Use?

[Puppeteer](https://github.com/puppeteer/puppeteer) - Headless browser Node.js library.

- Navigates to pages
- Enables interaction with Chrome Developer Tools Protocol
- Interacts with our pages as a user would

[Chrome Developer Tools Performance Timeline](https://developers.google.com/web/tools/chrome-devtools/evaluate-performance/reference)

- Enables recording events occuring in the browser on page load or user interactions.

![Perf Timeline View](/assets/images/chrome-perf-click.webp)

## Process

Assuming you've already installed Node and Puppeteer, let's look at a starting point for your Node.js script taken from [this great blog post](https://addyosmani.com/blog/puppeteer-recipes/) by Addy Osmani .

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

2. Navigate to a url

3. Start a trace

4. Select an element on the page and click it

5. Stop the trace

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

<figcaption>Here is the [documentation](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/edit) on the trace file and what the key value pairs represent.</figcaption>

The trace file contains a traceEvents array. If we analyze these events we can identify the browser rendering events of "click", "Layout", "UpdateLayoutTree", "Paint", "CompositeLayers". Each event has the values of

<pre>ts = time start; dur = duration</pre>

<pre>ts: The tracing clock timestamp of the event. The timestamps are provided at microsecond granularity.
There is an extra parameter dur to specify the tracing clock duration of complete events in microseconds. All other parameters are the same as in duration events. The ts parameter indicate the time of the start of the complete event. Unlike duration events, the timestamps of complete events can be in any order. An optional parameter tdur specifies the thread clock duration of complete events in microseconds.</pre>

In order to accurately measure how long the user interaction has taken we need to look at the trace file and identify the browser rendering events. [This article](https://developers.google.com/web/fundamentals/performance/rendering) by Paul Irish explains it as the "Pixel Pipeline."

- User Interaction Event (often JavaScript or CSS Animations)
- Style
- Layout
- Paint
- Composite

If we follow the order of events of the "pixel pipeline" we should be able to determine the total duration of the rendering process initiated from the user interaction by subtracting the "User Interaction" event start time (click in our case) from the sum of the start time and the duration of the final "Composite" event.

![Perf Timeline View](/assets/images/chrome-perf-full.webp)

<figcaption>If we upload the trace event file into the Chrome Developer Tools Performance Timeline tool we can view the events graphically.</figcaption>

More coming soon.....
