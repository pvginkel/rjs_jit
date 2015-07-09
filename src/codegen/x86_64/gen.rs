#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

/*
 * jit-gen-x86-64.h - Macros for generating x86_64 code.
 *
 * Copyright (C) 2008  Southern Storm Software, Pty Ltd.
 *
 * This file is part of the libjit library.
 *
 * The libjit library is free software: you can redistribute it and/or
 * modify it under the terms of the GNU Lesser General Public License
 * as published by the Free Software Foundation, either version 2.1 of
 * the License, or (at your option) any later version.
 *
 * The libjit library is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public
 * License along with the libjit library.  If not, see
 * <http://www.gnu.org/licenses/>.
 */

use codegen::Emit;
use std::mem::transmute;
use codegen::x86::gen::*;
use std::i32;

/*
 * X86_64 64 bit general purpose integer registers.
 */

pub type X86_64_Reg_No = u8;

pub const X86_64_RAX : X86_64_Reg_No = 0;
pub const X86_64_RCX : X86_64_Reg_No = 1;
pub const X86_64_RDX : X86_64_Reg_No = 2;
pub const X86_64_RBX : X86_64_Reg_No = 3;
pub const X86_64_RSP : X86_64_Reg_No = 4;
pub const X86_64_RBP : X86_64_Reg_No = 5;
pub const X86_64_RSI : X86_64_Reg_No = 6;
pub const X86_64_RDI : X86_64_Reg_No = 7;
pub const X86_64_R8  : X86_64_Reg_No = 8;
pub const X86_64_R9  : X86_64_Reg_No = 9;
pub const X86_64_R10 : X86_64_Reg_No = 10;
pub const X86_64_R11 : X86_64_Reg_No = 11;
pub const X86_64_R12 : X86_64_Reg_No = 12;
pub const X86_64_R13 : X86_64_Reg_No = 13;
pub const X86_64_R14 : X86_64_Reg_No = 14;
pub const X86_64_R15 : X86_64_Reg_No = 15;
pub const X86_64_RIP : X86_64_Reg_No = 16;
						/* This register encoding doesn't exist in the */
						/* instructions. It's used for RIP relative encoding. */

/*
 * X86-64 xmm registers.
 */

pub type X86_64_XMM_Reg_No = u8;

pub const X86_64_XMM0 : X86_64_XMM_Reg_No = 0;
pub const X86_64_XMM1 : X86_64_XMM_Reg_No = 1;
pub const X86_64_XMM2 : X86_64_XMM_Reg_No = 2;
pub const X86_64_XMM3 : X86_64_XMM_Reg_No = 3;
pub const X86_64_XMM4 : X86_64_XMM_Reg_No = 4;
pub const X86_64_XMM5 : X86_64_XMM_Reg_No = 5;
pub const X86_64_XMM6 : X86_64_XMM_Reg_No = 6;
pub const X86_64_XMM7 : X86_64_XMM_Reg_No = 7;
pub const X86_64_XMM8 : X86_64_XMM_Reg_No = 8;
pub const X86_64_XMM9 : X86_64_XMM_Reg_No = 9;
pub const X86_64_XMM10 : X86_64_XMM_Reg_No = 10;
pub const X86_64_XMM11 : X86_64_XMM_Reg_No = 11;
pub const X86_64_XMM12 : X86_64_XMM_Reg_No = 12;
pub const X86_64_XMM13 : X86_64_XMM_Reg_No = 13;
pub const X86_64_XMM14 : X86_64_XMM_Reg_No = 14;
pub const X86_64_XMM15 : X86_64_XMM_Reg_No = 15;

/*
 * Bits in the REX prefix byte.
 */
pub type X86_64_REX_Bits = u8;

pub const X86_64_REX_B : X86_64_REX_Bits = 1;
                        /* 1-bit (high) extension of the ModRM r/m field */
						/* SIB base field, or opcode reg field, thus */
						/* permitting access to 16 registers. */
pub const X86_64_REX_X : X86_64_REX_Bits = 2;
                    	/* 1-bit (high) extension of the SIB index field */
						/* thus permitting access to 16 registers. */
pub const X86_64_REX_R : X86_64_REX_Bits = 4;
                    	/* 1-bit (high) extension of the ModRM reg field, */
						/* thus permitting access to 16 registers. */
pub const X86_64_REX_W : X86_64_REX_Bits = 8;
                    	/* 0 = Default operand size */
						/* 1 = 64 bit operand size */

/*
 * Third part of the opcodes for xmm instructions which are encoded
 * Opcode1: 0xF3 (single precision) or 0xF2 (double precision)
 *          This is handled as a prefix.
 * Opcode2: 0x0F
 */

pub type X86_64_XMM1_OP = u8;

pub const XMM1_MOV : X86_64_XMM1_OP = 0x10;
pub const XMM1_MOV_REV : X86_64_XMM1_OP = 0x11;
pub const XMM1_ADD : X86_64_XMM1_OP = 0x58;
pub const XMM1_MUL : X86_64_XMM1_OP = 0x59;
pub const XMM1_SUB : X86_64_XMM1_OP = 0x5C;
pub const XMM1_DIV : X86_64_XMM1_OP = 0x5E;

/*
 * Logical opcodes used with packed single and double precision values.
 */

pub type X86_64_XMM_PLOP = u8;

pub const XMM_ANDP : X86_64_XMM_PLOP = 0x54;
pub const XMM_ORP : X86_64_XMM_PLOP = 0x56;
pub const XMM_XORP : X86_64_XMM_PLOP = 0x57;

/*
 * Rounding modes for xmm rounding instructions, the mxcsr register and
 * the fpu control word.
 */

pub type X86_64_ROUNDMODE = u8;

pub const X86_ROUND_NEAREST : X86_64_ROUNDMODE = 0x00;		/* Round to the nearest integer */
pub const X86_ROUND_DOWN : X86_64_ROUNDMODE = 0x01;		/* Round towards negative infinity */
pub const X86_ROUND_UP : X86_64_ROUNDMODE = 0x02;		/* Round towards positive infinity */
pub const X86_ROUND_ZERO : X86_64_ROUNDMODE= 0x03;		/* Round towards zero (truncate) */

pub fn x86_64_imm_emit64(inst: &mut Emit, imm: i64) {
    let imb = unsafe { transmute::<_, [u8; 8]>(imm) };
    inst.push(imb[0]);
    inst.push(imb[1]);
    inst.push(imb[2]);
    inst.push(imb[3]);
    inst.push(imb[4]);
    inst.push(imb[5]);
    inst.push(imb[6]);
    inst.push(imb[7]);
}

pub fn x86_64_imm_emit_max32(inst: &mut Emit, imm: i32, size: i32) {
    match size {
        1 =>  {
            x86_imm_emit8(inst, (imm));
        }
        2 => {
            x86_imm_emit16(inst, (imm));
        }
        4 | 8 => {
            x86_imm_emit32((inst), (imm));
        }
        _ => jit_assert!()
    }
}

pub fn x86_64_imm_emit_max64(inst: &mut Emit, imm: i64, size: i32) {
    match size {
        1 => {
            x86_imm_emit8(inst, (imm) as i32);
        }
        2 => {
            x86_imm_emit16(inst, (imm) as i32);
        }
        4 => {
            x86_imm_emit32((inst), (imm) as i32);
        }
        8 => {
            x86_64_imm_emit64(inst, (imm));
        }
        _ => jit_assert!()
    }
}

/*
 * Emit the Rex prefix.
 * The natural size is a power of 2 (1, 2, 4 or 8).
 * For accessing the low byte registers DIL, SIL, BPL and SPL we have to
 * generate a Rex prefix with the value 0x40 too.
 * To enable this OR the natural size with 1.
 */
pub fn x86_64_rex(rex_bits: u8) -> u8 {
    (0x40 | (rex_bits))
}

pub fn x86_64_rex_emit(inst: &mut Emit, width: i32, modrm_reg: u8, index_reg: u8, rm_base_opcode_reg: u8) {
    let __rex_bits =
        if ((width) & 8) != 0 { X86_64_REX_W } else { 0 } |
        if ((modrm_reg) & 8) != 0 { X86_64_REX_R } else { 0 } |
        if ((index_reg) & 8) != 0 { X86_64_REX_X } else { 0 } |
        if ((rm_base_opcode_reg) & 8) != 0 { X86_64_REX_B } else { 0 };
    if((__rex_bits != 0)) {
         inst.push(x86_64_rex(__rex_bits));
    } else if(((width) & 1) != 0 && ((modrm_reg & 4) != 0 || (rm_base_opcode_reg & 4) != 0)) {
         inst.push(x86_64_rex(0));
    }
}

/*
 * Helper for emitting the rex prefix for opcodes with 64bit default size.
 */
pub fn x86_64_rex_emit64(inst: &mut Emit, width: i32, modrm_reg: u8, index_reg: u8, rm_base_opcode_reg: u8) {
    x86_64_rex_emit((inst), 0, (modrm_reg), (index_reg), (rm_base_opcode_reg));
}

pub fn x86_64_reg_emit(inst: &mut Emit, r: u8, regno: u8) {
    x86_reg_emit((inst), ((r) & 0x7), ((regno) & 0x7));
}

pub fn x86_64_mem_emit(inst: &mut Emit, r: u8, disp: i32) {
    x86_address_byte ((inst), 0, ((r) & 0x7), 4);
    x86_address_byte ((inst), 0, 4, 5);
    x86_imm_emit32((inst), (disp));
}

pub fn x86_64_mem64_emit(inst: &mut Emit, r: u8, disp: i64) {
    x86_address_byte ((inst), 0, ((r) & 0x7), 4);
    x86_address_byte ((inst), 0, 4, 5);
    x86_64_imm_emit64((inst), (disp));
}

pub fn x86_64_membase_emit(inst: &mut Emit, reg: u8, basereg: u8, disp: i32) {
    if((basereg) == X86_64_RIP) {
        x86_address_byte((inst), 0, ((reg) & 0x7), 5);
        x86_imm_emit32((inst), (disp));
    } else {
        x86_membase_emit((inst), ((reg) & 0x7), ((basereg) & 0x7), (disp));
    }
}

pub fn x86_64_memindex_emit(inst: &mut Emit, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_memindex_emit((inst), ((r) & 0x7), ((basereg) & 0x7), (disp), ((indexreg) & 0x7), (shift));
}

/*
 * RSP, RBP and the corresponding upper registers (R12 and R13) can't be used
 * for relative addressing without displacement because their codes are used
 * for encoding addressing modes with diplacement.
 * So we do a membase addressing in this case with a zero offset.
 */
pub fn x86_64_regp_emit(inst: &mut Emit, r: u8, regno: u8) {
    match regno {
        X86_64_RSP | X86_64_RBP | X86_64_R12 | X86_64_R13 => {
            x86_64_membase_emit((inst), (r), (regno), 0);
        }
        _ => {
            x86_address_byte((inst), 0, ((r) & 0x7), ((regno) & 0x7));
        }
    }
}

/*
 * Helper to encode an opcode where the encoding is different between
 * 8bit and 16 ... 64 bit width in the following way:
 * 8 bit == opcode given
 * 16 ... 64 bit = opcode given | 0x1
 */
pub fn x86_64_opcode1_emit(inst: &mut Emit, opc: u8, size: i32) {
    match size {
        1 => {
            inst.push((opc));
        }
        2 | 4 | 8 => {
            inst.push(((opc) | 0x1));
        }
        _ => jit_assert!()
    }
}

/*
 * Macros to implement the simple opcodes.
 */
pub fn x86_64_alu_reg_reg_size(inst: &mut Emit, opc: u8, dreg: u8, sreg: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (dreg), 0, (sreg));
            inst.push((((opc)) << 3) + 2);
            x86_64_reg_emit((inst), (dreg), (sreg));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (dreg), 0, (sreg));
            inst.push((((opc)) << 3) + 3);
            x86_64_reg_emit((inst), (dreg), (sreg));
        }
        _ => {}
    }
}

pub fn x86_64_alu_regp_reg_size(inst: &mut Emit, opc: u8, dregp: u8, sreg: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (sreg), 0, (dregp));
            inst.push((((opc)) << 3));
            x86_64_regp_emit((inst), (sreg), (dregp));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (sreg), 0, (dregp));
            inst.push((((opc)) << 3) + 1);
            x86_64_regp_emit((inst), (sreg), (dregp));
        }
        _ => {}
    }
}

pub fn x86_64_alu_mem_reg_size(inst: &mut Emit, opc: u8, mem: i32, sreg: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (sreg), 0, 0);
            inst.push((((opc)) << 3));
            x86_64_mem_emit((inst), (sreg), (mem));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (sreg), 0, 0);
            inst.push((((opc)) << 3) + 1);
            x86_64_mem_emit((inst), (sreg), (mem));
        }
        _ => {}
    }
}

pub fn x86_64_alu_membase_reg_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, sreg: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (sreg), 0, (basereg));
            inst.push((((opc)) << 3));
            x86_64_membase_emit((inst), (sreg), (basereg), (disp));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (sreg), 0, (basereg));
            inst.push((((opc)) << 3) + 1);
            x86_64_membase_emit((inst), (sreg), (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_alu_memindex_reg_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (sreg), (indexreg), (basereg));
            inst.push((((opc)) << 3));
            x86_64_memindex_emit((inst), (sreg), (basereg), (disp), (indexreg), (shift));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (sreg), (indexreg), (basereg));
            inst.push((((opc)) << 3) + 1);
            x86_64_memindex_emit((inst), (sreg), (basereg), (disp), (indexreg), (shift));
        }
        _ => {}
    }
}

