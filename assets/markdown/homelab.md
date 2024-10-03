# What I learned setting up my first HomeLab

## Discovering the Joys of Self-Hosting

After a month of Reddit suggesting posts from a community called "Homelab" based on my other interests, I gave in to curiosity and scanned through some posts. I saw a lot of familiar server and network terminology that I picked up from hosting websites and tinkering with virtual machines and containers except instead of renting servers from a provider like AWS, everything was hosted on on some sort of home computer or server. There was a lot of talk of scoring old mini PC's or server racks for cheap from office building liquidations, and self-hosted alternatives to popular services which keep your data private and safe. I was excited by the thrifty, DIY spirit of it all so I decided to set up my own homelab server for media management, self-hosting, and virtual machine experiments. It was admittedly pretty intimidating to be working on bare metal for the first time, without the separation or abstraction that Docker and VPS's provide. Its just you and a server, in a room. You can see it, touch it, and potentially render it inoperable by running the wrong command. Luckily, comitting and giving it a shot is proving to be an incredibly rewarding experience.

It all started with an old Raspberry Pi I had lying around. I set it up with a few Docker services, including PiHole for ad-blocking and local DNS, media servers for watching tv/movies and a smart home app to control some smart lightbulbs and electronics around my house. The fact that I could do all of this just inside my home network, without relying on any monolithic hosting providers, was enough to get me hooked. 

## Upgrading to More Powerful Hardware

Eventually the limited hardware resources of the Raspberry Pi proved insufficient for the amount of exploration I wanted to do with different self hosted applications. I began scouring secondhand electronics stores for some corporate e-waste and stumbled upon an HP Elitedesk. While I had initially been on the lookout for an HP Elite Mini, a small but powerful mini PC, I was trigger happy, misread the posting, and accidentally ended up with an awkward, heavy, metal chassis the size of a large briefcase. 

This was a blessing in disguise though. Within a few days I had a hardware issue. Knowing basically nothing about computer repair, I opened up the case and was surprised to see that, blown up at this large scale, I was able to clearly identify the hard drive, RAM, and CPU. I did some hardware tests via the BIOS and narrowed it down to RAM. I carefully removed all of the RAM sticks and re-slotted them and it booted up like normal. Even though it was a complete hail mary, being able to diagnose that hardware issue and get past it in one night felt like a massive accomplishment. Had I gotten the machine that was 1/4 its size, it likely would have been much riskier and more time consuming process.


## Exploring Hypervisors and Linux Containers

Before acquiring my homelab PC, I experimented with a few hypervisors and opinionated homelab operating systems that came pre-packaged with popular services. However, I found that these solutions didn't quite fit my needs. I didn't really need a full-fledged desktop environment just to spin up VMs and containers, and I wanted to improve my server administration skills.

That's when I discovered Proxmox Virtual Environment. Proxmox VE is a hypervisor operating system for bare metal servers, it allows you to easily spin up and manage virtual machines, containers, file systems, and networks. I moved some docker containers from my Raspberry Pi onto a VM in proxmox, but while troubleshooting I encountered a common piece of advice: don't use docker if you dont have to.

The reasoning behind this was that by creating a VM, you're adding a layer of abstraction on top of the host machines resources. Adding the Docker on top of that is an abstraction of an abstraction. While the actual performance overhead of that may be neglibible, if you can run everything with less abstraction, why not do that? On Proxmox the best way to do that is with Linux Containers (LXCs). They use a Linux-specific method of virtualization where parts of the host operating system provides a shared kernel for a number of isolated environments. After learning this I created a new LXC for each of the services I had previously run in Docker. I was actually surprised by the improvements in overall resource consumption.

## Practical uses of Homelabs for web development

Having more resources available on the new machine, I started experimenting with ways a homelab could improve my experience working on web development projects. The obvious one is hosting your own websites and API's. I moved my side projects to Proxmox containers with the help of Cloudflare tunnels, which provide a safe connection between self hosted servers and the internet. That combined with the fact that all of my containers are isolated from eachother and the host OS and that I'm not dealing with massive amounts of traffic has made self hosting an easy and sustainable solution so far.  

I came across this other use out of neccesity but its comes in handy all the time. I was working on a client project that was using a mySQL database, not uncommon for me, so I already had a mySQL docker image on my machine, but also not uncommon I was running into some networking issues having some services running on docker and some locally. The production API worked fine, and the production mySQL database was hosted on a separate server. I wanted to recreate the working production environment as close as possible to rule out points of failure, so I found a template that let me create a mySQL container in Proxmox in a couple commands, pointed my local project at the IP address and instantly everything worked. I now have Redis, mySQL, MongoDB, and a handful other services I use often hosted on my local network. I can split caches and db servers up between apps using different namespaces, never have to check whats running or worry about occupying important ports on my personal machine.

## The Catalyst for Switching to Linux

This homelab journey came at a fortuitous time for me. I had been growing increasingly frustrated with my Windows laptop. Certain client projects I was involved with at work were originally developed on Unix-like systems, and aspects of the setup and npm scripts just didn't work on Windows. Additionally, I had been experiencing issues with annoying background updates randomly spiking my CPU.

At the same time, I had been delving into customizing Linux distributions and window managers in my Proxmox VMs, and the lack of customization options on Windows was severely disappointing. The final straw came when I decided to upgrade WSL (Windows Subsystem for Linux) and ended up frying my SSD :(

The fear of losing all my data had been a major barrier to trying to dual-boot my laptop or install Linux directly. I spent a few weeks messing with all kinds of linux distros. I tried Fedora Workstation, Manjaro, Arch, Debian, and ended up finding out that I really like NixOS. I decided to take the plunge and install it on my laptop. It's a decision I haven't looked back on since, as I've been thoroughly enjoying the freedom and customization that Linux offers. (blog post coming soon)

## The Homelab Learning Experience

Setting up my first homelab has been an incredibly rewarding experience. It has not only introduced me to the world of self-hosting and the joys of owning my own data, but it has also been a catalyst for my transition to Linux as a daily driver. Along the way, I've learned valuable lessons about server administration and hypervisors.

The homelab journey is an ongoing process, and I'm excited to see where it takes me next. Whether you're looking to dive into self-hosting, improve your server administration skills, or simply explore the world of Linux, setting up a homelab can be a rewarding experience.
