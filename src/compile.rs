#![allow(unused, unused_variables)]
use crate::{main, Operation};
use cranelift::codegen::ir;
use cranelift::prelude::*;
use cranelift_module::{FuncId, Linkage, Module};
use cranelift_object::{ObjectBuilder, ObjectModule};
use std::sync::Arc;
use cranelift_codegen::settings;

pub struct Context{
    pub objmodule: ObjectModule,

}

impl Context {
    pub fn new() -> Self {
        let mut shared_builder = settings::builder();
        // shared_builder.set("opt_level", "speed").unwrap();
        let shared_flags = settings::Flags::new(shared_builder);

        let isa = isa().finish(shared_flags).unwrap();

        let builder = ObjectBuilder::new(isa, b"main".to_vec(), cranelift_module::default_libcall_names()).unwrap();
        let mut objmodule = ObjectModule::new(builder);
        Self { objmodule }
    }

    // pub fn define_bf(&mut self, instructions: Vec<Operation>) -> FuncId {
    //     let mut sig = self.objmodule.make_signature();

    // }

    pub fn define_entrypoint(&mut self, _bf: Option<FuncId>) -> (FuncId, cranelift_codegen::Context) {
        let mut main_sig = self.objmodule.make_signature();
        main_sig.returns.push(AbiParam::new(types::I32));
        let main_func_id = self.objmodule
        .declare_function("main", cranelift_module::Linkage::Export, &main_sig)
        .unwrap();
        let mut main_ctx = self.objmodule.make_context();
        main_ctx.func.signature = main_sig;
        let mut main_builder_context = FunctionBuilderContext::new();
        let mut main_builder = FunctionBuilder::new(&mut main_ctx.func, &mut main_builder_context);
        let main_block = main_builder.create_block();
        main_builder.switch_to_block(main_block);
        main_builder.seal_block(main_block);
        // Define extern functions getchar and putchar
        
        let ret = main_builder.ins().iconst(types::I32, 12);
        main_builder.ins().return_(&[ret]);
        println!("{}", main_builder.func);
        (main_func_id, main_ctx)
            
    }

}

fn isa() -> isa::Builder {
    isa::lookup_by_name("x86_64-unknown-linux-gnu").unwrap()
}


macro_rules! add {
    ($builder:expr, $lhs:expr, $rhs:expr) => {
        $builder.ins().iadd($lhs, $rhs)
    };
}

macro_rules! sub {
    ($builder:expr, $lhs:expr, $rhs:expr) => {
        $builder.ins().isub($lhs, $rhs)
    };
}

macro_rules! left {
    ($builder:expr, $ptr:expr, $offset:expr) => {
        $builder.ins().iadd_imm($ptr, $offset)
    };
}

macro_rules! right {
    ($builder:expr, $ptr:expr, $offset:expr) => {
        $builder.ins().isub_imm($ptr, $offset)
    };
}

macro_rules! loop_start {
    ($builder:expr, $ptr:expr, $loop_start:expr, $loop_end:expr) => {
        let cond =
            $builder
                .ins()
                .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::NotEqual, $ptr, 0);
        let brz = $builder.ins().brz(cond, $loop_end);
        $builder.ins().jump($loop_start);
        $builder.seal_block($loop_end);
        $builder.seal_block($loop_start);
        brz
    };
}

macro_rules! loop_end {
    ($builder:expr, $ptr:expr, $loop_start:expr, $loop_end:expr) => {
        let cond = $builder
            .ins()
            .icmp_imm(cranelift_codegen::ir::condcodes::IntCC::Equal, $ptr, 0);
        let brz = $builder.ins().brz(cond, $loop_start);
        $builder.ins().jump($loop_end);
        $builder.seal_block($loop_end);
        $builder.seal_block($loop_start);
        brz
    };
}

macro_rules! input {
    ($builder:expr, $ptr:expr) => {
        $builder
            .ins()
            .call_indirect(types::I32, $ptr, &[$builder.ins().iconst(types::I32, 0)])
    };
}

macro_rules! output {
    ($builder:expr, $ptr:expr) => {
        $builder
            .ins()
            .call_indirect(types::I32, $ptr, &[$builder.ins().iconst(types::I32, 1)])
    };
}

macro_rules! zero {
    ($builder:expr, $ptr:expr) => {
        $builder.ins().store(
            MemFlags::new(),
            $builder.ins().iconst(types::I32, 0),
            $ptr,
            0,
        )
    };
}
