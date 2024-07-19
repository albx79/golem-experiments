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
                pub struct SkillRating {
                    pub rating: u32,
                    pub skills: _rt::Vec<_rt::String>,
                }
                impl ::core::fmt::Debug for SkillRating {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("SkillRating")
                            .field("rating", &self.rating)
                            .field("skills", &self.skills)
                            .finish()
                    }
                }
                #[derive(Clone)]
                pub struct Character {
                    pub name: _rt::String,
                    pub high_concept: _rt::String,
                    pub trouble: _rt::String,
                    pub aspects: _rt::Vec<_rt::String>,
                    pub skills: _rt::Vec<SkillRating>,
                    pub stunts: _rt::Vec<_rt::String>,
                    pub skill_list: _rt::Vec<_rt::String>,
                }
                impl ::core::fmt::Debug for Character {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Character")
                            .field("name", &self.name)
                            .field("high-concept", &self.high_concept)
                            .field("trouble", &self.trouble)
                            .field("aspects", &self.aspects)
                            .field("skills", &self.skills)
                            .field("stunts", &self.stunts)
                            .field("skill-list", &self.skill_list)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_get_character_cabi<T: Guest>() -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::get_character();
                    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let Character {
                        name: name2,
                        high_concept: high_concept2,
                        trouble: trouble2,
                        aspects: aspects2,
                        skills: skills2,
                        stunts: stunts2,
                        skill_list: skill_list2,
                    } = result0;
                    let vec3 = (name2.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr1.add(4).cast::<usize>() = len3;
                    *ptr1.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    let vec4 = (high_concept2.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr1.add(12).cast::<usize>() = len4;
                    *ptr1.add(8).cast::<*mut u8>() = ptr4.cast_mut();
                    let vec5 = (trouble2.into_bytes()).into_boxed_slice();
                    let ptr5 = vec5.as_ptr().cast::<u8>();
                    let len5 = vec5.len();
                    ::core::mem::forget(vec5);
                    *ptr1.add(20).cast::<usize>() = len5;
                    *ptr1.add(16).cast::<*mut u8>() = ptr5.cast_mut();
                    let vec7 = aspects2;
                    let len7 = vec7.len();
                    let layout7 = _rt::alloc::Layout::from_size_align_unchecked(vec7.len() * 8, 4);
                    let result7 = if layout7.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout7).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout7);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec7.into_iter().enumerate() {
                        let base = result7.add(i * 8);
                        {
                            let vec6 = (e.into_bytes()).into_boxed_slice();
                            let ptr6 = vec6.as_ptr().cast::<u8>();
                            let len6 = vec6.len();
                            ::core::mem::forget(vec6);
                            *base.add(4).cast::<usize>() = len6;
                            *base.add(0).cast::<*mut u8>() = ptr6.cast_mut();
                        }
                    }
                    *ptr1.add(28).cast::<usize>() = len7;
                    *ptr1.add(24).cast::<*mut u8>() = result7;
                    let vec11 = skills2;
                    let len11 = vec11.len();
                    let layout11 =
                        _rt::alloc::Layout::from_size_align_unchecked(vec11.len() * 12, 4);
                    let result11 = if layout11.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout11).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout11);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec11.into_iter().enumerate() {
                        let base = result11.add(i * 12);
                        {
                            let SkillRating {
                                rating: rating8,
                                skills: skills8,
                            } = e;
                            *base.add(0).cast::<i32>() = _rt::as_i32(rating8);
                            let vec10 = skills8;
                            let len10 = vec10.len();
                            let layout10 =
                                _rt::alloc::Layout::from_size_align_unchecked(vec10.len() * 8, 4);
                            let result10 = if layout10.size() != 0 {
                                let ptr = _rt::alloc::alloc(layout10).cast::<u8>();
                                if ptr.is_null() {
                                    _rt::alloc::handle_alloc_error(layout10);
                                }
                                ptr
                            } else {
                                {
                                    ::core::ptr::null_mut()
                                }
                            };
                            for (i, e) in vec10.into_iter().enumerate() {
                                let base = result10.add(i * 8);
                                {
                                    let vec9 = (e.into_bytes()).into_boxed_slice();
                                    let ptr9 = vec9.as_ptr().cast::<u8>();
                                    let len9 = vec9.len();
                                    ::core::mem::forget(vec9);
                                    *base.add(4).cast::<usize>() = len9;
                                    *base.add(0).cast::<*mut u8>() = ptr9.cast_mut();
                                }
                            }
                            *base.add(8).cast::<usize>() = len10;
                            *base.add(4).cast::<*mut u8>() = result10;
                        }
                    }
                    *ptr1.add(36).cast::<usize>() = len11;
                    *ptr1.add(32).cast::<*mut u8>() = result11;
                    let vec13 = stunts2;
                    let len13 = vec13.len();
                    let layout13 =
                        _rt::alloc::Layout::from_size_align_unchecked(vec13.len() * 8, 4);
                    let result13 = if layout13.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout13).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout13);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec13.into_iter().enumerate() {
                        let base = result13.add(i * 8);
                        {
                            let vec12 = (e.into_bytes()).into_boxed_slice();
                            let ptr12 = vec12.as_ptr().cast::<u8>();
                            let len12 = vec12.len();
                            ::core::mem::forget(vec12);
                            *base.add(4).cast::<usize>() = len12;
                            *base.add(0).cast::<*mut u8>() = ptr12.cast_mut();
                        }
                    }
                    *ptr1.add(44).cast::<usize>() = len13;
                    *ptr1.add(40).cast::<*mut u8>() = result13;
                    let vec15 = skill_list2;
                    let len15 = vec15.len();
                    let layout15 =
                        _rt::alloc::Layout::from_size_align_unchecked(vec15.len() * 8, 4);
                    let result15 = if layout15.size() != 0 {
                        let ptr = _rt::alloc::alloc(layout15).cast::<u8>();
                        if ptr.is_null() {
                            _rt::alloc::handle_alloc_error(layout15);
                        }
                        ptr
                    } else {
                        {
                            ::core::ptr::null_mut()
                        }
                    };
                    for (i, e) in vec15.into_iter().enumerate() {
                        let base = result15.add(i * 8);
                        {
                            let vec14 = (e.into_bytes()).into_boxed_slice();
                            let ptr14 = vec14.as_ptr().cast::<u8>();
                            let len14 = vec14.len();
                            ::core::mem::forget(vec14);
                            *base.add(4).cast::<usize>() = len14;
                            *base.add(0).cast::<*mut u8>() = ptr14.cast_mut();
                        }
                    }
                    *ptr1.add(52).cast::<usize>() = len15;
                    *ptr1.add(48).cast::<*mut u8>() = result15;
                    ptr1
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_get_character<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                    let l2 = *arg0.add(8).cast::<*mut u8>();
                    let l3 = *arg0.add(12).cast::<usize>();
                    _rt::cabi_dealloc(l2, l3, 1);
                    let l4 = *arg0.add(16).cast::<*mut u8>();
                    let l5 = *arg0.add(20).cast::<usize>();
                    _rt::cabi_dealloc(l4, l5, 1);
                    let l8 = *arg0.add(24).cast::<*mut u8>();
                    let l9 = *arg0.add(28).cast::<usize>();
                    let base10 = l8;
                    let len10 = l9;
                    for i in 0..len10 {
                        let base = base10.add(i * 8);
                        {
                            let l6 = *base.add(0).cast::<*mut u8>();
                            let l7 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l6, l7, 1);
                        }
                    }
                    _rt::cabi_dealloc(base10, len10 * 8, 4);
                    let l16 = *arg0.add(32).cast::<*mut u8>();
                    let l17 = *arg0.add(36).cast::<usize>();
                    let base18 = l16;
                    let len18 = l17;
                    for i in 0..len18 {
                        let base = base18.add(i * 12);
                        {
                            let l13 = *base.add(4).cast::<*mut u8>();
                            let l14 = *base.add(8).cast::<usize>();
                            let base15 = l13;
                            let len15 = l14;
                            for i in 0..len15 {
                                let base = base15.add(i * 8);
                                {
                                    let l11 = *base.add(0).cast::<*mut u8>();
                                    let l12 = *base.add(4).cast::<usize>();
                                    _rt::cabi_dealloc(l11, l12, 1);
                                }
                            }
                            _rt::cabi_dealloc(base15, len15 * 8, 4);
                        }
                    }
                    _rt::cabi_dealloc(base18, len18 * 12, 4);
                    let l21 = *arg0.add(40).cast::<*mut u8>();
                    let l22 = *arg0.add(44).cast::<usize>();
                    let base23 = l21;
                    let len23 = l22;
                    for i in 0..len23 {
                        let base = base23.add(i * 8);
                        {
                            let l19 = *base.add(0).cast::<*mut u8>();
                            let l20 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l19, l20, 1);
                        }
                    }
                    _rt::cabi_dealloc(base23, len23 * 8, 4);
                    let l26 = *arg0.add(48).cast::<*mut u8>();
                    let l27 = *arg0.add(52).cast::<usize>();
                    let base28 = l26;
                    let len28 = l27;
                    for i in 0..len28 {
                        let base = base28.add(i * 8);
                        {
                            let l24 = *base.add(0).cast::<*mut u8>();
                            let l25 = *base.add(4).cast::<usize>();
                            _rt::cabi_dealloc(l24, l25, 1);
                        }
                    }
                    _rt::cabi_dealloc(base28, len28 * 8, 4);
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_update_character_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                    arg6: *mut u8,
                    arg7: usize,
                    arg8: *mut u8,
                    arg9: usize,
                    arg10: *mut u8,
                    arg11: usize,
                    arg12: *mut u8,
                    arg13: usize,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let len2 = arg5;
                    let bytes2 = _rt::Vec::from_raw_parts(arg4.cast(), len2, len2);
                    let base6 = arg6;
                    let len6 = arg7;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);

                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    let base14 = arg8;
                    let len14 = arg9;
                    let mut result14 = _rt::Vec::with_capacity(len14);
                    for i in 0..len14 {
                        let base = base14.add(i * 12);
                        let e14 = {
                            let l7 = *base.add(0).cast::<i32>();
                            let l8 = *base.add(4).cast::<*mut u8>();
                            let l9 = *base.add(8).cast::<usize>();
                            let base13 = l8;
                            let len13 = l9;
                            let mut result13 = _rt::Vec::with_capacity(len13);
                            for i in 0..len13 {
                                let base = base13.add(i * 8);
                                let e13 = {
                                    let l10 = *base.add(0).cast::<*mut u8>();
                                    let l11 = *base.add(4).cast::<usize>();
                                    let len12 = l11;
                                    let bytes12 =
                                        _rt::Vec::from_raw_parts(l10.cast(), len12, len12);

                                    _rt::string_lift(bytes12)
                                };
                                result13.push(e13);
                            }
                            _rt::cabi_dealloc(base13, len13 * 8, 4);

                            SkillRating {
                                rating: l7 as u32,
                                skills: result13,
                            }
                        };
                        result14.push(e14);
                    }
                    _rt::cabi_dealloc(base14, len14 * 12, 4);
                    let base18 = arg10;
                    let len18 = arg11;
                    let mut result18 = _rt::Vec::with_capacity(len18);
                    for i in 0..len18 {
                        let base = base18.add(i * 8);
                        let e18 = {
                            let l15 = *base.add(0).cast::<*mut u8>();
                            let l16 = *base.add(4).cast::<usize>();
                            let len17 = l16;
                            let bytes17 = _rt::Vec::from_raw_parts(l15.cast(), len17, len17);

                            _rt::string_lift(bytes17)
                        };
                        result18.push(e18);
                    }
                    _rt::cabi_dealloc(base18, len18 * 8, 4);
                    let base22 = arg12;
                    let len22 = arg13;
                    let mut result22 = _rt::Vec::with_capacity(len22);
                    for i in 0..len22 {
                        let base = base22.add(i * 8);
                        let e22 = {
                            let l19 = *base.add(0).cast::<*mut u8>();
                            let l20 = *base.add(4).cast::<usize>();
                            let len21 = l20;
                            let bytes21 = _rt::Vec::from_raw_parts(l19.cast(), len21, len21);

                            _rt::string_lift(bytes21)
                        };
                        result22.push(e22);
                    }
                    _rt::cabi_dealloc(base22, len22 * 8, 4);
                    T::update_character(Character {
                        name: _rt::string_lift(bytes0),
                        high_concept: _rt::string_lift(bytes1),
                        trouble: _rt::string_lift(bytes2),
                        aspects: result6,
                        skills: result14,
                        stunts: result18,
                        skill_list: result22,
                    });
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_toggle_editable_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result0 = T::toggle_editable();
                    match result0 {
                        true => 1,
                        false => 0,
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_submit_character_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: *mut u8,
                    arg3: usize,
                    arg4: *mut u8,
                    arg5: usize,
                    arg6: *mut u8,
                    arg7: usize,
                    arg8: *mut u8,
                    arg9: usize,
                    arg10: *mut u8,
                    arg11: usize,
                    arg12: *mut u8,
                    arg13: usize,
                ) {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let len1 = arg3;
                    let bytes1 = _rt::Vec::from_raw_parts(arg2.cast(), len1, len1);
                    let len2 = arg5;
                    let bytes2 = _rt::Vec::from_raw_parts(arg4.cast(), len2, len2);
                    let base6 = arg6;
                    let len6 = arg7;
                    let mut result6 = _rt::Vec::with_capacity(len6);
                    for i in 0..len6 {
                        let base = base6.add(i * 8);
                        let e6 = {
                            let l3 = *base.add(0).cast::<*mut u8>();
                            let l4 = *base.add(4).cast::<usize>();
                            let len5 = l4;
                            let bytes5 = _rt::Vec::from_raw_parts(l3.cast(), len5, len5);

                            _rt::string_lift(bytes5)
                        };
                        result6.push(e6);
                    }
                    _rt::cabi_dealloc(base6, len6 * 8, 4);
                    let base14 = arg8;
                    let len14 = arg9;
                    let mut result14 = _rt::Vec::with_capacity(len14);
                    for i in 0..len14 {
                        let base = base14.add(i * 12);
                        let e14 = {
                            let l7 = *base.add(0).cast::<i32>();
                            let l8 = *base.add(4).cast::<*mut u8>();
                            let l9 = *base.add(8).cast::<usize>();
                            let base13 = l8;
                            let len13 = l9;
                            let mut result13 = _rt::Vec::with_capacity(len13);
                            for i in 0..len13 {
                                let base = base13.add(i * 8);
                                let e13 = {
                                    let l10 = *base.add(0).cast::<*mut u8>();
                                    let l11 = *base.add(4).cast::<usize>();
                                    let len12 = l11;
                                    let bytes12 =
                                        _rt::Vec::from_raw_parts(l10.cast(), len12, len12);

                                    _rt::string_lift(bytes12)
                                };
                                result13.push(e13);
                            }
                            _rt::cabi_dealloc(base13, len13 * 8, 4);

                            SkillRating {
                                rating: l7 as u32,
                                skills: result13,
                            }
                        };
                        result14.push(e14);
                    }
                    _rt::cabi_dealloc(base14, len14 * 12, 4);
                    let base18 = arg10;
                    let len18 = arg11;
                    let mut result18 = _rt::Vec::with_capacity(len18);
                    for i in 0..len18 {
                        let base = base18.add(i * 8);
                        let e18 = {
                            let l15 = *base.add(0).cast::<*mut u8>();
                            let l16 = *base.add(4).cast::<usize>();
                            let len17 = l16;
                            let bytes17 = _rt::Vec::from_raw_parts(l15.cast(), len17, len17);

                            _rt::string_lift(bytes17)
                        };
                        result18.push(e18);
                    }
                    _rt::cabi_dealloc(base18, len18 * 8, 4);
                    let base22 = arg12;
                    let len22 = arg13;
                    let mut result22 = _rt::Vec::with_capacity(len22);
                    for i in 0..len22 {
                        let base = base22.add(i * 8);
                        let e22 = {
                            let l19 = *base.add(0).cast::<*mut u8>();
                            let l20 = *base.add(4).cast::<usize>();
                            let len21 = l20;
                            let bytes21 = _rt::Vec::from_raw_parts(l19.cast(), len21, len21);

                            _rt::string_lift(bytes21)
                        };
                        result22.push(e22);
                    }
                    _rt::cabi_dealloc(base22, len22 * 8, 4);
                    T::submit_character(Character {
                        name: _rt::string_lift(bytes0),
                        high_concept: _rt::string_lift(bytes1),
                        trouble: _rt::string_lift(bytes2),
                        aspects: result6,
                        skills: result14,
                        stunts: result18,
                        skill_list: result22,
                    });
                }
                pub trait Guest {
                    fn get_character() -> Character;
                    fn update_character(character: Character);
                    fn toggle_editable() -> bool;
                    fn submit_character(character: Character);
                }
                #[doc(hidden)]

                macro_rules! __export_golem_component_api_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "golem:component/api#get-character"]
    unsafe extern "C" fn export_get_character() -> *mut u8 {
      $($path_to_types)*::_export_get_character_cabi::<$ty>()
    }
    #[export_name = "cabi_post_golem:component/api#get-character"]
    unsafe extern "C" fn _post_return_get_character(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_get_character::<$ty>(arg0)
    }
    #[export_name = "golem:component/api#update-character"]
    unsafe extern "C" fn export_update_character(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,arg4: *mut u8,arg5: usize,arg6: *mut u8,arg7: usize,arg8: *mut u8,arg9: usize,arg10: *mut u8,arg11: usize,arg12: *mut u8,arg13: usize,) {
      $($path_to_types)*::_export_update_character_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13)
    }
    #[export_name = "golem:component/api#toggle-editable"]
    unsafe extern "C" fn export_toggle_editable() -> i32 {
      $($path_to_types)*::_export_toggle_editable_cabi::<$ty>()
    }
    #[export_name = "golem:component/api#submit-character"]
    unsafe extern "C" fn export_submit_character(arg0: *mut u8,arg1: usize,arg2: *mut u8,arg3: usize,arg4: *mut u8,arg5: usize,arg6: *mut u8,arg7: usize,arg8: *mut u8,arg9: usize,arg10: *mut u8,arg11: usize,arg12: *mut u8,arg13: usize,) {
      $($path_to_types)*::_export_submit_character_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, arg11, arg12, arg13)
    }
  };);
}
                #[doc(hidden)]
                pub(crate) use __export_golem_component_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 56]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 56]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::alloc;

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
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
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

