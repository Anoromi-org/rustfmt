pub struct ActiveWindowData {
    pub window_title: std::sync::Arc<str>,
    pub process_name: Option<std::sync::Arc<str>>,
    pub app_id: Option<std::sync::Arc<str>>,
}

pub fn very_long_function_taking_2_parameters(a: &ActiveWindowData, b: &ActiveWindowData) {}

pub fn very_long_function_taking_1_parameter(a: u32) -> u32 {
    a
}

pub fn very_long_function_taking_1_closure(a: impl Fn(u32, u32)) -> u32 {
    3
}

pub fn very_long_function_taking_1_closures_1_param(b: u32, a: impl Fn(u32, u32)) -> u32 {
    3
}
