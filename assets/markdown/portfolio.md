# I over-engineered my portfolio

A common problem I run into is wanting to test out new technologies but not knowing what to build. The best path forward is usually to start an easy goal and keep things simple. While working with new tech hardly ever feels “simple”, the point is to work through some initial hurdles and end up with a working product. 

Portfolio sites are low stakes. You get to be your own boss, set your own deadlines, and be your most invested user. They’re usually also not that complex. They should be able to show some static content, and maybe support form submissions, so as long as those work the implementation is completely up to you.

## Special cases

1. If you don’t already have a live portfolio then use what you know. Deploying your portfolio site for the first time is just one step down an endless road of iterations. Technology and design trends are always changing and so will your tastes. Just make sure you have something live before getting too experimental with it.
2. If you’re shooting for super creative UI/UX roles, want to win awards and gain clout, then maybe show off a little bit. There are cases for having impressive and ultra interactive portfolio sites, but for a majority of developers it’s probably not necessary and not worth the potential bugs and UX hindrances if done sub-optimally.

## Invest your effort wisely

In past iterations of my portfolio, I was very invested in creating an impressive UI with webGL, 3D, and fancy animations. While slightly over-engineered this was more out of the pursuit of over-selling myself as a “front end guru” in the most generic way possible. I didn’t need scroll hijacking, view transitions, or particle systems to communicate my experience and I felt like the in-your-face vanity of it all didn’t reflect my personal tastes.

I would liken it to a resume. You can create a verbose 4 page resume that lists every single skill you used on every project and positions you as the best thing to ever happen to software development, but compared to hundreds of identical resumes all you’re doing is giving the recruiter a headache. The resumes that stick out are succinct and list business impact in a pleasing and accessible way.

If you approach your portfolio with a similar mindset, it gives you a lot of space to experiment behind the scenes. In my case, I decided to try to build a clean and modern feeling portfolio using as little JavaScript as possible.

## Why avoid JavaScript?

Working with the big JavaScript frameworks I got very comfortable in high levels of abstraction. In NextJS for example, whether you’re building a single page application, server rendering pages, or doing some combo of the two, the source code is essentially the same with some minor syntax differences. All the complexity involved in these of these approaches is out of sight and out of mind. But it does exist somewhere, in some obfuscated minified node_modules file, and it’s eventually affecting load times and using resources in the browser.

This is negligible at scale, and made up for with better developer experience. Readymade solutions make it easier to trust that the website or app is going to do what you need it to do, and allow you to focus on intricate business logic while rarely having to think about how the content is getting from the server to the client. But for a simple portfolio site, its like buying a $4000 John Deere riding lawnmower to prune a houseplant. 

## Setting clear goals

It may seem contradictory that I’m advocating for over-engineering a portfolio but dismissing JavaScript frameworks as overkill. The thing is, in environments where experimentation is permitted, there can be such a thing as productive overkill. It comes down to understanding your goals and finding new and novel ways of accomplishing them.

My goals were as follows:

- Give a general overview of my background & skills
- Highlight some interesting projects
- Make it fast and easy to use
- Host some blog posts

So I knew I needed some pages with static content and maybe some dynamic components like a carousel to display content. I had seen people sharing intriguing results doing simple HTML template server rendering using a low level language and this felt like the perfect opportunity to try it out. I was also curious to try Rust and HTMX because at the time they were inescapable fixtures of development discourse and I wanted to see what made them so special. 

## Rust? Seriously?

You can build ridiculously complex and powerful software with Rust, so why squander it on a simple website? I could have built the same exact site with the same or even better performance using HTML, CSS, and JavaScript but I would have learned nothing. Heck I would have written much cleaner and better optimized code had I used the basics yet I was able to plow the baby steps of the Rust ecosystem learning curve without biting off way more than I could chew. I learned how to write Rust code that actually compiles, manage dependencies with cargo, spin up an API, compile some templates and send them to the browser, and how to build and deploy a Rust app. And that felt really rewarding in the end. 

## Unproductive over-engineering

Now, regarding HTMX, this is where I got caught up in unproductive over-engineering. My pitfall was approaching it with the mindset of “How can I use this for something cool?” instead of “How does this contribute to my goals?”. So I went through dozens of iterations of UI components that could and should have been done using plain old <a> tags and other browser standards. I ended up ripping out all of the HTMX in the end but I did eventually figure out how to use it productively. The advantage of HTMX in this situation is that you are talking directly to the server from your markup, that works great for something like an admin backend where I’d want to do things like editing content, uploading files, and creating a simple analytics dashboard.

## I’d do it again

Basic hello world programs don’t teach you much and social media clones & to-do apps just sit unused for the most part. The next time I get an inkling to try out an unfamiliar technology, I’ll start by rebuilding my portfolio site. It’s an achievable project that leaves you lots of room to learn and experiment, and it’s something I will actually maintain and use. That motivation makes all the difference when pushing through the inevitable learning curves of new tech.