pub fn x86_64_alu_reg_regp_size(inst: &mut Emit, opc: u8, dreg: u8, sregp: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (dreg), 0, (sregp));
            inst.push((((opc)) << 3) + 2);
            x86_64_regp_emit((inst), (dreg), (sregp));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (dreg), 0, (sregp));
            inst.push((((opc)) << 3) + 3);
            x86_64_regp_emit((inst), (dreg), (sregp));
        }
        _ => {}
    }
}

pub fn x86_64_alu_reg_mem_size(inst: &mut Emit, opc: u8, dreg: u8, mem: i32, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (dreg), 0, 0);
            inst.push((((opc)) << 3) + 2);
            x86_64_mem_emit((inst), (dreg), (mem));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (dreg), 0, 0);
            inst.push((((opc)) << 3) + 3);
            x86_64_mem_emit((inst), (dreg), (mem));
        }
        _ => {}
    }
}

pub fn x86_64_alu_reg_membase_size(inst: &mut Emit, opc: u8, dreg: u8, basereg: u8, disp: i32, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (dreg), 0, (basereg));
            inst.push((((opc)) << 3) + 2);
            x86_64_membase_emit((inst), (dreg), (basereg), (disp));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (dreg), 0, (basereg));
            inst.push((((opc)) << 3) + 3);
            x86_64_membase_emit((inst), (dreg), (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_alu_reg_memindex_size(inst: &mut Emit, opc: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    match size {
        1 => {
            x86_64_rex_emit(inst, size, (dreg), (indexreg), (basereg));
            inst.push((((opc)) << 3) + 2);
            x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
        }
        2 | 4 | 8 => {
            if size == 2 {
                inst.push(0x66);
            }
            x86_64_rex_emit(inst, size, (dreg), (indexreg), (basereg));
            inst.push((((opc)) << 3) + 3);
            x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
        }
        _ => {}
    }
}

/*
 * The immediate value has to be at most 32 bit wide.
 */
pub fn x86_64_alu_reg_imm_size(inst: &mut Emit, opc: u8, dreg: u8, imm: i32, size: i32) {
    if((dreg) == X86_64_RAX) {
        match size {
            1 => {
                inst.push((((opc)) << 3) + 4);
                x86_imm_emit8((inst), (imm));
            }
            2 => {
                inst.push(0x66);
                inst.push((((opc)) << 3) + 5);
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push((((opc)) << 3) + 5);
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    } else if(x86_is_imm8((imm))) {
        match size {
            1 => {
                x86_64_rex_emit(inst, size, 0, 0, (dreg));
                inst.push(0x80);
            }
            2 | 4 | 8 => {
                if size == 2 {
                    inst.push(0x66);
                }
                x86_64_rex_emit(inst, size, 0, 0, (dreg));
                inst.push(0x83);
            }
            _ => {}
        }
        x86_64_reg_emit((inst), (opc), (dreg));
        x86_imm_emit8((inst), (imm));
    } else {
        match size {
            1 => {
                x86_64_rex_emit(inst, size, 0, 0, (dreg));
                inst.push(0x80);
                x86_64_reg_emit((inst), (opc), (dreg));
                x86_imm_emit8((inst), (imm));
//                jit_assert!(1);
            }
            2 => {
                inst.push(0x66);
                x86_64_rex_emit(inst, size, 0, 0, (dreg));
                inst.push(0x81);
                x86_64_reg_emit((inst), (opc), (dreg));
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit(inst, size, 0, 0, (dreg));
                inst.push(0x81);
                x86_64_reg_emit((inst), (opc), (dreg));
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_alu_regp_imm_size(inst: &mut Emit, opc: u8, reg: u8, imm: i32, size: i32) {
    if(x86_is_imm8((imm))) {
        match size {
            1 => {
                x86_64_rex_emit(inst, size, 0, 0, (reg));
                inst.push(0x80);
            }
            2 | 4 | 8 => {
                if size == 2 {
                    inst.push(0x66);
                }
                x86_64_rex_emit(inst, size, 0, 0, (reg));
                inst.push(0x83);
            }
            _ => {}
        }
        x86_64_regp_emit((inst), (opc), (reg));
        x86_imm_emit8((inst), (imm));
    } else {
        match size {
            1 => {
                x86_64_rex_emit(inst, size, 0, 0, (reg));
                inst.push(0x80);
                x86_64_regp_emit((inst), (opc), (reg));
                x86_imm_emit8((inst), (imm));
//                jit_assert!(1);
            }
            2 => {
                inst.push(0x66);
                x86_64_rex_emit(inst, size, 0, 0, (reg));
                inst.push(0x81);
                x86_64_regp_emit((inst), (opc), (reg));
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit(inst, size, 0, 0, (reg));
                inst.push(0x81);
                x86_64_regp_emit((inst), (opc), (reg));
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_alu_mem_imm_size(inst: &mut Emit, opc: u8, mem: i32, imm: i32, size: i32) {
    if (x86_is_imm8((imm))) {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push(0x80);
            }
            2 | 4 | 8 => {
                if size == 2 {
                    inst.push(0x66);
                }
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push(0x83);
            }
            _ => {}
        }
        x86_64_mem_emit((inst), (opc), (mem));
        x86_imm_emit8((inst), (imm));
    } else {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push(0x80);
                x86_64_mem_emit((inst), (opc), (mem));
                x86_imm_emit8((inst), (imm));
//                jit_assert!(1);
            }
            2 => {
                inst.push(0x66);
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push(0x81);
                x86_64_mem_emit((inst), (opc), (mem));
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit((inst), (size), 0, 0, 0);
                inst.push(0x81);
                x86_64_mem_emit((inst), (opc), (mem));
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_alu_membase_imm_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, imm: i32, size: i32) {
    if (x86_is_imm8((imm))) {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, 0, (basereg));
                inst.push(0x80);
            }
            2 | 4 | 8 => {
                if size == 2 {
                    inst.push(0x66);
                }
                x86_64_rex_emit((inst), (size), 0, 0, (basereg));
                inst.push(0x83);
            }
            _ => {}
        }
        x86_64_membase_emit((inst), (opc), (basereg), (disp));
        x86_imm_emit8((inst), (imm));
    } else {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, 0, (basereg));
                inst.push(0x80);
                x86_64_membase_emit((inst), (opc), (basereg), (disp));
                x86_imm_emit8((inst), (imm));
//                jit_assert!(1);
            }
            2 => {
                inst.push(0x66);
                x86_64_rex_emit((inst), (size), 0, 0, (basereg));
                inst.push(0x81);
                x86_64_membase_emit((inst), (opc), (basereg), (disp));
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit((inst), (size), 0, 0, (basereg));
                inst.push(0x81);
                x86_64_membase_emit((inst), (opc), (basereg), (disp));
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_alu_memindex_imm_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if (x86_is_imm8((imm))) {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
                inst.push(0x80);
            }
            2 | 4 | 8 => {
                if size == 2 {
                    inst.push(0x66);
                }
                x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
                inst.push(0x83);
            }
            _ => {}
        }
        x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
        x86_imm_emit8((inst), (imm));
    } else {
        match size {
            1 => {
                x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
                inst.push(0x80);
                x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
                x86_imm_emit8((inst), (imm));
//                jit_assert!(1);
            }
            2 => {
                inst.push(0x66);
                x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
                inst.push(0x81);
                x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
                x86_imm_emit16((inst), (imm));
            }
            4 | 8 => {
                x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
                inst.push(0x81);
                x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
                x86_imm_emit32((inst), (imm));
            }
            _ => {}
        }
    }
}

/*
 * Instructions with one opcode (plus optional r/m)
 */

/*
 * Unary opcodes
 */
pub fn x86_64_alu1_reg(inst: &mut Emit, opc1: u8, r: u8, reg: u8) {
    x86_64_rex_emit((inst), 0, 0, 0, (reg));
    inst.push((opc1));
    x86_64_reg_emit((inst), (r), (reg));
}

pub fn x86_64_alu1_regp(inst: &mut Emit, opc1: u8, r: u8, regp: u8) {
    x86_64_rex_emit((inst), 0, 0, 0, (regp));
    inst.push((opc1));
    x86_64_regp_emit((inst), (r), (regp));
}

pub fn x86_64_alu1_mem(inst: &mut Emit, opc1: u8, r: u8, mem: i32) {
    inst.push((opc1));
    x86_64_mem_emit((inst), (r), (mem));
}

pub fn x86_64_alu1_membase(inst: &mut Emit, opc1: u8, r: u8, basereg: u8, disp: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    inst.push((opc1));
    x86_64_membase_emit((inst), (r), (basereg), (disp));
}

pub fn x86_64_alu1_memindex(inst: &mut Emit, opc1: u8, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_rex_emit((inst), 0, 0, (indexreg), (basereg));
    inst.push((opc1));
    x86_64_memindex_emit((inst), (r), (basereg), (disp), (indexreg), (shift));
}

pub fn x86_64_alu1_reg_size(inst: &mut Emit, opc1: u8, r: u8, reg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (reg));
    x86_64_opcode1_emit((inst), (opc1), (size));
    x86_64_reg_emit((inst), (r), (reg));
}

pub fn x86_64_alu1_regp_size(inst: &mut Emit, opc1: u8, r: u8, regp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (regp));
    x86_64_opcode1_emit((inst), (opc1), (size));
    x86_64_regp_emit((inst), (r), (regp));
}

pub fn x86_64_alu1_mem_size(inst: &mut Emit, opc1: u8, r: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, 0);
    x86_64_opcode1_emit((inst), (opc1), (size));
    x86_64_mem_emit((inst), (r), (mem));
}

pub fn x86_64_alu1_membase_size(inst: &mut Emit, opc1: u8, r: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (basereg));
    x86_64_opcode1_emit((inst), (opc1), (size));
    x86_64_membase_emit((inst), (r), (basereg), (disp));
}

pub fn x86_64_alu1_memindex_size(inst: &mut Emit, opc1: u8, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
    x86_64_opcode1_emit((inst), (opc1), (size));
    x86_64_memindex_emit((inst), (r), (basereg), (disp), (indexreg), (shift));
}

pub fn x86_64_alu1_reg_reg_size(inst: &mut Emit, opc1: u8, dreg: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sreg));
    inst.push((opc1));
    x86_64_reg_emit((inst), (dreg), (sreg));
}

pub fn x86_64_alu1_reg_regp_size(inst: &mut Emit, opc1: u8, dreg: u8, sregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sregp));
    inst.push((opc1));
    x86_64_regp_emit((inst), (dreg), (sregp));
}

pub fn x86_64_alu1_reg_mem_size(inst: &mut Emit, opc1: u8, dreg: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, 0);
    inst.push((opc1));
    x86_64_mem_emit((inst), (dreg), (mem));
}

pub fn x86_64_alu1_reg_membase_size(inst: &mut Emit, opc1: u8, dreg: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (basereg));
    inst.push((opc1));
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}

pub fn x86_64_alu1_reg_memindex_size(inst: &mut Emit, opc1: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    inst.push((opc1));
    x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

pub fn x86_64_alu2_reg_reg_size(inst: &mut Emit, opc1: u8, opc2: u8, dreg: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sreg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_reg_emit((inst), (dreg), (sreg));
}

pub fn x86_64_alu2_reg_regp_size(inst: &mut Emit, opc1: u8, opc2: u8, dreg: u8, sregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sregp));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_regp_emit((inst), (dreg), (sregp));
}

pub fn x86_64_alu2_reg_mem_size(inst: &mut Emit, opc1: u8, opc2: u8, dreg: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, 0);
    inst.push((opc1));
    inst.push((opc2));
    x86_64_mem_emit((inst), (dreg), (mem));
}

pub fn x86_64_alu2_reg_membase_size(inst: &mut Emit, opc1: u8, opc2: u8, dreg: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}

pub fn x86_64_alu2_reg_memindex_size(inst: &mut Emit, opc1: u8, opc2: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Group1 general instructions
 */
pub fn x86_64_alu_reg_reg(inst: &mut Emit, opc: u8, dreg: u8, sreg: u8) {
    x86_64_alu_reg_reg_size((inst), (opc), (dreg), (sreg), 8);
}

pub fn x86_64_alu_reg_imm(inst: &mut Emit, opc: u8, dreg: u8, imm: i32) {
    x86_64_alu_reg_imm_size((inst), (opc), (dreg), (imm), 8);
}

/*
 * ADC: Add with carry
 */
pub fn x86_64_adc_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 2, (dreg), (sreg), (size));
}

pub fn x86_64_adc_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 2, (dregp), (sreg), (size));
}

pub fn x86_64_adc_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 2, (mem), (sreg), (size));
}

pub fn x86_64_adc_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 2, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_adc_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 2, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_adc_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 2, (dreg), (sregp), (size));
}

pub fn x86_64_adc_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 2, (dreg), (mem), (size));
}

pub fn x86_64_adc_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 2, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_adc_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 2, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_adc_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 2, (dreg), (imm), (size));
}

pub fn x86_64_adc_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 2, (reg), (imm), (size));
}

pub fn x86_64_adc_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 2, mem, imm, size);
}

pub fn x86_64_adc_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 2, (basereg), (disp), (imm), (size));
}

