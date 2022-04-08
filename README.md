# Nel

[![Twitter handle][]][Twitter badge]
[![Discord Chat](https://img.shields.io/discord/684898665143206084?logo=discord&style=social)](https://discord.gg/deno)

<img align="right" src="https://github.com/pierorolando1/nel/blob/main/docs/logo/nel.png?raw=true" height="150px" alt="the deno mascot dinosaur standing in the rain">

Nel is a _strange_, _modern_ and _secure_ interpreted language and is built in Rust.

### Features

- I do it cause I love Nelida.
- I'm gonna give you 3 dolars if u uses this shit.
- Built in Rust

### Install

Shell (Mac, Linux):

```sh
curl -fsSL https://nel.vercel.app/install.sh | sh
```

PowerShell (Windows):

```powershell
iwr https://nel.vercel.app/install.ps1 -useb | iex
```

[Homebrew](https://formulae.brew.sh/formula/deno) (Mac):

```sh
brew install nel
```

[Chocolatey](https://chocolatey.org/packages/deno) (Windows):

```powershell
choco install nel
```

[Scoop](https://scoop.sh/) (Windows):

```powershell
scoop install nel
```

Build and install from source using [Cargo](https://crates.io/crates/deno):

```sh
cargo install nel --locked
```

See [releases](https://github.com/pierorolando1/nel/releases) for other options.

### Getting Started

Try running a simple program:

```sh
nel run https://nel.vercel.app/std/examples/welcome.ts
```

Or a more complex one:

```ts
const listener = Deno.listen({ port: 8000 });
console.log("http://localhost:8000/");
for await (const conn of listener) {
  serve(conn);
}
async function serve(conn: Deno.Conn) {
  for await (const { respondWith } of Deno.serveHttp(conn)) {
    respondWith(new Response("Hello world"));
  }
}
```

You can find a deeper introduction, examples, and environment setup guides in
the [manual](https://deno.land/manual).

The complete API reference is available at the runtime
[documentation](https://doc.deno.land).

### Contributing

We appreciate your help!

To contribute, please read our
[contributing instructions](https://deno.land/manual/contributing).


[Twitter badge]: https://twitter.com/intent/follow?screen_name=deno_land
[Twitter handle]: https://img.shields.io/twitter/follow/deno_land.svg?style=social&label=Follow
