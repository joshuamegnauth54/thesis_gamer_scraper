# What is this project?

Hi! I wrote **Gamer Scraper** to gather data for my thesis. I initially planned to write my scraper in Python. I dithered for weeks by working on small data science projects which I ported to both Python and R.

A crazy idea struck soon after completing my first project: What if I wrote my scraper in [Rust](https://rust-lang.org)---a language I adore despite my ~rusty~ paltry skills?

Call my coup uninspired and a further manifestation of dithering. I won't blame you. After a few days of work, however, I managed to scrape together a working, uh, scraper!

The goal of the scraper is to gather posters on a subset of gamer subreddits along with other subs frequented by said users. The final network is a sample of the gamers and their dispersion on [Reddit](https://reddit.com) in terms of subs while the transpose shows the connections between the gamers. 

# Implementation

I implemented a small, incomplete, and likely wrong subset of the [Pushshift API](https://pushshift.io) for Reddit using reqwests. My program works yet is flawed in many ways. Errors could be handled better as I mostly just consume them. Functionality bleeds into two unrelated modules: the Pushshift implementation and the actual scraper. The function to perform the scraping itself is a terrible mess.

