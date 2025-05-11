struct Test
{
  a : u32,
  b : u32,
}


fn hello( param_1 : u32, param_2 : u32 )
{
  call_some_fn
  (
    ||
    Test
    {
      a : param_1,
      b : param_2,
    },
    ||
    OtherStruct
    {
      test : param_1 + param_2,
    },
  );
}
