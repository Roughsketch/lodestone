# lodestone
Library for scraping data off of FFXIV's lodestone

# Examples

## Get a profile from a user id
```rust
fn get_profile(user_id: u32) -> Result<Profile, Error> {
  use model::profile::Profile;
  
  Profile::get(user_id)
}
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
