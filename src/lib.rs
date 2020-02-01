struct S {
    db: u8,
}

fn g() -> Result<S, ()> {
    let db = new_db();

    if db.is_err() {
        // We need special handling here so writing
        // `new_db()?` wasn't an option.
    }

    Ok(S { db: db? })
    //    Ok(S { db? })
}

fn new_db() -> Result<u8, ()> {
    Ok(0)
}

/*
error: expected one of `,` or `}`, found `?`
--> src\lib.rs:14:14
|
14 |     Ok(S { db? })
|        -     ^ expected one of `,` or `}`
|        |
|        while parsing this struct

error[E0063]: missing field `db` in initializer of `S`
--> src\lib.rs:14:8
|
14 |     Ok(S { db? })
|        ^ missing `db`

error: aborting due to 2 previous errors
*/
