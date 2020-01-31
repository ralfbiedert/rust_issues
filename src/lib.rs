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
