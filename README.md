# lodestone
Library for scraping data off of FFXIV's lodestone

# Examples

## Get a profile from a user id
```rust
use model::profile::Profile;
  
let profile = Profile::get(user_id).unwrap();
```

## Search for a profile
```rust
fn search_user(name: &str, server: Option<Server>) -> Result<Vec<Profile>, Error> {
  let search = SearchBuilder::new().character(name);
        
  if let Some(s) = server {
    search = search.server(s);
  }
    
  search.send()
}
```
