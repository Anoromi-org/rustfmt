impl< IProvider, ITimerRecorderDao, ITaskDao > TimerRecordService
for TimerRecordServiceImpl< IProvider, ITimerRecorderDao, ITaskDao >
where
  X : X,
{
}

impl< 'a, Collection, T1 : PartialEq + 'a, T2 : Default + 'a > From< Collection > for Many< T1, T2 >
where
  Collection : IntoIterator< Item = &'a mod1::Floats< T1, T2 > >,
{
  #[ inline ]
  fn from( src : Collection ) -> Self
  {
    let src2 = src
    .into_iter()
    .map( | e | *e )
    .collect::<the_module::_Vec< mod1::Floats< T1, T2 > >>();
    Self( src2 )
  }
}
