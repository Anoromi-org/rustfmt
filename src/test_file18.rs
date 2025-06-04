#[ tauri::command ]
#[ specta::specta ]
pub async fn start
(
   app : State< '_, Application >, project_id : String, task : Option< String >
) -> Result< (), ApplicationError >
{
}
