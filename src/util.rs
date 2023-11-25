use std::io;

pub fn extract_noattr(result: io::Result<Vec<u8>>) -> io::Result<Option<Vec<u8>>> {
    #[cfg(not(any(target_os = "freebsd", target_os = "netbsd")))]
    const ENOATTR: i32 = rustix::io::Errno::NODATA.raw_os_error();
    #[cfg(any(target_os = "freebsd", target_os = "netbsd"))]
    const ENOATTR: i32 = libc::ENOATTR;

    result.map(Some).or_else(|e| {
        if e.raw_os_error() == Some(ENOATTR) {
            Ok(None)
        } else {
            Err(e)
        }
    })
}

pub fn allocate_loop<F: FnMut(&mut [u8]) -> rustix::io::Result<usize>>(
    mut f: F,
) -> io::Result<Vec<u8>> {
    let mut vec: Vec<u8> = Vec::new();
    loop {
        let ret = f(&mut [])?;
        vec.resize(ret, 0);

        match f(&mut vec) {
            Ok(size) => {
                vec.truncate(size);
                vec.shrink_to_fit();
                return Ok(vec);
            }

            Err(rustix::io::Errno::RANGE) => continue,

            Err(e) => return Err(e.into()),
        }
    }
}
