

Intuitively I would expect this to work:

```
struct S {
    db: u8,
}

fn g() -> Result<S, ()> {
    let db = new_db();
    
 // Ok(S { db: db? })
    Ok(S { db? })
}

fn new_db() -> Result<u8, ()> {
    Ok(0)
}
```


However, I can see this is a special case (not depicted: we need to
check `db` before applying `?`), so most people could call `new_db()?`
instead.
