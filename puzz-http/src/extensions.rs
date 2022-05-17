use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::fmt;
use std::hash::{BuildHasherDefault, Hasher};

type AnyMap = HashMap<TypeId, Box<dyn Any>, BuildHasherDefault<IdHasher>>;

// 使用`TypeId`作为键时，不需要进行散列。
#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

/// 请求和响应的扩展组件。
///
/// 请求和响应可以使用扩展来存储额外的数据。
#[derive(Default)]
pub struct Extensions {
    // 如果从不使用扩展，则无需携带空的`HashMap`，并且这只占用一个字长。
    map: Option<Box<AnyMap>>,
}

impl Extensions {
    /// 创建一个空的扩展。
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// 向扩展中插入一个类型。
    ///
    /// 如果这种类型已经存在，将替换并返回先前插入的值。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    ///
    /// assert!(ext.insert(5i32).is_none());
    /// assert!(ext.insert(4u8).is_none());
    /// assert_eq!(ext.insert(9i32), Some(5i32));
    /// ```
    pub fn insert<T: 'static>(&mut self, val: T) -> Option<T> {
        self.map
            .get_or_insert_with(|| Box::new(HashMap::default()))
            .insert(TypeId::of::<T>(), Box::new(val))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    /// 获取先前插入扩展的类型的引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    /// assert!(ext.get::<i32>().is_none());
    ///
    /// ext.insert(5i32);
    /// assert_eq!(ext.get::<i32>(), Some(&5i32));
    /// ```
    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .as_ref()
            .and_then(|map| map.get(&TypeId::of::<T>()))
            .and_then(|boxed| boxed.downcast_ref())
    }

    /// 获取先前插入扩展的类型的可变引用。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    ///
    /// ext.insert(String::from("Hello"));
    /// ext.get_mut::<String>().unwrap().push_str(" World");
    ///
    /// assert_eq!(ext.get::<String>().unwrap(), "Hello World");
    /// ```
    pub fn get_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.map
            .as_mut()
            .and_then(|map| map.get_mut(&TypeId::of::<T>()))
            .and_then(|boxed| boxed.downcast_mut())
    }

    /// 从扩展中删除一个类型。
    ///
    /// 如果这种类型存在，将删除并返回此类型。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    ///
    /// ext.insert(5i32);
    ///
    /// assert_eq!(ext.remove::<i32>(), Some(5i32));
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    pub fn remove<T: 'static>(&mut self) -> Option<T> {
        self.map
            .as_mut()
            .and_then(|map| map.remove(&TypeId::of::<T>()))
            .and_then(|boxed| boxed.downcast().ok().map(|boxed| *boxed))
    }

    /// 清空扩展中的所有类型。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    ///
    /// ext.insert(5i32);
    /// ext.clear();
    ///
    /// assert!(ext.get::<i32>().is_none());
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        if let Some(ref mut map) = self.map {
            map.clear();
        }
    }

    /// 检查扩展是否为空。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    /// assert!(ext.is_empty());
    ///
    /// ext.insert(5i32);
    /// assert!(!ext.is_empty());
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.map.as_ref().map_or(true, |map| map.is_empty())
    }

    /// 获取扩展中已插入类型的数量。
    ///
    /// # 例子
    ///
    /// ```
    /// use puzz_http::Extensions;
    ///
    /// let mut ext = Extensions::new();
    /// assert_eq!(ext.len(), 0);
    ///
    /// ext.insert(5i32);
    /// assert_eq!(ext.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.map.as_ref().map_or(0, |map| map.len())
    }
}

impl fmt::Debug for Extensions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Extensions").finish()
    }
}
