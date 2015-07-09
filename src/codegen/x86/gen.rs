#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

use codegen::Emit;
use std::mem::transmute;

/*;
 * jit-gen-x86.h: Macros for generating x86 code
 *
 * Authors:
 *   Paolo Molaro (lupus@ximian.com)
 *   Intel Corporation (ORP Project)
 *   Sergey Chaban (serge@wildwestsoftware.com)
 *   Dietmar Maurer (dietmar@ximian.com)
 *   Patrik Torstensson
 * 
 * Copyright (C)  2000 Intel Corporation.  All rights reserved.
 * Copyright (C)  2001, 2002 Ximian, Inc.
 *
 * This file originated with the Mono project (www.go-mono.com), and may
 * be redistributed under the terms of the Lesser General Public License.
 */

/*
// x86 register numbers
*/

type X86_Reg_No = u8;

const X86_EAX : X86_Reg_No = 0;
const X86_ECX : X86_Reg_No = 1;
const X86_EDX : X86_Reg_No = 2;
const X86_EBX : X86_Reg_No = 3;
const X86_ESP : X86_Reg_No = 4;
const X86_EBP : X86_Reg_No = 5;
const X86_ESI : X86_Reg_No = 6;
const X86_EDI : X86_Reg_No = 7;
const X86_NREG : X86_Reg_No = 8;

/*
// opcodes for alu instructions
*/

type X86_ALU_Opcode = u8;

const X86_ADD : X86_ALU_Opcode = 0;
const X86_OR  : X86_ALU_Opcode = 1;
const X86_ADC : X86_ALU_Opcode = 2;
const X86_SBB : X86_ALU_Opcode = 3;
const X86_AND : X86_ALU_Opcode = 4;
const X86_SUB : X86_ALU_Opcode = 5;
const X86_XOR : X86_ALU_Opcode = 6;
const X86_CMP : X86_ALU_Opcode = 7;
const X86_NALU : X86_ALU_Opcode = 8;

/*
// opcodes for shift instructions
*/

type X86_Shift_Opcode = i32;

const X86_SHLD : X86_Shift_Opcode = 0;
const X86_SHLR : X86_Shift_Opcode = 1;
const X86_ROL : X86_Shift_Opcode = 0;
const X86_ROR : X86_Shift_Opcode = 1;
const X86_RCL : X86_Shift_Opcode = 2;
const X86_RCR : X86_Shift_Opcode = 3;
const X86_SHL : X86_Shift_Opcode = 4;
const X86_SHR : X86_Shift_Opcode = 5;
const X86_SAR : X86_Shift_Opcode = 7;
const X86_NSHIFT : X86_Shift_Opcode = 8;

/*
// opcodes for floating-point instructions
*/

type X86_FP_Opcode = i32;

const X86_FADD  : X86_FP_Opcode = 0;
const X86_FMUL  : X86_FP_Opcode = 1;
const X86_FCOM  : X86_FP_Opcode = 2;
const X86_FCOMP : X86_FP_Opcode = 3;
const X86_FSUB  : X86_FP_Opcode = 4;
const X86_FSUBR : X86_FP_Opcode = 5;
const X86_FDIV  : X86_FP_Opcode = 6;
const X86_FDIVR : X86_FP_Opcode = 7;
const X86_NFP   : X86_FP_Opcode = 8;

/*
// integer conditions codes
*/

type X86_CC = i32;

const X86_CC_EQ : X86_CC = 0;
const X86_CC_E : X86_CC = 0;
const X86_CC_Z : X86_CC = 0;
const X86_CC_NE : X86_CC = 1;
const X86_CC_NZ : X86_CC = 1;
const X86_CC_LT : X86_CC = 2;
const X86_CC_B : X86_CC = 2;
const X86_CC_C : X86_CC = 2;
const X86_CC_NAE : X86_CC = 2;
const X86_CC_LE : X86_CC = 3;
const X86_CC_BE : X86_CC = 3;
const X86_CC_NA : X86_CC = 3;
const X86_CC_GT : X86_CC = 4;
const X86_CC_A : X86_CC = 4;
const X86_CC_NBE : X86_CC = 4;
const X86_CC_GE : X86_CC = 5;
const X86_CC_AE : X86_CC = 5;
const X86_CC_NB : X86_CC = 5;
const X86_CC_NC : X86_CC = 5;
const X86_CC_LZ : X86_CC = 6;
const X86_CC_S : X86_CC = 6;
const X86_CC_GEZ : X86_CC = 7;
const X86_CC_NS : X86_CC = 7;
const X86_CC_P : X86_CC = 8;
const X86_CC_PE : X86_CC = 8;
const X86_CC_NP : X86_CC = 9;
const X86_CC_PO : X86_CC = 9;
const X86_CC_O : X86_CC = 10;
const X86_CC_NO : X86_CC = 11;
const X86_NCC : X86_CC = 12;

/* FP status */

type X86_FP_Status = i32;

const X86_FP_C0 : X86_FP_Status = 0x100;
const X86_FP_C1 : X86_FP_Status = 0x200;
const X86_FP_C2 : X86_FP_Status = 0x400;
const X86_FP_C3 : X86_FP_Status = 0x4000;
const X86_FP_CC_MASK : X86_FP_Status = 0x4500;

/* FP control word */

type X86_FP_ControlWord = i32;

const X86_FPCW_INVOPEX_MASK : X86_FP_ControlWord = 0x1;
const X86_FPCW_DENOPEX_MASK : X86_FP_ControlWord = 0x2;
const X86_FPCW_ZERODIV_MASK : X86_FP_ControlWord = 0x4;
const X86_FPCW_OVFEX_MASK   : X86_FP_ControlWord = 0x8;
const X86_FPCW_UNDFEX_MASK  : X86_FP_ControlWord = 0x10;
const X86_FPCW_PRECEX_MASK  : X86_FP_ControlWord = 0x20;
const X86_FPCW_PRECC_MASK   : X86_FP_ControlWord = 0x300;
const X86_FPCW_ROUNDC_MASK  : X86_FP_ControlWord = 0xc00;

/* values for precision control */
const X86_FPCW_PREC_SINGLE    : X86_FP_ControlWord = 0;
const X86_FPCW_PREC_DOUBLE    : X86_FP_ControlWord = 0x200;
const X86_FPCW_PREC_EXTENDED  : X86_FP_ControlWord = 0x300;

/* values for rounding control */
const X86_FPCW_ROUND_NEAREST  : X86_FP_ControlWord = 0;
const X86_FPCW_ROUND_DOWN     : X86_FP_ControlWord = 0x400;
const X86_FPCW_ROUND_UP       : X86_FP_ControlWord = 0x800;
const X86_FPCW_ROUND_TOZERO   : X86_FP_ControlWord = 0xc00;

/*
// prefix code
*/

type X86_Prefix = i32;

