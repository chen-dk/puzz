use std::fmt;

use crate::{Extensions, HeaderMap, HeaderName, HeaderValue, Result, StatusCode, Version};

/// 一个HTTP响应。
///
/// HTTP响应由头部和可选的正文组成。正文是泛型的，允许任意类型来表示HTTP响应的正文。
pub struct Response<T> {
    head: Head,
    body: T,
}

/// HTTP响应的头部。
///
/// HTTP响应的头部由状态码、版本和一组标头组成。
#[derive(Default)]
pub struct Head {
    /// HTTP响应的状态码
    pub status: StatusCode,

    /// HTTP响应的版本
    pub version: Version,

    /// HTTP响应的标头集
    pub headers: HeaderMap<HeaderValue>,

    /// HTTP响应的扩展
    pub extensions: Extensions,

    _priv: (),
}

/// HTTP响应的构建器。
#[derive(Debug)]
pub struct Builder {
    inner: Result<Head>,
}

impl Response<()> {
    /// 创建新的构建器以构建响应。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder()
    ///     .status(200)
    ///     .header("X-Custom-Foo", "Bar")
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<T> Response<T> {
    /// 使用给定的正文创建一个空白的响应。
    ///
    /// 此响应的头部将被设置为默认值。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let response = Response::new("hello world");
    ///
    /// assert_eq!(response.status(), StatusCode::OK);
    /// assert_eq!(*response.body(), "hello world");
    /// ```
    #[inline]
    pub fn new(body: T) -> Response<T> {
        Response {
            head: Head::new(),
            body,
        }
    }

    /// 使用给定的头部和正文创建响应。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let response = Response::new("hello world");
    /// let (mut head, body) = response.into_head();
    ///
    /// head.status = StatusCode::BAD_REQUEST;
    /// let response = Response::from_head(head, body);
    ///
    /// assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    /// assert_eq!(*response.body(), "hello world");
    /// ```
    #[inline]
    pub fn from_head(head: Head, body: T) -> Response<T> {
        Response { head, body }
    }

    /// 获取响应的状态码。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let response: Response<()> = Response::default();
    ///
    /// assert_eq!(response.status(), StatusCode::OK);
    /// ```
    #[inline]
    pub fn status(&self) -> StatusCode {
        self.head.status
    }

    /// 获取响应的状态码的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let mut response: Response<()> = Response::default();
    /// *response.status_mut() = StatusCode::CREATED;
    ///
    /// assert_eq!(response.status(), StatusCode::CREATED);
    /// ```
    #[inline]
    pub fn status_mut(&mut self) -> &mut StatusCode {
        &mut self.head.status
    }

    /// 获取响应的版本的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, Version};
    ///
    /// let response: Response<()> = Response::default();
    ///
    /// assert_eq!(response.version(), Version::HTTP_11);
    /// ```
    #[inline]
    pub fn version(&self) -> Version {
        self.head.version
    }

    /// 获取响应的版本的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, Version};
    ///
    /// let mut response: Response<()> = Response::default();
    /// *response.version_mut() = Version::HTTP_2;
    ///
    /// assert_eq!(response.version(), Version::HTTP_2);
    /// ```
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    /// 获取响应的标头集的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response: Response<()> = Response::default();
    ///
    /// assert!(response.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.head.headers
    }

    /// 获取响应的标头集的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    /// use puzz_http::header::*;
    ///
    /// let mut response: Response<()> = Response::default();
    /// response.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    ///
    /// assert!(!response.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.head.headers
    }

    /// 获取响应的扩展的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response: Response<()> = Response::default();
    ///
    /// assert!(response.extensions().get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.head.extensions
    }

    /// 获取响应的扩展的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let mut response: Response<()> = Response::default();
    /// response.extensions_mut().insert("hello");
    ///
    /// assert_eq!(response.extensions().get(), Some(&"hello"));
    /// ```
    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.head.extensions
    }

    /// 获取响应的正文的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response: Response<String> = Response::default();
    ///
    /// assert!(response.body().is_empty());
    /// ```
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }

    /// 获取响应的正文的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let mut response: Response<String> = Response::default();
    /// response.body_mut().push_str("hello world");
    ///
    /// assert!(!response.body().is_empty());
    /// ```
    #[inline]
    pub fn body_mut(&mut self) -> &mut T {
        &mut self.body
    }

    /// 消耗响应，只返回响应的正文。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::new(10);
    /// let body = response.into_body();
    ///
    /// assert_eq!(body, 10);
    /// ```
    #[inline]
    pub fn into_body(self) -> T {
        self.body
    }

    /// 消耗响应，返回响应的头部和正文。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let response: Response<()> = Response::default();
    /// let (head, body) = response.into_head();
    ///
    /// assert_eq!(head.status, StatusCode::OK);
    /// ```
    #[inline]
    pub fn into_head(self) -> (Head, T) {
        (self.head, self.body)
    }

    /// 消耗响应，返回带有给定正文的新响应，其正文为传入函数的返回值。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder().body("some string").unwrap();
    /// let mapped_response: Response<&[u8]> = response.map(|b| {
    ///   assert_eq!(b, "some string");
    ///   b.as_bytes()
    /// });
    /// assert_eq!(mapped_response.body(), &"some string".as_bytes());
    /// ```
    #[inline]
    pub fn map<F, U>(self, f: F) -> Response<U>
    where
        F: FnOnce(T) -> U,
    {
        Response {
            body: f(self.body),
            head: self.head,
        }
    }
}

