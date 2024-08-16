use cranelift_codegen::ir::{types::*, AbiParam, FuncRef, InstBuilder};
use cranelift_codegen::isa::CallConv;
use cranelift_codegen::settings::FlagsOrIsa;
use cranelift_codegen::verify_function;
use cranelift_frontend::{FunctionBuilder, FunctionBuilderContext};
use cranelift_module::{FuncId, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};

fn main() {
    // Create the Cranelift ISA (Instruction Set Architecture) builder
    let isa = cranelift_native::builder()
        .unwrap()
        .finish(cranelift_codegen::settings::Flags::new(
            cranelift_codegen::settings::builder(),
        ));

    // Initialize the Cranelift module
    let mut module = ObjectModule::new(
        ObjectBuilder::new(
            isa.expect("ISA initialization failed"),
            "example",
            cranelift_module::default_libcall_names(),
        )
        .unwrap(),
    );

    // Declare the "add" function
    let mut add_sig = module.make_signature();
    add_sig.params.push(AbiParam::new(I32)); // x: i32
    add_sig.params.push(AbiParam::new(I32)); // y: i32
    add_sig.returns.push(AbiParam::new(I32)); // return value: i32
    let add_func_id = module
        .declare_function("add", cranelift_module::Linkage::Export, &add_sig)
        .unwrap();

    // Define the "add" function
    let mut add_ctx = module.make_context();
    add_ctx.func.signature = add_sig;
    let mut add_builder_context = FunctionBuilderContext::new();
    let mut add_builder = FunctionBuilder::new(&mut add_ctx.func, &mut add_builder_context);
    let add_block = add_builder.create_block();
    add_builder.switch_to_block(add_block);
    add_builder.append_block_params_for_function_params(add_block);

    let x = add_builder.block_params(add_block)[0];
    let y = add_builder.block_params(add_block)[1];
    let sum = add_builder.ins().iadd(x, y);
    add_builder.ins().return_(&[sum]);

    add_builder.seal_block(add_block);
    add_builder.finalize();
    module.define_function(add_func_id, &mut add_ctx).unwrap();
    //

    // Declare the "main" function
    let mut main_sig = module.make_signature();
    main_sig.returns.push(AbiParam::new(I32)); // return value: i32
    let main_func_id = module
        .declare_function("main", cranelift_module::Linkage::Export, &main_sig)
        .unwrap();

    // Define the "main" function
    let mut main_ctx = module.make_context();
    main_ctx.func.signature = main_sig;
    let mut main_builder_context = FunctionBuilderContext::new();
    let mut main_builder = FunctionBuilder::new(&mut main_ctx.func, &mut main_builder_context);
    let main_block = main_builder.create_block();
    main_builder.switch_to_block(main_block);

    // Call the "add" function and return the result
    let args = [
        main_builder.ins().iconst(I32, 1), // argument 1
        main_builder.ins().iconst(I32, 2), // argument 2
    ];
    let add_func_ref = module.declare_func_in_func(add_func_id, &mut main_builder.func);
    let call_inst = main_builder.ins().call(add_func_ref, &args);
    let result = main_builder.inst_results(call_inst)[0];

    main_builder.ins().return_(&[result]);

    main_builder.seal_block(main_block);

    main_builder.finalize();
    module.define_function(main_func_id, &mut main_ctx).unwrap();
    // Finish and write the object file
    let product = module.finish();
    std::fs::write("output.o", product.emit().unwrap()).unwrap();

    // Print the IR for the "main" function
    println!("{}", main_ctx.func.display());
    // use the ir verifier to check the correctness of the generated code
    // Link the object file
    std::process::Command::new("ld")
        .arg("output.o")
        .output()
        .unwrap();
}