pub fn x86_64_adc_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 2, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * ADD
 */
pub fn x86_64_add_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 0, (dreg), (sreg), (size));
}

pub fn x86_64_add_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 0, (dregp), (sreg), (size));
}

pub fn x86_64_add_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 0, (mem), (sreg), (size));
}

pub fn x86_64_add_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 0, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_add_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 0, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_add_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 0, (dreg), (sregp), (size));
}

pub fn x86_64_add_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 0, (dreg), (mem), (size));
}

pub fn x86_64_add_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 0, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_add_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 0, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_add_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 0, (dreg), (imm), (size));
}

pub fn x86_64_add_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 0, (reg), (imm), (size));
}

pub fn x86_64_add_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 0, mem, imm, size);
}

pub fn x86_64_add_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 0, (basereg), (disp), (imm), (size));
}

pub fn x86_64_add_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 0, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * AND
 */
pub fn x86_64_and_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 4, (dreg), (sreg), (size));
}

pub fn x86_64_and_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 4, (dregp), (sreg), (size));
}

pub fn x86_64_and_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 4, (mem), (sreg), (size));
}

pub fn x86_64_and_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 4, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_and_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 4, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_and_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 4, (dreg), (sregp), (size));
}

pub fn x86_64_and_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 4, (dreg), (mem), (size));
}

pub fn x86_64_and_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 4, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_and_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 4, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_and_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 4, (dreg), (imm), (size));
}

pub fn x86_64_and_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 4, (reg), (imm), (size));
}

pub fn x86_64_and_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 4, mem, imm, size);
}

pub fn x86_64_and_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 4, (basereg), (disp), (imm), (size));
}

pub fn x86_64_and_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 4, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * CMP: compare
 */
pub fn x86_64_cmp_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 7, (dreg), (sreg), (size));
}

pub fn x86_64_cmp_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 7, (dregp), (sreg), (size));
}

pub fn x86_64_cmp_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 7, (mem), (sreg), (size));
}

pub fn x86_64_cmp_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 7, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_cmp_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 7, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_cmp_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 7, (dreg), (sregp), (size));
}

pub fn x86_64_cmp_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 7, (dreg), (mem), (size));
}

pub fn x86_64_cmp_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 7, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cmp_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 7, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_cmp_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 7, (dreg), (imm), (size));
}

pub fn x86_64_cmp_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 7, (reg), (imm), (size));
}

pub fn x86_64_cmp_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 7, mem, imm, size);
}

pub fn x86_64_cmp_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 7, (basereg), (disp), (imm), (size));
}

pub fn x86_64_cmp_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 7, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * OR
 */
pub fn x86_64_or_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 1, (dreg), (sreg), (size));
}

pub fn x86_64_or_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 1, (dregp), (sreg), (size));
}

pub fn x86_64_or_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 1, (mem), (sreg), (size));
}

pub fn x86_64_or_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 1, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_or_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 1, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_or_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 1, (dreg), (sregp), (size));
}

pub fn x86_64_or_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 1, (dreg), (mem), (size));
}

pub fn x86_64_or_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 1, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_or_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 1, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_or_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 1, (dreg), (imm), (size));
}

pub fn x86_64_or_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 1, (reg), (imm), (size));
}

pub fn x86_64_or_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 1, mem, imm, size);
}

pub fn x86_64_or_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 1, (basereg), (disp), (imm), (size));
}

pub fn x86_64_or_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 1, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * SBB: Subtract with borrow from al
 */
pub fn x86_64_sbb_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 3, (dreg), (sreg), (size));
}

pub fn x86_64_sbb_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 3, (dregp), (sreg), (size));
}

pub fn x86_64_sbb_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 3, (mem), (sreg), (size));
}

pub fn x86_64_sbb_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 3, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_sbb_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 3, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_sbb_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 3, (dreg), (sregp), (size));
}

pub fn x86_64_sbb_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 3, (dreg), (mem), (size));
}

pub fn x86_64_sbb_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 3, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_sbb_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 3, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_sbb_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 3, (dreg), (imm), (size));
}

pub fn x86_64_sbb_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 3, (reg), (imm), (size));
}

pub fn x86_64_sbb_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 3, mem, imm, size);
}

pub fn x86_64_sbb_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 3, (basereg), (disp), (imm), (size));
}

pub fn x86_64_sbb_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 3, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * SUB: Subtract
 */
pub fn x86_64_sub_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 5, (dreg), (sreg), (size));
}

pub fn x86_64_sub_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 5, (dregp), (sreg), (size));
}

pub fn x86_64_sub_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 5, (mem), (sreg), (size));
}

pub fn x86_64_sub_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 5, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_sub_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 5, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_sub_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 5, (dreg), (sregp), (size));
}

pub fn x86_64_sub_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 5, (dreg), (mem), (size));
}

pub fn x86_64_sub_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 5, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_sub_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 5, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_sub_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 5, (dreg), (imm), (size));
}

pub fn x86_64_sub_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 5, (reg), (imm), (size));
}

pub fn x86_64_sub_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 5, mem, imm, size);
}

pub fn x86_64_sub_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 5, (basereg), (disp), (imm), (size));
}

pub fn x86_64_sub_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 5, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * XOR
 */
pub fn x86_64_xor_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu_reg_reg_size((inst), 6, (dreg), (sreg), (size));
}

pub fn x86_64_xor_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    x86_64_alu_regp_reg_size((inst), 6, (dregp), (sreg), (size));
}

pub fn x86_64_xor_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    x86_64_alu_mem_reg_size((inst), 6, (mem), (sreg), (size));
}

pub fn x86_64_xor_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    x86_64_alu_membase_reg_size((inst), 6, (basereg), (disp), (sreg), (size));
}

pub fn x86_64_xor_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    x86_64_alu_memindex_reg_size((inst), 6, (basereg), (disp), (indexreg), (shift), (sreg), (size));
}

pub fn x86_64_xor_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu_reg_regp_size((inst), 6, (dreg), (sregp), (size));
}

pub fn x86_64_xor_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu_reg_mem_size((inst), 6, (dreg), (mem), (size));
}

pub fn x86_64_xor_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu_reg_membase_size((inst), 6, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_xor_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu_reg_memindex_size((inst), 6, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_xor_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_alu_reg_imm_size((inst), 6, (dreg), (imm), (size));
}

pub fn x86_64_xor_regp_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    x86_64_alu_regp_imm_size((inst), 6, (reg), (imm), (size));
}

pub fn x86_64_xor_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_alu_mem_imm_size(inst, 6, mem, imm, size);
}

pub fn x86_64_xor_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_alu_membase_imm_size((inst), 6, (basereg), (disp), (imm), (size));
}

pub fn x86_64_xor_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_alu_memindex_imm_size((inst), 6, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

/*
 * dec
 */
pub fn x86_64_dec_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xfe, 1, (reg), (size));
}

pub fn x86_64_dec_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xfe, 1, (regp), (size));
}

pub fn x86_64_dec_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xfe, 1, (mem), (size));
}

pub fn x86_64_dec_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xfe, 1, (basereg), (disp), (size));
}

pub fn x86_64_dec_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xfe, 1, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * div: unsigned division RDX:RAX / operand
 */
pub fn x86_64_div_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xf6, 6, (reg), (size));
}

pub fn x86_64_div_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xf6, 6, (regp), (size));
}

pub fn x86_64_div_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xf6, 6, (mem), (size));
}

pub fn x86_64_div_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xf6, 6, (basereg), (disp), (size));
}

pub fn x86_64_div_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xf6, 6, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * idiv: signed division RDX:RAX / operand
 */
pub fn x86_64_idiv_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xf6, 7, (reg), (size));
}

pub fn x86_64_idiv_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xf6, 7, (regp), (size));
}

pub fn x86_64_idiv_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xf6, 7, (mem), (size));
}

pub fn x86_64_idiv_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xf6, 7, (basereg), (disp), (size));
}

pub fn x86_64_idiv_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xf6, 7, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * inc
 */
pub fn x86_64_inc_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xfe, 0, (reg), (size));
}

pub fn x86_64_inc_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xfe, 0, (regp), (size));
}

pub fn x86_64_inc_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xfe, 0, (mem), (size));
}

pub fn x86_64_inc_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xfe, 0, (basereg), (disp), (size));
}

pub fn x86_64_inc_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xfe, 0, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * mul: multiply RDX:RAX = RAX * operand
 * is_signed == 0 -> unsigned multiplication
 * signed multiplication otherwise.
 */
pub fn x86_64_mul_reg_issigned_size(inst: &mut Emit, reg: u8, is_signed: bool, size: i32) {
    x86_64_alu1_reg_size((inst), 0xf6, if is_signed { 5 } else { 4 }, (reg), (size));
}

pub fn x86_64_mul_regp_issigned_size(inst: &mut Emit, regp: u8, is_signed: bool, size: i32) {
    x86_64_alu1_regp_size((inst), 0xf6, if is_signed { 5 } else { 4 }, (regp), (size));
}

pub fn x86_64_mul_mem_issigned_size(inst: &mut Emit, mem: i32, is_signed: bool, size: i32) {
    x86_64_alu1_mem_size((inst), 0xf6, if is_signed { 5 } else { 4 }, (mem), (size));
}

pub fn x86_64_mul_membase_issigned_size(inst: &mut Emit, basereg: u8, disp: i32, is_signed: bool, size: i32) {
    x86_64_alu1_membase_size((inst), 0xf6, if is_signed { 5 } else { 4 }, (basereg), (disp), (size));
}

pub fn x86_64_mul_memindex_issigned_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_signed: bool, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xf6, if is_signed { 5 } else { 4 }, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * neg 
 */
pub fn x86_64_neg_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xf6, 3, (reg), (size));
}

pub fn x86_64_neg_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xf6, 3, (regp), (size));
}

pub fn x86_64_neg_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xf6, 3, (mem), (size));
}

pub fn x86_64_neg_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xf6, 3, (basereg), (disp), (size));
}

pub fn x86_64_neg_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xf6, 3, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * not
 */
pub fn x86_64_not_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    x86_64_alu1_reg_size((inst), 0xf6, 2, (reg), (size));
}

pub fn x86_64_not_regp_size(inst: &mut Emit, regp: u8, size: i32) {
    x86_64_alu1_regp_size((inst), 0xf6, 2, (regp), (size));
}

pub fn x86_64_not_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_alu1_mem_size((inst), 0xf6, 2, (mem), (size));
}

pub fn x86_64_not_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_membase_size((inst), 0xf6, 2, (basereg), (disp), (size));
}

pub fn x86_64_not_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_memindex_size((inst), 0xf6, 2, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * Note: x86_64_clear_reg () changes the condition code!
 */
pub fn x86_64_clear_reg(inst: &mut Emit, reg: u8) {
    x86_64_xor_reg_reg_size((inst), (reg), (reg), 4)
}

/*
 * shift instructions
 */
pub fn x86_64_shift_reg_imm_size(inst: &mut Emit, opc: u8, dreg: u8, imm: i32, size: i32) {
    if ((imm) == 1) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (dreg));
        x86_64_opcode1_emit((inst), 0xd0, (size));
        x86_64_reg_emit((inst), (opc), (dreg));
    } else {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (dreg));
        x86_64_opcode1_emit((inst), 0xc0, (size));
        x86_64_reg_emit((inst), (opc), (dreg));
        x86_imm_emit8((inst), (imm));
    }
}

pub fn x86_64_shift_mem_imm_size(inst: &mut Emit, opc: u8, mem: i32, imm: i32, size: i32) {
    if ((imm) == 1) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, 0);
        x86_64_opcode1_emit((inst), 0xd0, (size));
        x86_64_mem_emit((inst), (opc), (mem));
    } else {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, 0);
        x86_64_opcode1_emit((inst), 0xc0, (size));
        x86_64_mem_emit((inst), (opc), (mem));
        x86_imm_emit8((inst), (imm));
    }
}

pub fn x86_64_shift_regp_imm_size(inst: &mut Emit, opc: u8, dregp: u8, imm: i32, size: i32) {
    if ((imm) == 1) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (dregp));
        x86_64_opcode1_emit((inst), 0xd0, (size));
        x86_64_regp_emit((inst), (opc), (dregp));
    } else {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (dregp));
        x86_64_opcode1_emit((inst), 0xc0, (size));
        x86_64_regp_emit((inst), (opc), (dregp));
        x86_imm_emit8((inst), (imm));
    }
}

pub fn x86_64_shift_membase_imm_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, imm: i32, size: i32) {
    if ((imm) == 1) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (basereg));
        x86_64_opcode1_emit((inst), 0xd0, (size));
        x86_64_membase_emit((inst), (opc), (basereg), (disp));
    } else {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, 0, (basereg));
        x86_64_opcode1_emit((inst), 0xc0, (size));
        x86_64_membase_emit((inst), (opc), (basereg), (disp));
        x86_imm_emit8((inst), (imm));
    }
}

pub fn x86_64_shift_memindex_imm_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if ((imm) == 1) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
        x86_64_opcode1_emit((inst), 0xd0, (size));
        x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
    } else {
        if ((size) == 2) {
            inst.push(0x66);
        }
        x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
        x86_64_opcode1_emit((inst), 0xc0, (size));
        x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
        x86_imm_emit8((inst), (imm));
    }
}

/*
 * shift by the number of bits in %cl
 */
