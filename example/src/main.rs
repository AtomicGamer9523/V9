fn main() {
  let platform = v9::new_default_platform(0, false).make_shared();
  v9::V9::initialize_platform(platform);
  v9::V9::initialize();

  let isolate = &mut v9::Isolate::new(Default::default());

  let scope = &mut v9::HandleScope::new(isolate);
  let context = v9::Context::new(scope);
  let scope = &mut v9::ContextScope::new(scope, context);

  let code = v9::String::new(scope, "'Hello' + ' World!'").unwrap();
  println!("javascript code: {}", code.to_rust_string_lossy(scope));

  let script = v9::Script::compile(scope, code, None).unwrap();
  let result = script.run(scope).unwrap();
  let result = result.to_string(scope).unwrap();
  println!("result: {}", result.to_rust_string_lossy(scope));
}