const X86_LOCK_PREFIX : X86_Prefix = 0xF0;
const X86_REPNZ_PREFIX : X86_Prefix = 0xF2;
const X86_REPZ_PREFIX : X86_Prefix = 0xF3;
const X86_REP_PREFIX : X86_Prefix = 0xF3;
const X86_CS_PREFIX : X86_Prefix = 0x2E;
const X86_SS_PREFIX : X86_Prefix = 0x36;
const X86_DS_PREFIX : X86_Prefix = 0x3E;
const X86_ES_PREFIX : X86_Prefix = 0x26;
const X86_FS_PREFIX : X86_Prefix = 0x64;
const X86_GS_PREFIX : X86_Prefix = 0x65;
const X86_UNLIKELY_PREFIX : X86_Prefix = 0x2E;
const X86_LIKELY_PREFIX : X86_Prefix = 0x3E;
const X86_OPERAND_PREFIX : X86_Prefix = 0x66;
const X86_ADDRESS_PREFIX : X86_Prefix = 0x67;

const x86_cc_unsigned_map : [u8; X86_NCC as usize] = [
	0x74, /* eq  */
	0x75, /* ne  */
	0x72, /* lt  */
	0x76, /* le  */
	0x77, /* gt  */
	0x73, /* ge  */
	0x78, /* lz  */
	0x79, /* gez */
	0x7a, /* p   */
	0x7b, /* np  */
	0x70, /* o  */
	0x71, /* no  */
];

const x86_cc_signed_map : [u8; X86_NCC as usize] = [
	0x74, /* eq  */
	0x75, /* ne  */
	0x7c, /* lt  */
	0x7e, /* le  */
	0x7f, /* gt  */
	0x7d, /* ge  */
	0x78, /* lz  */
	0x79, /* gez */
	0x7a, /* p   */
	0x7b, /* np  */
	0x70, /* o  */
	0x71, /* no  */
];

const X86_NOBASEREG : X86_Reg_No = 0xFF;

/*
// bitvector mask for callee-saved registers
*/
const X86_ESI_MASK: i32 = 1<<X86_ESI;
const X86_EDI_MASK: i32 = 1<<X86_EDI;
const X86_EBX_MASK: i32 = 1<<X86_EBX;
const X86_EBP_MASK: i32 = 1<<X86_EBP;

const X86_CALLEE_REGS: i32 = (1<<X86_EAX) | (1<<X86_ECX) | (1<<X86_EDX);
const X86_CALLER_REGS: i32 = (1<<X86_EBX) | (1<<X86_EBP) | (1<<X86_ESI) | (1<<X86_EDI);
const X86_BYTE_REGS  : i32 = (1<<X86_EAX) | (1<<X86_ECX) | (1<<X86_EDX) | (1<<X86_EBX);

fn X86_IS_SCRATCH(reg: X86_Reg_No) -> bool {
    X86_CALLER_REGS & (1 << (reg)) != 0 /* X86_EAX, X86_ECX, or X86_EDX */
}

fn X86_IS_CALLEE(reg: X86_Reg_No) -> bool {
    X86_CALLEE_REGS & (1 << (reg)) != 0 	/* X86_ESI, X86_EDI, X86_EBX, or X86_EBP */
}

fn X86_IS_BYTE_REG(reg: X86_Reg_No) -> bool {
    reg < 4
}


/*
// Frame structure:
//
//      +--------------------------------+
//      | in_arg[0]       = var[0]	     |
//      | in_arg[1]	      = var[1]	     |
//      |	      . . .			         |
//      | in_arg[n_arg-1] = var[n_arg-1] |
//      +--------------------------------+
//      |       return IP                |
//      +--------------------------------+
//      |       saved EBP                | <-- frame pointer (EBP)
//      +--------------------------------+
//      |            ...                 |  n_extra
//      +--------------------------------+
//      |	    var[n_arg]	             |
//      |	    var[n_arg+1]             |  local variables area
//      |          . . .                 |
//      |	    var[n_var-1]             | 
//      +--------------------------------+
//      |			                     |
//      |			                     |  
//      |		spill area               | area for spilling mimic stack
//      |			                     |
//      +--------------------------------|
//      |          ebx                   |
//      |          ebp [ESP_Frame only]  |
//      |	       esi                   |  0..3 callee-saved regs
//      |          edi                   | <-- stack pointer (ESP)
//      +--------------------------------+
//      |	stk0	                     |
//      |	stk1	                     |  operand stack area/
//      |	. . .	                     |  out args
//      |	stkn-1	                     |
//      +--------------------------------|
//
//
*/


/*
 * useful building blocks
 */
fn x86_modrm_mod(modrm: i32) -> i32 {
    (modrm) >> 6
}
fn x86_modrm_reg(modrm: i32) -> i32 {
    ((modrm) >> 3) & 0x7
}
fn x86_modrm_rm(modrm: i32) -> i32 {
    (modrm) & 0x7
}

fn x86_address_byte(inst: &mut Emit, m: u8, o: u8, r: u8) {
    inst.emit(((((m)&0x03)<<6)|(((o)&0x07)<<3)|(((r)&0x07))));
}

fn x86_imm_emit32(inst: &mut Emit, imm: i32) {
    let imb = unsafe { transmute::<_, [u8; 4]>(imm) };
    inst.emit(imb [0]);
    inst.emit(imb [1]);
    inst.emit(imb [2]);
    inst.emit(imb [3]);
}

fn x86_imm_emit32_at(inst: &mut Emit, pos: i32, imm: i32) {
    let imb = unsafe { transmute::<_, [u8; 4]>(imm) };
    inst.emit_at(imb [0], pos);
    inst.emit_at(imb [1], pos + 1);
    inst.emit_at(imb [2], pos + 2);
    inst.emit_at(imb [3], pos + 3);
}

// TODO: inst is the offset into the stream!
fn x86_imm_emit16(inst: &mut Emit, imm: i32) {
    let imb = unsafe { transmute::<_, [u8; 2]>(imm as i16) };
    inst.emit(imb [0]);
    inst.emit(imb [1]);
}

fn x86_imm_emit8(inst: &mut Emit, imm: i32) {
    inst.emit(imm as u8);
}

fn x86_imm_emit8_at(inst: &mut Emit, pos: i32, imm: i32) {
    inst.emit_at(imm as u8, pos);
}

fn x86_is_imm8(imm: i32) -> bool {
    ((imm) >= -128 && (imm) <= 127)
}

fn x86_is_imm16(imm: i32) -> bool {
    ((imm) >= -(1<<16) && (imm) <= ((1<<16)-1))
}

fn x86_reg_emit(inst: &mut Emit, r: u8, regno: u8) {
    x86_address_byte ((inst), 3, (r), (regno));
}

fn x86_reg8_emit(inst: &mut Emit, r: u8, regno: u8, is_rh: bool, is_rnoh: bool) {
    x86_address_byte ((inst), 3, if (is_rh) { ((r)|4) } else { (r) }, if (is_rnoh) { ((regno)|4) } else { (regno) });
}

fn x86_regp_emit(inst: &mut Emit, r: u8, regno: u8) {
    x86_address_byte ((inst), 0, (r), (regno));
}

fn x86_mem_emit(inst: &mut Emit, r: u8, disp: i32) {
    x86_address_byte ((inst), 0, (r), 5);
    x86_imm_emit32((inst), (disp));
}