pub fn x86_64_shift_reg_size(inst: &mut Emit, opc: u8, dreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (dreg));
    x86_64_opcode1_emit((inst), 0xd2, (size));
    x86_64_reg_emit((inst), (opc), (dreg));
}

pub fn x86_64_shift_mem_size(inst: &mut Emit, opc: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, 0);
    x86_64_opcode1_emit((inst), 0xd2, (size));
    x86_64_mem_emit((inst), (opc), (mem));
}

pub fn x86_64_shift_regp_size(inst: &mut Emit, opc: u8, dregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (dregp));
    x86_64_opcode1_emit((inst), 0xd2, (size));
    x86_64_regp_emit((inst), (opc), (dregp));
}

pub fn x86_64_shift_membase_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (basereg));
    x86_64_opcode1_emit((inst), 0xd2, (size));
    x86_64_membase_emit((inst), (opc), (basereg), (disp));
}

pub fn x86_64_shift_memindex_size(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
    x86_64_opcode1_emit((inst), 0xd2, (size));
    x86_64_memindex_emit((inst), (opc), (basereg), (disp), (indexreg), (shift));
}

/*
 * shl: Shit left (clear the least significant bit)
 */
pub fn x86_64_shl_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_shift_reg_imm_size((inst), 4, (dreg), (imm), (size));
}

pub fn x86_64_shl_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_shift_mem_imm_size((inst), 4, (mem), (imm), (size));
}

pub fn x86_64_shl_regp_imm_size(inst: &mut Emit, dregp: u8, imm: i32, size: i32) {
    x86_64_shift_regp_imm_size((inst), 4, (dregp), (imm), (size));
}

pub fn x86_64_shl_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_shift_membase_imm_size((inst), 4, (basereg), (disp), (imm), (size));
}

pub fn x86_64_shl_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_shift_memindex_imm_size((inst), 4, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

pub fn x86_64_shl_reg_size(inst: &mut Emit, dreg: u8, size: i32) {
    x86_64_shift_reg_size((inst), 4, (dreg), (size));
}

pub fn x86_64_shl_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_shift_mem_size((inst), 4, (mem), (size));
}

pub fn x86_64_shl_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    x86_64_shift_regp_size((inst), 4, (dregp), (size));
}

pub fn x86_64_shl_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_shift_membase_size((inst), 4, (basereg), (disp), (size));
}

pub fn x86_64_shl_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_shift_memindex_size((inst), 4, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * shr: Unsigned shit right (clear the most significant bit)
 */
pub fn x86_64_shr_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_shift_reg_imm_size((inst), 5, (dreg), (imm), (size));
}

pub fn x86_64_shr_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_shift_mem_imm_size((inst), 5, (mem), (imm), (size));
}

pub fn x86_64_shr_regp_imm_size(inst: &mut Emit, dregp: u8, imm: i32, size: i32) {
    x86_64_shift_regp_imm_size((inst), 5, (dregp), (imm), (size));
}

pub fn x86_64_shr_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_shift_membase_imm_size((inst), 5, (basereg), (disp), (imm), (size));
}

pub fn x86_64_shr_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_shift_memindex_imm_size((inst), 5, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

pub fn x86_64_shr_reg_size(inst: &mut Emit, dreg: u8, size: i32) {
    x86_64_shift_reg_size((inst), 5, (dreg), (size));
}

pub fn x86_64_shr_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_shift_mem_size((inst), 5, (mem), (size));
}

pub fn x86_64_shr_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    x86_64_shift_regp_size((inst), 5, (dregp), (size));
}

pub fn x86_64_shr_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_shift_membase_size((inst), 5, (basereg), (disp), (size));
}

pub fn x86_64_shr_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_shift_memindex_size((inst), 5, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * sar: Signed shit right (keep the most significant bit)
 */
pub fn x86_64_sar_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i32, size: i32) {
    x86_64_shift_reg_imm_size((inst), 7, (dreg), (imm), (size));
}

pub fn x86_64_sar_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    x86_64_shift_mem_imm_size((inst), 7, (mem), (imm), (size));
}

pub fn x86_64_sar_regp_imm_size(inst: &mut Emit, dregp: u8, imm: i32, size: i32) {
    x86_64_shift_regp_imm_size((inst), 7, (dregp), (imm), (size));
}

pub fn x86_64_sar_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    x86_64_shift_membase_imm_size((inst), 7, (basereg), (disp), (imm), (size));
}

pub fn x86_64_sar_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    x86_64_shift_memindex_imm_size((inst), 7, (basereg), (disp), (indexreg), (shift), (imm), (size));
}

pub fn x86_64_sar_reg_size(inst: &mut Emit, dreg: u8, size: i32) {
    x86_64_shift_reg_size((inst), 7, (dreg), (size));
}

pub fn x86_64_sar_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    x86_64_shift_mem_size((inst), 7, (mem), (size));
}

pub fn x86_64_sar_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    x86_64_shift_regp_size((inst), 7, (dregp), (size));
}

pub fn x86_64_sar_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_shift_membase_size((inst), 7, (basereg), (disp), (size));
}

pub fn x86_64_sar_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_shift_memindex_size((inst), 7, (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * test: and tha values and set sf, zf and pf according to the result
 */
pub fn x86_64_test_reg_imm_size(inst: &mut Emit, reg: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (reg));
    if((reg) == X86_64_RAX) {
        x86_64_opcode1_emit((inst), 0xa8, (size));
    } else {
        x86_64_opcode1_emit((inst), 0xf6, (size));
        x86_64_reg_emit((inst), 0, (reg));
    }
    x86_64_imm_emit_max32((inst), (imm), (size));
}

pub fn x86_64_test_regp_imm_size(inst: &mut Emit, regp: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (regp));
    x86_64_opcode1_emit((inst), 0xf6, (size));
    x86_64_regp_emit((inst), 0, (regp));
    x86_64_imm_emit_max32((inst), (imm), (size));
}

pub fn x86_64_test_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, 0);
    x86_64_opcode1_emit((inst), 0xf6, (size));
    x86_64_mem_emit((inst), 0, (mem));
    x86_64_imm_emit_max32((inst), (imm), (size));
}

pub fn x86_64_test_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (basereg));
    x86_64_opcode1_emit((inst), 0xf6, (size));
    x86_64_membase_emit((inst), 0, (basereg), (disp));
    x86_64_imm_emit_max32((inst), (imm), (size));
}

pub fn x86_64_test_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
    x86_64_opcode1_emit((inst), 0xf6, (size));
    x86_64_memindex_emit((inst), 0, (basereg), (disp), (indexreg), (shift));
    x86_64_imm_emit_max32((inst), (imm), (size));
}

pub fn x86_64_test_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), 0, (dreg));
    x86_64_opcode1_emit((inst), 0x84, (size));
    x86_64_reg_emit((inst), (sreg), (dreg));
}

pub fn x86_64_test_regp_reg_size(inst: &mut Emit, dregp: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), 0, (dregp));
    x86_64_opcode1_emit((inst), 0x84, (size));
    x86_64_regp_emit((inst), (sreg), (dregp));
}

pub fn x86_64_test_mem_reg_size(inst: &mut Emit, mem: i32, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), 0, 0);
    x86_64_opcode1_emit((inst), 0x84, (size));
    x86_64_mem_emit((inst), (sreg), (mem));
}

pub fn x86_64_test_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), 0, (basereg));
    x86_64_opcode1_emit((inst), 0x84, (size));
    x86_64_membase_emit((inst), (sreg), (basereg), (disp));
}

pub fn x86_64_test_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), (indexreg), (basereg));
    x86_64_opcode1_emit((inst), 0x84, (size));
    x86_64_memindex_emit((inst), (sreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * imul: signed multiply
 */
pub fn x86_64_imul_reg_reg_imm_size(inst: &mut Emit, dreg: u8, sreg: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sreg));
    if (x86_is_imm8((imm))) {
        inst.push(0x6b);
        x86_64_reg_emit((inst), (dreg), (sreg));
        x86_imm_emit8((inst), (imm));
    } else {
        inst.push(0x69);
        x86_64_reg_emit((inst), (dreg), (sreg));
        match size {
            2 => {
                x86_imm_emit16(inst, (imm));
            }
            4 | 8 => {
                x86_imm_emit32(inst, (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_imul_reg_regp_imm_size(inst: &mut Emit, dreg: u8, sregp: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sregp));
    if (x86_is_imm8((imm))) {
        inst.push(0x6b);
        x86_64_regp_emit((inst), (dreg), (sregp));
        x86_imm_emit8((inst), (imm));
    } else {
        inst.push(0x69);
        x86_64_regp_emit((inst), (dreg), (sregp));
        match size {
            2 => {
                x86_imm_emit16(inst, (imm));
            }
            4 | 8 => {
                x86_imm_emit32(inst, (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_imul_reg_mem_imm_size(inst: &mut Emit, dreg: u8, mem: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, 0);
    if (x86_is_imm8((imm))) {
        inst.push(0x6b);
        x86_64_mem_emit((inst), (dreg), (mem));
        x86_imm_emit8((inst), (imm));
    } else {
        inst.push(0x69);
        x86_64_mem_emit((inst), (dreg), (mem));
        match size {
            2 => {
                x86_imm_emit16(inst, (imm));
            }
            4 | 8 => {
                x86_imm_emit32(inst, (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_imul_reg_membase_imm_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (basereg));
    if (x86_is_imm8((imm))) {
        inst.push(0x6b);
        x86_64_membase_emit((inst), (dreg), (basereg), (disp));
        x86_imm_emit8((inst), (imm));
    } else {
        inst.push(0x69);
        x86_64_membase_emit((inst), (dreg), (basereg), (disp));
        match size {
            2 => {
                x86_imm_emit16(inst, (imm));
            }
            4 | 8 => {
                x86_imm_emit32(inst, (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_imul_reg_memindex_imm_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    if (x86_is_imm8((imm))) {
        inst.push(0x6b);
        x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
        x86_imm_emit8((inst), (imm));
    } else {
        inst.push(0x69);
        x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
        match size {
            2 => {
                x86_imm_emit16(inst, (imm));
            }
            4 | 8 => {
                x86_imm_emit32(inst, (imm));
            }
            _ => {}
        }
    }
}

pub fn x86_64_imul_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sreg));
    inst.push(0x0F);
    inst.push(0xAF);
    x86_64_reg_emit((inst), (dreg), (sreg));
}

pub fn x86_64_imul_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sregp));
    inst.push(0x0F);
    inst.push(0xAF);
    x86_64_regp_emit((inst), (dreg), (sregp));
}

pub fn x86_64_imul_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, 0);
    inst.push(0x0F);
    inst.push(0xAF);
    x86_64_mem_emit((inst), (dreg), (mem));
}

pub fn x86_64_imul_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (basereg));
    inst.push(0x0F);
    inst.push(0xAF);
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}

pub fn x86_64_imul_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    inst.push(0x0F);
    inst.push(0xAF);
    x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * cwd, cdq, cqo: sign extend ax to dx (used for div and idiv)
 */
pub fn x86_64_cwd(inst: &mut Emit) {
    inst.push(0x66);
    inst.push(0x99);
}

pub fn x86_64_cdq(inst: &mut Emit) {
    inst.push(0x99);
}

pub fn x86_64_cqo(inst: &mut Emit) {
    inst.push(0x48);
    inst.push(0x99);
}

/*
 * Lea instructions
 */
pub fn x86_64_lea_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, (dreg));
    x86_lea_mem((inst), ((dreg) & 0x7), (mem));
}

pub fn x86_64_lea_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (dreg), 0, (basereg));
    inst.push(0x8d);
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}

pub fn x86_64_lea_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    inst.push(0x8d);
    x86_64_memindex_emit ((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Move instructions.
 */
pub fn x86_64_mov_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (dreg), 0, (sreg));
    x86_64_opcode1_emit(inst, 0x8a, (size));
    x86_64_reg_emit((inst), ((dreg) & 0x7), ((sreg) & 0x7));
}

pub fn x86_64_mov_regp_reg_size(inst: &mut Emit, regp: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (sreg), 0, (regp));
    x86_64_opcode1_emit(inst, 0x88, (size));
    x86_64_regp_emit((inst), (sreg), (regp));
}

pub fn x86_64_mov_membase_reg_size(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (sreg), 0, (basereg));
    x86_64_opcode1_emit(inst, 0x88, (size));
    x86_64_membase_emit((inst), (sreg), (basereg), (disp));
}

pub fn x86_64_mov_memindex_reg_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (sreg), (indexreg), (basereg));
    x86_64_opcode1_emit(inst, 0x88, (size));
    x86_64_memindex_emit((inst), (sreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Using the AX register is the only possibility to address 64bit.
 * All other registers are bound to 32bit values.
 */
pub fn x86_64_mov_mem_reg_size(inst: &mut Emit, mem: i64, sreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (sreg), 0, 0);
    if ((sreg) == X86_64_RAX) {
        x86_64_opcode1_emit(inst, 0xa2, (size));
        x86_64_imm_emit64(inst, (mem));
    } else {
        x86_64_opcode1_emit(inst, 0x88, (size));
        x86_address_byte((inst), 0, ((sreg) & 0x7), 4);
        x86_address_byte((inst), 0, 4, 5);
        x86_imm_emit32((inst), (mem) as i32);
    }
}

pub fn x86_64_mov_reg_imm_size(inst: &mut Emit, dreg: u8, imm: i64, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), 0, 0, (dreg));
    match size {
        1 => {
            inst.push(0xb0 + ((dreg) & 0x7));
            x86_imm_emit8(inst, (imm) as i32);
        }
        2 => {
            inst.push(0xb8 + ((dreg) & 0x7));
            x86_imm_emit16(inst, (imm) as i32);
        }
        4 => {
            inst.push(0xb8 + ((dreg) & 0x7));
            x86_imm_emit32(inst, (imm) as i32);
        }
        8 => {
            let __x86_64_imm = (imm);
            if (__x86_64_imm >= i32::MIN as i64 && __x86_64_imm <= i32::MAX as i64) {
                inst.push(0xc7);
                x86_64_reg_emit((inst), 0, (dreg));
                x86_imm_emit32(inst, (__x86_64_imm) as i32);
            } else {
                inst.push(0xb8 + ((dreg) & 0x7));
                x86_64_imm_emit64(inst, (__x86_64_imm));
            }
        }
        _ => {}
    }
}

