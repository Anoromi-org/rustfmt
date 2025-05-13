fn test()
{
  let images_result = async_runtime::spawn_blocking( move || recorder.screenshot_recorder.make_screenshots( &screenshot_path ) )
  .await
  .expect( "blocking operation should succeed" );
  3
}