fn x86_membase_emit(inst: &mut Emit, r: u8, basereg: u8, disp: i32) {
    if ((basereg) == X86_ESP) {
        if ((disp) == 0) {
            x86_address_byte ((inst), 0, (r), X86_ESP);
            x86_address_byte ((inst), 0, X86_ESP, X86_ESP);
        } else if (x86_is_imm8((disp))) {
            x86_address_byte ((inst), 1, (r), X86_ESP);
            x86_address_byte ((inst), 0, X86_ESP, X86_ESP);
            x86_imm_emit8 ((inst), (disp));
        } else {
            x86_address_byte ((inst), 2, (r), X86_ESP);
            x86_address_byte ((inst), 0, X86_ESP, X86_ESP);
            x86_imm_emit32 ((inst), (disp));
        }
        return;
    }
    
    if ((disp) == 0 && (basereg) != X86_EBP) {
        x86_address_byte ((inst), 0, (r), (basereg));
        return;
    }
    
    if (x86_is_imm8((disp))) {
        x86_address_byte ((inst), 1, (r), (basereg));
        x86_imm_emit8 ((inst), (disp));
    } else {
        x86_address_byte ((inst), 2, (r), (basereg));
        x86_imm_emit32 ((inst), (disp));
    }
}

fn x86_memindex_emit(inst: &mut Emit, r: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    if ((basereg) == X86_NOBASEREG) {
        x86_address_byte ((inst), 0, (r), 4);
        x86_address_byte ((inst), (shift), (indexreg), 5);
        x86_imm_emit32 ((inst), (disp));
    } else if ((disp) == 0 && (basereg) != X86_EBP) {
        x86_address_byte ((inst), 0, (r), 4);
        x86_address_byte ((inst), (shift), (indexreg), (basereg));
    } else if (x86_is_imm8((disp))) {
        x86_address_byte ((inst), 1, (r), 4);
        x86_address_byte ((inst), (shift), (indexreg), (basereg));
        x86_imm_emit8 ((inst), (disp));
    } else {
        x86_address_byte ((inst), 2, (r), 4);
        x86_address_byte ((inst), (shift), (indexreg), (basereg));
        x86_imm_emit32 ((inst), (disp));
    }
}

/*
 * target is the position in the code where to jump to:
 * target = code;
 * .. output loop code...
 * x86_mov_reg_imm (code, X86_EAX, 0);
 * loop = code;
 * x86_loop (code, -1);
 * ... finish method
 *
 * patch displacement
 * x86_patch (loop, target);
 *
 * ins should point at the start of the instruction that encodes a target.
 * the instruction is inspected for validity and the correct displacement
 * is inserted.
 */
fn x86_patch(inst: &mut Emit, target: i32) {
    let mut pos = 1;
    let disp;
    let mut size = 0;
    
    match inst.get() {
        0xe8 | 0xe9 => {
            size += 1;
        }
        /* call, jump32 */
        0x0f => {
            if (!(inst.get_at(pos) >= 0x70 && inst.get_at(pos) <= 0x8f)) {
                jit_assert! ();
            }
            size += 1;
            pos += 1;
        }
            
        /* prefix for 32-bit disp */
        0xe0 | 0xe1 | 0xe2 /* loop */ | 0xeb /* jump8 *//* conditional jump opcodes */ |
        0x70 | 0x71 | 0x72 | 0x73 | 0x74 | 0x75 | 0x76 | 0x77 | 0x78 | 0x79 | 0x7a | 
        0x7b | 0x7c | 0x7d | 0x7e | 0x7f => {}
        _ => jit_assert! ()
    }
    
    disp = (target) - pos;
    if (size != 0) {
        x86_imm_emit32_at (inst, pos, disp - 4);
    } else if (x86_is_imm8 (disp - 1)) {
        x86_imm_emit8_at (inst, pos, disp - 1);
    } else {
        jit_assert! ();
    }
}

fn x86_breakpoint(inst: &mut Emit) {
    inst.emit(0xcc);
}

fn x86_cld(inst: &mut Emit) {
    inst.emit(0xfc);
}

fn x86_stosb(inst: &mut Emit) {
    inst.emit(0xaa);
}

fn x86_stosl(inst: &mut Emit) {
    inst.emit(0xab);
}

fn x86_stosd(inst: &mut Emit) {
    x86_stosl((inst));
}

fn x86_movsb(inst: &mut Emit) {
    inst.emit(0xa4);
}

fn x86_movsl(inst: &mut Emit) {
    inst.emit(0xa5);
}

fn x86_movsd(inst: &mut Emit) {
    x86_movsl((inst));
}

fn x86_prefix(inst: &mut Emit, p: u8) {
    inst.emit( (p));
}

fn x86_rdtsc(inst: &mut Emit) {
    inst.emit(0x0f);
    inst.emit(0x31);
}

fn x86_cmpxchg_reg_reg(inst: &mut Emit, dreg: u8, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xb1);
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_cmpxchg_mem_reg(inst: &mut Emit, mem: i32, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xb1);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_cmpxchg_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xb1);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_xchg_reg_reg(inst: &mut Emit, dreg: u8, reg: u8, size: i32) {
    if ((size) == 1){
        inst.emit(0x86);
    } else {
        inst.emit(0x87);
    }
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_xchg_mem_reg(inst: &mut Emit, mem: i32, reg: u8, size: i32) {
    if ((size) == 1) {
        inst.emit(0x86);
    } else {
        inst.emit(0x87);
    }
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_xchg_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, reg: u8, size: i32) {
    if ((size) == 1) {
        inst.emit(0x86);
    } else {
        inst.emit(0x87);
    }
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_xadd_reg_reg(inst: &mut Emit, dreg: u8, reg: u8, size: i32) {
    inst.emit(0x0F);
    if ((size) == 1) {
        inst.emit(0xC0);
    } else {
        inst.emit(0xC1);
    }
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_xadd_mem_reg(inst: &mut Emit, mem: i32, reg: u8, size: i32) {
    inst.emit(0x0F);
    if ((size) == 1) {
        inst.emit(0xC0);
    } else {
        inst.emit(0xC1);
    }
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_xadd_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, reg: u8, size: i32) {
    inst.emit(0x0F);
    if ((size) == 1) {
        inst.emit(0xC0);
    } else {
        inst.emit(0xC1);
    }
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_inc_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xff);
    x86_mem_emit ((inst), 0, (mem));
}

fn x86_inc_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xff);
    x86_membase_emit ((inst), 0, (basereg), (disp));
}

fn x86_inc_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0x40 + (reg));
}

fn x86_dec_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xff);
    x86_mem_emit ((inst), 1, (mem));
}

fn x86_dec_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xff);
    x86_membase_emit ((inst), 1, (basereg), (disp));
}

fn x86_dec_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0x48 + (reg));
}

fn x86_not_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xf7);
    x86_mem_emit ((inst), 2, (mem));
}

fn x86_not_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xf7);
    x86_membase_emit ((inst), 2, (basereg), (disp));
}

fn x86_not_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0xf7);
    x86_reg_emit ((inst), 2, (reg));
}

fn x86_neg_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xf7);
    x86_mem_emit ((inst), 3, (mem));
}