/*
 * Using the AX register is the only possibility to address 64bit.
 * All other registers are bound to 32bit values.
 */
pub fn x86_64_mov_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i64, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (dreg), 0, 0);
    if ((dreg) == X86_64_RAX) {
        x86_64_opcode1_emit(inst, 0xa0, (size));
        x86_64_imm_emit64(inst, (mem));
    } else {
        x86_64_opcode1_emit(inst, 0x8a, (size));
        x86_address_byte ((inst), 0, (dreg), 4);
        x86_address_byte ((inst), 0, 4, 5);
        x86_imm_emit32 ((inst), (mem) as i32);
    }
}

pub fn x86_64_mov_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (dreg), 0, (sregp));
    x86_64_opcode1_emit(inst, 0x8a, (size));
    x86_64_regp_emit((inst), (dreg), (sregp));
}

pub fn x86_64_mov_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), (dreg), 0, (basereg));
    x86_64_opcode1_emit(inst, 0x8a, (size));
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}


pub fn x86_64_mov_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    x86_64_opcode1_emit(inst, 0x8a, (size));
    x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Only 32bit mem and imm values are allowed here.
 * mem is be RIP relative.
 * 32 bit imm will be sign extended to 64 bits for 64 bit size.
 */
pub fn x86_64_mov_mem_imm_size(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, 0, 0);
    x86_64_opcode1_emit(inst, 0xc6, (size));
    x86_64_mem_emit((inst), 0, (mem));
    x86_64_imm_emit_max32(inst, (imm), (size));
}

pub fn x86_64_mov_regp_imm_size(inst: &mut Emit, dregp: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), 0, 0, (dregp));
    x86_64_opcode1_emit(inst, 0xc6, (size));
    x86_64_regp_emit((inst), 0, (dregp));
    x86_64_imm_emit_max32(inst, (imm), (size));
}

pub fn x86_64_mov_membase_imm_size(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit(inst, (size), 0, 0, (basereg));
    x86_64_opcode1_emit(inst, 0xc6, (size));
    x86_64_membase_emit((inst), 0, (basereg), (disp));
    x86_64_imm_emit_max32(inst, (imm), (size));
}

pub fn x86_64_mov_memindex_imm_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), 0, (indexreg), (basereg));
    x86_64_opcode1_emit(inst, 0xc6, (size));
    x86_64_memindex_emit((inst), 0, (basereg), (disp), (indexreg), (shift));
    x86_64_imm_emit_max32(inst, (imm), (size));
}

/*
 * Move with sign extension to the given size (signed)
 */
pub fn x86_64_movsx8_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu2_reg_reg_size((inst), 0x0f, 0xbe, (dreg), (sreg), (size) | 1);
}

pub fn x86_64_movsx8_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu2_reg_regp_size((inst), 0x0f, 0xbe, (dreg), (sregp), (size));
}

pub fn x86_64_movsx8_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu2_reg_mem_size((inst), 0x0f, 0xbe, (dreg), (mem), (size));
}

pub fn x86_64_movsx8_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu2_reg_membase_size((inst), 0x0f, 0xbe, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_movsx8_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu2_reg_memindex_size((inst), 0x0f, 0xbe, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_movsx16_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu2_reg_reg_size((inst), 0x0f, 0xbf, (dreg), (sreg), (size));
}

pub fn x86_64_movsx16_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu2_reg_regp_size((inst), 0x0f, 0xbf, (dreg), (sregp), (size));
}

pub fn x86_64_movsx16_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu2_reg_mem_size((inst), 0x0f, 0xbf, (dreg), (mem), (size));
}

pub fn x86_64_movsx16_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu2_reg_membase_size((inst), 0x0f, 0xbf, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_movsx16_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu2_reg_memindex_size((inst), 0x0f, 0xbf, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_movsx32_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu1_reg_reg_size((inst), 0x63, (dreg), (sreg), (size));
}

pub fn x86_64_movsx32_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu1_reg_regp_size((inst), 0x63, (dreg), (sregp), (size));
}

pub fn x86_64_movsx32_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu1_reg_mem_size((inst), 0x63, (dreg), (mem), (size));
}

pub fn x86_64_movsx32_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu1_reg_membase_size((inst), 0x63, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_movsx32_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu1_reg_memindex_size((inst), 0x63, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * Move with zero extension to the given size (unsigned)
 */
pub fn x86_64_movzx8_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu2_reg_reg_size((inst), 0x0f, 0xb6, (dreg), (sreg), (size) | 1);
}

pub fn x86_64_movzx8_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu2_reg_regp_size((inst), 0x0f, 0xb6, (dreg), (sregp), (size));
}

pub fn x86_64_movzx8_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu2_reg_mem_size((inst), 0x0f, 0xb6, (dreg), (mem), (size));
}

pub fn x86_64_movzx8_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu2_reg_membase_size((inst), 0x0f, 0xb6, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_movzx8_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu2_reg_memindex_size((inst), 0x0f, 0xb6, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

pub fn x86_64_movzx16_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    x86_64_alu2_reg_reg_size((inst), 0x0f, 0xb7, (dreg), (sreg), (size));
}

pub fn x86_64_movzx16_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_alu2_reg_regp_size((inst), 0x0f, 0xb7, (dreg), (sregp), (size));
}

pub fn x86_64_movzx16_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_alu2_reg_mem_size((inst), 0x0f, 0xb7, (dreg), (mem), (size));
}

pub fn x86_64_movzx16_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_alu2_reg_membase_size((inst), 0x0f, 0xb7, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_movzx16_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_alu2_reg_memindex_size((inst), 0x0f, 0xb7, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cmov: conditional move
 */
pub fn x86_64_cmov_reg_reg_size(inst: &mut Emit, cond: i32, dreg: u8, sreg: u8, is_signed: bool, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sreg));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
    }
    x86_64_reg_emit((inst), (dreg), (sreg));
}

pub fn x86_64_cmov_reg_regp_size(inst: &mut Emit, cond: i32, dreg: u8, sregp: u8, is_signed: bool, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (sregp));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
    }
    x86_64_regp_emit((inst), (dreg), (sregp));
}

pub fn x86_64_cmov_reg_mem_size(inst: &mut Emit, cond: i32, dreg: u8, mem: i32, is_signed: bool, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, 0);
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
    }
    x86_64_mem_emit((inst), (dreg), (mem));
}

pub fn x86_64_cmov_reg_membase_size(inst: &mut Emit, cond: i32, dreg: u8, basereg: u8, disp: i32, is_signed: bool, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), 0, (basereg));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
    }
    x86_64_membase_emit((inst), (dreg), (basereg), (disp));
}

pub fn x86_64_cmov_reg_memindex_size(inst: &mut Emit, cond: i32, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_signed: bool, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit((inst), (size), (dreg), (indexreg), (basereg));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] - 0x30);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] - 0x30);
    }
    x86_64_memindex_emit((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Stack manupulation instructions (push and pop)
 */

/*
 * Push instructions have a default size of 64 bit. mode.
 * There is no way to encode a 32 bit push.
 * So only the sizes 8 and 2 are allowed in 64 bit mode.
 */
pub fn x86_64_push_reg_size(inst: &mut Emit, reg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0, (reg));
    inst.push(0x50 + ((reg) & 0x7));
}

pub fn x86_64_push_regp_size(inst: &mut Emit, sregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0, (sregp));
    inst.push(0xff);
    x86_64_regp_emit((inst), 6, (sregp));
}

pub fn x86_64_push_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0, 0);
    inst.push(0xff);
    x86_64_mem_emit((inst), 6, (mem));
}

pub fn x86_64_push_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0, (basereg));
    inst.push(0xff);
    x86_64_membase_emit((inst), 6, (basereg), (disp));
}

pub fn x86_64_push_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, (indexreg), (basereg));
    inst.push(0xff);
    x86_64_memindex_emit((inst), 6, (basereg), (disp), (indexreg), (shift));
}

/*
 * We can push only 32 bit immediate values.
 * The value is sign extended to 64 bit on the stack.
 */
pub fn x86_64_push_imm(inst: &mut Emit, imm: i32) {
    let _imm = (imm);
    if (x86_is_imm8(_imm)) {
        inst.push(0x6A);
        x86_imm_emit8 ((inst), (_imm));
    } else {
        inst.push(0x68);
        x86_imm_emit32((inst), (_imm));
    }
}

/*
 * Use this version if you need a specific width of the value
 * pushed. The Value on the stack will allways be 64bit wide.
 */
pub fn x86_64_push_imm_size(inst: &mut Emit, imm: i32, size: i32) {
    match size {
        1 => {
            inst.push(0x6A);
            x86_imm_emit8((inst), (imm));
        }
        2 => {
            inst.push(0x66);
            inst.push(0x68);
            x86_imm_emit16((inst), (imm));
        }
        4 => {
            inst.push(0x68);
            x86_imm_emit32((inst), (imm));
        }
        _ => {}
    }
}


/*
 * Pop instructions have a default size of 64 bit in 64 bit mode.
 * There is no way to encode a 32 bit pop.
 * So only the sizes 2 and 8 are allowed.
 */
pub fn x86_64_pop_reg_size(inst: &mut Emit, dreg: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), 0, 0, 0, (dreg));
    inst.push(0x58 + ((dreg) & 0x7));
}

pub fn x86_64_pop_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0, (dregp));
    inst.push(0x8f);
    x86_64_regp_emit((inst), 0, (dregp));
}

pub fn x86_64_pop_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    inst.push(0x8f);
    x86_64_mem_emit((inst), 0, (mem));
}

pub fn x86_64_pop_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, 0,(basereg));
    inst.push(0x8f);
    x86_64_membase_emit((inst), 0, (basereg), (disp));
}

pub fn x86_64_pop_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    if ((size) == 2) {
        inst.push(0x66);
    }
    x86_64_rex_emit64((inst), (size), 0, (indexreg), (basereg));
    inst.push(0x8f);
    x86_64_memindex_emit((inst), 0, (basereg), (disp), (indexreg), (shift));
}

/*
 * control flow change instructions
 */

/*
 * call
 */

/*
 * call_imm is a relative call.
 * imm has to be a 32bit offset from the instruction following the
 * call instruction (absolute - (inst + 5)).
 * For offsets greater that 32bit an indirect call (via register)
 * has to be used.
 */
pub fn x86_64_call_imm(inst: &mut Emit, imm: i32) {
    x86_call_imm((inst), (imm));
}

pub fn x86_64_call_reg(inst: &mut Emit, reg: u8) {
    x86_64_alu1_reg((inst), 0xff, 2, (reg));
}

pub fn x86_64_call_regp(inst: &mut Emit, regp: u8) {
    x86_64_alu1_regp((inst), 0xff, 2, (regp));
}

/*
 * call_mem is a absolute indirect call.
 * To be able to use this instruction the address must be either
 * in the lowest 2GB or in the highest 2GB addressrange.
 * This is because mem is sign extended to 64bit.
 */
pub fn x86_64_call_mem(inst: &mut Emit, mem: i32) {
    x86_64_alu1_mem((inst), 0xff, 2, (mem));
}

pub fn x86_64_call_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    x86_64_alu1_membase((inst), 0xff, 2, (basereg), (disp));
}

pub fn x86_64_call_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_alu1_memindex((inst), 0xff, 2, (basereg), (disp), (indexreg), (shift));
}

/*
 * jmp
 */

/*
 * unconditional relative jumps
 */
pub fn x86_64_jmp_imm8(inst: &mut Emit, disp: i32) {
    inst.push(0xEB);
    x86_imm_emit8((inst), (disp));
}

pub fn x86_64_jmp_imm(inst: &mut Emit, disp: i32) {
    inst.push(0xE9);
    x86_imm_emit32((inst), (disp));
}

/*
 * unconditional indirect jumps
 */
pub fn x86_64_jmp_reg(inst: &mut Emit, reg: u8) {
    x86_64_alu1_reg((inst), 0xff, 4, (reg));
}

pub fn x86_64_jmp_regp(inst: &mut Emit, regp: u8) {
    x86_64_alu1_regp((inst), 0xff, 4, (regp));
}

pub fn x86_64_jmp_mem(inst: &mut Emit, mem: i32) {
    x86_64_alu1_mem((inst), 0xff, 4, (mem));
}

pub fn x86_64_jmp_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    x86_64_alu1_membase((inst), 0xff, 4, (basereg), (disp));
}

