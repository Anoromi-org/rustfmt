use crate::config_test::{ ActiveWindowData, very_long_function_taking_1_parameter, very_long_function_taking_2_parameters };
// fn hehe()
// {
//   #[ allow(
//     unused_imports,
//     unused_variables,
//     dead_code,
//     unsafe_code,
//     unknown_lints,
//     unexpected_cfgs,
//     uncovered_param_in_projection,
//     clippy::option_env_unwrap
//   ) ]
//   let b = 4;
// }
// #[ cfg(  any(
//   feature = "derive_as_mut",
//   feature = "derive_as_ref",
//   feature = "derive_deref",
//   feature = "derive_deref_mut",
//   feature = "derive_from",
//   feature = "derive_index",
//   feature = "derive_index_mut",
//   feature = "derive_inner_from",
//   feature = "derive_new",
//   feature = "derive_variadic_from",
//   feature = "derive_not",
//   feature = "derive_phantom"
// ) ) ]
// use std::collections::{  HashMap, HashSet };
// use std::
// {
//   cfg,
//   mem::
//   {
//     Discriminant,
//     ManuallyDrop,
//     offset_of,
//     drop,
//     replace,
//     forget,
//     swap,
//     size_of,
//     size_of_val,
//     align_of,
//     align_of_val,
//     transmute,
//     transmute_copy,
//     uninitialized,
//     zeroed,
//     needs_drop,
//     MaybeUninit,
//   },
// };
// use std::
// {
//   env,
//   env::JoinPathsError,
//   env::var,
//   hint::unreachable_unchecked,
//   mem,
//   mem::Discriminant,
//   mem::ManuallyDrop,
//   mem::offset_of,
//   vec,
//   vec::IntoIter,
// };
// struct V;
//
// enum E
// {
//   A
//   {
//     v_eaaaaariableeeaeaeaa : u32,
//     v_eaaaaariableeeaeaeaae : i32,
//     v_eaaaaariableeeaeaeaae3 : i32,
//     v_eaaaaariableeeaeaeaae4 : i32,
//   },
// }

use std::sync::{ Arc, Mutex };

const VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO : u32 = 1;
// fn ts()
// {
//   let b =
//           |v_eaaaaariableeeaeaeaa : u32,
//            v_eaaaaariableeeaeaeaae : i32,
//            v_eaaaaariableeeaeaeaae3 : i32,
//            v_eaaaaariableeeaeaeaae4 : i32
//           | {
//     let k = VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO;
//     let v = VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO;
//   };
// }
// impl V
// {
//   #[ allow( unused_imports ) ]

fn hello_there( v_eaaaaariableeeaeaeaa_v_eaaaaariableeeaeaeaa : u32, v_eaaaaariableeeaeaeaae_v_eaaaaariableeeaeaeaa : i32 )
{
  // let h = 3u32
  // .max( 4 )
  // .max( 5 )
  // .max( 10 )
  // .max( 11 )
  // .max( 14 )
  // .max( 4 )
  // .max( 5 )
  // .max( 10 )
  // .max( 11 )
  // .max( 14 );

  let test = Arc::new
  (
    Mutex::new( ActiveWindowData
    {
      window_title : Default::default(),
      process_name : Default::default(),
      app_id : Default::default(),
    } ),
  );
  // process_name : Default::default(),
  // app_id : Default::default(),
  // let b =
  // (
  //   |v_eaaaaariableeeaeaeaa : u32,
  //    v_eaaaaariableeeaeaeaae : i32,
  //    v_eaaaaariableeeaeaeaae3 : i32,
  //    v_eaaaaariableeeaeaeaae4 : i32| {
  //     let k = VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO;
  //     let v = VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO;
  //   },
  //   | v : u32 | VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO,
  //   | v : u32 | VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO,
  //   | v : u32 | VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO,
  // );
  // let src = ( 1, );
  // let hehe : fn( u32 ) -> u32 = | v | 3;
}

const VERY_LONG_SOMETHING_CONSTANT_HELLO_THERE_HOW_DO_YOU_DO_NEEDS_TO_BE_EVEN_LONGER : u32 = 1;

#[ allow( all ) ]
fn hello_there2()
{
  let test = very_long_function_taking_2_parameters
  (
    &std::sync::Arc::new( ActiveWindowData
    {
      window_title : Default::default(),
      process_name : Default::default(),
      app_id : Default::default(),
    } ),
    &std::sync::Arc::new( ActiveWindowData
    {
      window_title : Default::default(),
      process_name : Default::default(),
      app_id : Default::default(),
    } ),
  );
}

//
//   fn hello_there2
//   (
//     v_eaaaaariableeeaeaeaa : u32,
//     v_eaaaaariableeeaeaeaae : i32,
//     v_eaaaaariableeeaeaeaae3 : i32,
//     v_eaaaaariableeeaeaeaae4 : i32,
//   )
//   {
//   }
// }
