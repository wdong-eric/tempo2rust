//! Owned libc stream for jpleph.c's fread/fseek semantics; no JPL algorithm.
//! Rust File ownership replaces fopen; buffering and partial items remain libc's.
use std::{
    ffi::{c_char, c_int, c_long, c_void},
    fs::File,
    io,
    os::fd::{AsRawFd, IntoRawFd},
    ptr::NonNull,
};

// Necessity: source-visible stdio read-ahead cannot be replaced by Rust File.
// The opaque pointer is never dereferenced by Rust or exposed outside this module.
// Every call below has one live owner and exclusive access; libc retains no buffer.
unsafe extern "C" {
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut c_void;
    fn fread(ptr: *mut c_void, size: usize, count: usize, stream: *mut c_void) -> usize;
    fn fseek(stream: *mut c_void, offset: c_long, whence: c_int) -> c_int;
    fn fclose(stream: *mut c_void) -> c_int;
}

pub(crate) struct Stream {
    raw: NonNull<c_void>,
    #[cfg(test)]
    pub metadata_file: File,
}

impl Stream {
    pub fn from_file(file: File) -> io::Result<Self> {
        #[cfg(test)]
        let mode = if OBSERVER.with(|o| o.borrow().fail_fdopen) {
            c"invalid"
        } else {
            c"rb"
        };
        #[cfg(not(test))]
        let mode = c"rb";
        // Necessity: adopt the already opened descriptor without reopening its path.
        // File owns a live descriptor; mode is static NUL-terminated storage. On
        // failure File still owns/closes it; on success ownership transfers below.
        #[cfg(test)]
        let metadata_file = file.try_clone()?; // fstat observer only: never reads/seeks.
        let raw = unsafe { fdopen(file.as_raw_fd(), mode.as_ptr()) };
        let raw = NonNull::new(raw).ok_or_else(io::Error::last_os_error)?;
        let _fd = file.into_raw_fd();
        Ok(Self {
            raw,
            #[cfg(test)]
            metadata_file,
        })
    }

    pub fn seek(&mut self, offset: c_long, whence: c_int) -> c_int {
        #[cfg(test)]
        if observe(0, 0, 0) {
            return -1;
        }
        // Necessity: preserve source fseek ordering and libc buffer transitions.
        // &mut self exclusively borrows the live owned stream, with no escaped
        // pointers/aliases. The caller supplies a source-width offset and origin.
        unsafe { fseek(self.raw.as_ptr(), offset, whence) }
    }

    pub fn read(&mut self, bytes: &mut [u8], size: usize, count: usize) -> io::Result<usize> {
        let total = size
            .checked_mul(count)
            .filter(|n| *n <= bytes.len())
            .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "fread buffer bound"))?;
        #[cfg(test)]
        if observe(1, size, count) {
            return Ok(0);
        }
        // Necessity: retain fread's complete-item return and partial-item writes.
        // Checked product fits this initialized, exclusively borrowed byte slice;
        // libc needs no alignment, retains neither buffer nor borrow, and may write
        // only total bytes. The stream is live and exclusively borrowed, even at EOF.
        let result = unsafe {
            fread(
                bytes[..total].as_mut_ptr().cast(),
                size,
                count,
                self.raw.as_ptr(),
            )
        };
        #[cfg(test)]
        OBSERVER.with(|o| o.borrow_mut().last_read[2] = result);
        Ok(result)
    }
}

impl Drop for Stream {
    fn drop(&mut self) {
        // Necessity: match fdopen with fclose, releasing libc's buffer and fd.
        // This non-cloneable owner relinquished File only after fdopen success;
        // Drop runs once, after all exclusive calls end. No pointer escapes and
        // no Send/Sync impl permits concurrent stream/cache access.
        unsafe {
            fclose(self.raw.as_ptr());
        }
    }
}

#[cfg(test)]
#[derive(Default)]
pub(crate) struct Observer {
    pub calls: [usize; 3],
    pub last_read: [usize; 3],
    pub fail: [usize; 3],
    pub fail_fdopen: bool,
}

#[cfg(test)]
thread_local! {
    pub(crate) static OBSERVER: std::cell::RefCell<Observer> = Default::default();
}

#[cfg(test)]
pub(crate) fn observe(kind: usize, size: usize, count: usize) -> bool {
    OBSERVER.with(|o| {
        let mut o = o.borrow_mut();
        o.calls[kind] += 1;
        if kind == 1 {
            o.last_read = [size, count, 0];
        }
        if o.fail[kind] == 0 {
            false
        } else {
            o.fail[kind] -= 1;
            o.fail[kind] == 0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, io::Write, os::unix::fs::MetadataExt};

    #[test]
    fn descriptor_transfer_partial_items_and_failure_cleanup() {
        let path = std::env::temp_dir().join(format!("jpl-stdio-{}", std::process::id()));
        let mut file = File::create(&path).unwrap();
        file.write_all(b"abcdefghi").unwrap();
        drop(file);
        let file = File::open(&path).unwrap();
        let fd = file.as_raw_fd();
        let inode = file.metadata().unwrap().ino();
        let mut stream = Stream::from_file(file).unwrap();
        assert_eq!(fs::metadata(format!("/dev/fd/{fd}")).unwrap().ino(), inode);
        let mut bytes = [b'?'; 16];
        assert_eq!(stream.read(&mut bytes, 8, 2).unwrap(), 1);
        assert_eq!(&bytes[..9], b"abcdefghi");
        assert_eq!(&bytes[9..], b"???????");
        assert_eq!(stream.read(&mut bytes, 8, 2).unwrap(), 0);
        assert!(stream.read(&mut bytes, usize::MAX, 2).is_err());
        assert_eq!(stream.seek(0, 0), 0);
        assert_eq!(stream.read(&mut bytes, 1, 9).unwrap(), 9);
        drop(stream);
        assert!(fs::metadata(format!("/dev/fd/{fd}")).is_err());
        let file = File::open(&path).unwrap();
        let fd = file.as_raw_fd();
        OBSERVER.with(|o| o.borrow_mut().fail_fdopen = true);
        assert!(Stream::from_file(file).is_err());
        OBSERVER.with(|o| o.borrow_mut().fail_fdopen = false);
        assert!(fs::metadata(format!("/dev/fd/{fd}")).is_err());
        fs::remove_file(path).unwrap();
    }
}