pub fn x86_64_jmp_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_alu1_memindex((inst), 0xff, 4, (basereg), (disp), (indexreg), (shift));
}

/*
 * Set the low byte in a register to 0x01 if a condition is met
 * or 0x00 otherwise.
 */
pub fn x86_64_set_reg(inst: &mut Emit, cond: i32, dreg: u8, is_signed: bool) {
    x86_64_rex_emit((inst), 1, 0, 0, (dreg));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
    }
    x86_64_reg_emit((inst), 0, (dreg));
}

pub fn x86_64_set_mem(inst: &mut Emit, cond: i32, mem: i32, is_signed: bool) {
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
    }
    x86_64_mem_emit((inst), 0, (mem));
}

pub fn x86_64_set_membase(inst: &mut Emit, cond: i32, basereg: u8, disp: i32, is_signed: bool) {
    x86_64_rex_emit((inst), 4, 0, 0, (basereg));
    inst.push(0x0f);
    if ((is_signed)) {
        inst.push(x86_cc_signed_map[(cond) as usize] + 0x20);
    } else {
        inst.push(x86_cc_unsigned_map[(cond) as usize] + 0x20);
    }
    x86_64_membase_emit((inst), 0, (basereg), (disp));
}

/*
 * ret
 */
pub fn x86_64_ret(inst: &mut Emit) {
    x86_ret((inst));
}

/*
 * xchg: Exchange values
 */
pub fn x86_64_xchg_reg_reg_size(inst: &mut Emit, dreg: u8, sreg: u8, size: i32) {
    if (((size) > 1) && ((dreg) == X86_64_RAX || (sreg) == X86_64_RAX)) {
        if ((size) == 2) {
            inst.push(0x66);
        }
        if ((dreg) == X86_64_RAX) {
            x86_64_rex_emit((inst), (size), 0, 0, (sreg));
            inst.push((0x90 + (sreg & 0x7)));
        } else {
            x86_64_rex_emit((inst), (size), 0, 0, (dreg));
            inst.push((0x90 + (dreg & 0x7)));
        }
    } else {
        if ((size) == 1) {
            x86_64_alu1_reg_reg_size((inst), 0x86, (dreg), (sreg), (size));
        } else {
            x86_64_alu1_reg_reg_size((inst), 0x87, (dreg), (sreg), (size));
        }
    }
}

/*
 * XMM instructions
 */

/*
 * xmm instructions with two opcodes
 */
pub fn x86_64_xmm2_reg_reg(inst: &mut Emit, opc1: u8, opc2: u8, r: u8, reg: u8) {
    x86_64_rex_emit(inst, 0, (r), 0, (reg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_reg_emit(inst, (r), (reg));
}

pub fn x86_64_xmm2_reg_regp(inst: &mut Emit, opc1: u8, opc2: u8, r: u8, regp: u8) {
    x86_64_rex_emit(inst, 0, (r), 0, (regp));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_regp_emit(inst, (r), (regp));
}

pub fn x86_64_xmm2_reg_mem(inst: &mut Emit, opc1: u8, opc2: u8, r: u8, mem: i32) {
    x86_64_rex_emit(inst, 0, (r), 0, 0);
    inst.push((opc1));
    inst.push((opc2));
    x86_64_mem_emit(inst, (r), (mem));
}

pub fn x86_64_xmm2_reg_membase(inst: &mut Emit, opc1: u8, opc2: u8, r: u8, basereg: u8, disp: i32) {
    x86_64_rex_emit(inst, 0, (r), 0, (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_membase_emit(inst, (r), (basereg), (disp));
}

pub fn x86_64_xmm2_reg_memindex(inst: &mut Emit, opc1: u8, opc2: u8, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_rex_emit(inst, 0, (r), (indexreg), (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_memindex_emit((inst), (r), (basereg), (disp), (indexreg), (shift));
}

/*
 * xmm instructions with a prefix and two opcodes
 */
pub fn x86_64_p1_xmm2_reg_reg_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, r: u8, reg: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (reg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_reg_emit(inst, (r), (reg));
}

pub fn x86_64_p1_xmm2_reg_regp_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, r: u8, regp: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (regp));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_regp_emit(inst, (r), (regp));
}

pub fn x86_64_p1_xmm2_reg_mem_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, r: u8, mem: i32, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, 0);
    inst.push((opc1));
    inst.push((opc2));
    x86_64_mem_emit(inst, (r), (mem));
}

pub fn x86_64_p1_xmm2_reg_membase_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, r: u8, basereg: u8, disp: i32, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_membase_emit(inst, (r), (basereg), (disp));
}

pub fn x86_64_p1_xmm2_reg_memindex_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), (indexreg), (basereg));
    inst.push((opc1));
    inst.push((opc2));
    x86_64_memindex_emit((inst), (r), (basereg), (disp), (indexreg), (shift));
}

/*
 * xmm instructions with a prefix and three opcodes
 */
pub fn x86_64_p1_xmm3_reg_reg_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: u8, reg: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (reg));
    inst.push((opc1));
    inst.push((opc2));
    inst.push((opc3));
    x86_64_reg_emit(inst, (r), (reg));
}

pub fn x86_64_p1_xmm3_reg_regp_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: u8, regp: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (regp));
    inst.push((opc1));
    inst.push((opc2));
    inst.push((opc3));
    x86_64_regp_emit(inst, (r), (regp));
}

pub fn x86_64_p1_xmm3_reg_mem_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: u8, mem: i32, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, 0);
    inst.push((opc1));
    inst.push((opc2));
    inst.push((opc3));
    x86_64_mem_emit(inst, (r), (mem));
}

pub fn x86_64_p1_xmm3_reg_membase_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: u8, basereg: u8, disp: i32, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), 0, (basereg));
    inst.push((opc1));
    inst.push((opc2));
    inst.push((opc3));
    x86_64_membase_emit(inst, (r), (basereg), (disp));
}

pub fn x86_64_p1_xmm3_reg_memindex_size(inst: &mut Emit, p1: u8, opc1: u8, opc2: u8, opc3: u8, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    inst.push((p1));
    x86_64_rex_emit(inst, (size), (r), (indexreg), (basereg));
    inst.push((opc1));
    inst.push((opc2));
    inst.push((opc3));
    x86_64_memindex_emit((inst), (r), (basereg), (disp), (indexreg), (shift));
}

/*
 * xmm1: Macro for use of the X86_64_XMM1 enum
 */
pub fn x86_64_xmm1_reg_reg(inst: &mut Emit, opc: u8, dreg: u8, sreg: u8, is_double: bool) {
    x86_64_p1_xmm2_reg_reg_size((inst), if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (sreg), 0);
}

pub fn x86_64_xmm1_reg_regp(inst: &mut Emit, opc: u8, dreg: u8, sregp: u8, is_double: bool) {
    x86_64_p1_xmm2_reg_regp_size((inst), if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (sregp), 0);
}

pub fn x86_64_xmm1_reg_mem(inst: &mut Emit, opc: u8, dreg: u8, mem: i32, is_double: bool) {
    x86_64_p1_xmm2_reg_mem_size((inst), if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (mem), 0);
}

pub fn x86_64_xmm1_reg_membase(inst: &mut Emit, opc: u8, dreg: u8, basereg: u8, disp: i32, is_double: bool) {
    x86_64_p1_xmm2_reg_membase_size((inst), if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (basereg), (disp), 0);
}

pub fn x86_64_xmm1_reg_memindex(inst: &mut Emit, opc: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_double: bool) {
    x86_64_p1_xmm2_reg_memindex_size((inst), if is_double { 0xf2 } else { 0xf3 }, 0x0f, (opc), (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Load and store MXCSR register state
 */

/*
 * ldmxcsr: Load MXCSR register
 */
pub fn x86_64_ldmxcsr_regp(inst: &mut Emit, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0xae, 2, (sregp));
}

pub fn x86_64_ldmxcsr_mem(inst: &mut Emit, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0xae, 2, (mem));
}

pub fn x86_64_ldmxcsr_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0xae, 2, (basereg), (disp));
}

pub fn x86_64_ldmxcsr_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0xae, 2, (basereg), (disp), (indexreg), (shift));
}

/*
 * stmxcsr: Store MXCSR register
 */
pub fn x86_64_stmxcsr_regp(inst: &mut Emit, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0xae, 3, (sregp));
}

pub fn x86_64_stmxcsr_mem(inst: &mut Emit, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0xae, 3, (mem));
}

pub fn x86_64_stmxcsr_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0xae, 3, (basereg), (disp));
}

pub fn x86_64_stmxcsr_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0xae, 3, (basereg), (disp), (indexreg), (shift));
}

/*
 * Move instructions
 */

/*
 * movd: Move doubleword from/to xmm register
 */
pub fn x86_64_movd_xreg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x6e, (dreg), (sreg), 4);
}

pub fn x86_64_movd_xreg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x6e, (dreg), (mem), 4);
}

pub fn x86_64_movd_xreg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x6e, (dreg), (sregp), 4);
}

pub fn x86_64_movd_xreg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), 4);
}

pub fn x86_64_movd_xreg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), (indexreg), (shift), 4);
}

pub fn x86_64_movd_reg_xreg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x7e, (sreg), (dreg), 4);
}

pub fn x86_64_movd_mem_xreg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x7e, (sreg), (mem), 4);
}

pub fn x86_64_movd_regp_xreg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x7e, (sreg), (dregp), 4);
}

pub fn x86_64_movd_membase_xreg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), 4);
}

pub fn x86_64_movd_memindex_xreg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), (indexreg), (shift), 4);
}

/*
 * movq: Move quadword from/to xmm register
 */
pub fn x86_64_movq_xreg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x6e, (dreg), (sreg), 8);
}

pub fn x86_64_movq_xreg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x6e, (dreg), (mem), 8);
}

pub fn x86_64_movq_xreg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x6e, (dreg), (sregp), 8);
}

pub fn x86_64_movq_xreg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), 8);
}

pub fn x86_64_movq_xreg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x6e, (dreg), (basereg), (disp), (indexreg), (shift), 8);
}

pub fn x86_64_movq_reg_xreg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x7e, (sreg), (dreg), 8);
}

pub fn x86_64_movq_mem_xreg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x7e, (sreg), (mem), 8);
}

pub fn x86_64_movq_regp_xreg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x7e, (sreg), (dregp), 8);
}

pub fn x86_64_movq_membase_xreg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), 8);
}

pub fn x86_64_movq_memindex_xreg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x7e, (sreg), (basereg), (disp), (indexreg), (shift), 8);
}

/*
 * movaps: Move aligned quadword (16 bytes)
 */
pub fn x86_64_movaps_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x28, (dreg), (sreg));
}

pub fn x86_64_movaps_regp_reg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x29, (sreg), (dregp));
}

pub fn x86_64_movaps_mem_reg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x29, (sreg), (mem));
}

pub fn x86_64_movaps_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x29, (sreg), (basereg), (disp));
}

pub fn x86_64_movaps_memindex_reg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x29, (sreg), (basereg), (disp), (indexreg), (shift));
}

pub fn x86_64_movaps_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x28, (dreg), (sregp));
}

pub fn x86_64_movaps_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x28, (dreg), (mem));
}

pub fn x86_64_movaps_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x28, (dreg), (basereg), (disp));
}

pub fn x86_64_movaps_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x28, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * movups: Move unaligned quadword (16 bytes)
 */
pub fn x86_64_movups_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x10, (dreg), (sreg));
}

pub fn x86_64_movups_regp_reg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x11, (sreg), (dregp));
}

pub fn x86_64_movups_mem_reg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x11, (sreg), (mem));
}

pub fn x86_64_movups_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x11, (sreg), (basereg), (disp));
}

pub fn x86_64_movups_memindex_reg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift));
}

pub fn x86_64_movups_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x10, (dreg), (sregp));
}

pub fn x86_64_movups_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x10, (dreg), (mem));
}

pub fn x86_64_movups_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x10, (dreg), (basereg), (disp));
}

pub fn x86_64_movups_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * movsd: Move scalar double (64bit float)
 */
pub fn x86_64_movsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x10, (dreg), (sreg), 0);
}

pub fn x86_64_movsd_regp_reg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x11, (sreg), (dregp), 0);
}

pub fn x86_64_movsd_mem_reg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x11, (sreg), (mem), 0);
}

pub fn x86_64_movsd_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x11, (sreg), (basereg), (disp), 0);
}

pub fn x86_64_movsd_memindex_reg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift), 0);
}

pub fn x86_64_movsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x10, (dreg), (sregp), 0);
}

pub fn x86_64_movsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x10, (dreg), (mem), 0);
}

pub fn x86_64_movsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x10, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_movsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * movss: Move scalar single (32bit float)
 */
pub fn x86_64_movss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x10, (dreg), (sreg), 0);
}

pub fn x86_64_movss_regp_reg(inst: &mut Emit, dregp: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x11, (sreg), (dregp), 0);
}

pub fn x86_64_movss_mem_reg(inst: &mut Emit, mem: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x11, (sreg), (mem), 0);
}

pub fn x86_64_movss_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, sreg: u8) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x11, (sreg), (basereg), (disp), 0);
}

pub fn x86_64_movss_memindex_reg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x11, (sreg), (basereg), (disp), (indexreg), (shift), 0);
}

pub fn x86_64_movss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x10, (dreg), (sregp), 0);
}

pub fn x86_64_movss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x10, (dreg), (mem), 0);
}

