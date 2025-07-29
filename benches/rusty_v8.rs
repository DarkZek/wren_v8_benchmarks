use criterion::{black_box, Criterion};
use rusty_v8 as v8;

pub fn bench_v8(c: &mut Criterion) {

  let jit = std::env::var("JIT").unwrap_or("".to_string()) == "true";

  if !jit {
    v8::V8::set_flags_from_string(
      "--jitless",
    );
  }

  // Initialize V8.
  let platform = v8::new_default_platform(0, false).make_shared();
  v8::V8::initialize_platform(platform);
  v8::V8::initialize();

  {
    // Create a new Isolate and make it the current one.
    let isolate = &mut v8::Isolate::new(v8::CreateParams::default());

    // Create a stack-allocated handle scope.
    let handle_scope = &mut v8::HandleScope::new(isolate);

    // Create a new context.
    let context = v8::Context::new(handle_scope);

    // Enter the context for compiling and running the hello world script.
    let scope = &mut v8::ContextScope::new(handle_scope, context);

    // Define the JS function as a string.
    let code = v8::String::new(scope, include_str!("./script.js")).unwrap();

    // Compile the script.
    let script = v8::Script::compile(scope, code, None).unwrap();

    // Run the script to get the 'add' function.
    let result = script.run(scope).unwrap();
    let add_fn = v8::Local::<v8::Function>::try_from(result).unwrap();

    let mut run = |a: f64, b: f64| {
      let arg1 = v8::Number::new(scope, black_box(a));
      let arg2 = v8::Number::new(scope, black_box(b));

      // Call the JS function with Rust inputs.
      let args = [arg1.into(), arg2.into()];
      let undefined = v8::undefined(scope).into();
      let js_result = add_fn.call(scope, undefined, &args);

      // Convert result back to Rust.
      let sum = js_result.unwrap().number_value(scope).unwrap();

      black_box(sum)
    };

    c.bench_function(&format!("Rusty-v8 (1, 1) (JIT: {jit})"), |b| b.iter(|| { run(1.0, 1.0) }));
    c.bench_function(&format!("Rusty-v8 (300, 50) (JIT: {jit})"), |b| b.iter(|| { run(300.0, 50.0) }));
    c.bench_function(&format!("Rusty-v8 (3000, 500) (JIT: {jit})"), |b| b.iter(|| { run(3000.0, 500.0) }));
  }

  unsafe {
    v8::V8::dispose();
  }

}
