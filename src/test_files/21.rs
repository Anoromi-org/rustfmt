impl ActivityCalculator for ActivityCalculatorImpl
{
  fn calculate_activity_percentage()
  {
    let keyboard_activity_percentage = if expected_keyboard_clicks > 0.0
    {
      keyboard_clicks as f64 / expected_keyboard_clicks * 100.0
    }
    else
    {
      0.0
    };
  }
}
