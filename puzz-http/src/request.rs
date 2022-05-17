use std::fmt;

use crate::{Extensions, HeaderMap, HeaderName, HeaderValue, Method, Result, Uri, Version};

/// 一个HTTP请求。
///
/// HTTP请求由头部和可选的正文组成。正文是泛型的，允许任意类型来表示HTTP请求的正文。
pub struct Request<T> {
    head: Head,
    body: T,
}

/// HTTP请求的头部。
///
/// HTTP请求的头部由方法、URI、版本和一组标头组成。
#[derive(Default)]
pub struct Head {
    /// HTTP请求的方法
    pub method: Method,

    /// HTTP请求的URI
    pub uri: Uri,

    /// HTTP请求的版本
    pub version: Version,

    /// HTTP请求的标头集
    pub headers: HeaderMap<HeaderValue>,

    /// HTTP请求的扩展
    pub extensions: Extensions,

    _priv: (),
}

/// HTTP请求的构建器。
#[derive(Debug)]
pub struct Builder {
    inner: Result<Head>,
}

impl Request<()> {
    /// 创建新的构建器以构建请求。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request = Request::builder()
    ///     .method("GET")
    ///     .uri("https://www.rust-lang.org/")
    ///     .header("X-Custom-Foo", "Bar")
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<T> Request<T> {
    /// 使用给定的正文创建一个空白的请求。
    ///
    /// 此请求的头部将被设置为默认值。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let request = Request::new("hello world");
    ///
    /// assert_eq!(*request.method(), Method::GET);
    /// assert_eq!(*request.body(), "hello world");
    /// ```
    #[inline]
    pub fn new(body: T) -> Request<T> {
        Request {
            head: Head::new(),
            body,
        }
    }

    /// 使用给定的头部和正文创建请求。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let request = Request::new("hello world");
    /// let (mut head, body) = request.into_head();
    ///
    /// head.method = Method::POST;
    /// let request = Request::from_head(head, body);
    ///
    /// assert_eq!(request.method(), Method::POST);
    /// assert_eq!(*request.body(), "hello world");
    /// ```
    #[inline]
    pub fn from_head(head: Head, body: T) -> Request<T> {
        Request { head, body }
    }

    /// 获取请求的HTTP方法的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let request: Request<()> = Request::default();
    ///
    /// assert_eq!(*request.method(), Method::GET);
    /// ```
    #[inline]
    pub fn method(&self) -> &Method {
        &self.head.method
    }

    /// 获取请求的HTTP方法的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let mut request: Request<()> = Request::default();
    /// *request.method_mut() = Method::PUT;
    ///
    /// assert_eq!(*request.method(), Method::PUT);
    /// ```
    #[inline]
    pub fn method_mut(&mut self) -> &mut Method {
        &mut self.head.method
    }

    /// 获取请求的URI的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request: Request<()> = Request::default();
    ///
    /// assert_eq!(*request.uri(), *"/");
    /// ```
    #[inline]
    pub fn uri(&self) -> &Uri {
        &self.head.uri
    }

    /// 获取请求的URI的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let mut request: Request<()> = Request::default();
    /// *request.uri_mut() = "/hello".parse().unwrap();
    ///
    /// assert_eq!(*request.uri(), *"/hello");
    /// ```
    #[inline]
    pub fn uri_mut(&mut self) -> &mut Uri {
        &mut self.head.uri
    }

    /// 获取请求的版本的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Request, Version};
    ///
    /// let request: Request<()> = Request::default();
    ///
    /// assert_eq!(request.version(), Version::HTTP_11);
    /// ```
    #[inline]
    pub fn version(&self) -> Version {
        self.head.version
    }

    /// 获取请求的版本的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Request, Version};
    ///
    /// let mut request: Request<()> = Request::default();
    /// *request.version_mut() = Version::HTTP_2;
    ///
    /// assert_eq!(request.version(), Version::HTTP_2);
    /// ```
    #[inline]
    pub fn version_mut(&mut self) -> &mut Version {
        &mut self.head.version
    }

    /// 获取请求的标头集的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request: Request<()> = Request::default();
    ///
    /// assert!(request.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        &self.head.headers
    }

    /// 获取请求的标头集的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    /// use puzz_http::header::*;
    ///
    /// let mut request: Request<()> = Request::default();
    /// request.headers_mut().insert(HOST, HeaderValue::from_static("world"));
    ///
    /// assert!(!request.headers().is_empty());
    /// ```
    #[inline]
    pub fn headers_mut(&mut self) -> &mut HeaderMap<HeaderValue> {
        &mut self.head.headers
    }

