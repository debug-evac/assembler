/*
 * Copyright (c) 2023 Steven Becker
 *
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

//
// Optional optimizer that tries to reduce NOOP operations and
// maximizes parallelization (in the pipeline) of operations.
// Relevant techniques include register renaming, instruction
// scheduling, peephole optimization (reduction of certain
// operations into shorter-running operations), among other
// things. This will run on every compilation (? rather
// translations), however changes to the programm will only be
// made, if the programmer includes an (or possibly a number of)
// flag(s). If there are no flags, the assembler will simply
// issue a notice that performance COULD be gained if
// restructuring the assembly code (can be suppressed by
// another flag).

use crate::{
    parser::{Instruction, Operation, MacroInstr, Reg}, 
    linker::Namespaces
};

impl MacroInstr {
    fn translate(&self, namespace: &mut Namespaces, current_space: &usize, instructions: &mut Vec<Instruction>) -> () {
        // Do not forget to change the lines function in the parser when changing the amount of lines here! 
        // (TODO: Better method for this)
        match self {
            MacroInstr::NA => instructions.push(Instruction::NA),

            MacroInstr::NOP => instructions.push(Instruction::Addi(Reg::G0, Reg::G0, 0)),

            MacroInstr::Beq(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Beq(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Bne(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Bne(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Blt(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Blt(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Bltu(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Bltu(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Bge(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Bge(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Bgeu(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Bgeu(reg1.to_owned(), reg2.to_owned(), lines));
            },

            MacroInstr::Mv(reg1, reg2) => instructions.push(Instruction::Addi(reg1.to_owned(), reg2.to_owned(), 0)),
            MacroInstr::Li(reg, imm) => {
                // TODO: Evaluate better strategy?
                instructions.push(Instruction::Addi(reg.to_owned(), Reg::G0, *imm));
                instructions.push(Instruction::Lui(reg.to_owned(), imm >> 12));
            },

            MacroInstr::Jal(reg, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Jal(reg.to_owned(), lines));
            },
            MacroInstr::Jalr(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Jalr(reg1.to_owned(), reg2.to_owned(), lines));
            },

            MacroInstr::J(imm) => instructions.push(Instruction::Jal(Reg::G0, *imm)),
            MacroInstr::Jl(labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Jal(Reg::G0, lines));
            },
            MacroInstr::Jali(imm) => instructions.push(Instruction::Jal(Reg::G1, *imm)),
            MacroInstr::Jall(labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Jal(Reg::G1, lines));
            },
            MacroInstr::Jr(reg) => instructions.push(Instruction::Jalr(Reg::G0, reg.to_owned(), 0)),
            MacroInstr::Jalrs(reg) => instructions.push(Instruction::Jalr(Reg::G1, reg.to_owned(), 0)),

            MacroInstr::Ret => instructions.push(Instruction::Jalr(Reg::G0, Reg::G1, 0)),

            MacroInstr::Calli(imm) => {
                instructions.push(Instruction::Auipc(Reg::G1, imm >> 12));
                instructions.push(Instruction::Jalr(Reg::G1, Reg::G1, *imm));
            },
            MacroInstr::Calll(labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Auipc(Reg::G1, lines >> 12));
                instructions.push(Instruction::Jalr(Reg::G1, Reg::G1, lines));
            },
            MacroInstr::Taili(imm) => {
                instructions.push(Instruction::Auipc(Reg::G6, imm >> 12));
                instructions.push(Instruction::Jalr(Reg::G0, Reg::G6, *imm));
            },
            MacroInstr::Taill(labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Auipc(Reg::G6, lines >> 12));
                instructions.push(Instruction::Jalr(Reg::G0, Reg::G6, lines));
            },

            MacroInstr::Lui(reg, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lui(reg.to_owned(), lines));
            },
            MacroInstr::Auipc(reg, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Auipc(reg.to_owned(), lines));
            },

            MacroInstr::Slli(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Slli(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Srli(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Srli(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Srai(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Srai(reg1.to_owned(), reg2.to_owned(), lines));
            },

            MacroInstr::Srr(reg1, reg2, imm) => todo!("About to implement!"),
            MacroInstr::Slr(reg1, reg2, imm) => todo!("About to implement!"),

            MacroInstr::Divn(reg1, reg2, reg3) => todo!("About to implement!"),
            MacroInstr::Muln(reg1, reg2, reg3) => todo!("About to implement!"),

            MacroInstr::Xnor(reg1, reg2, reg3) => todo!("Not implemented yet!"),
            MacroInstr::Nor(reg1, reg2, reg3) => todo!("Not implemented yet!"),

            MacroInstr::Lb(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lb(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Lh(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lh(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Lw(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lw(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Lbu(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lbu(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Lhu(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Lhu(reg1.to_owned(), reg2.to_owned(), lines));
            },

            MacroInstr::Sb(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Sb(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Sh(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Sh(reg1.to_owned(), reg2.to_owned(), lines));
            },
            MacroInstr::Sw(reg1, reg2, labl) => {
                let lines = (instructions.len() as i32) - translate_label(labl.to_owned(), namespace, *current_space);
                instructions.push(Instruction::Sw(reg1.to_owned(), reg2.to_owned(), lines));
            },
        };
    }
}

fn translate_label(label: String, namespaces: &mut Namespaces, current_space: usize) -> i32 {
    match namespaces.get_label(label, Some(current_space)) {
        Some(label_elem) => <i128 as TryInto<i32>>::try_into(*label_elem.get_def()).unwrap(),
        None => i32::MIN,
    }
}

fn substitute_labels(mut code: (Namespaces, Vec<Operation>)) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = vec![];
    let mut namespace: usize = 0;

    /*
    for operation in code.1 {
        match operation {
            Operation::Namespace(space) => namespace = space,
            Operation::Macro(instr) | Operation::LablMacro(_, instr) => {
                let tr_instr = match instr {
                    Instruction::VJmp(labl) => Instruction::Jmp(translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBt(labl) => Instruction::Bt(translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBf(labl) => Instruction::Bf(translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBeq(reg1, reg2, labl) => Instruction::Beq(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBne(reg1, reg2, labl) => Instruction::Bne(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBlt(reg1, reg2, labl) => Instruction::Blt(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBltu(reg1, reg2, labl) => Instruction::Bltu(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBge(reg1, reg2, labl) => Instruction::Bge(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),
                    Instruction::VBgeu(reg1, reg2, labl) => Instruction::Bgeu(reg1, reg2, 
                        translate_label(labl, &mut code.0, namespace)),

                    _ => instr,
                };

                instructions.push(tr_instr)
            },
            Operation::Labl(_) => (),
        };
    }*/

    return instructions;
}

pub fn optimize(code: (Namespaces, Vec<Operation>)) -> Vec<Instruction> {
    return substitute_labels(code);
}

// TODO: Tests here