fn x86_neg_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xf7);
    x86_membase_emit ((inst), 3, (basereg), (disp));
}

fn x86_neg_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0xf7);
    x86_reg_emit ((inst), 3, (reg));
}

fn x86_nop(inst: &mut Emit) {
    inst.emit(0x90);
}

fn x86_alu_reg_imm(inst: &mut Emit, opc: u8, reg: u8, imm: i32) {
    if ((reg) == X86_EAX) {
        inst.emit((((opc)) << 3) + 5);
        x86_imm_emit32 ((inst), (imm));
        return;
    }
    if (x86_is_imm8((imm))) {
        inst.emit(0x83);
        x86_reg_emit ((inst), (opc), (reg));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x81);
        x86_reg_emit ((inst), (opc), (reg));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_alu_reg16_imm(inst: &mut Emit, opc: u8, reg: u8, imm: i32) {
    inst.emit(0x66);
    if ((reg) == X86_EAX) {
        inst.emit((((opc)) << 3) + 5);
        x86_imm_emit16 ((inst), (imm));
        return;
    }
    if (x86_is_imm8((imm))) {
        inst.emit(0x83);
        x86_reg_emit ((inst), (opc), (reg));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x81);
        x86_reg_emit ((inst), (opc), (reg));
        x86_imm_emit16 ((inst), (imm));
    }
}

fn x86_alu_mem_imm(inst: &mut Emit, opc: u8, mem: i32, imm: i32) {
    if (x86_is_imm8((imm))) {
        inst.emit(0x83);
        x86_mem_emit ((inst), (opc), (mem));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x81);
        x86_mem_emit ((inst), (opc), (mem));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_alu_membase_imm(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, imm: i32) {
    if (x86_is_imm8((imm))) {
        inst.emit(0x83);
        x86_membase_emit ((inst), (opc), (basereg), (disp));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x81);
        x86_membase_emit ((inst), (opc), (basereg), (disp));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_alu_membase8_imm(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, imm: i32) {
    inst.emit(0x80);
    x86_membase_emit ((inst), (opc), (basereg), (disp));
    x86_imm_emit8 ((inst), (imm));
}

fn x86_alu_mem_reg(inst: &mut Emit, opc: u8, mem: i32, reg: u8) {
    inst.emit((((opc)) << 3) + 1);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_alu_membase_reg(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, reg: u8) {
    inst.emit((((opc)) << 3) + 1);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_alu_reg_reg(inst: &mut Emit, opc: u8, dreg: u8, reg: u8) {
    inst.emit((((opc)) << 3) + 3);
    x86_reg_emit ((inst), (dreg), (reg));
}

/**
 * @x86_alu_reg8_reg8:
 * Supports ALU operations between two 8-bit registers.
 * dreg := dreg opc reg
 * X86_Reg_No enum is used to specify the registers.
 * Additionally is_*_h flags are used to specify what part
 * of a given 32-bit register is used - high (TRUE) or low (FALSE).
 * For example: dreg = X86_EAX, is_dreg_h = TRUE -> use AH
 */
fn x86_alu_reg8_reg8(inst: &mut Emit, opc: u8, dreg: u8, reg: u8, is_dreg_h: bool, is_reg_h: bool) {
    inst.emit((((opc)) << 3) + 2);
    x86_reg8_emit ((inst), (dreg), (reg), (is_dreg_h), (is_reg_h));
}

fn x86_alu_reg_mem(inst: &mut Emit, opc: u8, reg: u8, mem: i32) {
    inst.emit((((opc)) << 3) + 3);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_alu_reg_membase(inst: &mut Emit, opc: u8, reg: u8, basereg: u8, disp: i32) {
    inst.emit((((opc)) << 3) + 3);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_test_reg_imm(inst: &mut Emit, reg: u8, imm: i32) {
    if ((reg) == X86_EAX) {
        inst.emit(0xa9);
    } else {
        inst.emit(0xf7);
        x86_reg_emit ((inst), 0, (reg));
    }
    x86_imm_emit32 ((inst), (imm));
}

fn x86_test_mem_imm(inst: &mut Emit, mem: i32, imm: i32) {
    inst.emit(0xf7);
    x86_mem_emit ((inst), 0, (mem));
    x86_imm_emit32 ((inst), (imm));
}

fn x86_test_membase_imm(inst: &mut Emit, basereg: u8, disp: i32, imm: i32) {
    inst.emit(0xf7);
    x86_membase_emit ((inst), 0, (basereg), (disp));
    x86_imm_emit32 ((inst), (imm));
}

fn x86_test_reg_reg(inst: &mut Emit, dreg: u8, reg: u8) {
    inst.emit(0x85);
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_test_mem_reg(inst: &mut Emit, mem: i32, reg: u8) {
    inst.emit(0x85);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_test_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, reg: u8) {
    inst.emit(0x85);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_shift_reg_imm(inst: &mut Emit, opc: u8, reg: u8, imm: i32) {
    if ((imm) == 1) {
        inst.emit(0xd1);
        x86_reg_emit ((inst), (opc), (reg));
    } else {
        inst.emit(0xc1);
        x86_reg_emit ((inst), (opc), (reg));
        x86_imm_emit8 ((inst), (imm));
    }
}

fn x86_shift_mem_imm(inst: &mut Emit, opc: u8, mem: i32, imm: i32) {
    if ((imm) == 1) {
        inst.emit(0xd1);
        x86_mem_emit ((inst), (opc), (mem));
    } else {
        inst.emit(0xc1);
        x86_mem_emit ((inst), (opc), (mem));
        x86_imm_emit8 ((inst), (imm));
    }
}

fn x86_shift_membase_imm(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, imm: i32) {
    if ((imm) == 1) {
        inst.emit(0xd1);
        x86_membase_emit ((inst), (opc), (basereg), (disp));
    } else {
        inst.emit(0xc1);
        x86_membase_emit ((inst), (opc), (basereg), (disp));
        x86_imm_emit8 ((inst), (imm));
    }
}

fn x86_shift_reg(inst: &mut Emit, opc: u8, reg: u8) {
    inst.emit(0xd3);
    x86_reg_emit ((inst), (opc), (reg));
}

fn x86_shift_mem(inst: &mut Emit, opc: u8, mem: i32) {
    inst.emit(0xd3);
    x86_mem_emit ((inst), (opc), (mem));
}

fn x86_shift_membase(inst: &mut Emit, opc: u8, basereg: u8, disp: i32) {
    inst.emit(0xd3);
    x86_membase_emit ((inst), (opc), (basereg), (disp));
}

/*
 * Multi op shift missing.
 */

fn x86_shrd_reg(inst: &mut Emit, dreg: u8, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xad);
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_shrd_reg_imm(inst: &mut Emit, dreg: u8, reg: u8, shamt: i32) {
    inst.emit(0x0f);
    inst.emit(0xac);
    x86_reg_emit ((inst), (reg), (dreg));
    x86_imm_emit8 ((inst), (shamt));
}

fn x86_shld_reg(inst: &mut Emit, dreg: u8, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xa5);
    x86_reg_emit ((inst), (reg), (dreg));
}

fn x86_shld_reg_imm(inst: &mut Emit, dreg: u8, reg: u8, shamt: i32) {
    inst.emit(0x0f);
    inst.emit(0xa4);
    x86_reg_emit ((inst), (reg), (dreg));
    x86_imm_emit8 ((inst), (shamt));
}

/*
 * EDX:EAX = EAX * rm
 */
fn x86_mul_reg(inst: &mut Emit, reg: u8, is_signed: bool) {
    inst.emit(0xf7);
    x86_reg_emit ((inst), 4 + if (is_signed) { 1 } else { 0 }, (reg));
}

fn x86_mul_mem(inst: &mut Emit, mem: i32, is_signed: bool) {
    inst.emit(0xf7);
    x86_mem_emit ((inst), 4 + if (is_signed) { 1 } else { 0 }, (mem));
}

fn x86_mul_membase(inst: &mut Emit, basereg: u8, disp: i32, is_signed: bool) {
    inst.emit(0xf7);
    x86_membase_emit ((inst), 4 + if (is_signed) { 1 } else { 0 }, (basereg), (disp));
}

/*
 * r *= rm
 */
fn x86_imul_reg_reg(inst: &mut Emit, dreg: u8, reg: u8) {
    inst.emit(0x0f);
    inst.emit(0xaf);
    x86_reg_emit ((inst), (dreg), (reg));
}

fn x86_imul_reg_mem(inst: &mut Emit, reg: u8, mem: i32) {
    inst.emit(0x0f);
    inst.emit(0xaf);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_imul_reg_membase(inst: &mut Emit, reg: u8, basereg: u8, disp: i32) {
    inst.emit(0x0f);
    inst.emit(0xaf);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

/*
 * dreg = rm * imm
 */
fn x86_imul_reg_reg_imm(inst: &mut Emit, dreg: u8, reg: u8, imm: i32) {
    if (x86_is_imm8 ((imm))) {
        inst.emit(0x6b);
        x86_reg_emit ((inst), (dreg), (reg));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x69);
        x86_reg_emit ((inst), (dreg), (reg));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_imul_reg_mem_imm(inst: &mut Emit, reg: u8, mem: i32, imm: i32) {
    if (x86_is_imm8 ((imm))) {
        inst.emit(0x6b);
        x86_mem_emit ((inst), (reg), (mem));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x69);
        x86_reg_emit ((inst), (reg), (mem) as u8);
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_imul_reg_membase_imm(inst: &mut Emit, reg: u8, basereg: u8, disp: i32, imm: i32) {
    if (x86_is_imm8 ((imm))) {
        inst.emit(0x6b);
        x86_membase_emit ((inst), (reg), (basereg), (disp));
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x69);
        x86_membase_emit ((inst), (reg), (basereg), (disp));
        x86_imm_emit32 ((inst), (imm));
    }
}

/*
 * divide EDX:EAX by rm;
 * eax = quotient, edx = remainder
 */

fn x86_div_reg(inst: &mut Emit, reg: u8, is_signed: bool) {
    inst.emit(0xf7);
    x86_reg_emit ((inst), 6 + if (is_signed) { 1 } else { 0 }, (reg));
}

fn x86_div_mem(inst: &mut Emit, mem: i32, is_signed: bool) {
    inst.emit(0xf7);
    x86_mem_emit ((inst), 6 + if is_signed { 1 } else { 0 }, (mem));
}

fn x86_div_membase(inst: &mut Emit, basereg: u8, disp: i32, is_signed: bool) {
    inst.emit(0xf7);
    x86_membase_emit ((inst), 6 + if is_signed { 1 } else { 0 }, (basereg), (disp));
}

fn x86_mov_mem_reg(inst: &mut Emit, mem: i32, reg: u8, size: i32) {
    match size {
        1 => inst.emit(0x88),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x89);
        }
        _ => jit_assert! ()
    }
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_mov_regp_reg(inst: &mut Emit, regp: u8, reg: u8, size: i32) {
    match size {
        1 => inst.emit(0x88),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x89);
        }
        _ => jit_assert! ()
    }
    x86_regp_emit ((inst), (reg), (regp));
}

fn x86_mov_membase_reg(inst: &mut Emit, basereg: u8, disp: i32, reg: u8, size: i32) {
    match size {
        1 => inst.emit(0x88),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x89);
        }
        _ => jit_assert! ()
    }
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_mov_memindex_reg(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, reg: u8, size: i32) {
    match size {
        1 => inst.emit(0x88),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x89);
        }
        _ => jit_assert! ()
    }
    x86_memindex_emit ((inst), (reg), (basereg), (disp), (indexreg), (shift));
}

fn x86_mov_reg_reg(inst: &mut Emit, dreg: u8, reg: u8, size: i32) {
    match size {
        1 => inst.emit(0x8a),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x8b);
        }
        _ => jit_assert! ()
    }
    x86_reg_emit ((inst), (dreg), (reg));
}

fn x86_mov_reg_mem(inst: &mut Emit, reg: u8, mem: i32, size: i32) {
    match size {
        1 => inst.emit(0x8a),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x8b);
        }
        _ => jit_assert! ()
    }
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_mov_reg_membase(inst: &mut Emit, reg: u8, basereg: u8, disp: i32, size: i32) {
    match size {
        1 => inst.emit(0x8a),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x8b);
        }
        _ => jit_assert! ()
    }
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_mov_reg_memindex(inst: &mut Emit, reg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, size: i32) {
    match size {
        1 => inst.emit(0x8a),
        2 | 4 => {
            if size == 2 {
                inst.emit(0x66);
            }
            inst.emit(0x8b);
        }
        _ => jit_assert! ()
    }
    x86_memindex_emit ((inst), (reg), (basereg), (disp), (indexreg), (shift));
}

