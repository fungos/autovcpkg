# AutoVcpkg

`AutoVcpkg` is a `CMake` (and `Cargo`) utility to automatically manage your `vcpkg` dependencies.

It uses CMake built-in features to download, compile and build vcpkg and your vcpkg dependencies before building your own CMake project.

It can be used with any CMake based project and even with Rust's Cargo to **download, build and install** native dependencies transparently as if they were simple Rust crates.

There are a bunch of interesting things about this:
- You can use your own vcpkg installation if already have one, by setting `AUTO_VCPKG_ROOT` environment variable; Otherwise,
- It can manage different and isolated vcpkg installations per crate/cmake project;
- It will cache compiled artifacts and no recompilation will happen, even when switching between `debug` and `release` builds;
- (Rust) Any crate in your dependency tree can have different `vcpkg` dependencies and everything will be built in your own project's `vcpkg` root for your crate's `target` folder (or at your own desired `vcpkg` installation root).

Note that by using `AutoVcpkg`, you're downloading a lot of things from internet (vcpkg itself and any dependency library) from source and building locally (both debug and release). This means:
- If you're wary with security, you should look twice (as for any `build.rs` out there);
- That you may be building a lot of dependencies at once and this can take a lot of time. Example, when building GTK from source on a Windows laptop it can take about one hour with cargo stuck at the same `building` message, which can be confusing to users;

> WARNING: this is still a proof-of-concept, but works amazingly well.

# Try it

```
$ git clone https://github.com/fungos/autovcpkg.git
$ cd autovcpkg/examples/top_level
$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.13s
     Running `target\debug\autovcpkg-test.exe`
zlib version: 1.2.11
curl version: libcurl/7.66.0-DEV Schannel zlib/1.2.11
```

The `top_level` example crate depends on `zlib` and `curl` vcpkg ports. This will build everything needed to make it work. When it is done you can take a look at the crate's `target` folder, where a `vcpkg` root installation should be present with all debug and release dependencies sources, libraries and resulting binaries (eg. dll's) as expected in a vcpkg environment. 

> TIP: To see the magic really happening in the background try passing `-vv` to cargo, eg. `cargo run -vv` (on a clean build).

# Usage

## CMake based projects (C, C++, etc.)

You'll only need the `AutoVcpkg.cmake` and `vcpkg-bootstrap.cmake` from [here](autovcpkg/shim-sys), copy them to your project's folder, then add this to your CMakeLists.txt:

```CMake
list(APPEND CMAKE_MODULE_PATH /path/to/AutoVcpkgCMakeFiles)
include(AutoVcpkg)
vcpkg_install(sdl2 opengl glew curl) # your dependencies
```

## Rust (Cargo) projects

For Rust projects, this can be easily used by adding two dependencies in your `Cargo.toml` and a few `build.rs` lines.

> Take a look at the project samples [here](examples/) for different possible usages.

If your're implementing a library that uses a native `vcpkg` dependency (eg. `-sys` crate), then you only need to reference `AutoVcpkg` it as a dependency on you crate with the desired `vcpkg` packages as `features`:

```toml
[dependencies.autovcpkg]
version = "0.1.2019-10"
features = ["sdl2", "curl", "zlib"]
```

For a final application crate, that depends on any native `vcpkg` or on another `AutoVcpkg` based crate, a build time dependency is required:

```toml
[build-dependencies.autovcpkg]
version = "0.1.2019-10"
```

And an accompaining `build.rs` scripts to instruct cargo how to link against the native dependencies - and additionally on Windows, this will copy any necessary DLL required at run-time. Here an example from our [top_level](examples/top_level) example application crate:

```rust
use autovcpkg;
fn main() {
    autovcpkg::configure(&["curl", "zlib"]);
}
```

# Use case: gtk-rs

I created `AutoVcpkg` by [frustration](https://github.com/mcgoo/vcpkg-rs/issues/9) with [GTK state on Windows](https://github.com/gtk-rs/gtk/issues/702) where the process of acquiring and building all GTK dependencies requires a lot of manual downloading and building. Take a look at these instructions [here](https://www.gtk.org/download/windows.php), it is almost 3 pages of instructions, which is an improved version based on [this user's post](https://www.reddit.com/r/rust/comments/bzkhmt/how_to_use_gtkrs_on_windows_using_the_msvc/).

In my opinion this is a big obstacle on adoption, Windows developers already dislike GTK for its _wrong look & feel_, but when you add all this aforementioned complexity, them there is simply no reason to even try GTK. **It is just not worth**.

Now, with `AutoVcpkg` we can solve most (if any all*) of these issues transparently. For example. I did test it with [reml](https://github.com/antoyo/relm) and it **simply works**. Take a look at our [relm-test](examples/relm-test) and try it yourself. **No manual downloading, build and installs required.**

To make any GTK-dependent crate take advantage of all this, everything is needed is here below:

Add this to the `Cargo.toml`:
```toml
[dependencies.autovcpkg]
version = "0.1.2019-10"
features = ["gtk"]

[build-dependencies.autovcpkg]
version = "0.1.2019-10"
```

And this to `build.rs`:
```rust
use autovcpkg;
fn main() {
    autovcpkg::configure(&["gtk"]);
    #[cfg(target_os = "windows")]
    // vcpkg generate gtk adn gdk libraries with 3.0, where gtk-rs et al. expect only 3, we duplicate and rename them so rust will be able to find and link correctly
    autovcpkg::lib_fixup(&[("gtk-3.0.lib", "gtk-3.lib"), ("gdk-3.0.lib", "gdk-3.lib")]);
}
```

## Remarks

Even after all this work `AutoVcpkg` does it is still not ideal nor perfect.
It can (and probably should) be used **right now** though.

There are a few issues that need attention and any help is welcome:

- GTK: it requires [some assets](https://github.com/tschoonj/GTK-for-Windows-Runtime-Environment-Installer/tree/master/gtk-nsis-pack/share) which are build with its original build system but doesn't come with the `vcpkg` version (eg. minimize, restore, maximize buttons and possible more), these must be manually copied within the final application. Current work to solve this is being done [here](https://github.com/microsoft/vcpkg/issues/6554), and this is the biggest blocker for a seamless Rust+GTK experience;
- `AutoVcpkg` is still a proof-of-concept and it requires some more work to deal with idiosyncrasy of some libraries like cyclic dependency. Because cargo build metadata does not support linker groups or custom linker arguments, which require some brittle workarounds;
- The requirement of another step as a build-dependency `AutoVcpkg` in the top-level crate is not really ideal. If this can be solved then everything would be a lot better and dependency management would be fully transparent;