    /// 获取请求的扩展的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request: Request<()> = Request::default();
    ///
    /// assert!(request.extensions().get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn extensions(&self) -> &Extensions {
        &self.head.extensions
    }

    /// 获取请求的扩展的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let mut request: Request<()> = Request::default();
    /// request.extensions_mut().insert("hello");
    ///
    /// assert_eq!(request.extensions().get(), Some(&"hello"));
    /// ```
    #[inline]
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.head.extensions
    }

    /// 获取请求的正文的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request: Request<String> = Request::default();
    ///
    /// assert!(request.body().is_empty());
    /// ```
    #[inline]
    pub fn body(&self) -> &T {
        &self.body
    }

    /// 获取请求的正文的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let mut request: Request<String> = Request::default();
    /// request.body_mut().push_str("hello world");
    ///
    /// assert!(!request.body().is_empty());
    /// ```
    #[inline]
    pub fn body_mut(&mut self) -> &mut T {
        &mut self.body
    }

    /// 消耗请求，只返回请求的正文。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request = Request::new(10);
    /// let body = request.into_body();
    ///
    /// assert_eq!(body, 10);
    /// ```
    #[inline]
    pub fn into_body(self) -> T {
        self.body
    }

    /// 消耗请求，返回请求的头部和正文。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let request = Request::new(());
    /// let (head, body) = request.into_head();
    ///
    /// assert_eq!(head.method, Method::GET);
    /// ```
    #[inline]
    pub fn into_head(self) -> (Head, T) {
        (self.head, self.body)
    }

    /// 消耗请求，返回带有给定正文的新请求，其正文为传入函数的返回值。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request = Request::builder().body("some string").unwrap();
    /// let mapped_request: Request<&[u8]> = request.map(|b| {
    ///   assert_eq!(b, "some string");
    ///   b.as_bytes()
    /// });
    /// assert_eq!(mapped_request.body(), &"some string".as_bytes());
    /// ```
    #[inline]
    pub fn map<F, U>(self, f: F) -> Request<U>
    where
        F: FnOnce(T) -> U,
    {
        Request {
            body: f(self.body),
            head: self.head,
        }
    }
}

impl<T: Default> Default for Request<T> {
    fn default() -> Request<T> {
        Request::new(T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for Request<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Request")
            .field("method", self.method())
            .field("uri", self.uri())
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
            .field("method", &self.method)
            .field("uri", &self.uri)
            .field("version", &self.version)
            .field("headers", &self.headers)
            .finish()
    }
}

impl Builder {
    /// 创建构建器的默认实例以构建请求。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::request::Builder;
    ///
    /// let req = Builder::new()
    ///     .method("POST")
    ///     .body(())
    ///     .unwrap();
    /// ```
    #[inline]
    pub fn new() -> Builder {
        Builder::default()
    }

