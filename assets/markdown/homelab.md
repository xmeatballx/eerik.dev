# What I learned setting up my first HomeLab

## Discovering the Joys of Self-Hosting

After a month of Reddit suggesting posts from a community called "Homelab" based on my other interests, I gave in to curiosity and scanned through some posts. I saw a lot of familiar server and network terminology that I picked up from hosting websites and tinkering with virtual machines and containers except instead of renting servers from a provider like AWS, everything was hosted on self hosted. There was a lot of talk of scoring old mini PC's or server racks for cheap from office building liquidations, and self-hosted alternatives to popular services which keep your data private and safe. I was excited by the thrifty, DIY spirit of all of it so I decided to set up my own homelab server for media management, self-hosting, and virtual machine experiments. It was admittedly pretty terrifying to be working on bare metal for the first time, but it is proving to be an incredibly rewarding experience.

It all started with an old Raspberry Pi I had lying around. I set it up with a few Docker services, including PiHole for ad-blocking and local DNS, media servers for watching tv/movies and a smart home app to control some smart lightbulbs and electronics around my house. The fact that I could do all of this just inside my home network, without relying on any monolithic hosting providers or exposing anything to the internet, was enough to get me hooked. 

## Upgrading to More Powerful Hardware

As I delved deeper into self-hosting, I quickly outgrew the limited hardware resources of the Raspberry Pi. I began scouring secondhand electronics stores for some corporate e-waste and stumbled upon an HP Elitedesk - a monster of a PC. While I had initially been on the lookout for an HP Elite Mini, a small-form-factor powerhouse, I was trigger happy, misread the posting, and accidentally ended up an awkward, heavy 14"x4"x4" black metal PC tower. 

This was a blessing in disguise though. After a few days I ended up with a hardware issue. Knowing basically nothing about computer repair, I opened up the case and was surprised to see that, blown up at this large scale, I was able to clearly identify the hard drive, RAM, and CPU. I did some hardware tests via the BIOS and narrowed it down to RAM. With I carefully removed all of the RAM sticks and re-slotted them and it booted up like normal. Even though it was a complete hail mary and I have no idea why it worked, being able to diagnose that hardware issue and get past it in one night felt like a massive accomplishment, and had I gotten the one that was 1/4 its size, it likely would have been much riskier and more time consuming.



## Exploring Hypervisors and Linux Containers

Before acquiring my homelab PC, I had experimented with a few hypervisors and opinionated homelab operating systems that came pre-packaged with the most popular services via Virtualbox. However, I found that these solutions didn't quite fit my needs. I didn't really need a full-fledged desktop environment just to spin up VMs and containers, and I wanted to improve my server administration skills.

That's when I discovered Proxmox Virtual Environment. I essentially copied my Raspberry Pi Docker setup to an Ubuntu server VM and picked up where I had left off. But as I delved deeper into the world of hypervisors and Proxmox, I encountered a common piece of advice: use LXCs (Linux Containers) instead of Docker on a virtual machine.

The reasoning behind this was that by creating a VM, you're adding an extra layer of abstraction, then you go and add another layer of abstraction with Docker. If you can run your services directly on the bare metal, why not do that? So, I set about creating a new LXC for each of the services I had previously run in Docker. This process was both educational and satisfying, as I got to see how the Docker Compose configuration related to the actual configuration files for each service, and the performance upgrade was quite noticeable.

## Practical uses of Homelabs for web development

Having more resources available on the new machine, I started experimenting with ways a homelab could improve my experience working on web development projects. The obvious one is hosting your own websites and API's. I moved my side projects to Proxmox containers with the help of Cloudflare tunnels, which provide a safe connection between self hosted servers and the internet. That combined with the fact that all of my containers isolated from eachother and the host OS and that I'm not dealing with massive amounts of traffic has made self hosting an easy and sustainable solution so far.  

I kind of came across this other use out of neccesity but its come in handy time and time again. I was working on a client project that was using a mySQL database, not uncommon for me and so I already had a mySQL docker container on my machine, but also not uncommon I was running into some networking issues having some services running on docker and some locally. The staging and production servers worked fine, both were pointing to a hosted mySQL database on a separate server. In order to reduce points of failure involving docker and managing multiple runtimes on the same machine, I found a template that let me create a mySQL container in Proxmox in a couple commands, pointed my local project at the IP address and instantly everything worked. I now have Redis, mySQL, MongoDB, and a handful other services I use often hosted on my local network. I can share caches and db servers between apps using different namespaces, never have to check whats running or worry about occupying important ports on my personal machine, and it helps simulate environments where not all services are being run on the same host.

## The Catalyst for Switching to Linux

This homelab journey came at a fortuitous time for me. I had been growing increasingly frustrated with my Windows laptop. Certain client projects I was involved with at work were originally developed on Unix-like systems, and certain aspects of the setup and npm scripts just didn't work on Windows. Additionally, I had been experiencing issues with my laptop's fans ramping up to maximum and CPU usage exploding due to background updates.

At the same time, I had been delving into customizing Linux distributions and window managers in my Proxmox VMs, and the lack of customization options on Windows was severely disappointing. The final straw came when I decided to upgrade WSL (Windows Subsystem for Linux) and ended up frying my SSD :(

The fear of losing all my data had been a major barrier to trying to dual-boot my laptop or install Linux directly. I spent a few weeks messing with all kinds of linux distros, I tried Fedora Workstation, Manjaro, Arch, Debian, and ended up finding out that I really like NixOS.  I decided to take the plunge and install it on my laptop. It's a decision I haven't looked back on since, as I've been thoroughly enjoying the freedom and customization that Linux offers. (blog post coming soon)

## The Homelab Learning Experience

Setting up my first homelab has been an incredibly rewarding experience. It has not only introduced me to the world of self-hosting and the joys of owning my own data, but it has also been a catalyst for my transition to Linux as a daily driver. Along the way, I've learned valuable lessons about server administration and hypervisors.

The homelab journey is an ongoing process, and I'm excited to see where it takes me next. Whether you're looking to dive into self-hosting, improve your server administration skills, or simply explore the world of Linux, setting up a homelab can be a transformative experience.
