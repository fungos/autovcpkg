# AutoVcpkg

`AutoVcpkg` is a `CMake` (and `Cargo`*) utility to automatically manage your `vcpkg` dependencies.

It uses CMake built-in features to download and compile vcpkg and build your listed dependencies before building your own CMake project.

This can be used with any CMake based project and Cargo to **download, build and install** native dependencies.

There are a bunch of interesting things about this:
- You can use your own vcpkg root with it by setting the `AUTO_VCPKG_ROOT` environment variable; otherwise,
- It can manage different vcpkg roots per `leaf` crate/cmake project. It will cache compiled artifacts and no recompilation will happen, even when changing between `debug` and `release` builds;
- (Rust) Any crate in your dependency tree can have different `vcpkg` dependencies and everything will be built in your root or in a `vcpkg` root in your crate's `target` folder.

Note that by using this, you're downloading a lot of things from internet (vcpkg and any dependency library) from source and building locally (both debug and release). This means that you can be building a lot of stuff and things can take a lot of time. Eg. building GTK from source on a Windows laptop took about one hour with cargo stuck at the same `building` message, which can be confusing to people.

Also, not that this is still a proof-of-concept, but works amazingly well.

# Try it

```
$ git clone https://github.com/fungos/autovcpkg.git
$ cd autovcpkg/autovcpkg-test
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target\debug\autovcpkg-test.exe`
zlib version: 1.2.11
curl version: libcurl/7.66.0-DEV Schannel zlib/1.2.11
```

The `autovcpkg-test` crate depends on `zlib` and `curl` vcpkg ports. This will build everything needed to make it work, then after it is done, you can take a look at the crate's `target` folder, where a `vcpkg` root installation is present with all debug and release dependencies sources, libraries and resulting binaries (eg. dll's) as expected in a vcpkg environemnt. 

# Usage

## CMake based projects

You'll need only the `AutoVcpkg.cmake` and `vcpkg-bootstrap.cmake` from [here](autovcpkg/shim-sys), copy then to your project folder and add this to your CMakeLists.txt:

```CMake
list(APPEND CMAKE_MODULE_PATH /path/to/AutoVcpkgCMakeFiles)
include(AutoVcpkg)
vcpkg_install(sdl2 opengl glew curl) # your dependencies
```

## Rust (Cargo) projects

For Rust projects, this can be used by adding two dependencies in your `Cargo.toml` and a few `build.rs` lines.

> Take a look at the project sample [here](examples/) for different example usages.

If your're implementing a library that uses a native `vcpkg` dependency, you only need to reference it as a dependency with the `vcpkg` packages as `features` in your`Cargo.toml`:

```toml
[dependencies.autovcpkg]
path = "../autovcpkg"
version = "0.1.2019-10"
features = ["sdl2", "curl", "zlib"]
```

For a final application (top-level) crate that depends on any native `vcpkg` or on a crate based on a `vcpkg`, a build time dependency is required:

```toml
[build-dependencies.autovcpkg-build]
path = "../autovcpkg-build"
version = "0.1.2019-10"
```

And an accompaining `build.rs` scripts to help cargo find how to link agains these dependencies (and on Windows, copy any necessary DLL required at run-time), here an example from our [top_level](examples/top_level) example crate:

```rust
use autovcpkg_build;
fn main() {
    autovcpkg_build::configure(&["curl", "zlib"]);
}
```

# Use case: gtk-rs

I [created this](https://github.com/mcgoo/vcpkg-rs/issues/9) by [frustration with the GTK state on Windows](https://github.com/gtk-rs/gtk/issues/702) where the process of acquiring and building all GTK dependencies depends on a lot of manual downloading and building. Don't believe? Take a look at these instructions [here](https://www.gtk.org/download/windows.php) - it is almost 3 pages of instructions! Then look at [this user's post](https://www.reddit.com/r/rust/comments/bzkhmt/how_to_use_gtkrs_on_windows_using_the_msvc/) from a developer perspective with *improved* instructions.

In my opinion, this is a big obstacle on adoption, Windows developers already dislike GTK for its _wrong_ look&feel, but when you add all this booststrap complexity, there is simply no reason to even try GTK, **it is just not worth**.

Now, enter `AutoVcpkg`, and we can solve most if any all of these issues transparently, for example, I did test it with [reml](https://github.com/antoyo/relm), and it **simply works** without anything more than `cargo run --example button`. **No downloading, no building and no installs required.**

```toml
[dependencies.autovcpkg]
path = "autovcpkg"
version = "0.1.2019-10"
features = ["gtk"]

[build-dependencies.autovcpkg-build]
path = "autovcpkg-build"
version = "0.1.2019-10"
```

```rust
use autovcpkg_build;

fn main() {
    autovcpkg_build::configure(&["gtk"]);
    #[cfg(target_os = "windows")]
    // vcpkg generate gtk adn gdk libraries with 3.0, where gtk-rs et al. expect only 3, we duplicate and rename them so rust will be able to find and link correctly
    autovcpkg_build::lib_fixup(&[("gtk-3.0.lib", "gtk-3.lib"), ("gdk-3.0.lib", "gdk-3.lib")]);
}
```

**But**, this is not yet ideal or perfect but can be used **right now**. There are a few issues that need attention:

- GTK requires some pixmaps for some graphics (eg. minimize, restore, maximize buttons) that must still be installed. Current work to solve this is being done [here](https://github.com/microsoft/vcpkg/issues/6554), and this is the biggest blocker for a seamless Rust+GTK experience.
- `AutoVcpk` is still a proof-of-concept, it requires some more work to deal with idiosyncrasy of some libraries, mainly cyclic dependency, because cargo build metadata does not support linker groups or custom linker arguments.
- There are a few things that looks like Cargo bugs or limitations (or my own misunderstanding), forcing us to require `autovcpkg-build` in the top-level crate. If this can be solved, then everything would be a lot better and probably all this dependency management would be transparent for any `gtk-rs` crate user.