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
