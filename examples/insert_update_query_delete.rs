use gdbm_compat::{datum, dbm_close, dbm_delete, dbm_fetch, dbm_open, dbm_store, StoreMode};

#[derive(Debug, PartialEq, Eq)]
#[repr(C)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    const SIZE: usize = std::mem::size_of::<Self>();
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

fn main() {
    unsafe { main_() }
}

unsafe fn main_() {
    let db_filename = libc::tmpnam(std::ptr::null_mut());
    let dbm_ptr = dbm_open(
        db_filename,
        libc::O_RDWR | libc::O_CREAT,
        libc::S_IRUSR | libc::S_IWUSR,
    );
    let mut key = *b"black\0";
    let key_datum = datum {
        dptr: key.as_mut_ptr().cast(),
        // strlen exclude the nul terminator
        dsize: key.len() as i32 - 1,
    };
    let mut black = Color::new(0, 0, 0);

    // insert
    let value_datum = datum {
        dptr: (&mut black as *mut Color).cast(),
        dsize: Color::SIZE as i32,
    };
    dbm_store(dbm_ptr, key_datum, value_datum, StoreMode::DBM_INSERT);

    // update
    black.red = 255;
    let value_datum = datum {
        dptr: (&mut black as *mut Color).cast(),
        dsize: Color::SIZE as i32,
    };
    dbm_store(dbm_ptr, key_datum, value_datum, StoreMode::DBM_REPLACE);

    // query
    let value_datum = dbm_fetch(dbm_ptr, key_datum);
    if !value_datum.dptr.is_null() {
        let mut color = std::mem::zeroed::<Color>();
        std::ptr::copy(
            value_datum.dptr.cast(),
            &mut color as *mut Color,
            value_datum.dsize as usize,
        );
        assert_eq!(color, black);
    }

    // delete
    if dbm_delete(dbm_ptr, key_datum) == 0 {
        println!("delete success");
    }

    dbm_close(dbm_ptr);
}