pub fn x86_64_movss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x10, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_movss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x10, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Conversion opcodes
 */

/*
 * cvtsi2ss: Convert signed integer to float32
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvtsi2ss_reg_reg_size(inst: &mut Emit, dxreg: u8, sreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x2a, (dxreg), (sreg), (size));
}

pub fn x86_64_cvtsi2ss_reg_regp_size(inst: &mut Emit, dxreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x2a, (dxreg), (sregp), (size));
}

pub fn x86_64_cvtsi2ss_reg_mem_size(inst: &mut Emit, dxreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x2a, (dxreg), (mem), (size));
}

pub fn x86_64_cvtsi2ss_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x2a, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvtsi2ss_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x2a, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvtsi2sd: Convert signed integer to float64
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvtsi2sd_reg_reg_size(inst: &mut Emit, dxreg: u8, sreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x2a, (dxreg), (sreg), (size));
}

pub fn x86_64_cvtsi2sd_reg_regp_size(inst: &mut Emit, dxreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x2a, (dxreg), (sregp), (size));
}

pub fn x86_64_cvtsi2sd_reg_mem_size(inst: &mut Emit, dxreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x2a, (dxreg), (mem), (size));
}

pub fn x86_64_cvtsi2sd_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x2a, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvtsi2sd_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x2a, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvtss2si: Convert float32.to a signed integer using the rounding mode
 * in the mxcsr register
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvtss2si_reg_reg_size(inst: &mut Emit, dreg: u8, sxreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x2d, (dreg), (sxreg), (size));
}

pub fn x86_64_cvtss2si_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x2d, (dreg), (sregp), (size));
}

pub fn x86_64_cvtss2si_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x2d, (dreg), (mem), (size));
}

pub fn x86_64_cvtss2si_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x2d, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvtss2si_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x2d, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvttss2si: Convert float32.to a signed integer using the truncate rounding mode.
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvttss2si_reg_reg_size(inst: &mut Emit, dreg: u8, sxreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x2c, (dreg), (sxreg), (size));
}

pub fn x86_64_cvttss2si_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x2c, (dreg), (sregp), (size));
}

pub fn x86_64_cvttss2si_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x2c, (dreg), (mem), (size));
}

pub fn x86_64_cvttss2si_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x2c, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvttss2si_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x2c, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvtsd2si: Convert float64 to a signed integer using the rounding mode
 * in the mxcsr register
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvtsd2si_reg_reg_size(inst: &mut Emit, dreg: u8, sxreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x2d, (dreg), (sxreg), (size));
}

pub fn x86_64_cvtsd2si_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x2d, (dreg), (sregp), (size));
}

pub fn x86_64_cvtsd2si_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x2d, (dreg), (mem), (size));
}

pub fn x86_64_cvtsd2si_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x2d, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvtsd2si_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x2d, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvttsd2si: Convert float64 to a signed integer using the truncate rounding mode.
 * The size is the size of the integer value (4 or 8)
 */
pub fn x86_64_cvttsd2si_reg_reg_size(inst: &mut Emit, dreg: u8, sxreg: u8, size: i32) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x2c, (dreg), (sxreg), (size));
}

pub fn x86_64_cvttsd2si_reg_regp_size(inst: &mut Emit, dreg: u8, sregp: u8, size: i32) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x2c, (dreg), (sregp), (size));
}

pub fn x86_64_cvttsd2si_reg_mem_size(inst: &mut Emit, dreg: u8, mem: i32, size: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x2c, (dreg), (mem), (size));
}

pub fn x86_64_cvttsd2si_reg_membase_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, size: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x2c, (dreg), (basereg), (disp), (size));
}

pub fn x86_64_cvttsd2si_reg_memindex_size(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x2c, (dreg), (basereg), (disp), (indexreg), (shift), (size));
}

/*
 * cvtss2sd: Convert float32 to float64
 */
pub fn x86_64_cvtss2sd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x5a, (dreg), (sreg), 0);
}

pub fn x86_64_cvtss2sd_reg_regp(inst: &mut Emit, dxreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x5a, (dxreg), (sregp), 0);
}

pub fn x86_64_cvtss2sd_reg_mem(inst: &mut Emit, dxreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x5a, (dxreg), (mem), 0);
}

pub fn x86_64_cvtss2sd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x5a, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_cvtss2sd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x5a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * cvtsd2ss: Convert float64 to float32
 */
pub fn x86_64_cvtsd2ss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x5a, (dreg), (sreg), 0);
}

pub fn x86_64_cvtsd2ss_reg_regp(inst: &mut Emit, dxreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x5a, (dxreg), (sregp), 0);
}

pub fn x86_64_cvtsd2ss_reg_mem(inst: &mut Emit, dxreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x5a, (dxreg), (mem), 0);
}

pub fn x86_64_cvtsd2ss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x5a, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_cvtsd2ss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x5a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Compare opcodes
 */

/*
 * comiss: Compare ordered scalar single precision values
 */
pub fn x86_64_comiss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x2f, (dreg), (sreg));
}

pub fn x86_64_comiss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x2f, (dreg), (sregp));
}

pub fn x86_64_comiss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x2f, (dreg), (mem));
}

pub fn x86_64_comiss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x2f, (dreg), (basereg), (disp));
}

pub fn x86_64_comiss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x2f, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * comisd: Compare ordered scalar double precision values
 */
pub fn x86_64_comisd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x2f, (dreg), (sreg), 0);
}

pub fn x86_64_comisd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x2f, (dreg), (sregp), 0);
}

pub fn x86_64_comisd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x2f, (dreg), (mem), 0);
}

pub fn x86_64_comisd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x2f, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_comisd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x2f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * ucomiss: Compare unordered scalar single precision values
 */
pub fn x86_64_ucomiss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x2e, (dreg), (sreg));
}

pub fn x86_64_ucomiss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x2e, (dreg), (sregp));
}

pub fn x86_64_ucomiss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x2e, (dreg), (mem));
}

pub fn x86_64_ucomiss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x2e, (dreg), (basereg), (disp));
}

pub fn x86_64_ucomiss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x2e, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * ucomisd: Compare unordered scalar double precision values
 */
pub fn x86_64_ucomisd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x2e, (dreg), (sreg), 0);
}

pub fn x86_64_ucomisd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x2e, (dreg), (sregp), 0);
}

pub fn x86_64_ucomisd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x2e, (dreg), (mem), 0);
}

pub fn x86_64_ucomisd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x2e, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_ucomisd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x2e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Arithmetic opcodes
 */

/*
 * addss: Add scalar single precision float values
 */
pub fn x86_64_addss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x58, (dreg), (sreg), 0);
}

pub fn x86_64_addss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x58, (dreg), (sregp), 0);
}

pub fn x86_64_addss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x58, (dreg), (mem), 0);
}

pub fn x86_64_addss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x58, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_addss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x58, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * subss: Substract scalar single precision float values
 */
pub fn x86_64_subss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x5c, (dreg), (sreg), 0);
}

pub fn x86_64_subss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x5c, (dreg), (sregp), 0);
}

pub fn x86_64_subss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x5c, (dreg), (mem), 0);
}

pub fn x86_64_subss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x5c, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_subss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x5c, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * mulss: Multiply scalar single precision float values
 */
pub fn x86_64_mulss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x59, (dreg), (sreg), 0);
}

pub fn x86_64_mulss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x59, (dreg), (sregp), 0);
}

pub fn x86_64_mulss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x59, (dreg), (mem), 0);
}

pub fn x86_64_mulss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x59, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_mulss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x59, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * divss: Divide scalar single precision float values
 */
pub fn x86_64_divss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x5e, (dreg), (sreg), 0);
}

pub fn x86_64_divss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x5e, (dreg), (sregp), 0);
}

pub fn x86_64_divss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x5e, (dreg), (mem), 0);
}

pub fn x86_64_divss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x5e, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_divss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x5e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Macros for the logical operations with packed single precision values.
 */
pub fn x86_64_plops_reg_reg(inst: &mut Emit, op: u8, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, (op), (dreg), (sreg));
}

pub fn x86_64_plops_reg_regp(inst: &mut Emit, op: u8, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, (op), (dreg), (sregp));
}

pub fn x86_64_plops_reg_mem(inst: &mut Emit, op: u8, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, (op), (dreg), (mem));
}

pub fn x86_64_plops_reg_membase(inst: &mut Emit, op: u8, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, (op), (dreg), (basereg), (disp));
}

pub fn x86_64_plops_reg_memindex(inst: &mut Emit, op: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, (op), (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * andps: And
 */
pub fn x86_64_andps_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x54, (dreg), (sreg));
}

pub fn x86_64_andps_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x54, (dreg), (sregp));
}

pub fn x86_64_andps_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x54, (dreg), (mem));
}

pub fn x86_64_andps_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x54, (dreg), (basereg), (disp));
}

pub fn x86_64_andps_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x54, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * orps: Or
 */
pub fn x86_64_orps_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x56, (dreg), (sreg));
}

pub fn x86_64_orps_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x56, (dreg), (sregp));
}

pub fn x86_64_orps_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x56, (dreg), (mem));
}

pub fn x86_64_orps_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x56, (dreg), (basereg), (disp));
}

pub fn x86_64_orps_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x56, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * xorps: Xor
 */
pub fn x86_64_xorps_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_xmm2_reg_reg((inst), 0x0f, 0x57, (dreg), (sreg));
}

pub fn x86_64_xorps_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_xmm2_reg_regp((inst), 0x0f, 0x57, (dreg), (sregp));
}

pub fn x86_64_xorps_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_xmm2_reg_mem((inst), 0x0f, 0x57, (dreg), (mem));
}

pub fn x86_64_xorps_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_xmm2_reg_membase((inst), 0x0f, 0x57, (dreg), (basereg), (disp));
}

pub fn x86_64_xorps_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_xmm2_reg_memindex((inst), 0x0f, 0x57, (dreg), (basereg), (disp), (indexreg), (shift));
}

/*
 * maxss: Maximum value
 */
pub fn x86_64_maxss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x5f, (dreg), (sreg), 0);
}

pub fn x86_64_maxss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x5f, (dreg), (sregp), 0);
}

pub fn x86_64_maxss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x5f, (dreg), (mem), 0);
}

pub fn x86_64_maxss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x5f, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_maxss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x5f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * minss: Minimum value
 */
pub fn x86_64_minss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x5d, (dreg), (sreg), 0);
}

pub fn x86_64_minss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x5d, (dreg), (sregp), 0);
}

pub fn x86_64_minss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x5d, (dreg), (mem), 0);
}

pub fn x86_64_minss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x5d, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_minss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x5d, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * sqrtss: Square root
 */
pub fn x86_64_sqrtss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf3, 0x0f, 0x51, (dreg), (sreg), 0);
}

pub fn x86_64_sqrtss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf3, 0x0f, 0x51, (dreg), (sregp), 0);
}

pub fn x86_64_sqrtss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf3, 0x0f, 0x51, (dreg), (mem), 0);
}

pub fn x86_64_sqrtss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf3, 0x0f, 0x51, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_sqrtss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf3, 0x0f, 0x51, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}


/*
 * Macros for the logical operations with packed double precision values.
 */
pub fn x86_64_plopd_reg_reg(inst: &mut Emit, op: u8, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, (op), (dreg), (sreg), 0);
}

pub fn x86_64_plopd_reg_regp(inst: &mut Emit, op: u8, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, (op), (dreg), (sregp), 0);
}

pub fn x86_64_plopd_reg_mem(inst: &mut Emit, op: u8, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, (op), (dreg), (mem), 0);
}

pub fn x86_64_plopd_reg_membase(inst: &mut Emit, op: u8, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, (op), (dreg), (basereg), (disp), 0);
}

pub fn x86_64_plopd_reg_memindex(inst: &mut Emit, op: u8, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, (op), (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * addsd: Add scalar double precision float values
 */
pub fn x86_64_addsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x58, (dreg), (sreg), 0);
}

pub fn x86_64_addsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x58, (dreg), (sregp), 0);
}

pub fn x86_64_addsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x58, (dreg), (mem), 0);
}

pub fn x86_64_addsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x58, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_addsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x58, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * subsd: Substract scalar double precision float values
 */
pub fn x86_64_subsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x5c, (dreg), (sreg), 0);
}

pub fn x86_64_subsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x5c, (dreg), (sregp), 0);
}

pub fn x86_64_subsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x5c, (dreg), (mem), 0);
}

pub fn x86_64_subsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x5c, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_subsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x5c, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * mulsd: Multiply scalar double precision float values
 */
pub fn x86_64_mulsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x59, (dreg), (sreg), 0);
}

pub fn x86_64_mulsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x59, (dreg), (sregp), 0);
}

pub fn x86_64_mulsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x59, (dreg), (mem), 0);
}

pub fn x86_64_mulsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x59, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_mulsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x59, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * divsd: Divide scalar double precision float values
 */
pub fn x86_64_divsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x5e, (dreg), (sreg), 0);
}

pub fn x86_64_divsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x5e, (dreg), (sregp), 0);
}

pub fn x86_64_divsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x5e, (dreg), (mem), 0);
}

pub fn x86_64_divsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x5e, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_divsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x5e, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * andpd: And
 */
pub fn x86_64_andpd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x54, (dreg), (sreg), 0);
}

pub fn x86_64_andpd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x54, (dreg), (sregp), 0);
}

pub fn x86_64_andpd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x54, (dreg), (mem), 0);
}

