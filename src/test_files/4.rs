pub const fn new() -> Lazy< Arc< Mutex< Registry< Context > > > >
{
  Lazy::new( ||
  {
    let contexts = DashMap::new();
    let contexts_with_name = DashMap::new();
    let current_context_name = None;
    Arc::new(
      Mutex::new( Registry::< Context >
      {
        contexts,
        contexts_with_name,
        current_context_name,
      }),
    )
  })
}
