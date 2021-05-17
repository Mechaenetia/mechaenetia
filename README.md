# Mechaenetia

Repository for the Game Mechaenetia

This is my first real public Project, so I need to learn quite a lot about managing things.
Other than that, just having fun learning and creating things, and hope everyone else will too. ^^

If you want to know what is going on, or are interested in the Community, check out the Forum:
https://forum.mechaenetia.com


I should mention that there is a few significant Delay Factors on this Project, so I'm trying to do other personal Projects until that is resolved, such as experimenting with Operating Systems and Hardware. Progress will be made one way or another!


## Build Requirements

The engine needs to talk to the OS libraries to be able to actually 'do' things, thus there are some requirements to be able to compile this:

### OS Specific

#### Linux

You need to make sure you have vulkan drivers installed to be able to run the GUI client.  Apt-based is the most detailed so look at it for the overall setup for the others as well, but adapt as necessary for other distros.

##### Apt-based distributions (Ubuntu/Kubuntu/Debian/Etc..)

```zsh
sudo apt-get install build-essential pkg-config libx11-dev libasound2-dev libudev-dev libxcb-composite0-dev
```

Depending on your graphics card, you may have to install one of the following to make certain your GPU drivers have vulkan interfaces if you don't already: `vulkan-radeon`, `vulkan-intel`, or `mesa-vulkan-drivers`

If you are performing development (see the [Development](#Development) section in this Readme for more details) then you can use the less optimizing but *much* faster linker from clang to perform debug object linking for faster development by installing: `clang` and `lld` 

##### Fedora (dnf)

```zsh
sudo dnf install gcc-c++ libX11-devel alsa-lib-devel systemd-devel
```

##### Pacman-based distributions (Arch/Manjaro/Etc...)

```zsh
sudo pacman -S libx11 pkgconf alsa-lib
```

##### Solus

```zsh
sudo eopkg install pkg-config libx11-devel g++ alsa-lib-devel
```

##### Void

```zsh
sudo xbps-install -S pkgconf alsa-lib-devel libX11-devel eudev-libudev-devel
```

##### NixOS

This packaging system can run on about any system and is popular for its containerized and reproducible building.

Define `shell.nix` with content similar to below, adapted for your setup (this might be a good default to add to the repo, thoughts?):
```nix
{ pkgs ? import <nixpkgs> { } }:
with pkgs;
mkShell {
  buildInputs = [
    cargo
    pkgconfig udev alsaLib lutris
    x11 xorg.libXcursor xorg.libXrandr xorg.libXi
    vulkan-tools vulkan-headers vulkan-loader vulkan-validation-layers
  ];
}
```

And enter the build container via `nix-shell`.

#### Windows

Make sure to have the [VS2019 Build Tools](https://visualstudio.microsoft.com/thank-you-downloading-visual-studio/?sku=BuildTools&rel=16) (or newer maybe?) to have everything required to compile.

For faster development (see the [Development](#Development) section for omre details) you can also install the lld linker by running:

```cmd
cargo install -f cargo-binutils
rustup component add llvm-tools-preview
```

##### Windows Subsystem for Linux (WSL 2)

Graphics and audio need to be configured for them to work with WSL 2 backend. Please see the ubuntu [WSL documentation](https://wiki.ubuntu.com/WSL) on how to set up graphics and audio.

#### MacOSX

Just make sure to have XCode and you're good to go as its the monolithic thing that contains it all.

### IDE

It is highly recommend to have a good IDE when working on such programming source code if you are going to be developing, and there are two main suggestions:

* **IntelliJ/CLion:** The free version does not have a gdb-based debugger, but it's rather easy to get full versions, such as if you have an active student email account.  This tends to be the most featureful and powerful IDE for Rust work at the current time.

* **RustAnalyzer:** This is a replacement for the old Rust Language Server, it is quickly coming up to feature parity with IntelliJ/CLion however.  VSCode is the program that uses the most features of RustAnalyzer but it also works with other language-server based IDE's such as Atom, EMacs, VIM, etc...  This setup is fully free however, and although the debugging isn't as nice as in CLion, it is entirely usable, and of course free via both definitions.

### Rust Information

And of course you would want good materials to learn Rust with:

* **[The Rust Book](https://doc.rust-lang.org/book/):** The best place to learn Rust from scratch

* **[Rust by Example](https://doc.rust-lang.org/rust-by-example/):** A great reference to see how to use Rust constructs.

* **[Rust Standard Library Documentation](https://doc.rust-lang.org/std/index.html):** Documentation on the Rust Standard Library, a must-use reference, some of the best documentation anywhere.

* **[Rust Learning Site](https://www.rust-lang.org/learn):** Has the above links as well, but more overall, a great site to peruse.

### Bevy Information

This project is using the Bevy Engine for rendering and game asset handling.  You can see its [book and associated documentation here](https://bevyengine.org/learn).

The Bevy project is still new, effectively a redesign and replacement of the seemingly dead Amethyst framework built to be simple and fast with a very active community.  It still has a lot to be developed however and they could use your assistance to get up to par!  Helping development on their OpenGL renderer would be a massive boon for older systems even though it is less efficient than vulkan.


## Development

There are a number of things you can do to make recompilation time much faster for quicker development and testing:

### Bevy's Dynamic Linking

Enable Bevy's Dynamic Linker when running by enabling its `bevy/dynamic` cargo feature when you run or debug the program.  This will execute slightly slower but will give the biggest loading speed boost to development outright.

From the commandline you can run it via:

```zsh
cargo run --features bevy/dynamic
```

Otherwise just add `bevy/dynamic` as a feature to the cargo task that you run/debug with in your IDE.

### Clang's LLD Linker

Clang's LLD Linker doesn't know certain Rustisms but it works just fine if not makes slightly code, however it runs substantially faster, thus making it useful to link with in development for faster turn-around.  This is unsupported on MacOS due to their odd compilation ecosystem for native code.

To use the LLD linker (or ZLD on mac) copy this file into this project at `.cargo/config.toml`:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld"]

# NOTE: you must manually install https://github.com/michaeleisel/zld on mac. you can easily do this with the "brew" package manager:
# `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"

# Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
# In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
#[profile.dev]
#debug = 1
```

Or if you are running the nightly compiler then you can enable generics sharing for even further increased linking speed, use this file instead:

```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]

# NOTE: you must manually install https://github.com/michaeleisel/zld on mac. you can easily do this with the "brew" package manager:
# `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld", "-Zshare-generics=y", "-Csplit-debuginfo=unpacked"]

[target.x86_64-pc-windows-msvc]
linker = "rust-lld.exe"
rustflags = ["-Zshare-generics=y"]

# Optional: Uncommenting the following improves compile times, but reduces the amount of debug info to 'line number tables only'
# In most cases the gains are negligible, but if you are on macos and have slow compile times you should see significant gains.
#[profile.dev]
#debug = 1
```


## TODO:

Learn about a bunch of Operating System things and different Hardware.

Stop getting distracted and start learning more Rust and Bevy Stuff.

Setup Bevy and make a Basic Launcher UI.

Setup a Default Workspace of sorts.