pub fn x86_64_andpd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x54, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_andpd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x54, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * orpd: Or
 */
pub fn x86_64_orpd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x56, (dreg), (sreg), 0);
}

pub fn x86_64_orpd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x56, (dreg), (sregp), 0);
}

pub fn x86_64_orpd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x56, (dreg), (mem), 0);
}

pub fn x86_64_orpd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x56, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_orpd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x56, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * xorpd: Xor
 */
pub fn x86_64_xorpd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0x66, 0x0f, 0x57, (dreg), (sreg), 0);
}

pub fn x86_64_xorpd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0x66, 0x0f, 0x57, (dreg), (sregp), 0);
}

pub fn x86_64_xorpd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0x66, 0x0f, 0x57, (dreg), (mem), 0);
}

pub fn x86_64_xorpd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0x66, 0x0f, 0x57, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_xorpd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0x66, 0x0f, 0x57, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * maxsd: Maximum value
 */
pub fn x86_64_maxsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x5f, (dreg), (sreg), 0);
}

pub fn x86_64_maxsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x5f, (dreg), (sregp), 0);
}

pub fn x86_64_maxsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x5f, (dreg), (mem), 0);
}

pub fn x86_64_maxsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x5f, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_maxsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x5f, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * minsd: Minimum value
 */
pub fn x86_64_minsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x5d, (dreg), (sreg), 0);
}

pub fn x86_64_minsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x5d, (dreg), (sregp), 0);
}

pub fn x86_64_minsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x5d, (dreg), (mem), 0);
}

pub fn x86_64_minsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x5d, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_minsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x5d, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * sqrtsd: Square root
 */
pub fn x86_64_sqrtsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8) {
    x86_64_p1_xmm2_reg_reg_size((inst), 0xf2, 0x0f, 0x51, (dreg), (sreg), 0);
}

pub fn x86_64_sqrtsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8) {
    x86_64_p1_xmm2_reg_regp_size((inst), 0xf2, 0x0f, 0x51, (dreg), (sregp), 0);
}

pub fn x86_64_sqrtsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32) {
    x86_64_p1_xmm2_reg_mem_size((inst), 0xf2, 0x0f, 0x51, (dreg), (mem), 0);
}

pub fn x86_64_sqrtsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32) {
    x86_64_p1_xmm2_reg_membase_size((inst), 0xf2, 0x0f, 0x51, (dreg), (basereg), (disp), 0);
}

pub fn x86_64_sqrtsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    x86_64_p1_xmm2_reg_memindex_size((inst), 0xf2, 0x0f, 0x51, (dreg), (basereg), (disp), (indexreg), (shift), 0);
}

/*
 * Rounding: Available in SSE 4.1 only
 */

/*
 * roundss: Round scalar single precision value
 */
pub fn x86_64_roundss_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8, mode: i32) {
    x86_64_p1_xmm3_reg_reg_size((inst), 0x66, 0x0f, 0x3a, 0x0a, (dreg), (sreg), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundss_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8, mode: i32) {
    x86_64_p1_xmm3_reg_regp_size((inst), 0x66, 0x0f, 0x3a, 0x0a, (dreg), (sregp), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundss_reg_mem(inst: &mut Emit, dreg: u8, mem: i32, mode: i32) {
    x86_64_p1_xmm3_reg_mem_size((inst), 0x66, 0x0f, 0x3a, 0x0a, (dreg), (mem), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundss_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, mode: i32) {
    x86_64_p1_xmm3_reg_membase_size((inst), 0x66, 0x0f, 0x3a, 0x0a, (dreg), (basereg), (disp), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundss_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, mode: i32) {
    x86_64_p1_xmm3_reg_memindex_size((inst), 0x66, 0x0f, 0x3a, 0x0a, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    x86_imm_emit8((inst), (mode));
}

/*
 * roundsd: Round scalar double precision value
 */
pub fn x86_64_roundsd_reg_reg(inst: &mut Emit, dreg: u8, sreg: u8, mode: i32) {
    x86_64_p1_xmm3_reg_reg_size((inst), 0x66, 0x0f, 0x3a, 0x0b, (dreg), (sreg), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundsd_reg_regp(inst: &mut Emit, dreg: u8, sregp: u8, mode: i32) {
    x86_64_p1_xmm3_reg_regp_size((inst), 0x66, 0x0f, 0x3a, 0x0b, (dreg), (sregp), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundsd_reg_mem(inst: &mut Emit, dreg: u8, mem: i32, mode: i32) {
    x86_64_p1_xmm3_reg_mem_size((inst), 0x66, 0x0f, 0x3a, 0x0b, (dreg), (mem), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundsd_reg_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, mode: i32) {
    x86_64_p1_xmm3_reg_membase_size((inst), 0x66, 0x0f, 0x3a, 0x0b, (dreg), (basereg), (disp), 0);
    x86_imm_emit8((inst), (mode));
}

pub fn x86_64_roundsd_reg_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, mode: i32) {
    x86_64_p1_xmm3_reg_memindex_size((inst), 0x66, 0x0f, 0x3a, 0x0b, (dreg), (basereg), (disp), (indexreg), (shift), 0);
    x86_imm_emit8((inst), (mode));
}

/*
 * Clear xmm register
 */
pub fn x86_64_clear_xreg(inst: &mut Emit, reg: u8) {
    x86_64_xorps_reg_reg((inst), (reg), (reg));
}

/*
 * fpu instructions
 */

/*
 * fld
 */

pub fn x86_64_fld_regp_size(inst: &mut Emit, sregp: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (sregp));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_regp_emit((inst), 0, (sregp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_regp_emit((inst), 0, (sregp));
        }
        10 => {
            inst.push(0xdb);
            x86_64_regp_emit((inst), 5, (sregp));
        }
        _ => {}
    }
}

pub fn x86_64_fld_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_mem_emit((inst), 0, (mem));
        }
        8 => {
            inst.push(0xdd);
            x86_64_mem_emit((inst), 0, (mem));
        }
        10 => {
            inst.push(0xdb);
            x86_64_mem_emit((inst), 5, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fld_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_membase_emit((inst), 0, (basereg), (disp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_membase_emit((inst), 0, (basereg), (disp));
        }
        10 => {
            inst.push(0xdb);
            x86_64_membase_emit((inst), 5, (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_fld_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, (indexreg), (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_memindex_emit((inst), 0, (basereg), (disp), (indexreg), (shift));
        }
        8 => {
            inst.push(0xdd);
            x86_64_memindex_emit((inst), 0, (basereg), (disp), (indexreg), (shift));
        }
        10 => {
            inst.push(0xdb);
            x86_64_memindex_emit((inst), 5, (basereg), (disp), (indexreg), (shift));
        }
        _ => {}
    }
}

/*
 * fild: Load an integer and convert it to long double
 */
pub fn x86_64_fild_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_mem_emit((inst), 0, (mem));
        }
        4 => {
            inst.push(0xdb);
            x86_64_mem_emit((inst), 0, (mem));
        }
        8 => {
            inst.push(0xdf);
            x86_64_mem_emit((inst), 5, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fild_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_membase_emit((inst), 0, (basereg), (disp));
        }
        4 => {
            inst.push(0xdb);
            x86_64_membase_emit((inst), 0, (basereg), (disp));
        }
        8 => {
            inst.push(0xdf);
            x86_64_membase_emit((inst), 5, (basereg), (disp));
        }
        _ => {}
    }
}

/*
 * fst: Store fpu register to memory (only float32 and float64 allowed)
 */

pub fn x86_64_fst_regp_size(inst: &mut Emit, sregp: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (sregp));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_regp_emit((inst), 2, (sregp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_regp_emit((inst), 2, (sregp));
        }
        _ => {}
    }
}

pub fn x86_64_fst_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_mem_emit((inst), 2, (mem));
        }
        8 => {
            inst.push(0xdd);
            x86_64_mem_emit((inst), 2, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fst_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_membase_emit((inst), 2, (basereg), (disp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_membase_emit((inst), 2, (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_fst_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, (indexreg), (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_memindex_emit((inst), 2, (basereg), (disp), (indexreg), (shift));
        }
        8 => {
            inst.push(0xdd);
            x86_64_memindex_emit((inst), 2, (basereg), (disp), (indexreg), (shift));
        }
        _ => {}
    }
}

/*
 * fstp: store top fpu register to memory and pop it from the fpu stack
 */
pub fn x86_64_fstp_regp_size(inst: &mut Emit, sregp: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (sregp));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_regp_emit((inst), 3, (sregp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_regp_emit((inst), 3, (sregp));
        }
        10 => {
            inst.push(0xdb);
            x86_64_regp_emit((inst), 7, (sregp));
        }
        _ => {}
    }
}

pub fn x86_64_fstp_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_mem_emit((inst), 3, (mem));
        }
        8 => {
            inst.push(0xdd);
            x86_64_mem_emit((inst), 3, (mem));
        }
        10 => {
            inst.push(0xdb);
            x86_64_mem_emit((inst), 7, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fstp_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_membase_emit((inst), 3, (basereg), (disp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_membase_emit((inst), 3, (basereg), (disp));
        }
        10 => {
            inst.push(0xdb);
            x86_64_membase_emit((inst), 7, (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_fstp_memindex_size(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, (indexreg), (basereg));
    match size {
        4 => {
            inst.push(0xd9);
            x86_64_memindex_emit((inst), 3, (basereg), (disp), (indexreg), (shift));
        }
        8 => {
            inst.push(0xdd);
            x86_64_memindex_emit((inst), 3, (basereg), (disp), (indexreg), (shift));
        }
        10 => {
            inst.push(0xdb);
            x86_64_memindex_emit((inst), 7, (basereg), (disp), (indexreg), (shift));
        }
        _ => {}
    }
}

/*
 * fistp: Convert long double to integer
 */
pub fn x86_64_fistp_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_mem_emit((inst), 3, (mem));
        }
        4 => {
            inst.push(0xdb);
            x86_64_mem_emit((inst), 3, (mem));
        }
        8 => {
            inst.push(0xdf);
            x86_64_mem_emit((inst), 7, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fistp_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (dregp));
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_regp_emit((inst), 3, (dregp));
        }
        4 => {
            inst.push(0xdb);
            x86_64_regp_emit((inst), 3, (dregp));
        }
        8 => {
            inst.push(0xdf);
            x86_64_regp_emit((inst), 7, (dregp));
        }
        _ => {}
    }
}

pub fn x86_64_fistp_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_membase_emit((inst), 3, (basereg), (disp));
        }
        4 => {
            inst.push(0xdb);
            x86_64_membase_emit((inst), 3, (basereg), (disp));
        }
        8 => {
            inst.push(0xdf);
            x86_64_membase_emit((inst), 7, (basereg), (disp));
        }
        _ => {}
    }
}

/*
 * frndint: Round st(0) to integer according to the rounding mode set in the fpu control word.
 */
pub fn x86_64_frndint(inst: &mut Emit) {
    inst.push(0xd9);
    inst.push(0xfc);
}

/*
 * fisttp: Convert long double to integer using truncation as rounding mode Available in SSE 3 only
 */
pub fn x86_64_fisttp_regp_size(inst: &mut Emit, dregp: u8, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (dregp));
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_regp_emit((inst), 1, (dregp));
        }
        4 => {
            inst.push(0xdb);
            x86_64_regp_emit((inst), 1, (dregp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_regp_emit((inst), 1, (dregp));
        }
        _ => {}
    }
}

pub fn x86_64_fisttp_mem_size(inst: &mut Emit, mem: i32, size: i32) {
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_mem_emit((inst), 1, (mem));
        }
        4 => {
            inst.push(0xdb);
            x86_64_mem_emit((inst), 1, (mem));
        }
        8 => {
            inst.push(0xdd);
            x86_64_mem_emit((inst), 1, (mem));
        }
        _ => {}
    }
}

pub fn x86_64_fisttp_membase_size(inst: &mut Emit, basereg: u8, disp: i32, size: i32) {
    x86_64_rex_emit((inst), 0, 0, 0, (basereg));
    match size {
        2 => {
            inst.push(0xdf);
            x86_64_membase_emit((inst), 1, (basereg), (disp));
        }
        4 => {
            inst.push(0xdb);
            x86_64_membase_emit((inst), 1, (basereg), (disp));
        }
        8 => {
            inst.push(0xdd);
            x86_64_membase_emit((inst), 1, (basereg), (disp));
        }
        _ => {}
    }
}

pub fn x86_64_fabs(inst: &mut Emit) {
    inst.push(0xd9);
    inst.push(0xe1);
}

pub fn x86_64_fchs(inst: &mut Emit) {
    inst.push(0xd9);
    inst.push(0xe0);
}

/*
 * Store fpu control word after checking for pending unmasked fpu exceptions
 */
pub fn x86_64_fnstcw(inst: &mut Emit, mem: i32) {
    inst.push(0xd9);
    x86_64_mem_emit((inst), 7, (mem));
}

pub fn x86_64_fnstcw_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.push(0xd9);
    x86_64_membase_emit((inst), 7, (basereg), (disp));
}

/*
 * Load fpu control word
 */
pub fn x86_64_fldcw(inst: &mut Emit, mem: i32) {
    inst.push(0xd9);
    x86_64_mem_emit((inst), 5, (mem));
}

pub fn x86_64_fldcw_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.push(0xd9);
    x86_64_membase_emit ((inst), 5, (basereg), (disp));
}
