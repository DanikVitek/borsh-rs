use core::future::Future;

pub trait AsyncRead: Unpin + Send {
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<usize>> + Send + 'a;
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a;
}
#[cfg(feature = "tokio")]
impl<R: tokio::io::AsyncReadExt + Unpin + Send> AsyncRead for R {
    #[inline]
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<usize>> + Send + 'a {
        tokio::io::AsyncReadExt::read(self, buf)
    }
    #[inline]
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a {
        async move {
            tokio::io::AsyncReadExt::read_exact(self, buf)
                .await
                .map(|_| ())
        }
    }
}
#[cfg(feature = "async-std")]
impl<R: async_std::io::ReadExt + Unpin + Send> AsyncRead for R {
    #[inline]
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<usize>> + Send + 'a {
        async_std::io::ReadExt::read(self, buf)
    }
    #[inline]
    fn read_exact<'a>(&'a mut self, buf: &'a mut [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a {
        async_std::io::ReadExt::read_exact(self, buf)
    }
}
pub trait AsyncWrite: Unpin + Send {
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a;
}
#[cfg(feature = "tokio")]
impl<R: tokio::io::AsyncWriteExt + Unpin + Send> AsyncWrite for R {
    #[inline]
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a {
        tokio::io::AsyncWriteExt::write_all(self, buf)
    }
}
#[cfg(feature = "async-std")]
impl<R: async_std::io::WriteExt + Unpin + Send> AsyncWrite for R {
    #[inline]
    fn write_all<'a>(&'a mut self, buf: &'a [u8]) -> impl Future<Output=std::io::Result<()>> + Send + 'a {
        async_std::io::WriteExt::write_all(self, buf)
    }
}
