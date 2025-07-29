use criterion::{black_box, Criterion};
use ruwren::{FunctionSignature, ModuleLibrary, VMConfig};

pub fn bench_wren(c: &mut Criterion) {
    let lib = ModuleLibrary::new();

    let vm = VMConfig::new().library(&lib).build();

    vm.interpret(
        "main",
        include_str!("script.wren"),
    )
    .unwrap();

    vm.execute(|vm| {
        vm.ensure_slots(1);
        vm.get_variable("main", "Math", 0);
    });

    let main_class = vm.get_slot_handle(0);
    let main_function = vm.make_call_handle(
        FunctionSignature::new_function("math", 2)
    );

    let run = |a: f64, b: f64| {
        vm.execute(|vm| {
            vm.ensure_slots(3);
            vm.set_slot_double(1, black_box(a));
            vm.set_slot_double(2, black_box(b));
        });

        vm.set_slot_handle(0, &main_class);

        vm.call_handle(&main_function).unwrap();

        vm.execute(|vm| {
            black_box(vm.get_slot_double(0).unwrap())
        });
    };

    c.bench_function("Rewren (1, 1)", |b| b.iter(|| { run(1.0, 1.0) }));
    c.bench_function("Rewren (300, 50)", |b| b.iter(|| { run(300.0, 50.0) }));
    c.bench_function("Rewren (3000, 500)", |b| b.iter(|| { run(3000.0, 500.0) }));
}