/*
 * Note: x86_clear_reg () chacnges the condition code!
 */
fn x86_clear_reg(inst: &mut Emit, reg: u8) {
    x86_alu_reg_reg((inst), X86_XOR, (reg), (reg));
}

fn x86_mov_reg_imm(inst: &mut Emit, reg: u8, imm: i32) {
    inst.emit(0xb8 + (reg));
    x86_imm_emit32 ((inst), (imm));
}

fn x86_mov_mem_imm(inst: &mut Emit, mem: i32, imm: i32, size: i32) {
    if ((size) == 1) {
        inst.emit(0xc6);
        x86_mem_emit ((inst), 0, (mem));
        x86_imm_emit8 ((inst), (imm));
    } else if ((size) == 2) {
        inst.emit(0x66);
        inst.emit(0xc7);
        x86_mem_emit ((inst), 0, (mem));
        x86_imm_emit16 ((inst), (imm));
    } else {
        inst.emit(0xc7);
        x86_mem_emit ((inst), 0, (mem));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_mov_membase_imm(inst: &mut Emit, basereg: u8, disp: i32, imm: i32, size: i32) {
    if ((size) == 1) {
        inst.emit(0xc6);
        x86_membase_emit ((inst), 0, (basereg), (disp));
        x86_imm_emit8 ((inst), (imm));
    } else if ((size) == 2) {
        inst.emit(0x66);
        inst.emit(0xc7);
        x86_membase_emit ((inst), 0, (basereg), (disp));
        x86_imm_emit16 ((inst), (imm));
    } else {
        inst.emit(0xc7);
        x86_membase_emit ((inst), 0, (basereg), (disp));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_mov_memindex_imm(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, imm: i32, size: i32) {
    if ((size) == 1) {
        inst.emit(0xc6);
        x86_memindex_emit ((inst), 0, (basereg), (disp), (indexreg), (shift));
        x86_imm_emit8 ((inst), (imm));
    } else if ((size) == 2) {
        inst.emit(0x66);
        inst.emit(0xc7);
        x86_memindex_emit ((inst), 0, (basereg), (disp), (indexreg), (shift));
        x86_imm_emit16 ((inst), (imm));
    } else {
        inst.emit(0xc7);
        x86_memindex_emit ((inst), 0, (basereg), (disp), (indexreg), (shift));
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_lea_mem(inst: &mut Emit, reg: u8, mem: i32) {
    inst.emit(0x8d);
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_lea_membase(inst: &mut Emit, reg: u8, basereg: u8, disp: i32) {
    inst.emit(0x8d);
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_lea_memindex(inst: &mut Emit, reg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    inst.emit(0x8d);
    x86_memindex_emit ((inst), (reg), (basereg), (disp), (indexreg), (shift));
}

fn x86_widen_reg(inst: &mut Emit, dreg: u8, reg: u8, is_signed: bool, is_half: bool) {
    let mut op = 0xb6;
    jit_assert! (is_half ||  X86_IS_BYTE_REG (reg));
    inst.emit(0x0f);
    if ((is_signed)) {
        op += 0x08;
    }
    if ((is_half)) {
        op += 0x01;
    }
    inst.emit(op);
    x86_reg_emit ((inst), (dreg), (reg));
}

fn x86_widen_mem(inst: &mut Emit, dreg: u8, mem: i32, is_signed: bool, is_half: bool) {
    let mut op = 0xb6;
    inst.emit(0x0f);
    if ((is_signed)) {
        op += 0x08;
    }
    if ((is_half)) {
        op += 0x01;
    }
    inst.emit(op);
    x86_mem_emit ((inst), (dreg), (mem));
}

fn x86_widen_membase(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, is_signed: bool, is_half: bool) {
    let mut op = 0xb6;
    inst.emit(0x0f);
    if ((is_signed)) {
        op += 0x08;
    }
    if ((is_half)) {
        op += 0x01;
    }
    inst.emit(op);
    x86_membase_emit ((inst), (dreg), (basereg), (disp));
}

fn x86_widen_memindex(inst: &mut Emit, dreg: u8, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_signed: bool, is_half: bool) {
    let mut op = 0xb6;
    inst.emit(0x0f);
    if ((is_signed)) {
        op += 0x08;
    }
    if ((is_half)) {
        op += 0x01;
    }
    inst.emit(op);
    x86_memindex_emit ((inst), (dreg), (basereg), (disp), (indexreg), (shift));
}

fn x86_cdq(inst: &mut Emit) {
    inst.emit(0x99);
}

fn x86_wait(inst: &mut Emit) {
    inst.emit(0x9b);
}

fn x86_fp_op_mem(inst: &mut Emit, opc: u8, mem: i32, is_double: bool) {
    inst.emit(if (is_double) { 0xdc } else { 0xd8 });
    x86_mem_emit ((inst), (opc), (mem));
}

fn x86_fp_op_membase(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, is_double: bool) {
    inst.emit(if (is_double) { 0xdc } else { 0xd8 });
    x86_membase_emit ((inst), (opc), (basereg), (disp));
}

fn x86_fp_op(inst: &mut Emit, opc: u8, index: u8) {
    inst.emit(0xd8);
    inst.emit(0xc0+((opc)<<3)+((index)&0x07));
}

const x86_fp_op_reg_map : [u8; 9] = [0, 1, 2, 3, 5, 4, 7, 6, 8];

fn x86_fp_op_reg(inst: &mut Emit, opc: u8, index: u8, pop_stack: bool) {
    inst.emit(if (pop_stack) { 0xde } else { 0xdc });
    inst.emit(0xc0+(x86_fp_op_reg_map[(opc) as usize]<<3)+((index)&0x07));
}

/**
 * @x86_fp_int_op_membase
 * Supports FPU operations between ST(0) and integer operand in memory.
 * Operation encoded using X86_FP_Opcode enum.
 * Operand is addressed by [basereg + disp].
 * is_int specifies whether operand is int32 (TRUE) or int16 (FALSE).
 */
fn x86_fp_int_op_membase(inst: &mut Emit, opc: u8, basereg: u8, disp: i32, is_int: bool) {
    inst.emit(if (is_int) { 0xda } else { 0xde });
    x86_membase_emit ((inst), opc, (basereg), (disp));
}

fn x86_fstp(inst: &mut Emit, index: u8) {
    inst.emit(0xdd);
    inst.emit(0xd8+(index));
}

fn x86_fcompp(inst: &mut Emit) {
    inst.emit(0xde);
    inst.emit(0xd9);
}

fn x86_fucompp(inst: &mut Emit) {
    inst.emit(0xda);
    inst.emit(0xe9);
}

fn x86_fnstsw(inst: &mut Emit) {
    inst.emit(0xdf);
    inst.emit(0xe0);
}

fn x86_fnstcw(inst: &mut Emit, mem: i32) {
    inst.emit(0xd9);
    x86_mem_emit ((inst), 7, (mem));
}

fn x86_fnstcw_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xd9);
    x86_membase_emit ((inst), 7, (basereg), (disp));
}

fn x86_fldcw(inst: &mut Emit, mem: i32) {
    inst.emit(0xd9);
    x86_mem_emit ((inst), 5, (mem));
}

fn x86_fldcw_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xd9);
    x86_membase_emit ((inst), 5, (basereg), (disp));
}

fn x86_fchs(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xe0);
}

fn x86_frem(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xf8);
}

fn x86_fxch(inst: &mut Emit, index: u8) {
    inst.emit(0xd9);
    inst.emit(0xc8 + ((index) & 0x07));
}

fn x86_fcomi(inst: &mut Emit, index: u8) {
    inst.emit(0xdb);
    inst.emit(0xf0 + ((index) & 0x07));
}

fn x86_fcomip(inst: &mut Emit, index: u8) {
    inst.emit(0xdf);
    inst.emit(0xf0 + ((index) & 0x07));
}

fn x86_fucomi(inst: &mut Emit, index: u8) {
    inst.emit(0xdb);
    inst.emit(0xe8 + ((index) & 0x07));
}

fn x86_fucomip(inst: &mut Emit, index: u8) {
    inst.emit(0xdf);
    inst.emit(0xe8 + ((index) & 0x07));
}

fn x86_fld(inst: &mut Emit, mem: i32, is_double: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_mem_emit ((inst), 0, (mem));
}

fn x86_fld_membase(inst: &mut Emit, basereg: u8, disp: i32, is_double: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_membase_emit ((inst), 0, (basereg), (disp));
}

fn x86_fld_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_double: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_memindex_emit ((inst), 0, (basereg), (disp), (indexreg), (shift));
}

fn x86_fld80_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xdb);
    x86_mem_emit ((inst), 5, (mem));
}

