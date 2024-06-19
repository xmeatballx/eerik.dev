# What I learned setting up my first HomeLab

## Discovering the Joys of Self-Hosting

I recently embarked on a journey to create my own homelab server for media management, self-hosting, and virtual machine experiments. This was my first foray into the world of bare metal server setups, and it proved to be an incredibly rewarding experience.

It all started with an old Raspberry Pi I had lying around. I set it up with a few Docker services, including PiHole for ad-blocking and local DNS, as well as some media servers for my music and film collections. This introduction to the vast ecosystem of self-hosted applications was exhilarating. I really liked the idea of owning my own data and not relying on monolithic hosting providers like AWS to manage my servers and hosting.

## Upgrading to More Powerful Hardware

As I delved deeper into self-hosting, I quickly outgrew the limited hardware resources of the Raspberry Pi. I began scouring secondhand electronics stores for some corporate e-waste and stumbled upon an HP Elitedesk - a monster of a PC. While I had initially been on the lookout for an HP Elite Mini, a small-form-factor powerhouse, I was trigger happy, misread the posting, and accidentally ended up with the monolith from 2001: A Space Odyssey. 

This was a blessing in disguise though. After a few days I ended up with a hardware issue. Knowing basically nothing about computer repair, I opened up the case and was surprised to see that, blown up at this large scale, I was able to clearly identify the hard drive, RAM, and CPU. I did some hardware tests via the BIOS and narrowed it down to RAM. With absolutely no experience I carefully removed all of the RAM sticks and re-slotted them and it booted up like normal. Even though it was a complete hail mary and I have no idea why it worked, being able to diagnose that hardware issue and get past it in one night felt like a massive accomplishment.

## Exploring Hypervisors and Linux Containers

Before acquiring my homelab PC, I had experimented with a few hypervisors and opinionated homelab operating systems that came pre-packaged with the most popular services via Virtualbox. However, I found that these solutions didn't quite fit my needs. I didn't really need a full-fledged desktop environment just to spin up VMs and containers, and I wanted to improve my server administration skills.

That's when I discovered Proxmox Virtual Environment. I essentially copied my Raspberry Pi Docker setup to an Ubuntu server VM and picked up where I had left off. But as I delved deeper into the world of hypervisors and Proxmox, I encountered a common piece of advice: use LXCs (Linux Containers) instead of Docker on a virtual machine.

The reasoning behind this was that by creating a VM, you're adding an extra layer of abstraction, then you go and add another layer of abstraction with Docker. If you can run your services directly on the bare metal, why not do that? So, I set about creating a new LXC for each of the services I had previously run in Docker. This process was both educational and satisfying, as I got to see how the Docker Compose configuration related to the actual configuration files for each service, and the performance upgrade was quite noticeable.

## The Catalyst for Switching to Linux

This homelab journey came at a fortuitous time for me. I had been growing increasingly frustrated with my Windows laptop. Certain client projects I was involved with at work were originally developed on Unix-like systems, and certain aspects of the setup and npm scripts just didn't work on Windows. Additionally, I had been experiencing issues with my laptop's fans ramping up to maximum and CPU usage exploding due to background updates.

At the same time, I had been delving into customizing Linux distributions and window managers in my Proxmox VMs, and the lack of customization options on Windows was severely disappointing. The final straw came when I decided to upgrade WSL (Windows Subsystem for Linux) and ended up bricking my SSD.

The fear of losing all my data had been a major barrier to trying to dual-boot my laptop or install Linux directly. I spent a few weeks messing with all kinds of linux distros, I tried Fedora Workstation, Manjaro, Arch, Debian, and ended up finding out that I really like NixOS.  I decided to take the plunge and install it on my laptop. It's a decision I haven't looked back on since, as I've been thoroughly enjoying the freedom and customization that Linux offers. (blog post coming soon)

## The Homelab Learning Experience

Setting up my first homelab has been an incredibly rewarding experience. It has not only introduced me to the world of self-hosting and the joys of owning my own data, but it has also been a catalyst for my transition to Linux as a daily driver. Along the way, I've learned valuable lessons about server administration and hypervisors.

The homelab journey is an ongoing process, and I'm excited to see where it takes me next. Whether you're looking to dive into self-hosting, improve your server administration skills, or simply explore the world of Linux, setting up a homelab can be a transformative experience.
