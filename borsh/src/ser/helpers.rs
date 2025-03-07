use async_generic::async_generic;

#[cfg(feature = "unstable__async")]
use crate::{async_io::AsyncWrite, BorshSerializeAsync};
use crate::{
    BorshSerialize,
    __private::maybestd::vec::Vec,
    io::{ErrorKind, Result, Write},
};

pub(super) const DEFAULT_SERIALIZER_CAPACITY: usize = 1024;

/// Serialize an object into a vector of bytes.
/// # Example
///
/// ```
/// assert_eq!(vec![12, 0, 0, 0, 0, 0, 0, 0], borsh::to_vec(&12u64).unwrap());
/// ```
pub fn to_vec<T>(value: &T) -> Result<Vec<u8>>
where
    T: BorshSerialize + ?Sized,
{
    let mut result = Vec::with_capacity(DEFAULT_SERIALIZER_CAPACITY);
    value.serialize(&mut result)?;
    Ok(result)
}

/// Serializes an object directly into a `Writer`.
///
/// # Example
#[async_generic(
    /// ```
    /// # #[cfg(feature = "std")]
    /// let stderr = std::io::stderr();
    /// # #[cfg(feature = "std")]
    /// borsh::to_writer(&stderr, "hello_0x0a").unwrap();
    /// ```
    sync_signature;

    /// ```
    /// # tokio_test::block_on(async {
    /// # #[cfg(feature = "unstable__tokio")]
    /// let mut stderr = tokio::io::stderr();
    /// # #[cfg(feature = "unstable__tokio")]
    /// borsh::to_writer_async(&mut stderr, "hello_0x0a").await.unwrap();
    /// # })
    /// ```
    #[cfg(feature = "unstable__async")]
    async_signature<T, W: AsyncWrite>(mut writer: W, value: &T) -> Result<()>
    where
        T: BorshSerializeAsync + ?Sized,
)]
pub fn to_writer<T, W: Write>(mut writer: W, value: &T) -> Result<()>
where
    T: BorshSerialize + ?Sized,
{
    let res = value.serialize(&mut writer);
    if _sync {
        res
    } else {
        res.await
    }
}

/// Serializes an object without allocation to compute and return its length
/// # Example
///
/// ```
/// use borsh::BorshSerialize;
///
/// /// derive is only available if borsh is built with `features = ["derive"]`
/// # #[cfg(feature = "derive")]
/// #[derive(BorshSerialize)]
/// struct A {
///     tag: String,
///     value: u64,
/// }
///
/// # #[cfg(feature = "derive")]
/// let a = A { tag: "hello".to_owned(), value: 42 };
///
/// assert_eq!(8, borsh::object_length(&12u64).unwrap());
/// # #[cfg(feature = "derive")]
/// assert_eq!(17, borsh::object_length(&a).unwrap());
/// ```
pub fn object_length<T>(value: &T) -> Result<usize>
where
    T: BorshSerialize + ?Sized,
{
    // copy-paste of solution provided by @matklad
    // in https://github.com/near/borsh-rs/issues/23#issuecomment-816633365
    struct LengthWriter {
        len: usize,
    }
    impl Write for LengthWriter {
        #[inline]
        fn write(&mut self, buf: &[u8]) -> Result<usize> {
            let res = self.len.checked_add(buf.len());
            self.len = match res {
                Some(res) => res,
                None => {
                    return Err(ErrorKind::OutOfMemory.into());
                }
            };
            Ok(buf.len())
        }
        #[inline]
        fn flush(&mut self) -> Result<()> {
            Ok(())
        }
    }
    let mut w = LengthWriter { len: 0 };
    value.serialize(&mut w)?;
    Ok(w.len)
}