fn x86_fld80_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xdb);
    x86_membase_emit ((inst), 5, (basereg), (disp));
}

fn x86_fld80_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    inst.emit(0xdb);
    x86_memindex_emit ((inst), 5, (basereg), (disp), (indexreg), (shift));
}

fn x86_fild(inst: &mut Emit, mem: i32, is_long: bool) {
    if ((is_long)) {
        inst.emit(0xdf);
        x86_mem_emit ((inst), 5, (mem));
    } else {
        inst.emit(0xdb);
        x86_mem_emit ((inst), 0, (mem));
    }
}

fn x86_fild_membase(inst: &mut Emit, basereg: u8, disp: i32, is_long: bool) {
    if ((is_long)) {
        inst.emit(0xdf);
        x86_membase_emit ((inst), 5, (basereg), (disp));
    } else {
        inst.emit(0xdb);
        x86_membase_emit ((inst), 0, (basereg), (disp));
    }
}

fn x86_fld_reg(inst: &mut Emit, index: u8) {
    inst.emit(0xd9);
    inst.emit(0xc0 + ((index) & 0x07));
}

fn x86_fldz(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xee);
}

fn x86_fld1(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xe8);
}

fn x86_fldpi(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xeb);
}

fn x86_fst(inst: &mut Emit, mem: i32, is_double: bool, pop_stack: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_mem_emit ((inst), 2 + if pop_stack { 1 } else { 0 }, (mem));
}

