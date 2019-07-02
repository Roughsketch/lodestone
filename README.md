[![ci-badge][]][ci] [![docs-badge][]][docs] [![crates.io version]][crates.io link]

# lodestone
Library for scraping data off of FFXIV's lodestone

# Examples

## Get a profile from a user id
```rust
use model::profile::Profile;
  
let profile = Profile::get(user_id).unwrap();
```

## Search for a profile in a datacenter
```rust
fn search_user(name: &str, dc: Option<Datacenter>) -> Result<Vec<Profile>, Error> {
  let search = SearchBuilder::new().character(name);
        
  if let Some(d) = dc {
    search = search.datacenter(d);
  }
    
  search.send()
}
```


## A more targeted search
```rust
let profiles = SearchBuilder::new()
    .character("Strawberry Custard")
    .datacenter(Datacenter::Primal)
    .lang(Language::English)
    .grand_company(GrandCompany::Maelstrom)
    .send()
    .unwrap();

let strawberry = profiles.first().unwrap();
```

[ci]: https://travis-ci.org/Roughsketch/lodestone
[ci-badge]: https://img.shields.io/travis/Roughsketch/lodestone.svg?style=flat-square
[crates.io link]: https://crates.io/crates/lodestone
[crates.io version]: https://img.shields.io/crates/v/lodestone.svg?style=flat-square
[docs]: https://docs.rs/lodestone
[docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
