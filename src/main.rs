use components::transpiler::Transpiler;
pub mod components;

fn main() {
  let mut transpiler: Transpiler = Transpiler::new();
  transpiler.transpile_file("./programs/test.ir");
}
