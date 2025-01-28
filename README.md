# oxiproxy
### a simple and elegant reverse-proxy solution for self-hosted systems.

## What's going on?
I wound up dropping this project to focus on others, as well as school. I'd like to come back to this at some point, but I'm not sure.
Frankly, Caddy is filling my needs quite well, so I don't really *need* an alternative.

## Why?
I've been meaning to learn more Rust recently, so I figured I'd start a new project. As it turns out, I've also been thinking about replacing my use of Caddy for my personal homelab, in regards to reverse-proxying (taking in a generic domain request and routing to the appropriate service, at home). Works out!

## What's the goal?
What I'm aiming to achieve here is essentially just a reverse-proxy, controlled by one simple config file. Caddy is a fantastic tool, and I appreciate how straightforward the configuration is, but I feel like it could be more straightforward, in my use-case. [Do one thing, and do it well.](https://en.wikipedia.org/wiki/Unix_philosophy)
