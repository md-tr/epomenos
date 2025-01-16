# This is the `epomenos` kernel, 0.1.0

> :warning: **WARNING**
>
> This project has no contribution to our society.  
> <small><small><small><small><small><small><small>
> Does it really have to? Like, where does your cocky behaviour about beginner
> hobby projects get you?  
> </small></small></small></small></small></small></small>
> Also, it is in very early stage (See Figure 1.), so don't install this to
> real hardware (for now!).
>
> $project\_version = pre^{pre^{pre^{pre^{.·^{.}}}}} alpha$  
> <small><small><small>Figure 1. "Very accurate math formula" for the current
> `epomenos` version</small></small></small>

> 📒 **NOTE**
>
> For now, only `x86_64` is supported. Other 64-bit targets will be added,
> eventually.

A 64-bit Rusty kernel, powered by Limine.

Because of the fact that the `epomenos` kernel is in really early stages, all
I can do for now is to state my goals about it: for the type of my kernel, I
will probably take a monolithic-like approach, and I plan to make it a fully
functional operating system (although it's a very difficult task!) with a GUI.

## Compiling

For Windows users, it is recommended to use Cygwin to compile `epomenos`.

Make sure that you have GCC and [`just`](https://github.com/casey/just)
installed. Also, for the `run` group of recipes to properly work, you
might need QEMU.

First of all, retrieve the source files from this GitHub repository, and
`cd` into the folder where `git` has installed it, for most cases, `epomenos`:

```sh
git clone https://github.com/md-tr/epomenos
cd epomenos
```

It is `just` as simple as that to compile the `epomenos`' source (a little
punny :wink:):

```sh
just target="target-name" build_iso
```

### Extras (other `just` recipes that can be used)

```sh
# build the ISO and run it in QEMU w/ BIOS (x86_64-only for now!)
just rb
just run_bios       # alternative

# build the ISO and run it in QEMU w/ UEFI (via OVMF)
just run_x86_64

# get list of usage recipes
just -l
just --list         # alternative

# build the binary only
just bb
just build_binary   # alternative

# build the ISO for x86_64 (default target)
just build_iso
```

## Contributions

Since it's in a very early stage (See Figure 1.) and I'm a beginner in OS
development, I'd rather wandering around and finding out by myself. In
contrast to that, still, any contributions are appreciated! You can issue
or PR a minor bug you find in this clusterhell I call "my hobby operating
system".

Try your best to keep the project lightweight and free from code taken from
foreign sources!

## Licensing

This project is licensed under MIT license. See the LICENSE file for more
information.

```diff
--- epilog.txt 13:37
+++ epilog.txt 18:87
@@ -1,2 +1,1 @@
- i hate the structure of this /^f(?!a[ink]|ec|i[lnrs]|o[lr]|u[ns]).{2}kin[g']?$/gm
- project
+ i restructured this project and still hate its structure

farewell
```