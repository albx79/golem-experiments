// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod golem {
        #[allow(dead_code)]
        pub mod component {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct ProductItem {
                    pub product_id: _rt::String,
                    pub name: _rt::String,
                    pub price: f32,
                    pub quantity: u32,
                }
                impl ::core::fmt::Debug for ProductItem {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("ProductItem")
                            .field("product-id", &self.product_id)
                            .field("name", &self.name)
                            .field("price", &self.price)
                            .field("quantity", &self.quantity)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct OrderConfirmation {
                    pub order_id: _rt::String,
                }
                impl ::core::fmt::Debug for OrderConfirmation {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("OrderConfirmation")
                            .field("order-id", &self.order_id)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub enum CheckoutResult {
                    Error(_rt::String),
                    Success(OrderConfirmation),
                }
                impl ::core::fmt::Debug for CheckoutResult {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        match self {
                            CheckoutResult::Error(e) => {
                                f.debug_tuple("CheckoutResult::Error").field(e).finish()
                            }
                            CheckoutResult::Success(e) => {
                                f.debug_tuple("CheckoutResult::Success").field(e).finish()
                            }
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_initialize_cart_cabi<T: Guest>(arg0: *mut u8, arg1: usize) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    T::initialize_cart(_rt::string_lift(bytes0));
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_add_item_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: f32,
                    arg5: i32,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    T::add_item(ProductItem {
                        product_id: _rt::string_lift(bytes0),
                        name: _rt::string_lift(bytes1),
                        price: arg4,
                        quantity: arg5 as u32,
                    });
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_remove_item_cabi<T: Guest>(arg0: *mut u8, arg1: usize) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    T::remove_item(_rt::string_lift(bytes0));
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_update_item_quantity_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    T::update_item_quantity(_rt::string_lift(bytes0), arg2 as u32);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_checkout_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::checkout();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result0 {
                        CheckoutResult::Error(e) => {
                            *ptr1.add(0).cast::<u8>() = (0i32) as u8;
                            let vec2 = (e.into_bytes()).into_boxed_slice();
                            let ptr2 = vec2.as_ptr().cast::<u8>();
                            let len2 = vec2.len();
                            ::core::mem::forget(vec2);
                            *ptr1.add(8).cast::<usize>() = len2;
                            *ptr1.add(4).cast::<*mut u8>() = ptr2.cast_mut();
                        }
                        CheckoutResult::Success(e) => {
                            *ptr1.add(0).cast::<u8>() = (1i32) as u8;
                            let OrderConfirmation {
                                order_id: order_id3,
                            } = e;
                            let vec4 = (order_id3.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *ptr1.add(8).cast::<usize>() = len4;
                            *ptr1.add(4).cast::<*mut u8>() = ptr4.cast_mut();
                        }
                    }
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_checkout<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                        _ => {
                            let l3 = *arg0.add(4).cast::<*mut u8>();
                            let l4 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l3, l4, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_cart_contents_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::get_cart_contents();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec5 = result0;
                    let len5 = vec5.len();
                    let layout5 = _rt::alloc::Layout::from_size_align_unchecked(vec5.len() * 24, 4);
                    let result5 = if layout5.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout5).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout5);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec5.into_iter().enumerate() {
                        let base = result5.add(i * 24);
                        {
                            let ProductItem {
                                product_id: product_id2,
                                name: name2,
                                price: price2,
                                quantity: quantity2,
                            } = e;
                            let vec3 = (product_id2.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *base.add(4).cast::<usize>() = len3;
                            *base.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                            let vec4 = (name2.into_bytes()).into_boxed_slice();
                            let ptr4 = vec4.as_ptr().cast::<u8>();
                            let len4 = vec4.len();
                            ::core::mem::forget(vec4);
                            *base.add(12).cast::<usize>() = len4;
                            *base.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                            *base.add(16).cast::<f32>() = _rt::as_f32(price2);
                            *base.add(20).cast::<i32>() = _rt::as_i32(quantity2);
                        }
                    }
                    *ptr1.add(4).cast::<usize>() = len5;
                    *ptr1.add(0).cast::<*mut u8>() = result5;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_cart_contents<T: Guest>(arg0: *mut u8) {
                    let l4 = *arg0.add(0).cast::<*mut u8>();
                    let l5 = *arg0.add(4).cast::<usize>();
                    let base6 = l4;
                    let len6 = l5;
                    for i in 0..len6 {
                        let base = base6.add(i * 24);
                        {
                            let l0 = *base.add(0).cast::<*mut u8>();
                            let l1 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l0, l1, 1);
                            let l2 = *base.add(8).cast::<*mut u8>();
                            let l3 = *base.add(12).cast::<usize>();
                            _rt::cabi_dealloc(l2, l3, 1);
                        }
                    }
                    _rt::cabi_dealloc(base6, len6 * 24, 4);
                }
                pub trait Guest {
                    fn initialize_cart(user_id: _rt::String);
                    fn add_item(item: ProductItem);
                    fn remove_item(product_id: _rt::String);
                    fn update_item_quantity(product_id: _rt::String, quantity: u32);
                    fn checkout() -> CheckoutResult;
                    fn get_cart_contents() -> _rt::Vec<ProductItem>;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_component_api_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "golem:component/api#initialize-cart"]
    unsafe extern "C" fn export_initialize_cart(arg0: *mut u8,arg1: usize,) {
      $($path_to_types)*::_export_initialize_cart_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "golem:component/api#add-item"]
    unsafe extern "C" fn export_add_item(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,arg4: f32,arg5: i32,) {
      $($path_to_types)*::_export_add_item_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4, arg5)
    }
    #[export_name = "golem:component/api#remove-item"]
    unsafe extern "C" fn export_remove_item(arg0: *mut u8,arg1: usize,) {
      $($path_to_types)*::_export_remove_item_cabi::<$ty>(arg0, arg1)
    }
    #[export_name = "golem:component/api#update-item-quantity"]
    unsafe extern "C" fn export_update_item_quantity(arg0: *mut u8,arg1: usize,arg2: i32,) {
      $($path_to_types)*::_export_update_item_quantity_cabi::<$ty>(arg0, arg1, arg2)
    }
    #[export_name = "golem:component/api#checkout"]
    unsafe extern "C" fn export_checkout() -> *mut u8 {
      $($path_to_types)*::_export_checkout_cabi::<$ty>()
    }
    #[export_name = "cabi_post_golem:component/api#checkout"]
    unsafe extern "C" fn _post_return_checkout(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_checkout::<$ty>(arg0)
    }
    #[export_name = "golem:component/api#get-cart-contents"]
    unsafe extern "C" fn export_get_cart_contents() -> *mut u8 {
      $($path_to_types)*::_export_get_cart_contents_cabi::<$ty>()
    }
    #[export_name = "cabi_post_golem:component/api#get-cart-contents"]
    unsafe extern "C" fn _post_return_get_cart_contents(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_cart_contents::<$ty>(arg0)
    }
  };);
}
                #[doc(hidden)]
                pub(crate) use __export_golem_component_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
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
        alloc::dealloc(ptr as *mut u8, layout);
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
    pub use alloc_crate::alloc;
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

macro_rules! __export_shopping_cart_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::golem::component::api::__export_golem_component_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::component::api);
  )
}
#[doc(inline)]
pub(crate) use __export_shopping_cart_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:shopping-cart:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 576] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xbc\x03\x01A\x02\x01\
A\x02\x01B\x15\x01r\x04\x0aproduct-ids\x04names\x05pricev\x08quantityy\x04\0\x0c\
product-item\x03\0\0\x01p\x01\x01r\x04\x08order-ids\x05items\x02\x05totalv\x09ti\
mestampw\x04\0\x05order\x03\0\x03\x01r\x01\x08order-ids\x04\0\x12order-confirmat\
ion\x03\0\x05\x01q\x02\x05error\x01s\0\x07success\x01\x06\0\x04\0\x0fcheckout-re\
sult\x03\0\x07\x01@\x01\x07user-ids\x01\0\x04\0\x0finitialize-cart\x01\x09\x01@\x01\
\x04item\x01\x01\0\x04\0\x08add-item\x01\x0a\x01@\x01\x0aproduct-ids\x01\0\x04\0\
\x0bremove-item\x01\x0b\x01@\x02\x0aproduct-ids\x08quantityy\x01\0\x04\0\x14upda\
te-item-quantity\x01\x0c\x01@\0\0\x08\x04\0\x08checkout\x01\x0d\x01@\0\0\x02\x04\
\0\x11get-cart-contents\x01\x0e\x04\x01\x13golem:component/api\x05\0\x04\x01\x1d\
golem:component/shopping-cart\x04\0\x0b\x13\x01\0\x0dshopping-cart\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rus\
t\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
