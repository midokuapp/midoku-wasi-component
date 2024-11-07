#[allow(dead_code)]
pub mod midoku {
    #[allow(dead_code)]
    pub mod http {
        #[allow(dead_code, clippy::all)]
        pub mod types {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Represents an HTTP method.
            #[repr(u8)]
            #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
            pub enum Method {
                Get,
                Post,
                Put,
                Head,
                Delete,
            }
            impl ::core::fmt::Debug for Method {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Method::Get => f.debug_tuple("Method::Get").finish(),
                        Method::Post => f.debug_tuple("Method::Post").finish(),
                        Method::Put => f.debug_tuple("Method::Put").finish(),
                        Method::Head => f.debug_tuple("Method::Head").finish(),
                        Method::Delete => f.debug_tuple("Method::Delete").finish(),
                    }
                }
            }
            impl Method {
                #[doc(hidden)]
                pub unsafe fn _lift(val: u8) -> Method {
                    if !cfg!(debug_assertions) {
                        return ::core::mem::transmute(val);
                    }
                    match val {
                        0 => Method::Get,
                        1 => Method::Post,
                        2 => Method::Put,
                        3 => Method::Head,
                        4 => Method::Delete,
                        _ => panic!("invalid enum discriminant"),
                    }
                }
            }
            /// Represents a response from an HTTP request.
            #[derive(Debug)]
            #[repr(transparent)]
            pub struct IncomingResponse {
                handle: _rt::Resource<IncomingResponse>,
            }
            impl IncomingResponse {
                #[doc(hidden)]
                pub unsafe fn from_handle(handle: u32) -> Self {
                    Self {
                        handle: _rt::Resource::from_handle(handle),
                    }
                }
                #[doc(hidden)]
                pub fn take_handle(&self) -> u32 {
                    _rt::Resource::take_handle(&self.handle)
                }
                #[doc(hidden)]
                pub fn handle(&self) -> u32 {
                    _rt::Resource::handle(&self.handle)
                }
            }
            unsafe impl _rt::WasmResource for IncomingResponse {
                #[inline]
                unsafe fn drop(_handle: u32) {
                    #[cfg(not(target_arch = "wasm32"))]
                    unreachable!();
                    #[cfg(target_arch = "wasm32")]
                    {
                        #[link(wasm_import_module = "midoku:http/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[resource-drop]incoming-response"]
                            fn drop(_: u32);
                        }
                        drop(_handle);
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the status code of the response.
                pub fn status_code(&self) -> u16 {
                    unsafe {
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "midoku:http/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.status-code"]
                            fn wit_import(_: i32) -> i32;
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32) -> i32 {
                            unreachable!()
                        }
                        let ret = wit_import((self).handle() as i32);
                        ret as u16
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the headers of the response.
                pub fn headers(&self) -> _rt::Vec<(_rt::String, _rt::String)> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "midoku:http/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.headers"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let base9 = l1;
                        let len9 = l2;
                        let mut result9 = _rt::Vec::with_capacity(len9);
                        for i in 0..len9 {
                            let base = base9.add(i * 16);
                            let e9 = {
                                let l3 = *base.add(0).cast::<*mut u8>();
                                let l4 = *base.add(4).cast::<usize>();
                                let len5 = l4;
                                let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);
                                let l6 = *base.add(8).cast::<*mut u8>();
                                let l7 = *base.add(12).cast::<usize>();
                                let len8 = l7;
                                let bytes8 = _rt::Vec::from_raw_parts(l6.cast(), len8, len8);
                                (_rt::string_lift(bytes5), _rt::string_lift(bytes8))
                            };
                            result9.push(e9);
                        }
                        _rt::cabi_dealloc(base9, len9 * 16, 4);
                        result9
                    }
                }
            }
            impl IncomingResponse {
                #[allow(unused_unsafe, clippy::all)]
                /// Returns the bytes of the response.
                pub fn bytes(&self) -> _rt::Vec<u8> {
                    unsafe {
                        #[repr(align(4))]
                        struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                        let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                        let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                        #[cfg(target_arch = "wasm32")]
                        #[link(wasm_import_module = "midoku:http/types@0.1.0")]
                        extern "C" {
                            #[link_name = "[method]incoming-response.bytes"]
                            fn wit_import(_: i32, _: *mut u8);
                        }
                        #[cfg(not(target_arch = "wasm32"))]
                        fn wit_import(_: i32, _: *mut u8) {
                            unreachable!()
                        }
                        wit_import((self).handle() as i32, ptr0);
                        let l1 = *ptr0.add(0).cast::<*mut u8>();
                        let l2 = *ptr0.add(4).cast::<usize>();
                        let len3 = l2;
                        _rt::Vec::from_raw_parts(l1.cast(), len3, len3)
                    }
                }
            }
        }
        #[allow(dead_code, clippy::all)]
        pub mod outgoing_handler {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            pub type Method = super::super::super::midoku::http::types::Method;
            pub type IncomingResponse = super::super::super::midoku::http::types::IncomingResponse;
            #[allow(unused_unsafe, clippy::all)]
            /// Sends an HTTP request.
            ///
            /// The `url` must be a valid URL.
            ///
            /// The `headers` must be a list of tuples where the first element is the
            /// header name and the second element is the header value.
            ///
            /// The `body` must be a list of bytes.
            ///
            /// Returns the response from the server.
            pub fn handle(
                method: Method,
                url: &str,
                headers: Option<&[(_rt::String, _rt::String)]>,
                body: Option<&[u8]>,
            ) -> Result<IncomingResponse, ()> {
                unsafe {
                    let mut cleanup_list = _rt::Vec::new();
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = url;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let (result5_0, result5_1, result5_2) = match headers {
                        Some(e) => {
                            let vec4 = e;
                            let len4 = vec4.len();
                            let layout4 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec4.len() * 16, 4);
                            let result4 = if layout4.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout4).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout4);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec4.into_iter().enumerate() {
                                let base = result4.add(i * 16);
                                {
                                    let (t1_0, t1_1) = e;
                                    let vec2 = t1_0;
                                    let ptr2 = vec2.as_ptr().cast::<u8>();
                                    let len2 = vec2.len();
                                    *base.add(4).cast::<usize>() = len2;
                                    *base.add(0).cast::<*mut u8>() = ptr2.cast_mut();
                                    let vec3 = t1_1;
                                    let ptr3 = vec3.as_ptr().cast::<u8>();
                                    let len3 = vec3.len();
                                    *base.add(12).cast::<usize>() = len3;
                                    *base.add(8).cast::<*mut u8>() = ptr3.cast_mut();
                                }
                            }
                            cleanup_list.extend_from_slice(&[(result4, layout4)]);
                            (1i32, result4, len4)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    let (result7_0, result7_1, result7_2) = match body {
                        Some(e) => {
                            let vec6 = e;
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            (1i32, ptr6.cast_mut(), len6)
                        }
                        None => (0i32, ::core::ptr::null_mut(), 0usize),
                    };
                    let ptr8 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:http/outgoing-handler@0.1.0")]
                    extern "C" {
                        #[link_name = "handle"]
                        fn wit_import(
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: i32,
                            _: *mut u8,
                            _: usize,
                            _: *mut u8,
                        );
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: i32,
                        _: *mut u8,
                        _: usize,
                        _: *mut u8,
                    ) {
                        unreachable!()
                    }
                    wit_import(
                        method.clone() as i32,
                        ptr0.cast_mut(),
                        len0,
                        result5_0,
                        result5_1,
                        result5_2,
                        result7_0,
                        result7_1,
                        result7_2,
                        ptr8,
                    );
                    let l9 = i32::from(*ptr8.add(0).cast::<u8>());
                    for (ptr, layout) in cleanup_list {
                        if layout.size() != 0 {
                            _rt::alloc::dealloc(ptr.cast(), layout);
                        }
                    }
                    match l9 {
                        0 => {
                            let e = {
                                let l10 = *ptr8.add(4).cast::<i32>();
                                super::super::super::midoku::http::types::IncomingResponse::from_handle(
                                    l10 as u32,
                                )
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = ();
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod limiter {
        #[allow(dead_code, clippy::all)]
        pub mod rate_limiter {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the burst size of the limiter. The burst size is the maximum
            /// number of calls that can be made in a single period.
            ///
            /// Returns `Some(burst)` if the burst size is set, `None` otherwise.
            pub fn burst() -> Option<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "burst"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                l2 as u32
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the period in milliseconds of the limiter. The period is the
            /// duration in milliseconds over which the burst size is enforced.
            ///
            /// Returns `Some(period)` if the period is set, `None` otherwise.
            pub fn period_ms() -> Option<u32> {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let ptr0 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "period-ms"]
                        fn wit_import(_: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0);
                    let l1 = i32::from(*ptr0.add(0).cast::<u8>());
                    match l1 {
                        0 => None,
                        1 => {
                            let e = {
                                let l2 = *ptr0.add(4).cast::<i32>();
                                l2 as u32
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Sets the burst size of the limiter. The burst size is the maximum number
            /// of calls that can be made in a single period.
            ///
            /// Returns `Ok` if the burst size was set successfully, `Err` otherwise.
            pub fn set_burst(burst: u32) -> Result<(), ()> {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "set-burst"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&burst));
                    match ret {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = ();
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Sets the period in milliseconds of the limiter. The period is the
            /// duration in milliseconds over which the burst size is enforced.
            ///
            /// Returns `Ok` if the period was set successfully, `Err` otherwise.
            pub fn set_period_ms(period_ms: u32) -> Result<(), ()> {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "set-period-ms"]
                        fn wit_import(_: i32) -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: i32) -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import(_rt::as_i32(&period_ms));
                    match ret {
                        0 => {
                            let e = ();
                            Ok(e)
                        }
                        1 => {
                            let e = ();
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Returns the readyness of the limiter. This function allow a single cell
            /// to go through the limiter without blocking if the limiter is ready.
            ///
            /// Calling this function consumes the limiter's burst size, meaning calling
            /// it multiple times will return `false` after the burst size has been
            /// consumed.
            ///
            /// Returns `true` if the limiter is ready, `false` otherwise.
            pub fn ready() -> bool {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "ready"]
                        fn wit_import() -> i32;
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() -> i32 {
                        unreachable!()
                    }
                    let ret = wit_import();
                    _rt::bool_lift(ret as u8)
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// `block` returns immediately if the rate limit is not exceeded, otherwise
            /// it blocks until the limiter allows the call to proceed.
            pub fn block() {
                unsafe {
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:limiter/rate-limiter@0.1.0")]
                    extern "C" {
                        #[link_name = "block"]
                        fn wit_import();
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import() {
                        unreachable!()
                    }
                    wit_import();
                }
            }
        }
    }
    #[allow(dead_code)]
    pub mod settings {
        #[allow(dead_code, clippy::all)]
        pub mod settings {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() =
                super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            /// Represents an arbitrary number.
            #[derive(Clone, Copy)]
            pub enum Number {
                S64(i64),
                U64(u64),
                F64(f64),
            }
            impl ::core::fmt::Debug for Number {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Number::S64(e) => f.debug_tuple("Number::S64").field(e).finish(),
                        Number::U64(e) => f.debug_tuple("Number::U64").field(e).finish(),
                        Number::F64(e) => f.debug_tuple("Number::F64").field(e).finish(),
                    }
                }
            }
            /// Represents an arbitrary value.
            #[derive(Clone)]
            pub enum Value {
                Bool(bool),
                Number(Number),
                String(_rt::String),
                /// Represents an array of strings.
                Array(_rt::Vec<_rt::String>),
                /// Represents a map with string keys and string values.
                Map(_rt::Vec<(_rt::String, _rt::String)>),
            }
            impl ::core::fmt::Debug for Value {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    match self {
                        Value::Bool(e) => f.debug_tuple("Value::Bool").field(e).finish(),
                        Value::Number(e) => f.debug_tuple("Value::Number").field(e).finish(),
                        Value::String(e) => f.debug_tuple("Value::String").field(e).finish(),
                        Value::Array(e) => f.debug_tuple("Value::Array").field(e).finish(),
                        Value::Map(e) => f.debug_tuple("Value::Map").field(e).finish(),
                    }
                }
            }
            #[allow(unused_unsafe, clippy::all)]
            /// Get the value of a setting from the host.
            pub fn get(key: &str) -> Result<Value, ()> {
                unsafe {
                    #[repr(align(8))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 32]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 32]);
                    let vec0 = key;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "midoku:settings/settings@0.1.0")]
                    extern "C" {
                        #[link_name = "get"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = i32::from(*ptr1.add(0).cast::<u8>());
                    match l2 {
                        0 => {
                            let e = {
                                let l3 = i32::from(*ptr1.add(8).cast::<u8>());
                                let v28 = match l3 {
                                    0 => {
                                        let e28 = {
                                            let l4 = i32::from(*ptr1.add(16).cast::<u8>());
                                            _rt::bool_lift(l4 as u8)
                                        };
                                        Value::Bool(e28)
                                    }
                                    1 => {
                                        let e28 = {
                                            let l5 = i32::from(*ptr1.add(16).cast::<u8>());
                                            let v9 = match l5 {
                                                0 => {
                                                    let e9 = {
                                                        let l6 = *ptr1.add(24).cast::<i64>();
                                                        l6
                                                    };
                                                    Number::S64(e9)
                                                }
                                                1 => {
                                                    let e9 = {
                                                        let l7 = *ptr1.add(24).cast::<i64>();
                                                        l7 as u64
                                                    };
                                                    Number::U64(e9)
                                                }
                                                n => {
                                                    debug_assert_eq!(
                                                        n, 2,
                                                        "invalid enum discriminant"
                                                    );
                                                    let e9 = {
                                                        let l8 = *ptr1.add(24).cast::<f64>();
                                                        l8
                                                    };
                                                    Number::F64(e9)
                                                }
                                            };
                                            v9
                                        };
                                        Value::Number(e28)
                                    }
                                    2 => {
                                        let e28 = {
                                            let l10 = *ptr1.add(16).cast::<*mut u8>();
                                            let l11 = *ptr1.add(20).cast::<usize>();
                                            let len12 = l11;
                                            let bytes12 =
                                                _rt::Vec::from_raw_parts(l10.cast(), len12, len12);
                                            _rt::string_lift(bytes12)
                                        };
                                        Value::String(e28)
                                    }
                                    3 => {
                                        let e28 = {
                                            let l13 = *ptr1.add(16).cast::<*mut u8>();
                                            let l14 = *ptr1.add(20).cast::<usize>();
                                            let base18 = l13;
                                            let len18 = l14;
                                            let mut result18 = _rt::Vec::with_capacity(len18);
                                            for i in 0..len18 {
                                                let base = base18.add(i * 8);
                                                let e18 = {
                                                    let l15 = *base.add(0).cast::<*mut u8>();
                                                    let l16 = *base.add(4).cast::<usize>();
                                                    let len17 = l16;
                                                    let bytes17 = _rt::Vec::from_raw_parts(
                                                        l15.cast(),
                                                        len17,
                                                        len17,
                                                    );
                                                    _rt::string_lift(bytes17)
                                                };
                                                result18.push(e18);
                                            }
                                            _rt::cabi_dealloc(base18, len18 * 8, 4);
                                            result18
                                        };
                                        Value::Array(e28)
                                    }
                                    n => {
                                        debug_assert_eq!(n, 4, "invalid enum discriminant");
                                        let e28 = {
                                            let l19 = *ptr1.add(16).cast::<*mut u8>();
                                            let l20 = *ptr1.add(20).cast::<usize>();
                                            let base27 = l19;
                                            let len27 = l20;
                                            let mut result27 = _rt::Vec::with_capacity(len27);
                                            for i in 0..len27 {
                                                let base = base27.add(i * 16);
                                                let e27 = {
                                                    let l21 = *base.add(0).cast::<*mut u8>();
                                                    let l22 = *base.add(4).cast::<usize>();
                                                    let len23 = l22;
                                                    let bytes23 = _rt::Vec::from_raw_parts(
                                                        l21.cast(),
                                                        len23,
                                                        len23,
                                                    );
                                                    let l24 = *base.add(8).cast::<*mut u8>();
                                                    let l25 = *base.add(12).cast::<usize>();
                                                    let len26 = l25;
                                                    let bytes26 = _rt::Vec::from_raw_parts(
                                                        l24.cast(),
                                                        len26,
                                                        len26,
                                                    );
                                                    (
                                                        _rt::string_lift(bytes23),
                                                        _rt::string_lift(bytes26),
                                                    )
                                                };
                                                result27.push(e27);
                                            }
                                            _rt::cabi_dealloc(base27, len27 * 16, 4);
                                            result27
                                        };
                                        Value::Map(e28)
                                    }
                                };
                                v28
                            };
                            Ok(e)
                        }
                        1 => {
                            let e = ();
                            Err(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    }
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod midoku {
        #[allow(dead_code)]
        pub mod bindings {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                pub type Chapter =
                    super::super::super::super::exports::midoku::types::chapter::Chapter;
                pub type Filter =
                    super::super::super::super::exports::midoku::types::filter::Filter;
                pub type Manga = super::super::super::super::exports::midoku::types::manga::Manga;
                pub type Page = super::super::super::super::exports::midoku::types::page::Page;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_initialize_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::initialize();
                    let result1 = match result0 {
                        Ok(_) => 0i32,
                        Err(_) => 1i32,
                    };
                    result1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_manga_list_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let base7 = arg0;
                    let len7 = arg1;
                    let mut result7 = _rt::Vec::with_capacity(len7);
                    for i in 0..len7 {
                        let base = base7.add(i * 12);
                        let e7 = {
                            let l0 = i32::from(*base.add(0).cast::<u8>());
                            use super::super::super::super::exports::midoku::types::filter::Filter as V6;
                            let v6 = match l0 {
                                0 => {
                                    let e6 = {
                                        let l1 = *base.add(4).cast::<*mut u8>();
                                        let l2 = *base.add(8).cast::<usize>();
                                        let len3 = l2;
                                        let bytes3 =
                                            _rt::Vec::from_raw_parts(l1.cast(), len3, len3);
                                        super::super::super::super::exports::midoku::types::filter::FilterTitle {
                                            query: _rt::string_lift(bytes3),
                                        }
                                    };
                                    V6::Title(e6)
                                }
                                n => {
                                    debug_assert_eq!(n, 1, "invalid enum discriminant");
                                    let e6 = {
                                        let l4 = *base.add(4).cast::<i32>();
                                        let l5 = i32::from(*base.add(8).cast::<u8>());
                                        super::super::super::super::exports::midoku::types::filter::FilterSort {
                                            option_index: l4 as u32,
                                            option_reversed: _rt::bool_lift(l5 as u8),
                                        }
                                    };
                                    V6::Sort(e6)
                                }
                            };
                            v6
                        };
                        result7.push(e7);
                    }
                    _rt::cabi_dealloc(base7, len7 * 12, 4);
                    let result8 = T::get_manga_list(result7, arg2 as u32);
                    let ptr9 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result8 {
                        Ok(e) => {
                            *ptr9.add(0).cast::<u8>() = (0i32) as u8;
                            let (t10_0, t10_1) = e;
                            let vec21 = t10_0;
                            let len21 = vec21.len();
                            let layout21 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec21.len() * 68, 4);
                            let result21 = if layout21.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout21).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout21);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec21.into_iter().enumerate() {
                                let base = result21.add(i * 68);
                                {
                                    let super::super::super::super::exports::midoku::types::manga::Manga {
                                        id: id11,
                                        title: title11,
                                        url: url11,
                                        description: description11,
                                        cover_url: cover_url11,
                                        author_name: author_name11,
                                        artist_name: artist_name11,
                                        categories: categories11,
                                        status: status11,
                                        content_rating: content_rating11,
                                        reading_mode: reading_mode11,
                                    } = e;
                                    let vec12 = (id11.into_bytes()).into_boxed_slice();
                                    let ptr12 = vec12.as_ptr().cast::<u8>();
                                    let len12 = vec12.len();
                                    ::core::mem::forget(vec12);
                                    *base.add(4).cast::<usize>() = len12;
                                    *base.add(0).cast::<*mut u8>() = ptr12.cast_mut();
                                    let vec13 = (title11.into_bytes()).into_boxed_slice();
                                    let ptr13 = vec13.as_ptr().cast::<u8>();
                                    let len13 = vec13.len();
                                    ::core::mem::forget(vec13);
                                    *base.add(12).cast::<usize>() = len13;
                                    *base.add(8).cast::<*mut u8>() = ptr13.cast_mut();
                                    let vec14 = (url11.into_bytes()).into_boxed_slice();
                                    let ptr14 = vec14.as_ptr().cast::<u8>();
                                    let len14 = vec14.len();
                                    ::core::mem::forget(vec14);
                                    *base.add(20).cast::<usize>() = len14;
                                    *base.add(16).cast::<*mut u8>() = ptr14.cast_mut();
                                    let vec15 = (description11.into_bytes()).into_boxed_slice();
                                    let ptr15 = vec15.as_ptr().cast::<u8>();
                                    let len15 = vec15.len();
                                    ::core::mem::forget(vec15);
                                    *base.add(28).cast::<usize>() = len15;
                                    *base.add(24).cast::<*mut u8>() = ptr15.cast_mut();
                                    let vec16 = (cover_url11.into_bytes()).into_boxed_slice();
                                    let ptr16 = vec16.as_ptr().cast::<u8>();
                                    let len16 = vec16.len();
                                    ::core::mem::forget(vec16);
                                    *base.add(36).cast::<usize>() = len16;
                                    *base.add(32).cast::<*mut u8>() = ptr16.cast_mut();
                                    let vec17 = (author_name11.into_bytes()).into_boxed_slice();
                                    let ptr17 = vec17.as_ptr().cast::<u8>();
                                    let len17 = vec17.len();
                                    ::core::mem::forget(vec17);
                                    *base.add(44).cast::<usize>() = len17;
                                    *base.add(40).cast::<*mut u8>() = ptr17.cast_mut();
                                    let vec18 = (artist_name11.into_bytes()).into_boxed_slice();
                                    let ptr18 = vec18.as_ptr().cast::<u8>();
                                    let len18 = vec18.len();
                                    ::core::mem::forget(vec18);
                                    *base.add(52).cast::<usize>() = len18;
                                    *base.add(48).cast::<*mut u8>() = ptr18.cast_mut();
                                    let vec20 = categories11;
                                    let len20 = vec20.len();
                                    let layout20 = _rt::alloc::Layout::from_size_align_unchecked(
                                        vec20.len() * 8,
                                        4,
                                    );
                                    let result20 = if layout20.size() != 0 {
                                        let ptr = _rt::alloc::alloc(layout20).cast::<u8>();
                                        if ptr.is_null() {
                                            _rt::alloc::handle_alloc_error(layout20);
                                        }
                                        ptr
                                    } else {
                                        ::core::ptr::null_mut()
                                    };
                                    for (i, e) in vec20.into_iter().enumerate() {
                                        let base = result20.add(i * 8);
                                        {
                                            let vec19 = (e.into_bytes()).into_boxed_slice();
                                            let ptr19 = vec19.as_ptr().cast::<u8>();
                                            let len19 = vec19.len();
                                            ::core::mem::forget(vec19);
                                            *base.add(4).cast::<usize>() = len19;
                                            *base.add(0).cast::<*mut u8>() = ptr19.cast_mut();
                                        }
                                    }
                                    *base.add(60).cast::<usize>() = len20;
                                    *base.add(56).cast::<*mut u8>() = result20;
                                    *base.add(64).cast::<u8>() = (status11.clone() as i32) as u8;
                                    *base.add(65).cast::<u8>() =
                                        (content_rating11.clone() as i32) as u8;
                                    *base.add(66).cast::<u8>() =
                                        (reading_mode11.clone() as i32) as u8;
                                }
                            }
                            *ptr9.add(8).cast::<usize>() = len21;
                            *ptr9.add(4).cast::<*mut u8>() = result21;
                            *ptr9.add(12).cast::<u8>() = (match t10_1 {
                                true => 1,
                                false => 0,
                            }) as u8;
                        }
                        Err(_) => {
                            *ptr9.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr9
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_manga_list<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base22 = l1;
                            let len22 = l2;
                            for i in 0..len22 {
                                let base = base22.add(i * 68);
                                {
                                    let l3 = *base.add(0).cast::<*mut u8>();
                                    let l4 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(8).cast::<*mut u8>();
                                    let l6 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                    let l7 = *base.add(16).cast::<*mut u8>();
                                    let l8 = *base.add(20).cast::<usize>();
                                    _rt::cabi_dealloc(l7, l8, 1);
                                    let l9 = *base.add(24).cast::<*mut u8>();
                                    let l10 = *base.add(28).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                    let l11 = *base.add(32).cast::<*mut u8>();
                                    let l12 = *base.add(36).cast::<usize>();
                                    _rt::cabi_dealloc(l11, l12, 1);
                                    let l13 = *base.add(40).cast::<*mut u8>();
                                    let l14 = *base.add(44).cast::<usize>();
                                    _rt::cabi_dealloc(l13, l14, 1);
                                    let l15 = *base.add(48).cast::<*mut u8>();
                                    let l16 = *base.add(52).cast::<usize>();
                                    _rt::cabi_dealloc(l15, l16, 1);
                                    let l17 = *base.add(56).cast::<*mut u8>();
                                    let l18 = *base.add(60).cast::<usize>();
                                    let base21 = l17;
                                    let len21 = l18;
                                    for i in 0..len21 {
                                        let base = base21.add(i * 8);
                                        {
                                            let l19 = *base.add(0).cast::<*mut u8>();
                                            let l20 = *base.add(4).cast::<usize>();
                                            _rt::cabi_dealloc(l19, l20, 1);
                                        }
                                    }
                                    _rt::cabi_dealloc(base21, len21 * 8, 4);
                                }
                            }
                            _rt::cabi_dealloc(base22, len22 * 68, 4);
                        }
                        _ => {}
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_manga_details_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::get_manga_details(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let super::super::super::super::exports::midoku::types::manga::Manga {
                                id: id3,
                                title: title3,
                                url: url3,
                                description: description3,
                                cover_url: cover_url3,
                                author_name: author_name3,
                                artist_name: artist_name3,
                                categories: categories3,
                                status: status3,
                                content_rating: content_rating3,
                                reading_mode: reading_mode3,
                            } = e;
                            let vec4 = (id3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr2.add(8).cast::<usize>() = len4;
                            *ptr2.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                            let vec5 = (title3.into_bytes()).into_boxed_slice();
                            let ptr5 = vec5.as_ptr().cast::<u8>();
                            let len5 = vec5.len();
                            ::core::mem::forget(vec5);
                            *ptr2.add(16).cast::<usize>() = len5;
                            *ptr2.add(12).cast::<*mut u8>() = ptr5.cast_mut();
                            let vec6 = (url3.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *ptr2.add(24).cast::<usize>() = len6;
                            *ptr2.add(20).cast::<*mut u8>() = ptr6.cast_mut();
                            let vec7 = (description3.into_bytes()).into_boxed_slice();
                            let ptr7 = vec7.as_ptr().cast::<u8>();
                            let len7 = vec7.len();
                            ::core::mem::forget(vec7);
                            *ptr2.add(32).cast::<usize>() = len7;
                            *ptr2.add(28).cast::<*mut u8>() = ptr7.cast_mut();
                            let vec8 = (cover_url3.into_bytes()).into_boxed_slice();
                            let ptr8 = vec8.as_ptr().cast::<u8>();
                            let len8 = vec8.len();
                            ::core::mem::forget(vec8);
                            *ptr2.add(40).cast::<usize>() = len8;
                            *ptr2.add(36).cast::<*mut u8>() = ptr8.cast_mut();
                            let vec9 = (author_name3.into_bytes()).into_boxed_slice();
                            let ptr9 = vec9.as_ptr().cast::<u8>();
                            let len9 = vec9.len();
                            ::core::mem::forget(vec9);
                            *ptr2.add(48).cast::<usize>() = len9;
                            *ptr2.add(44).cast::<*mut u8>() = ptr9.cast_mut();
                            let vec10 = (artist_name3.into_bytes()).into_boxed_slice();
                            let ptr10 = vec10.as_ptr().cast::<u8>();
                            let len10 = vec10.len();
                            ::core::mem::forget(vec10);
                            *ptr2.add(56).cast::<usize>() = len10;
                            *ptr2.add(52).cast::<*mut u8>() = ptr10.cast_mut();
                            let vec12 = categories3;
                            let len12 = vec12.len();
                            let layout12 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec12.len() * 8, 4);
                            let result12 = if layout12.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout12).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout12);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec12.into_iter().enumerate() {
                                let base = result12.add(i * 8);
                                {
                                    let vec11 = (e.into_bytes()).into_boxed_slice();
                                    let ptr11 = vec11.as_ptr().cast::<u8>();
                                    let len11 = vec11.len();
                                    ::core::mem::forget(vec11);
                                    *base.add(4).cast::<usize>() = len11;
                                    *base.add(0).cast::<*mut u8>() = ptr11.cast_mut();
                                }
                            }
                            *ptr2.add(64).cast::<usize>() = len12;
                            *ptr2.add(60).cast::<*mut u8>() = result12;
                            *ptr2.add(68).cast::<u8>() = (status3.clone() as i32) as u8;
                            *ptr2.add(69).cast::<u8>() = (content_rating3.clone() as i32) as u8;
                            *ptr2.add(70).cast::<u8>() = (reading_mode3.clone() as i32) as u8;
                        }
                        Err(_) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_manga_details<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                            let l3 = *arg0.add(12).cast::<*mut u8>();
                            let l4 = *arg0.add(16).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                            let l5 = *arg0.add(20).cast::<*mut u8>();
                            let l6 = *arg0.add(24).cast::<usize>();
                            _rt::cabi_dealloc(l5, l6, 1);
                            let l7 = *arg0.add(28).cast::<*mut u8>();
                            let l8 = *arg0.add(32).cast::<usize>();
                            _rt::cabi_dealloc(l7, l8, 1);
                            let l9 = *arg0.add(36).cast::<*mut u8>();
                            let l10 = *arg0.add(40).cast::<usize>();
                            _rt::cabi_dealloc(l9, l10, 1);
                            let l11 = *arg0.add(44).cast::<*mut u8>();
                            let l12 = *arg0.add(48).cast::<usize>();
                            _rt::cabi_dealloc(l11, l12, 1);
                            let l13 = *arg0.add(52).cast::<*mut u8>();
                            let l14 = *arg0.add(56).cast::<usize>();
                            _rt::cabi_dealloc(l13, l14, 1);
                            let l15 = *arg0.add(60).cast::<*mut u8>();
                            let l16 = *arg0.add(64).cast::<usize>();
                            let base19 = l15;
                            let len19 = l16;
                            for i in 0..len19 {
                                let base = base19.add(i * 8);
                                {
                                    let l17 = *base.add(0).cast::<*mut u8>();
                                    let l18 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l17, l18, 1);
                                }
                            }
                            _rt::cabi_dealloc(base19, len19 * 8, 4);
                        }
                        _ => {}
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_chapter_list_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::get_chapter_list(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Ok(e) => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                            let vec9 = e;
                            let len9 = vec9.len();
                            let layout9 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec9.len() * 52, 4);
                            let result9 = if layout9.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout9).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout9);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec9.into_iter().enumerate() {
                                let base = result9.add(i * 52);
                                {
                                    let super::super::super::super::exports::midoku::types::chapter::Chapter {
                                        id: id3,
                                        title: title3,
                                        volume: volume3,
                                        chapter: chapter3,
                                        date_updated: date_updated3,
                                        scanlator: scanlator3,
                                        url: url3,
                                        language: language3,
                                    } = e;
                                    let vec4 = (id3.into_bytes()).into_boxed_slice();
                                    let ptr4 = vec4.as_ptr().cast::<u8>();
                                    let len4 = vec4.len();
                                    ::core::mem::forget(vec4);
                                    *base.add(4).cast::<usize>() = len4;
                                    *base.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                                    let vec5 = (title3.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *base.add(12).cast::<usize>() = len5;
                                    *base.add(8).cast::<*mut u8>() = ptr5.cast_mut();
                                    *base.add(16).cast::<f32>() = _rt::as_f32(volume3);
                                    *base.add(20).cast::<f32>() = _rt::as_f32(chapter3);
                                    *base.add(24).cast::<i32>() = _rt::as_i32(date_updated3);
                                    let vec6 = (scanlator3.into_bytes()).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *base.add(32).cast::<usize>() = len6;
                                    *base.add(28).cast::<*mut u8>() = ptr6.cast_mut();
                                    let vec7 = (url3.into_bytes()).into_boxed_slice();
                                    let ptr7 = vec7.as_ptr().cast::<u8>();
                                    let len7 = vec7.len();
                                    ::core::mem::forget(vec7);
                                    *base.add(40).cast::<usize>() = len7;
                                    *base.add(36).cast::<*mut u8>() = ptr7.cast_mut();
                                    let vec8 = (language3.into_bytes()).into_boxed_slice();
                                    let ptr8 = vec8.as_ptr().cast::<u8>();
                                    let len8 = vec8.len();
                                    ::core::mem::forget(vec8);
                                    *base.add(48).cast::<usize>() = len8;
                                    *base.add(44).cast::<*mut u8>() = ptr8.cast_mut();
                                }
                            }
                            *ptr2.add(8).cast::<usize>() = len9;
                            *ptr2.add(4).cast::<*mut u8>() = result9;
                        }
                        Err(_) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_chapter_list<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base13 = l1;
                            let len13 = l2;
                            for i in 0..len13 {
                                let base = base13.add(i * 52);
                                {
                                    let l3 = *base.add(0).cast::<*mut u8>();
                                    let l4 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(8).cast::<*mut u8>();
                                    let l6 = *base.add(12).cast::<usize>();
                                    _rt::cabi_dealloc(l5, l6, 1);
                                    let l7 = *base.add(28).cast::<*mut u8>();
                                    let l8 = *base.add(32).cast::<usize>();
                                    _rt::cabi_dealloc(l7, l8, 1);
                                    let l9 = *base.add(36).cast::<*mut u8>();
                                    let l10 = *base.add(40).cast::<usize>();
                                    _rt::cabi_dealloc(l9, l10, 1);
                                    let l11 = *base.add(44).cast::<*mut u8>();
                                    let l12 = *base.add(48).cast::<usize>();
                                    _rt::cabi_dealloc(l11, l12, 1);
                                }
                            }
                            _rt::cabi_dealloc(base13, len13 * 52, 4);
                        }
                        _ => {}
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_page_list_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let result2 =
                        T::get_page_list(_rt::string_lift(bytes0), _rt::string_lift(bytes1));
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result2 {
                        Ok(e) => {
                            *ptr3.add(0).cast::<u8>() = (0i32) as u8;
                            let vec7 = e;
                            let len7 = vec7.len();
                            let layout7 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 20, 4);
                            let result7 = if layout7.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout7);
                                }
                                ptr
                            } else {
                                ::core::ptr::null_mut()
                            };
                            for (i, e) in vec7.into_iter().enumerate() {
                                let base = result7.add(i * 20);
                                {
                                    let super::super::super::super::exports::midoku::types::page::Page {
                                        index: index4,
                                        url: url4,
                                        base64: base644,
                                    } = e;
                                    *base.add(0).cast::<i32>() = _rt::as_i32(index4);
                                    let vec5 = (url4.into_bytes()).into_boxed_slice();
                                    let ptr5 = vec5.as_ptr().cast::<u8>();
                                    let len5 = vec5.len();
                                    ::core::mem::forget(vec5);
                                    *base.add(8).cast::<usize>() = len5;
                                    *base.add(4).cast::<*mut u8>() = ptr5.cast_mut();
                                    let vec6 = (base644).into_boxed_slice();
                                    let ptr6 = vec6.as_ptr().cast::<u8>();
                                    let len6 = vec6.len();
                                    ::core::mem::forget(vec6);
                                    *base.add(16).cast::<usize>() = len6;
                                    *base.add(12).cast::<*mut u8>() = ptr6.cast_mut();
                                }
                            }
                            *ptr3.add(8).cast::<usize>() = len7;
                            *ptr3.add(4).cast::<*mut u8>() = result7;
                        }
                        Err(_) => {
                            *ptr3.add(0).cast::<u8>() = (1i32) as u8;
                        }
                    };
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_page_list<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            let base8 = l1;
                            let len8 = l2;
                            for i in 0..len8 {
                                let base = base8.add(i * 20);
                                {
                                    let l3 = *base.add(4).cast::<*mut u8>();
                                    let l4 = *base.add(8).cast::<usize>();
                                    _rt::cabi_dealloc(l3, l4, 1);
                                    let l5 = *base.add(12).cast::<*mut u8>();
                                    let l6 = *base.add(16).cast::<usize>();
                                    let base7 = l5;
                                    let len7 = l6;
                                    _rt::cabi_dealloc(base7, len7 * 1, 1);
                                }
                            }
                            _rt::cabi_dealloc(base8, len8 * 20, 4);
                        }
                        _ => {}
                    }
                }
                pub trait Guest {
                    /// Initialize the extension.
                    ///
                    /// Sources may have initialization logic that needs to be called before
                    /// calling other functions. This may include setting up rate limiters or
                    /// other configuration.
                    fn initialize() -> Result<(), ()>;
                    /// Get a list of manga from the source.
                    ///
                    /// This function should return a list of manga that can be displayed to the
                    /// user. The `filters` parameter is used to filter the results based on
                    /// user input. The `page` parameter is used to paginate the results if
                    /// necessary.
                    fn get_manga_list(
                        filters: _rt::Vec<Filter>,
                        page: u32,
                    ) -> Result<(_rt::Vec<Manga>, bool), ()>;
                    /// Get details for a specific manga.
                    ///
                    /// This function should return detailed information about a specific manga.
                    fn get_manga_details(manga_id: _rt::String) -> Result<Manga, ()>;
                    /// Get a list of chapters for a specific manga.
                    ///
                    /// This function should return a list of chapters for a specific manga.
                    fn get_chapter_list(manga_id: _rt::String) -> Result<_rt::Vec<Chapter>, ()>;
                    /// Get a list of pages for a specific chapter.
                    ///
                    /// This function should return a list of pages for a specific chapter.
                    fn get_page_list(
                        manga_id: _rt::String,
                        chapter_id: _rt::String,
                    ) -> Result<_rt::Vec<Page>, ()>;
                }
                #[doc(hidden)]
                macro_rules! __export_midoku_bindings_api_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name =
                        "midoku:bindings/api@0.1.0#initialize"] unsafe extern "C" fn
                        export_initialize() -> i32 { $($path_to_types)*::
                        _export_initialize_cabi::<$ty > () } #[export_name =
                        "midoku:bindings/api@0.1.0#get-manga-list"] unsafe extern "C" fn
                        export_get_manga_list(arg0 : * mut u8, arg1 : usize, arg2 : i32,)
                        -> * mut u8 { $($path_to_types)*::
                        _export_get_manga_list_cabi::<$ty > (arg0, arg1, arg2) }
                        #[export_name =
                        "cabi_post_midoku:bindings/api@0.1.0#get-manga-list"] unsafe
                        extern "C" fn _post_return_get_manga_list(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_manga_list::<$ty > (arg0)
                        } #[export_name = "midoku:bindings/api@0.1.0#get-manga-details"]
                        unsafe extern "C" fn export_get_manga_details(arg0 : * mut u8,
                        arg1 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_get_manga_details_cabi::<$ty > (arg0, arg1) }
                        #[export_name =
                        "cabi_post_midoku:bindings/api@0.1.0#get-manga-details"] unsafe
                        extern "C" fn _post_return_get_manga_details(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_manga_details::<$ty >
                        (arg0) } #[export_name =
                        "midoku:bindings/api@0.1.0#get-chapter-list"] unsafe extern "C"
                        fn export_get_chapter_list(arg0 : * mut u8, arg1 : usize,) -> *
                        mut u8 { $($path_to_types)*:: _export_get_chapter_list_cabi::<$ty
                        > (arg0, arg1) } #[export_name =
                        "cabi_post_midoku:bindings/api@0.1.0#get-chapter-list"] unsafe
                        extern "C" fn _post_return_get_chapter_list(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_chapter_list::<$ty >
                        (arg0) } #[export_name =
                        "midoku:bindings/api@0.1.0#get-page-list"] unsafe extern "C" fn
                        export_get_page_list(arg0 : * mut u8, arg1 : usize, arg2 : * mut
                        u8, arg3 : usize,) -> * mut u8 { $($path_to_types)*::
                        _export_get_page_list_cabi::<$ty > (arg0, arg1, arg2, arg3) }
                        #[export_name =
                        "cabi_post_midoku:bindings/api@0.1.0#get-page-list"] unsafe
                        extern "C" fn _post_return_get_page_list(arg0 : * mut u8,) {
                        $($path_to_types)*:: __post_return_get_page_list::<$ty > (arg0) }
                        };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_midoku_bindings_api_0_1_0_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 72]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 72]);
            }
        }
        #[allow(dead_code)]
        pub mod types {
            #[allow(dead_code, clippy::all)]
            pub mod chapter {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Chapter {
                    pub id: _rt::String,
                    pub title: _rt::String,
                    pub volume: f32,
                    pub chapter: f32,
                    /// The date the chapter was last updated. This is a Unix timestamp in seconds.
                    pub date_updated: u32,
                    pub scanlator: _rt::String,
                    pub url: _rt::String,
                    pub language: _rt::String,
                }
                impl ::core::fmt::Debug for Chapter {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Chapter")
                            .field("id", &self.id)
                            .field("title", &self.title)
                            .field("volume", &self.volume)
                            .field("chapter", &self.chapter)
                            .field("date-updated", &self.date_updated)
                            .field("scanlator", &self.scanlator)
                            .field("url", &self.url)
                            .field("language", &self.language)
                            .finish()
                    }
                }
                #[doc(hidden)]
                macro_rules! __export_midoku_types_chapter_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_midoku_types_chapter_0_1_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod filter {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct FilterTitle {
                    pub query: _rt::String,
                }
                impl ::core::fmt::Debug for FilterTitle {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("FilterTitle")
                            .field("query", &self.query)
                            .finish()
                    }
                }
                #[repr(C)]
                #[derive(Clone, Copy)]
                pub struct FilterSort {
                    pub option_index: u32,
                    /// If true, the sort order is reversed.
                    ///
                    /// This is analogous to an ascending if true or descending if false sort
                    /// order.
                    pub option_reversed: bool,
                }
                impl ::core::fmt::Debug for FilterSort {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("FilterSort")
                            .field("option-index", &self.option_index)
                            .field("option-reversed", &self.option_reversed)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub enum Filter {
                    Title(FilterTitle),
                    Sort(FilterSort),
                }
                impl ::core::fmt::Debug for Filter {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Filter::Title(e) => f.debug_tuple("Filter::Title").field(e).finish(),
                            Filter::Sort(e) => f.debug_tuple("Filter::Sort").field(e).finish(),
                        }
                    }
                }
                #[doc(hidden)]
                macro_rules! __export_midoku_types_filter_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_midoku_types_filter_0_1_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod manga {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum Status {
                    Unknown,
                    Ongoing,
                    Completed,
                    Hiatus,
                    Cancelled,
                }
                impl ::core::fmt::Debug for Status {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            Status::Unknown => f.debug_tuple("Status::Unknown").finish(),
                            Status::Ongoing => f.debug_tuple("Status::Ongoing").finish(),
                            Status::Completed => f.debug_tuple("Status::Completed").finish(),
                            Status::Hiatus => f.debug_tuple("Status::Hiatus").finish(),
                            Status::Cancelled => f.debug_tuple("Status::Cancelled").finish(),
                        }
                    }
                }
                impl Status {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> Status {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => Status::Unknown,
                            1 => Status::Ongoing,
                            2 => Status::Completed,
                            3 => Status::Hiatus,
                            4 => Status::Cancelled,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum ContentRating {
                    Safe,
                    Suggestive,
                    Nsfw,
                }
                impl ::core::fmt::Debug for ContentRating {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            ContentRating::Safe => f.debug_tuple("ContentRating::Safe").finish(),
                            ContentRating::Suggestive => {
                                f.debug_tuple("ContentRating::Suggestive").finish()
                            }
                            ContentRating::Nsfw => f.debug_tuple("ContentRating::Nsfw").finish(),
                        }
                    }
                }
                impl ContentRating {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> ContentRating {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => ContentRating::Safe,
                            1 => ContentRating::Suggestive,
                            2 => ContentRating::Nsfw,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[repr(u8)]
                #[derive(Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
                pub enum ReadingMode {
                    RightToLeft,
                    LeftToRight,
                    Vertical,
                    Scroll,
                }
                impl ::core::fmt::Debug for ReadingMode {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            ReadingMode::RightToLeft => {
                                f.debug_tuple("ReadingMode::RightToLeft").finish()
                            }
                            ReadingMode::LeftToRight => {
                                f.debug_tuple("ReadingMode::LeftToRight").finish()
                            }
                            ReadingMode::Vertical => {
                                f.debug_tuple("ReadingMode::Vertical").finish()
                            }
                            ReadingMode::Scroll => f.debug_tuple("ReadingMode::Scroll").finish(),
                        }
                    }
                }
                impl ReadingMode {
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: u8) -> ReadingMode {
                        if !cfg!(debug_assertions) {
                            return ::core::mem::transmute(val);
                        }
                        match val {
                            0 => ReadingMode::RightToLeft,
                            1 => ReadingMode::LeftToRight,
                            2 => ReadingMode::Vertical,
                            3 => ReadingMode::Scroll,
                            _ => panic!("invalid enum discriminant"),
                        }
                    }
                }
                #[derive(Clone)]
                pub struct Manga {
                    pub id: _rt::String,
                    pub title: _rt::String,
                    pub url: _rt::String,
                    pub description: _rt::String,
                    pub cover_url: _rt::String,
                    pub author_name: _rt::String,
                    pub artist_name: _rt::String,
                    pub categories: _rt::Vec<_rt::String>,
                    pub status: Status,
                    pub content_rating: ContentRating,
                    pub reading_mode: ReadingMode,
                }
                impl ::core::fmt::Debug for Manga {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Manga")
                            .field("id", &self.id)
                            .field("title", &self.title)
                            .field("url", &self.url)
                            .field("description", &self.description)
                            .field("cover-url", &self.cover_url)
                            .field("author-name", &self.author_name)
                            .field("artist-name", &self.artist_name)
                            .field("categories", &self.categories)
                            .field("status", &self.status)
                            .field("content-rating", &self.content_rating)
                            .field("reading-mode", &self.reading_mode)
                            .finish()
                    }
                }
                #[doc(hidden)]
                macro_rules! __export_midoku_types_manga_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_midoku_types_manga_0_1_0_cabi;
            }
            #[allow(dead_code, clippy::all)]
            pub mod page {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Page {
                    pub index: u32,
                    pub url: _rt::String,
                    /// The base64-encoded data of the page.
                    pub base64: _rt::Vec<u8>,
                }
                impl ::core::fmt::Debug for Page {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Page")
                            .field("index", &self.index)
                            .field("url", &self.url)
                            .field("base64", &self.base64)
                            .finish()
                    }
                }
                #[doc(hidden)]
                macro_rules! __export_midoku_types_page_0_1_0_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _: () = {};
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_midoku_types_page_0_1_0_cabi;
            }
        }
    }
}
mod _rt {
    use core::fmt;
    use core::marker;
    use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
    /// A type which represents a component model resource, either imported or
    /// exported into this component.
    ///
    /// This is a low-level wrapper which handles the lifetime of the resource
    /// (namely this has a destructor). The `T` provided defines the component model
    /// intrinsics that this wrapper uses.
    ///
    /// One of the chief purposes of this type is to provide `Deref` implementations
    /// to access the underlying data when it is owned.
    ///
    /// This type is primarily used in generated code for exported and imported
    /// resources.
    #[repr(transparent)]
    pub struct Resource<T: WasmResource> {
        handle: AtomicU32,
        _marker: marker::PhantomData<T>,
    }
    /// A trait which all wasm resources implement, namely providing the ability to
    /// drop a resource.
    ///
    /// This generally is implemented by generated code, not user-facing code.
    #[allow(clippy::missing_safety_doc)]
    pub unsafe trait WasmResource {
        /// Invokes the `[resource-drop]...` intrinsic.
        unsafe fn drop(handle: u32);
    }
    impl<T: WasmResource> Resource<T> {
        #[doc(hidden)]
        pub unsafe fn from_handle(handle: u32) -> Self {
            debug_assert!(handle != u32::MAX);
            Self {
                handle: AtomicU32::new(handle),
                _marker: marker::PhantomData,
            }
        }
        /// Takes ownership of the handle owned by `resource`.
        ///
        /// Note that this ideally would be `into_handle` taking `Resource<T>` by
        /// ownership. The code generator does not enable that in all situations,
        /// unfortunately, so this is provided instead.
        ///
        /// Also note that `take_handle` is in theory only ever called on values
        /// owned by a generated function. For example a generated function might
        /// take `Resource<T>` as an argument but then call `take_handle` on a
        /// reference to that argument. In that sense the dynamic nature of
        /// `take_handle` should only be exposed internally to generated code, not
        /// to user code.
        #[doc(hidden)]
        pub fn take_handle(resource: &Resource<T>) -> u32 {
            resource.handle.swap(u32::MAX, Relaxed)
        }
        #[doc(hidden)]
        pub fn handle(resource: &Resource<T>) -> u32 {
            resource.handle.load(Relaxed)
        }
    }
    impl<T: WasmResource> fmt::Debug for Resource<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Resource")
                .field("handle", &self.handle)
                .finish()
        }
    }
    impl<T: WasmResource> Drop for Resource<T> {
        fn drop(&mut self) {
            unsafe {
                match self.handle.load(Relaxed) {
                    u32::MAX => {}
                    other => T::drop(other),
                }
            }
        }
    }
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    pub use alloc_crate::alloc;
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub fn as_i32<T: AsI32>(t: T) -> i32 {
        t.as_i32()
    }
    pub trait AsI32 {
        fn as_i32(self) -> i32;
    }
    impl<'a, T: Copy + AsI32> AsI32 for &'a T {
        fn as_i32(self) -> i32 {
            (*self).as_i32()
        }
    }
    impl AsI32 for i32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u32 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u16 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for i8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for u8 {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for char {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    impl AsI32 for usize {
        #[inline]
        fn as_i32(self) -> i32 {
            self as i32
        }
    }
    pub unsafe fn bool_lift(val: u8) -> bool {
        if cfg!(debug_assertions) {
            match val {
                0 => false,
                1 => true,
                _ => panic!("invalid bool discriminant"),
            }
        } else {
            val != 0
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub fn as_f32<T: AsF32>(t: T) -> f32 {
        t.as_f32()
    }
    pub trait AsF32 {
        fn as_f32(self) -> f32;
    }
    impl<'a, T: Copy + AsF32> AsF32 for &'a T {
        fn as_f32(self) -> f32 {
            (*self).as_f32()
        }
    }
    impl AsF32 for f32 {
        #[inline]
        fn as_f32(self) -> f32 {
            self as f32
        }
    }
    extern crate alloc as alloc_crate;
}
/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
macro_rules! __export_endpoints_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::midoku::types::chapter::__export_midoku_types_chapter_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::midoku::types::chapter);
        $($path_to_types_root)*::
        exports::midoku::types::filter::__export_midoku_types_filter_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::midoku::types::filter);
        $($path_to_types_root)*::
        exports::midoku::types::manga::__export_midoku_types_manga_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::midoku::types::manga);
        $($path_to_types_root)*::
        exports::midoku::types::page::__export_midoku_types_page_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::midoku::types::page);
        $($path_to_types_root)*::
        exports::midoku::bindings::api::__export_midoku_bindings_api_0_1_0_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::midoku::bindings::api);
    };
}
#[doc(inline)]
pub(crate) use __export_endpoints_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:midoku:example-extension@0.1.0:endpoints:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 1977] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb9\x0e\x01A\x02\x01\
A\x18\x01B\x0d\x01m\x05\x03get\x04post\x03put\x04head\x06delete\x04\0\x06method\x03\
\0\0\x04\0\x11incoming-response\x03\x01\x01h\x02\x01@\x01\x04self\x03\0{\x04\0%[\
method]incoming-response.status-code\x01\x04\x01o\x02ss\x01p\x05\x01@\x01\x04sel\
f\x03\0\x06\x04\0![method]incoming-response.headers\x01\x07\x01p}\x01@\x01\x04se\
lf\x03\0\x08\x04\0\x1f[method]incoming-response.bytes\x01\x09\x03\x01\x17midoku:\
http/types@0.1.0\x05\0\x02\x03\0\0\x06method\x02\x03\0\0\x11incoming-response\x01\
B\x0d\x02\x03\x02\x01\x01\x04\0\x06method\x03\0\0\x02\x03\x02\x01\x02\x04\0\x11i\
ncoming-response\x03\0\x02\x01o\x02ss\x01p\x04\x01k\x05\x01p}\x01k\x07\x01i\x03\x01\
j\x01\x09\0\x01@\x04\x06method\x01\x03urls\x07headers\x06\x04body\x08\0\x0a\x04\0\
\x06handle\x01\x0b\x03\x01\"midoku:http/outgoing-handler@0.1.0\x05\x03\x01B\x0d\x01\
ky\x01@\0\0\0\x04\0\x05burst\x01\x01\x04\0\x09period-ms\x01\x01\x01j\0\0\x01@\x01\
\x05bursty\0\x02\x04\0\x09set-burst\x01\x03\x01@\x01\x09period-msy\0\x02\x04\0\x0d\
set-period-ms\x01\x04\x01@\0\0\x7f\x04\0\x05ready\x01\x05\x01@\0\x01\0\x04\0\x05\
block\x01\x06\x03\x01!midoku:limiter/rate-limiter@0.1.0\x05\x04\x01B\x0a\x01q\x03\
\x03s64\x01x\0\x03u64\x01w\0\x03f64\x01u\0\x04\0\x06number\x03\0\0\x01ps\x01o\x02\
ss\x01p\x03\x01q\x05\x04bool\x01\x7f\0\x06number\x01\x01\0\x06string\x01s\0\x05a\
rray\x01\x02\0\x03map\x01\x04\0\x04\0\x05value\x03\0\x05\x01j\x01\x06\0\x01@\x01\
\x03keys\0\x07\x04\0\x03get\x01\x08\x03\x01\x1emidoku:settings/settings@0.1.0\x05\
\x05\x01B\x02\x01r\x08\x02ids\x05titles\x06volumev\x07chapterv\x0cdate-updatedy\x09\
scanlators\x03urls\x08languages\x04\0\x07chapter\x03\0\0\x04\x01\x1amidoku:types\
/chapter@0.1.0\x05\x06\x01B\x06\x01r\x01\x05querys\x04\0\x0cfilter-title\x03\0\0\
\x01r\x02\x0coption-indexy\x0foption-reversed\x7f\x04\0\x0bfilter-sort\x03\0\x02\
\x01q\x02\x05title\x01\x01\0\x04sort\x01\x03\0\x04\0\x06filter\x03\0\x04\x04\x01\
\x19midoku:types/filter@0.1.0\x05\x07\x01B\x09\x01m\x05\x07unknown\x07ongoing\x09\
completed\x06hiatus\x09cancelled\x04\0\x06status\x03\0\0\x01m\x03\x04safe\x0asug\
gestive\x04nsfw\x04\0\x0econtent-rating\x03\0\x02\x01m\x04\x0dright-to-left\x0dl\
eft-to-right\x08vertical\x06scroll\x04\0\x0creading-mode\x03\0\x04\x01ps\x01r\x0b\
\x02ids\x05titles\x03urls\x0bdescriptions\x09cover-urls\x0bauthor-names\x0bartis\
t-names\x0acategories\x06\x06status\x01\x0econtent-rating\x03\x0creading-mode\x05\
\x04\0\x05manga\x03\0\x07\x04\x01\x18midoku:types/manga@0.1.0\x05\x08\x01B\x03\x01\
p}\x01r\x03\x05indexy\x03urls\x06base64\0\x04\0\x04page\x03\0\x01\x04\x01\x17mid\
oku:types/page@0.1.0\x05\x09\x02\x03\0\x04\x07chapter\x02\x03\0\x05\x06filter\x02\
\x03\0\x06\x05manga\x02\x03\0\x07\x04page\x01B\x1c\x02\x03\x02\x01\x0a\x04\0\x07\
chapter\x03\0\0\x02\x03\x02\x01\x0b\x04\0\x06filter\x03\0\x02\x02\x03\x02\x01\x0c\
\x04\0\x05manga\x03\0\x04\x02\x03\x02\x01\x0d\x04\0\x04page\x03\0\x06\x01j\0\0\x01\
@\0\0\x08\x04\0\x0ainitialize\x01\x09\x01p\x03\x01p\x05\x01o\x02\x0b\x7f\x01j\x01\
\x0c\0\x01@\x02\x07filters\x0a\x04pagey\0\x0d\x04\0\x0eget-manga-list\x01\x0e\x01\
j\x01\x05\0\x01@\x01\x08manga-ids\0\x0f\x04\0\x11get-manga-details\x01\x10\x01p\x01\
\x01j\x01\x11\0\x01@\x01\x08manga-ids\0\x12\x04\0\x10get-chapter-list\x01\x13\x01\
p\x07\x01j\x01\x14\0\x01@\x02\x08manga-ids\x0achapter-ids\0\x15\x04\0\x0dget-pag\
e-list\x01\x16\x04\x01\x19midoku:bindings/api@0.1.0\x05\x0e\x04\x01(midoku:examp\
le-extension/endpoints@0.1.0\x04\0\x0b\x0f\x01\0\x09endpoints\x03\0\0\0G\x09prod\
ucers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rust\x06\
0.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
