<div id="top"></div>

<!-- PROJECT LOGO -->
<br />

<div align="center">
    <div style="height: 50px; width: 100%"></div>

  <a>
    <img src="https://github.com/ABlockOfficial/weaver_core/blob/main/assets/hero.svg" alt="Logo" width="320px">
  </a>

  <div style="height: 50px; width: 100%"></div>

  <h3>weaver_core</h3>

  <!-- <div>
  <img src="https://img.shields.io/github/actions/workflow/status/Zenotta/Intercom/codeql-analysis.yml?branch=main" alt="Pipeline Status" />
    <img src="https://img.shields.io/github/package-json/v/Zenotta/Intercom" />
  </div> -->

  <p align="center">
    The core library used by all <a href="https://github.com/ABlockOfficial/Weaver">Weaver</a> nodes and their plug-ins
    <br />
    <br />
    <a href="https://a-block.io"><strong>Official documentation »</strong></a>
    <br />
    <br />
  </p>
</div>

<!-- GETTING STARTED -->

## How to Use

`weaver_core` is designed to be used as a core crate for Weaver functionality that is common across all node types and plug-ins. It is not designed to be used as a standalone crate, and will not compile as such.

..

### 🔧 Installation

If you have `cargo-add` installed, you can simply run the following command:

```sh
cargo add weaver_core
```

Otherwise, add the following to your `Cargo.toml` file:

```toml
[dependencies]
weaver_core = "0.1.0"
```

<p align="left">(<a href="#top">back to top</a>)</p>

..

### 🏎️ Use in Plug-ins

The `weaver_core` library exposes a few common methods and functionalities that are useful if you're either using plug-ins or writing your own. This core functionality includes:

- **api:** The module here contains most of the `struct`s and `enum`s that are used to communicate between nodes and plug-ins. This includes the `JsonReply`, `ApiErrorResponse` and `APIResponseStatus` structs, as well as functions for JSON serialisation and Warp API replies.

- **db:** The module here contains all the common code associated with data storage. This includes the `KvStoreConnection` trait, which ensures consistent interfacing with data handlers across the Weaver ecosystem

- **crypto:** The module here ensures consistency in the handling of cryptography across the Weaver ecosystem. If you want to do anything with cryptography in your plug-in, you should use the functions here.

<p align="left">(<a href="#top">back to top</a>)</p>

..

### Further Work

- [x] Add tests
- [x] Consider abstraction structure for API and DB handling
- [ ] Add traces

<p align="left">(<a href="#top">back to top</a>)</p>

..