fn x86_fst_membase(inst: &mut Emit, basereg: u8, disp: i32, is_double: bool, pop_stack: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_membase_emit ((inst), 2 + if pop_stack { 1 } else { 0 }, (basereg), (disp));
}

fn x86_fst_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8, is_double: bool, pop_stack: bool) {
    inst.emit(if (is_double) { 0xdd } else { 0xd9 });
    x86_memindex_emit ((inst), 2 + if pop_stack { 1 } else { 0 }, (basereg), (disp), (indexreg), (shift));
}

fn x86_fst80_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xdb);
    x86_mem_emit ((inst), 7, (mem));
}

fn x86_fst80_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xdb);
    x86_membase_emit ((inst), 7, (basereg), (disp));
}

fn x86_fst80_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    inst.emit(0xdb);
    x86_memindex_emit ((inst), 7, (basereg), (disp), (indexreg), (shift));
}

fn x86_fist_pop(inst: &mut Emit, mem: i32, is_long: bool) {
    if ((is_long)) {
        inst.emit(0xdf);
        x86_mem_emit ((inst), 7, (mem));
    } else {
        inst.emit(0xdb);
        x86_mem_emit ((inst), 3, (mem));
    }
}

fn x86_fist_pop_membase(inst: &mut Emit, basereg: u8, disp: i32, is_long: bool) {
    if ((is_long)) {
        inst.emit(0xdf);
        x86_membase_emit ((inst), 7, (basereg), (disp));
    } else {
        inst.emit(0xdb);
        x86_membase_emit ((inst), 3, (basereg), (disp));
    }
}

fn x86_fstsw(inst: &mut Emit) {
    inst.emit(0x9b);
    inst.emit(0xdf);
    inst.emit(0xe0);
}

/**
 * @x86_fist_membase
 * Converts content of ST(0) to integer and stores it at memory location
 * addressed by [basereg + disp].
 * is_int specifies whether destination is int32 (TRUE) or int16 (FALSE).
 */
fn x86_fist_membase(inst: &mut Emit, basereg: u8, disp: i32, is_int: bool) {
    if ((is_int)) {
        inst.emit(0xdb);
        x86_membase_emit ((inst), 2, (basereg), (disp));
    } else {
        inst.emit(0xdf);
        x86_membase_emit ((inst), 2, (basereg), (disp));
    }
}

fn x86_push_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0x50 + (reg));
}

fn x86_push_regp(inst: &mut Emit, reg: u8) {
    inst.emit(0xff);
    x86_regp_emit ((inst), 6, (reg));
}

fn x86_push_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xff);
    x86_mem_emit ((inst), 6, (mem));
}

fn x86_push_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xff);
    x86_membase_emit ((inst), 6, (basereg), (disp));
}

fn x86_push_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    inst.emit(0xff);
    x86_memindex_emit ((inst), 6, (basereg), (disp), (indexreg), (shift));
}

#[allow(overflowing_literals)]
fn x86_push_imm_template(inst: &mut Emit) {
    x86_push_imm (inst, 0xf0f0f0f0);
}
	
fn x86_push_imm(inst: &mut Emit, imm: i32) {
    if (x86_is_imm8 (imm)) {
        inst.emit(0x6A);
        x86_imm_emit8 ((inst), (imm));
    } else {
        inst.emit(0x68);
        x86_imm_emit32 ((inst), (imm));
    }
}

fn x86_pop_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0x58 + (reg));
}

fn x86_pop_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0x8f);
    x86_mem_emit ((inst), 0, (mem));
}

fn x86_pop_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0x8f);
    x86_membase_emit ((inst), 0, (basereg), (disp));
}

fn x86_pushad(inst: &mut Emit) {
    inst.emit(0x60);
}

fn x86_pushfd(inst: &mut Emit) {
    inst.emit(0x9c);
}

fn x86_popad(inst: &mut Emit) {
    inst.emit(0x61);
}

fn x86_popfd(inst: &mut Emit) {
    inst.emit(0x9d);
}

fn x86_loop(inst: &mut Emit, imm: i32) {
    inst.emit(0xe2);
    x86_imm_emit8 ((inst), (imm));
}

fn x86_loope(inst: &mut Emit, imm: i32) {
    inst.emit(0xe1);
    x86_imm_emit8 ((inst), (imm));
}

fn x86_loopne(inst: &mut Emit, imm: i32) {
    inst.emit(0xe0);
    x86_imm_emit8 ((inst), (imm));
}

fn x86_jump32(inst: &mut Emit, imm: i32) {
    inst.emit(0xe9);
    x86_imm_emit32 ((inst), (imm));
}

fn x86_jump8(inst: &mut Emit, imm: i32) {
    inst.emit(0xeb);
    x86_imm_emit8 ((inst), (imm));
}

fn x86_jump_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0xff);
    x86_reg_emit ((inst), 4, (reg));
}

fn x86_jump_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xff);
    x86_mem_emit ((inst), 4, (mem));
}

fn x86_jump_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xff);
    x86_membase_emit ((inst), 4, (basereg), (disp));
}

fn x86_jump_memindex(inst: &mut Emit, basereg: u8, disp: i32, indexreg: u8, shift: u8) {
    inst.emit(0xff);
    x86_memindex_emit ((inst), 4, (basereg), (disp), (indexreg), (shift));
}

/*
 * target is a pointer in our buffer.
 */
fn x86_jump_code(inst: &mut Emit, target: i32) {
    let mut t = (target) - 2;
    if (x86_is_imm8(t)) {
        x86_jump8 ((inst), t);
    } else {
        t -= 3;
        x86_jump32 ((inst), t);
    }
}