    /// 设置请求的HTTP方法。
    ///
    /// 默认情况下，这是`GET`。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder()
    ///     .method("POST")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn method<T>(self, method: T) -> Builder
    where
        Method: TryFrom<T>,
        <Method as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            let method = TryFrom::try_from(method).map_err(Into::into)?;
            head.method = method;
            Ok(head)
        })
    }

    /// 获取请求的HTTP方法。
    ///
    /// 默认情况下，这是`GET`。如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Method, Request};
    ///
    /// let mut req = Request::builder();
    /// assert_eq!(req.method_ref(), Some(&Method::GET));
    ///
    /// req = req.method("POST");
    /// assert_eq!(req.method_ref(), Some(&Method::POST));
    /// ```
    pub fn method_ref(&self) -> Option<&Method> {
        self.inner.as_ref().ok().map(|h| &h.method)
    }

    /// 设置请求的URI。
    ///
    /// 默认情况下，这是`/`。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder()
    ///     .uri("https://www.rust-lang.org/")
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn uri<T>(self, uri: T) -> Builder
    where
        Uri: TryFrom<T>,
        <Uri as TryFrom<T>>::Error: Into<crate::Error>,
    {
        self.and_then(move |mut head| {
            head.uri = TryFrom::try_from(uri).map_err(Into::into)?;
            Ok(head)
        })
    }

    /// 获取请求的URI。
    ///
    /// 默认情况下，这是`/`。如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let mut req = Request::builder();
    /// assert_eq!(req.uri_ref().unwrap(), "/");
    ///
    /// req = req.uri("https://www.rust-lang.org/");
    /// assert_eq!(req.uri_ref().unwrap(), "https://www.rust-lang.org/");
    /// ```
    pub fn uri_ref(&self) -> Option<&Uri> {
        self.inner.as_ref().ok().map(|h| &h.uri)
    }

    /// 设置请求的HTTP版本。
    ///
    /// 默认情况下，这是`HTTP/1.1`。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Request, Version};
    ///
    /// let req = Request::builder()
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

    /// 获取请求的HTTP版本。
    ///
    /// 默认情况下，这是`HTTP/1.1`。如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{Request, Version};
    ///
    /// let mut req = Request::builder();
    /// assert_eq!(req.version_ref().unwrap(), &Version::HTTP_11);
    ///
    /// req = req.version(Version::HTTP_2);
    /// assert_eq!(req.version_ref().unwrap(), &Version::HTTP_2);
    /// ```
    pub fn version_ref(&self) -> Option<&Version> {
        self.inner.as_ref().ok().map(|h| &h.version)
    }

    /// 将标头追加到请求中。
    ///
    /// 此函数将提供的键值对追加到请求内部的[`HeaderMap`]中。本质上，
    /// 这相当于调用[`HeaderMap::append`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder()
    ///     .header("Accept", "text/html")
    ///     .header("X-Custom-Foo", "bar")
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

    /// 获取请求的标头集的引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder()
    ///     .header("Accept", "text/html")
    ///     .header("X-Custom-Foo", "bar");
    ///
    /// let headers = req.headers_ref().unwrap();
    ///
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_ref(&self) -> Option<&HeaderMap<HeaderValue>> {
        self.inner.as_ref().ok().map(|h| &h.headers)
    }

    /// 获取请求的标头集的可变引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::{HeaderValue, Request};
    ///
    /// let mut req = Request::builder();
    ///
    /// let headers = req.headers_mut().unwrap();
    /// headers.insert("Accept", HeaderValue::from_static("text/html"));
    /// headers.insert("X-Custom-Foo", HeaderValue::from_static("bar"));
    ///
    /// let headers = req.headers_ref().unwrap();
    /// assert_eq!( headers["Accept"], "text/html" );
    /// assert_eq!( headers["X-Custom-Foo"], "bar" );
    /// ```
    pub fn headers_mut(&mut self) -> Option<&mut HeaderMap<HeaderValue>> {
        self.inner.as_mut().ok().map(|h| &mut h.headers)
    }

    /// 将一个类型添加到请求的扩展中。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder()
    ///     .extension("My Extension")
    ///     .body(())
    ///     .unwrap();
    ///
    /// assert_eq!(req.extensions().get::<&'static str>(),
    ///            Some(&"My Extension"));
    /// ```
    pub fn extension<T: 'static>(self, extension: T) -> Builder {
        self.and_then(move |mut head| {
            head.extensions.insert(extension);
            Ok(head)
        })
    }

    /// 获取请求的扩展的引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let req = Request::builder().extension("My Extension").extension(5u32);
    /// let extensions = req.extensions_ref().unwrap();
    ///
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_ref(&self) -> Option<&Extensions> {
        self.inner.as_ref().ok().map(|h| &h.extensions)
    }

    /// 获取请求的扩展的可变引用。
    ///
    /// 如果构建器有错误，则返回[`None`]。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let mut req = Request::builder().extension("My Extension");
    /// let mut extensions = req.extensions_mut().unwrap();
    /// assert_eq!(extensions.get::<&'static str>(), Some(&"My Extension"));
    ///
    /// extensions.insert(5u32);
    /// assert_eq!(extensions.get::<u32>(), Some(&5u32));
    /// ```
    pub fn extensions_mut(&mut self) -> Option<&mut Extensions> {
        self.inner.as_mut().ok().map(|h| &mut h.extensions)
    }

    /// 消耗构建器，使用给定的正文构建请求。
    ///
    /// # 错误
    ///
    /// 如果之前配置的任意一个参数发生错误，则在调用此函数时将错误返回。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Request;
    ///
    /// let request = Request::builder()
    ///     .body(())
    ///     .unwrap();
    /// ```
    pub fn body<T>(self, body: T) -> Result<Request<T>> {
        self.inner.map(move |head| Request { head, body })
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
