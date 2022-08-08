use sysinfo::{System, SystemExt};
use evalexpr::{eval_with_context_mut, HashMapContext, Value, EvalexprError, ContextWithMutableVariables};



pub fn eval_expression(e: &str) -> Result<Value, EvalexprError> {
    let mut sys = System::new_all();
    sys.refresh_all();
    let mut context = HashMapContext::new();
    context.set_value("answer".to_string(), Value::from(42)).unwrap();
    context.set_value("sysinfo::sysname".to_string(), Value::from(sys.name().unwrap())).unwrap();
    context.set_value("sysinfo::kernel".to_string(), Value::from(sys.kernel_version().unwrap())).unwrap();
    context.set_value("sysinfo::hostname".to_string(), Value::from(sys.host_name().unwrap())).unwrap();
    context.set_value("sysinfo::os_version".to_string(), Value::from(sys.os_version().unwrap())).unwrap();
    eval_with_context_mut(e, &mut context)
}