macro_rules! __export_fate_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::golem::component::api::__export_golem_component_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::component::api);
  )
}
#[doc(inline)]
pub(crate) use __export_fate_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:fate:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 484] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xe9\x02\x01A\x02\x01\
A\x02\x01B\x11\x01ps\x01r\x01\x0askill-list\0\x04\0\x04game\x03\0\x01\x01r\x02\x06\
ratingy\x06skills\0\x04\0\x0cskill-rating\x03\0\x03\x01p\x04\x01r\x07\x04names\x0c\
high-concepts\x07troubles\x07aspects\0\x06skills\x05\x06stunts\0\x0askill-list\0\
\x04\0\x09character\x03\0\x06\x01r\x02\x08editable\x7f\x09character\x07\x04\0\x0f\
character-sheet\x03\0\x08\x01@\0\0\x07\x04\0\x0dget-character\x01\x0a\x01@\x01\x09\
character\x07\x01\0\x04\0\x10update-character\x01\x0b\x01@\0\0\x7f\x04\0\x0ftogg\
le-editable\x01\x0c\x04\0\x10submit-character\x01\x0b\x04\x01\x13golem:component\
/api\x05\0\x04\x01\x14golem:component/fate\x04\0\x0b\x0a\x01\0\x04fate\x03\0\0\0\
G\x09producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindge\
n-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