fn x86_jump_disp(inst: &mut Emit, disp: i32) {
    let mut t = (disp) - 2;
    if (x86_is_imm8(t)) {
        x86_jump8 ((inst), t);
    } else {
        t -= 3;
        x86_jump32 ((inst), t);
    }
}

fn x86_branch8(inst: &mut Emit, cond: i32, imm: i32, is_signed: bool) {
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize]);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize]);
    }
    x86_imm_emit8 ((inst), (imm));
}

fn x86_branch32(inst: &mut Emit, cond: i32, imm: i32, is_signed: bool) {
    inst.emit(0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] + 0x10);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] + 0x10);
    }
    x86_imm_emit32 ((inst), (imm));
}

fn x86_branch(inst: &mut Emit, cond: i32, target: i32, is_signed: bool) {
    let mut offset = (target) - 2;
    if (x86_is_imm8 ((offset))) {
        x86_branch8 ((inst), (cond), offset, (is_signed));
    } else {
        offset -= 4;
        x86_branch32 ((inst), (cond), offset, (is_signed));
    }
}

fn x86_branch_disp(inst: &mut Emit, cond: i32, disp: i32, is_signed: bool) {
    let mut offset = (disp) - 2;
    if (x86_is_imm8 ((offset))) {
        x86_branch8 ((inst), (cond), offset, (is_signed));
    } else {
        offset -= 4;
        x86_branch32 ((inst), (cond), offset, (is_signed));
    }
}

fn x86_set_reg(inst: &mut Emit, cond: i32, reg: u8, is_signed: bool) {
    jit_assert! (X86_IS_BYTE_REG (reg));
    inst.emit(0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] + 0x20);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] + 0x20);
    }
    x86_reg_emit ((inst), 0, (reg));
}

fn x86_set_mem(inst: &mut Emit, cond: i32, mem: i32, is_signed: bool) {
    inst.emit(0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] + 0x20);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] + 0x20);
    }
    x86_mem_emit ((inst), 0, (mem));
}

fn x86_set_membase(inst: &mut Emit, cond: i32, basereg: u8, disp: i32, is_signed: bool) {
    inst.emit(0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] + 0x20);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] + 0x20);
    }
    x86_membase_emit ((inst), 0, (basereg), (disp));
}

fn x86_call_imm(inst: &mut Emit, disp: i32) {
    inst.emit(0xe8);
    x86_imm_emit32 ((inst), (disp));
}

fn x86_call_reg(inst: &mut Emit, reg: u8) {
    inst.emit(0xff);
    x86_reg_emit ((inst), 2, (reg));
}

fn x86_call_mem(inst: &mut Emit, mem: i32) {
    inst.emit(0xff);
    x86_mem_emit ((inst), 2, (mem));
}

fn x86_call_membase(inst: &mut Emit, basereg: u8, disp: i32) {
    inst.emit(0xff);
    x86_membase_emit ((inst), 2, (basereg), (disp));
}

fn x86_call_code(inst: &mut Emit, target: i32) {
    let mut _x86_offset = (target);
    _x86_offset -= 5;
    x86_call_imm ((inst), _x86_offset);
}

fn x86_ret(inst: &mut Emit) {
    inst.emit(0xc3);
}

fn x86_ret_imm(inst: &mut Emit, imm: i32) {
    if ((imm) == 0) {
        x86_ret ((inst));
    } else {
        inst.emit(0xc2);
        x86_imm_emit16 ((inst), (imm));
    }
}

fn x86_cmov_reg(inst: &mut Emit, cond: i32, is_signed: bool, dreg: u8, reg: u8) {
    inst.emit( 0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] - 0x30);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] - 0x30);
    }
    x86_reg_emit ((inst), (dreg), (reg));
}

fn x86_cmov_mem(inst: &mut Emit, cond: i32, is_signed: bool, reg: u8, mem: i32) {
    inst.emit( 0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] - 0x30);
    }  else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] - 0x30);
    }
    x86_mem_emit ((inst), (reg), (mem));
}

fn x86_cmov_membase(inst: &mut Emit, cond: i32, is_signed: bool, reg: u8, basereg: u8, disp: i32) {
    inst.emit( 0x0f);
    if ((is_signed)) {
        inst.emit(x86_cc_signed_map [(cond) as usize] - 0x30);
    } else {
        inst.emit(x86_cc_unsigned_map [(cond) as usize] - 0x30);
    }
    x86_membase_emit ((inst), (reg), (basereg), (disp));
}

fn x86_enter(inst: &mut Emit, framesize: i32) {
    inst.emit(0xc8);
    x86_imm_emit16 ((inst), (framesize));
    inst.emit(0);
}

fn x86_leave(inst: &mut Emit) {
    inst.emit(0xc9);
}

fn x86_sahf(inst: &mut Emit) {
    inst.emit(0x9e);
}

fn x86_fsin(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xfe);
}

fn x86_fcos(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xff);
}

fn x86_fabs(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xe1);
}

fn x86_ftst(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xe4);
}

fn x86_fxam(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xe5);
}

fn x86_fpatan(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xf3);
}

fn x86_fprem(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xf8);
}

fn x86_fprem1(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xf5);
}

fn x86_frndint(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xfc);
}

fn x86_fsqrt(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xfa);
}

fn x86_fptan(inst: &mut Emit) {
    inst.emit(0xd9);
    inst.emit(0xf2);
}

fn x86_padding(inst: &mut Emit, size: i32) {
    match size {
        1 => {
            x86_nop ((inst));
        }
        2 => {
            inst.emit(0x8b);
            inst.emit(0xc0);
        }
        3 => {
            inst.emit(0x8d);
            inst.emit(0x6d);
            inst.emit(0x00);
        }
        4 => {
            inst.emit(0x8d);
            inst.emit(0x64);
            inst.emit(0x24);
            inst.emit(0x00);
        }
        5 => {
            inst.emit(0x8d);
            inst.emit(0x64);
            inst.emit(0x24);
            inst.emit(0x00);
        }
        6 => {
            inst.emit(0x8d);
            inst.emit(0xad);
            inst.emit(0x00);
            inst.emit(0x00);
            inst.emit(0x00);
            inst.emit(0x00);
        }
        7 => {
            inst.emit(0x8d);
            inst.emit(0xa4);
            inst.emit(0x24);
            inst.emit(0x00);
            inst.emit(0x00);
            inst.emit(0x00);
            inst.emit(0x00);
        }
        _ => jit_assert! ()
    }
}

fn x86_prolog(inst: &mut Emit, frame_size: i32, reg_mask: i32) {
    x86_enter ((inst), (frame_size));
    let mut m = 1;
    for i in 0..X86_NREG {
        if ((reg_mask) & m) != 0 {
            x86_push_reg ((inst), i);
        }
        m <<= 1;
    }
}

fn x86_epilog(inst: &mut Emit, reg_mask: i32) {
    let mut m = 1 << X86_EDI;
    let mut i = X86_EDI;
    while m != 0 {
        if ((reg_mask) & m) != 0 {
            x86_pop_reg ((inst), i);
        }
        i -= 1;
        m=m>>1;
    }
    x86_leave ((inst));
    x86_ret ((inst));
}
