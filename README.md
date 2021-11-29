# Femtorinth
[![license](https://img.shields.io/github/license/phnixir/femtorinth)](https://github.com/phnixir/femtorinth/blob/master/LICENSE)
[![issues](https://img.shields.io/github/issues-raw/phnixir/femtorinth)](https://github.com/phnixir/femtorinth/issues)
[![stargazers](https://img.shields.io/github/stars/phnixir/femtorinth)](https://github.com/phnixir/femtorinth/stargazers)
![Crates.io](https://img.shields.io/crates/d/femtorinth)
[![Crates.io](https://img.shields.io/crates/v/femtorinth)](https://crates.io/crates/femtorinth)

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/)

⚠️ This library is no longer maintained, I lost interest in this project after discovering
a minecraft launcher called "MultiMC", however if you want to interact with [Modrinth][modrinth] api
have a look at the [Ferinth][ferinth] crate which is more up-to-date :D.

[ferinth]: https://crates.io/crates/ferinth

Femtorinth is a rust library to interact with a sub-set of the [Modrinth][modrinth] api,
it only includes the api calls that don't need auth (a.k.a only GET calls), it includes
parsing for all the structs that exist in the v1 documentation of Modrinth's api and the
structs themselves aswell, plus, helper functions that will return those structs and
handle calling the Modrinth api, currently this library is using the [ureq][ureq] crate
as it's back-end to call the Modrinth api but that's planned to change in the near future
so the library can become backend agnostic (although i have no idea what the use of this
library would then be).

Right now Femtorinth is still very much in infancy, it does have good enough error handling,
documentation and code but i think there's still room for improvement, have a look at the
`Roadmap` section of this readme if you're interested in contributing!

[modrinth]: https://modrinth.com
[ureq]: https://crates.io/crates/ureq

## Basic example
Getting the latest sodium version + a bit of flair
```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results = femtorinth::search_mods("Sodium".into(), None, Some(1))?;
    let modres = results.hits[0].clone();
    println!("Found: {} from {}", &modres.title, &modres.author);
    println!("Description: {}", &modres.description);
    println!(
        "Downloads and follows: {} and {}",
        &modres.downloads, &modres.follows
    );
    println!(
        "Latest supported minecraft version: {}",
        &modres.latest_version
    );
    println!("Licensed under: {}", &modres.license);

    let version_list = femtorinth::version_list(modres.get_clean_id())?;
    let mod_version_data = version_list[0].clone();

    println!("Name of the newest version: {}", &mod_version_data.name);
    println!("Files for download with filenames:");
    for i in mod_version_data.files {
        println!("{}: {}", i.filename, i.url);
    }

    Ok(())
}
```
For more examples check out the [examples/][examples] directory on github

[examples]: https://github.com/phnixir/femtorinth

## Why "Femtorinth"?
Femtorinth is a name that's made up of two words, Femtometre and Modrinth,
Femtometre is an extremely small metric (10^-15, even smaller than pico!)
and since this library only covers a subset of Modrinth's v1 API, this
name would be perfect!

## Roadmap
- Have somebody more experienced look at the code and give advice on how to improve it
- Add more examples to the project demonstrating more features
- Make the documentation and interface easier to both read and use
- Make the library back-end agnostic so http crates other than `ureq` can be used

## Contributing
Thanks for your interest in contributing! please open an issue or merge request
to contribute, and remember to look at the roadmap and see if you can help with
the goals on there. Code contributions submitted for inclusion in the work by
you, as defined in the MPL2.0 license, shall be licensed as the above without
any additional terms or conditions.

## License
This project is licenced under the [Mozilla Public License 2.0](https://www.mozilla.org/en-US/MPL/2.0/).
