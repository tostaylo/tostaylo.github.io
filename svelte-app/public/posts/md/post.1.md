Is your website fast? Fast can mean many things in the context of a website. Metrics. Website speed is often measured on page load. During page load, when does a user see a page element? Does that element move? When the user clicks a button does it function? These things matter not only on page load but also for the duration a user is interacting with your site.

There are many tools out there to help us measure user-centric metrics which occur on page load. Lighthouse, PageSpeedInsights, and WebpageSpeedTest to name a few. We see less of tooling which allows us to universally measure all user interactions on our site. This is due to how each site has its own unique requirements and user interactions. It is up to us to tailor post-load, user-centric performance testing based on what our site requirements are.

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

Here is the [documentation](https://docs.google.com/document/d/1CvAClvFfyA5R-PhYUmn5OOQtYMH4h6I0nSsKchNAySU/edit) on the trace file and what the key value pairs represent.

More coming soon.....
