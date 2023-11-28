#[cfg(test)]
pub mod tests {
    use super::super::*;
    use std::path::PathBuf;

    #[test]
    fn test_filebyteinstream() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/test_fileinstream");

        let mut stream = FileByteInStream::new(path.into_os_string().into_string().unwrap());

        assert_eq!(stream.read(), Some(0));
        assert_eq!(stream.read(), Some(1));
        assert_eq!(stream.read(), Some(2));
        assert_eq!(stream.read(), None);
    }

    #[test]
    fn test_filebyteoutstream() {
        let path = "/tmp/test";

        let _ = std::fs::remove_file(path);

        {
            let mut stream = FileByteOutStream::new(String::from(path));

            let _ = stream.write(0);
            let _ = stream.write(1);
            let _ = stream.write(2);
        } // The contents are now flushed

        let mut stream = FileByteInStream::new(String::from(path));

        assert_eq!(stream.read(), Some(0));
        assert_eq!(stream.read(), Some(1));
        assert_eq!(stream.read(), Some(2));
        assert_eq!(stream.read(), None);
    }

    #[test]
    fn test_vecinstream() {
        let mut stream = VecByteStream::new(vec![0, 1, 2, 3]);

        assert_eq!(stream.read(), Some(0));
        assert_eq!(stream.read(), Some(1));
        assert_eq!(stream.read(), Some(2));

        stream.clear();
        assert_eq!(stream.read(), None);
    }

    #[test]
    fn test_vecoutstream() {
        let mut stream = VecByteStream::new(Vec::new());

        let _ = stream.write(0);
        let _ = stream.write(1);
        let _ = stream.write(2);
        
        assert_eq!(stream.read(), Some(0));
        assert_eq!(stream.read(), Some(1));
        assert_eq!(stream.read(), Some(2));
    }

    #[test]
    fn test_fileinstream() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("test_files/a.txt");

        let mut stream = FileInStream::new(path.into_os_string().into_string().unwrap());

        assert_eq!(stream.peek(), Some('a'));
        assert_ne!(stream.peek(), Some('b'));
        assert_eq!(stream.read(), Some('a'));
        assert_eq!(stream.read(), Some('b'));
        assert_eq!(stream.read(), Some('c'));
        assert_eq!(stream.read(), Some('\n'));
        assert_eq!(stream.read(), None);
    }
}