impl<T: Default> Default for Response<T> {
    #[inline]
    fn default() -> Response<T> {
        Response::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status())
            .field("version", &self.version())
            .field("headers", self.headers())
            .field("body", self.body())
            .finish()
    }
}

impl Head {
    fn new() -> Head {
        Head::default()
    }
}

impl fmt::Debug for Head {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Head")
            .field("status", &self.status)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .finish()
    }
}

impl Builder {
    /// 创建构建器的默认实例以构建响应。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::response::Builder;
    ///
    /// let response = Builder::new()
    ///     .status(200)
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }

    /// 设置响应的状态码。
    ///
    /// 默认情况下，这是`200`。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder()
    ///     .status(200)
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn status<T>(self, status: T) -> Builder
    where
        StatusCode: TryFrom<T>,
        <StatusCode as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.status = TryFrom::try_from(status).map_err(Into::into)?;
            Ok(head)
        })
    }

    /// 获取响应的状态码。
    ///
    /// 默认情况下，这是`200`。如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, StatusCode};
    ///
    /// let mut req = Response::builder();
    ///
    /// assert_eq!(req.status_ref().unwrap(), &StatusCode::OK);
    /// ```
    pub fn status_ref(&self) -> Option<&StatusCode> {
        self.inner.as_ref().ok().map(|h| &h.status)
    }

    /// 设置响应的HTTP版本。
    ///
    /// 默认情况下，这是`HTTP/1.1`。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, Version};
    ///
    /// let response = Response::builder()
    ///     .version(Version::HTTP_2)
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn version(self, version: Version) -> Builder {
        self.and_then(move |mut head| {
            head.version = version;
            Ok(head)
        })
    }

    /// 获取响应的HTTP版本。
    ///
    /// 默认情况下，这是`HTTP/1.1`。如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Response, Version};
    ///
    /// let mut req = Response::builder();
    /// assert_eq!(req.version_ref().unwrap(), &Version::HTTP_11);
    ///
    /// req = req.version(Version::HTTP_2);
    /// assert_eq!(req.version_ref().unwrap(), &Version::HTTP_2);
    /// ```
    pub fn version_ref(&self) -> Option<&Version> {
        self.inner.as_ref().ok().map(|h| &h.version)
    }

    /// 将标头追加到响应中。
    ///
    /// 此函数将提供的键值对追加到响应内部的[`HeaderMap`]中。本质上，
    /// 这相当于调用[`HeaderMap::append`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder()
    ///     .header("Content-Type", "text/html")
    ///     .header("X-Custom-Foo", "bar")
    ///     .header("content-length", 0)
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn header<K, V>(self, key: K, value: V) -> Builder
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<crate::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let name = <HeaderName as TryFrom<K>>::try_from(key).map_err(Into::into)?;
            let value = <HeaderValue as TryFrom<V>>::try_from(value).map_err(Into::into)?;
            head.headers.append(name, value);
            Ok(head)
        })
    }

    /// 获取响应的标头集的引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let res = Response::builder()
    ///     .header("Accept", "text/html")
    ///     .header("X-Custom-Foo", "bar");
    /// let headers = res.headers_ref().unwrap();
    ///
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
        self.inner.as_ref().ok().map(|h| &h.headers)
    }

    /// 获取响应的标头集的可变引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{HeaderValue, Response};
    ///
    /// let mut res = Response::builder();
    ///
    /// let headers = res.headers_mut().unwrap();
    /// headers.insert("Accept", HeaderValue::from_static("text/html"));
    /// headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    ///
    /// let headers = res.headers_ref().unwrap();
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
        self.inner.as_mut().ok().map(|h| &mut h.headers)
    }

    /// 将一个类型添加到响应的扩展中。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder()
    ///     .extension("My Extension")
    ///     .body(())
    ///     .unwrap();
    ///
    /// assert_eq!(response.extensions().get::<&'static str>(),
    ///            Some(&"My Extension"));
    /// ```
    pub fn extension<T>(self, extension: T) -> Builder
    where
        T: 'static,
    {
        self.and_then(move |mut head| {
            head.extensions.insert(extension);
            Ok(head)
        })
    }

    /// 获取响应的扩展的引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let res = Response::builder().extension("My Extension").extension(5u32);
    /// let extensions = res.extensions_ref().unwrap();
    ///
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_ref(&self) -> Option<&Extensions> {
        self.inner.as_ref().ok().map(|h| &h.extensions)
    }

    /// 获取响应的扩展的可变引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let mut res = Response::builder().extension("My Extension");
    /// let mut extensions = res.extensions_mut().unwrap();
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    ///
    /// extensions.insert(5u32);
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_mut(&mut self) -> Option<&mut Extensions> {
        self.inner.as_mut().ok().map(|h| &mut h.extensions)
    }

    /// 消耗构建器，使用给定的正文构建响应。
    ///
    /// # 错误
    ///
    /// 如果之前配置的任意一个参数发生错误，则在调用此函数时将错误返回。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Response;
    ///
    /// let response = Response::builder()
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn body<T>(self, body: T) -> Result<Response<T>> {
        self.inner.map(move |head| Response { head, body })
    }

    fn and_then<F>(self, func: F) -> Self
    where
        F: FnOnce(Head) -> Result<Head>,
    {
        Builder {
            inner: self.inner.and_then(func),
        }
    }
}

impl Default for Builder {
    #[inline]
    fn default() -> Builder {
        Builder {
            inner: Ok(Head::new()),
        }
    }
}
