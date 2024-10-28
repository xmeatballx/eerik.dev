# My NixOS Laptop Setup

I like customizing things. So when the hard drive on my laptop died, wiping my Windows install that I had been running for 3 years and forcing me to start from scratch, I decided to do some research to find a platform where I could craft my ideal development machine. I found that in NixOS, a Linux distro that gives you a declarative way to manage all of your packages, services, users, and more. This also allows you to easily port your entire system to a new machine if necessary. This is less a tutorial and more a collection of snippets showcasing what can be done on a laptop with a declarative OS.

## Utility

While not the most exciting, it would be hard to get anything done without the proper battery saving and graphics rendering utilities.

### TLP

Linux distros can drain a laptop battery pretty quickly. Thankfully the NixOS github provides [example configurations](https://github.com/NixOS/nixos-hardware) for many common laptop models. Many of them feature some variation of this TLP setup, which allows you to throttle CPU performance to preserve battery life when not charging, and alter charging behavior to improve the battery longevity. 

```nix
  services.tlp = {
      enable = true;
      settings = {
        CPU_SCALING_GOVERNOR_ON_AC = "performance";
        CPU_SCALING_GOVERNOR_ON_BAT = "powersave";

        CPU_ENERGY_PERF_POLICY_ON_BAT = "power";
        CPU_ENERGY_PERF_POLICY_ON_AC = "performance";

        CPU_MIN_PERF_ON_AC = 0;
        CPU_MAX_PERF_ON_AC = 100;
        CPU_MIN_PERF_ON_BAT = 0;
        CPU_MAX_PERF_ON_BAT = 20;

       #Optional helps save long term battery health
       START_CHARGE_THRESH_BAT0 = 40; # 40 and below it starts to charge
       STOP_CHARGE_THRESH_BAT0 = 80; # 80 and above it stops charging

      };
  };
```

### Graphics

Also found in the common nix configurations is this graphics configuration. My laptop provides a virtual Intel graphics card that runs on the CPU and an Nvidia integrated graphics card. Depending on the setup you chose, you can utilize one or the other or both in varying ways. I chose the prime offloading setup, which allows you to offload certain applications to the Nvidia GPU. I don’t do a lot of gaming etc, I mostly have found the offloading useful for hardware accelerated terminals and browsers which do a lot rendering and can clog up the CPU.

```nix
  hardware.nvidia = {
    modesetting.enable = true;
    package = config.boot.kernelPackages.nvidiaPackages.stable;
    powerManagement.enable = true;
    open = false;
    prime = {
      offload = {
        enable = true;
        enableOffloadCmd = true;
      };
      intelBusId = "PCI:0:2:0";
      nvidiaBusId = "PCI:1:0:0";
    };
  };
```

## Theme

High level theming is so straightforward in Nix. It's nice to know that if I ever upgrade my machine my theme will just work.

### GTK theme

The Catppuccin GTK theme can be included as a package in home-manager

```nix
home.packages = with pkgs; [    
    (catppuccin-gtk.override {
      size = "compact";
      tweaks = [ "rimless" "black" ]; # You can also specify multiple tweaks here
      variant = "mocha";
    })
];
```

### Background

To set the background an image just needs to be added to the home directory with the name “.background-image”

```nix
  home.file = {
    ".background-image" = {
      source = "${wallpaper}";
    };
  };
```

### Global Colors

Having a global colors.nix file can be super helpful, often different aspects of a system like window managers, terminals, zsh/bash etc. will require different theme configurations, but as long as you can configure them from Nix (which you most likely can) you can apply your global color theme just by importing colors.nix and using the variables

```nix
{
  rosewater = "#f5e0dc";
  peach = "#fab387";
  lavender = "#b4befe";
  green = "#a6e3a1";
  blue = "#89b4fa";
  text = "#cdd6f4";
  overlay0 = "#6c7086";
  surface1 = "#45475a";
  base = "#1e1e2e";
}
```

## i3 Window Manager

### Base config

After doing a few i3 custom builds in VMs, I found it pretty annoying to navigate my huge config file to find a specific declaration. Co-locating keybindings with the theme and i3status can get unruly. Splitting concerns into files and importing them in the main config helps me find things much quicker and lets me scale my config without worrying about bloat.

```nix
{ config, pkgs, lib, ... }:
{
  imports = [
    ./keybindings.nix
    ./colors.nix
    ./bar.nix
    ./i3status.nix
    ./startup.nix
  ];

  xsession.windowManager.i3 = {
    enable = true;
    config = {
      terminal = "kitty";
      gaps = {
        inner = 5;
        outer = 1;
        smartGaps = true;
      };
      focus = {
        newWindow = "focus";
      };
    };
    extraConfig = ''
      for_window [class="Google-chrome"] border pixel 0
    '';
  };
}
```

### Keybindings

My personal keybindings are nothing special but It is worth noting that with the [xev](https://mynixos.com/nixpkgs/package/xorg.xev) package, you can find the name of any key on your laptop keyboard (even the non-standard ones). Here I set up the built in volume and brightness keys on my keyboard to function as you would expect.

```nix
{ config, pkgs, ... }:
let 
  mod = "Mod4";
in
{
  xsession.windowManager.i3.config.modifier = "Mod4";
  xsession.windowManager.i3.config.keybindings = with pkgs; {
        ...
    "XF86AudioLowerVolume" = "exec pamixer -d 5";
    "XF86AudioRaiseVolume" = "exec pamixer -i 5";
    "XF86AudioMute" = "exec pamixer -t";

    "XF86MonBrightnessUp" = "exec brightnessctl set 10%+";
    "XF86MonBrightnessDown" = "exec brightnessctl set 10%- -n 100";
  };
}
```

### i3status (menu bar)

Not much to say about this besides look at that declarative beauty. I much prefer this to the standard i3status syntax. I find this to be a completely reasonable status bar for laptop use

```nix
{config, ...}:

{
  programs.i3status = {
    enable = true;
    enableDefault = false;
    modules = {
      "tztime local" = {
        position = 1;
        settings = {
          format = "%I:%M%p %m/%d/%Y";
        };
      };
      "volume master" = {
        position = 2;
        settings = {
          format = "♪ %volume";
          format_muted = "♪ muted (%volume)";
          device = "default";
        };
      };
      "battery all" = {
        position = 3;
        settings = {
          format = "%status %percentage";
          status_bat = "🔋";
          status_chr = "⚡";
          status_full = "☻";
          status_unk = "?";
        };
      };
      "wireless _first_" = {
        position = 4;
        settings = {
          format_down = "no wifi";
          format_up = "%ip";
        };
      };
    };
  };
}
```

## Neovim

This tip comes on behalf of the awesome [Vimjoyer](https://www.youtube.com/@vimjoyer) youtube channel. Many of the Nix docs recommend adding extra config to packages via a template string directly in the .nix file. Some people prefer this approach as it keeps everything in Nix, however in the case of Neovim I find it much more convenient to do manage my config via Lua files. You can define a few small nix functions which convert Lua code and files into Nix friendly template strings to keep things clean. I keep these Lua files in a separate repo so I can use them in Nix but also any other OS using a standard package manager like Lazy.nvim.

```nix
{ config, pkgs, lib, ...}:
let
  toLua = str: "lua << EOF\n${str}\nEOF\n";
  toLuaFile = file: "lua << EOF\n${builtins.readFile file}\nEOF\n";
in
{
  programs.neovim = {
    enable = true;
    extraLuaConfig = '' ${builtins.readFile ../config/nvim/init.lua} '';
    extraPackages = with pkgs; [
      cmake
      gcc

      lua-language-server
      rnix-lsp
      nodePackages.typescript-language-server
      nodePackages.svelte-language-server
      efm-langserver
      emmet-ls
      vscode-langservers-extracted

      ripgrep
      xclip
      wl-clipboard
    ];

    plugins = with pkgs.vimPlugins; [
      telescope-fzf-native-nvim
      neo-tree-nvim
      bufferline-nvim
      comment-nvim
      efmls-configs-nvim
      vim-tmux-navigator
      cmp-buffer
      cmp-path
      luasnip
      cmp_luasnip
      friendly-snippets

      {
        plugin = vim-sleuth;
        config = toLua "require(\'bufferline\').setup()";
      }
      {
        plugin = indent-blankline-nvim;
        config = toLua "require(\'ibl\').setup()";
      }
      {
        plugin = telescope-nvim;
        config = toLuaFile ../config/nvim/plugins/telescope.lua;
      }
      {
        plugin = nvim-lspconfig;
        config = toLuaFile ../config/nvim/plugins/lsp.lua;
      }
      {
        plugin = nvim-cmp;
        config = toLuaFile ../config/nvim/plugins/cmp.lua;
      }
      {
        plugin = (nvim-treesitter.withPlugins (p: [
          p.tree-sitter-nix
          p.tree-sitter-vim
          p.tree-sitter-bash
          p.tree-sitter-lua
          p.tree-sitter-python
          p.tree-sitter-json
          p.tree-sitter-css
          p.tree-sitter-html
          p.tree-sitter-javascript
          p.tree-sitter-typescript
          p.tree-sitter-svelte
        ]));
        config = toLuaFile ../config/nvim/plugins/treesitter.lua;
      }
      {
        plugin = catppuccin-nvim;
        config = "colorscheme catppuccin-mocha";
      }
    ];
    viAlias = true;
    vimAlias = true;
    vimdiffAlias = true;
  };
}
```

## Tmux

Shout out [vim-tmux-navigator](https://github.com/christoomey/vim-tmux-navigator)!

```nix
{ config, pkgs, ... }:

{
  programs.tmux = {
    enable = true;
    mouse = true;
    keyMode = "vi";
    prefix = "C-Space";
    plugins = with pkgs; [
      tmuxPlugins.catppuccin
      tmuxPlugins.vim-tmux-navigator 
    ];
    extraConfig = '' unbind C-. '';
  };
}
```

## Apps

In other Linux distros, heck even on mac and windows, I find it really difficult to keep track of whats installed. I've had countless experiences where in an urgent debugging frenzy, I install a bunch of random packages which may or may not help me solve my problem, and then completely forget they exist. Or I’ll end up with 2 or 3 local media players etc. There’s something to be said for being able to see all your user's installed packages/applications in one place and pick and choose the right tool for each use case.

```nix
  home.packages = with pkgs; [

    kitty #terminal
    btop #htop but pretty
    du-dust #disk usage but pretty
    bat #cat but pretty

    brightnessctl #control backlight
    dunst #notification daemon
    pavucontrol #sound settings GUI
    lxappearance #app theming

    image-roll #image viewer

    google-chrome 
    spotify
    spotify-tray #add spotify to menu bar
    inkscape #svg editor
    strawberry #local music player
    haruna #local video player

    slack
    discord

    redis
    mongodb-compass
    typescript
    bruno #free Postman alternative

  ];
```

## Scripts

With home manager, a Nix-native way to manage individual user environments, you can declare a shell script as a package, which makes it into an executable binary that can be called by your user. Some scripts I know I’ll always want associated with my user, and I find it much easier to just declare once and never have to worry about adding execution privileges or adding it to the PATH.

### Reconfig

I got the idea for this from a [LibrePhoenix](https://www.youtube.com/@librephoenix) youtube video. One thing you find yourself doing a lot in NixOS, especially as you’re getting started, is editing and rebuilding your configuration. This script opens my NixOS configuration in Neovim. When I finish editing the config, I see a git diff including all of the changes I made. From there the configuration rebuilds. I opted to suppress the verbose build output and only show errors if they are encountered to prevent too much visual noise. If the home-manager and system configurations both successfully build, the changes are committed to a git repository.

```nix
{ config, pkgs, ... }:

{
  home.packages = [
    (pkgs.writeShellScriptBin "reconfig" ''
           function showProgress() {
             local command="$1"
             local commonName="$2"
             local FRAMES="/ | \\ -"
             local status=0
             local pid=0

             if [ $commonName == "System" ]; then
               read -s -p "Enter sudo password: " sudo_password
               #echo "$sudo_password" | 
               sudo -S $command <<< $sudo_password &> nixos-switch.log || (cat nixos-switch.log | grep --color error && false) & pid=$!
             else
               $command &> nixos-switch.log || (cat nixos-switch.log | grep --color error && false) & pid=$!
             fi

             while ps -p $pid > /dev/null; do
                   for frame in $FRAMES; do
                       printf "\r$frame Syncing $commonName configuration..."
                       sleep 0.2
                   done
                   if ! kill -0 $pid 2>/dev/null; then
                       wait $pid
                       status=$?
                       break
                   fi
               done

               if [ $status -eq 0 ]; then
                   printf "\r$GREEN✓$NC Syncing $commonName configuration...$GREEN [Success!]$NC\n"
               else
                   printf "\r$RED×$NC Syncing $commonName configuration...$RED [Failed!]$NC\n"
               fi
               printf "\n"
           }

           pushd ~/nixos-dotfiles &> /dev/null
           nvim .
           git diff -U0
           git add .
           showProgress "home-manager switch --flake .#meatball" "Home-Manager"
           showProgress "nixos-rebuild switch --flake .#nixos-laptop" "System"
           rm nixos-switch.log
           gen=$(nixos-rebuild list-generations | grep current);
           git commit -am "$gen"
           popd &> /dev/null
    '')
  ];
}
```

## Conclusion

This is barely scratching the surface of how I use Nix. There are a bunch more topics I'd like to cover in future posts. But for now I just wanted to share some interesting examples of setting up a dev machine declaratively. If you've been dreaming of a hyper specific and customizable dev setup maybe check out NixOS!