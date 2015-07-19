#![allow(unused_parens)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

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

use codegen::Writer;
use std::mem::transmute;

/*
// x86 register numbers
*/

const X86_EAX : u8 = 0;
const X86_ECX : u8 = 1;
const X86_EDX : u8 = 2;
const X86_EBX : u8 = 3;
const X86_ESP : u8 = 4;
const X86_EBP : u8 = 5;
const X86_ESI : u8 = 6;
const X86_EDI : u8 = 7;
const X86_NREG : u8 = 8;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Reg {
    EAX = X86_EAX,
    ECX = X86_ECX,
    EDX = X86_EDX,
    EBX = X86_EBX,
    ESP = X86_ESP,
    EBP = X86_EBP,
    ESI = X86_ESI,
    EDI = X86_EDI,
    NOBASEREG = 0xFF    
}

impl Reg {
    pub fn value(&self) -> u8 {
        unsafe { transmute(*self) }
    }
    
    fn from_u8(value: u8) -> Reg {
        unsafe { transmute(value) }
    }
}

/*
// opcodes for alu instructions
*/

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum AluOp {
    ADD = 0,
    OR  = 1,
    ADC = 2,
    SBB = 3,
    AND = 4,
    SUB = 5,
    XOR = 6,
    CMP = 7
}

impl AluOp {
    fn value(&self) -> u8 {
        unsafe { transmute(*self) }
    }
}

/*
// opcodes for shift instructions
*/

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum ShiftOp {
    ROL = 0,
    ROR = 1,
    RCL = 2,
    RCR = 3,
    SHL = 4,
    SHR = 5,
    SAR = 7
}

impl ShiftOp {
    fn value(&self) -> u8 {
        unsafe { transmute(*self) }
    }
}

/*
// opcodes for floating-point instructions
*/

#[repr(u8)]
#[derive(Copy, Clone)]
pub enum FpOp {
    FADD = 0,
    FMUL = 1,
    FCOM = 2,
    FCOMP = 3,
    FSUB = 4,
    FSUBR = 5,
    FDIV = 6,
    FDIVR = 7
}

impl FpOp {
    fn value(&self) -> u8 {
        unsafe { transmute(*self) }
    }
}

/*
// integer conditions codes
*/

#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Cond {
    EQ = 0,
    NE = 1,
    LT = 2,
    LE = 3,
    GT = 4,
    GE = 5,
    LZ = 6,
    GEZ = 7,
    P = 8,
    NP = 9,
    O = 10,
    NO = 11
}

impl Cond {
    fn value(&self) -> usize {
        unsafe { transmute(*self) }
    }
}

/* FP status */

// These constants are not used.
//
//pub type X86_FP_Status = i32;
//
//pub const X86_FP_C0 : X86_FP_Status = 0x100;
//pub const X86_FP_C1 : X86_FP_Status = 0x200;
//pub const X86_FP_C2 : X86_FP_Status = 0x400;
//pub const X86_FP_C3 : X86_FP_Status = 0x4000;
//pub const X86_FP_CC_MASK : X86_FP_Status = 0x4500;

/* FP control word */

// These constants are not used.
//
//pub type X86_FP_ControlWord = i32;
//
//pub const X86_FPCW_INVOPEX_MASK : X86_FP_ControlWord = 0x1;
//pub const X86_FPCW_DENOPEX_MASK : X86_FP_ControlWord = 0x2;
//pub const X86_FPCW_ZERODIV_MASK : X86_FP_ControlWord = 0x4;
//pub const X86_FPCW_OVFEX_MASK   : X86_FP_ControlWord = 0x8;
//pub const X86_FPCW_UNDFEX_MASK  : X86_FP_ControlWord = 0x10;
//pub const X86_FPCW_PRECEX_MASK  : X86_FP_ControlWord = 0x20;
//pub const X86_FPCW_PRECC_MASK   : X86_FP_ControlWord = 0x300;
//pub const X86_FPCW_ROUNDC_MASK  : X86_FP_ControlWord = 0xc00;
//
///* values for precision control */
//pub const X86_FPCW_PREC_SINGLE    : X86_FP_ControlWord = 0;
//pub const X86_FPCW_PREC_DOUBLE    : X86_FP_ControlWord = 0x200;
//pub const X86_FPCW_PREC_EXTENDED  : X86_FP_ControlWord = 0x300;
//
///* values for rounding control */
//pub const X86_FPCW_ROUND_NEAREST  : X86_FP_ControlWord = 0;
//pub const X86_FPCW_ROUND_DOWN     : X86_FP_ControlWord = 0x400;
//pub const X86_FPCW_ROUND_UP       : X86_FP_ControlWord = 0x800;
//pub const X86_FPCW_ROUND_TOZERO   : X86_FP_ControlWord = 0xc00;

/*
// prefix code
*/

// These constants are not used.
//
#[repr(usize)]
#[derive(Copy, Clone)]
pub enum Prefix {
    LOCK = 0,
    REPNZ = 1,
    REPZ = 2,
    REP = 3,
    CS = 4,
    SS = 5,
    DS = 6,
    ES = 7,
    FS = 8,
    GS = 9,
    UNLIKELY = 10,
    LIKELY = 11,
    OPERAND = 12,
    ADDRESS = 13
}

impl Prefix {
    fn value(&self) -> usize {
        unsafe { transmute(*self) }
    }
}

const x86_cc_unsigned_map : [u8; 12] = [
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

const x86_cc_signed_map : [u8; 12] = [
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

const x86_prefix_map : [u8; 14] = [
    0xF0,
    0xF2,
    0xF3,
    0xF3,
    0x2E,
    0x36,
    0x3E,
    0x26,
    0x64,
    0x65,
    0x2E,
    0x3E,
    0x66,
    0x67
];

const x86_fp_op_reg_map : [u8; 9] = [0, 1, 2, 3, 5, 4, 7, 6, 8];


/*
// bitvector mask for callee-saved registers
*/
pub const X86_ESI_MASK: i32 = 1<<X86_ESI;
pub const X86_EDI_MASK: i32 = 1<<X86_EDI;
pub const X86_EBX_MASK: i32 = 1<<X86_EBX;
pub const X86_EBP_MASK: i32 = 1<<X86_EBP;

pub const X86_CALLEE_REGS: i32 = (1<<X86_EAX) | (1<<X86_ECX) | (1<<X86_EDX);
pub const X86_CALLER_REGS: i32 = (1<<X86_EBX) | (1<<X86_EBP) | (1<<X86_ESI) | (1<<X86_EDI);
pub const X86_BYTE_REGS  : i32 = (1<<X86_EAX) | (1<<X86_ECX) | (1<<X86_EDX) | (1<<X86_EBX);

pub fn IS_SCRATCH(reg: Reg) -> bool {
    X86_CALLER_REGS & (1 << (reg.value())) != 0 /* X86_EAX, X86_ECX, or X86_EDX */
}

pub fn IS_CALLEE(reg: Reg) -> bool {
    X86_CALLEE_REGS & (1 << (reg.value())) != 0 	/* X86_ESI, X86_EDI, X86_EBX, or X86_EBP */
}

/* In 64 bit mode, all registers have a low byte subregister */
fn IS_BYTE_REG(reg: Reg) -> bool {
    if cfg!(target_arch = "x86") {
        reg.value() < 4
    } else if cfg!(target_arch = "x86_64") {
        true
    } else {
        panic!("unsupported architecture");
    }
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


pub struct Emit {
    inst: Writer
}

impl Emit {
    pub fn new() -> Emit {
        Emit {
            inst: Writer::new()
        }
    }

    /*
     * useful building blocks
     */
    fn modrm_mod(modrm: i32) -> i32 {
        (modrm) >> 6
    }
    fn modrm_reg(modrm: i32) -> i32 {
        ((modrm) >> 3) & 0x7
    }
    fn modrm_rm(modrm: i32) -> i32 {
        (modrm) & 0x7
    }
    
    fn address_byte(&mut self, m: u8, o: u8, r: u8) {
        self.inst.push(((((m)&0x03)<<6)|(((o)&0x07)<<3)|(((r)&0x07))));
    }
    
    fn imm_emit32(&mut self, imm: i32) {
        let imb = unsafe { transmute::<_, [u8; 4]>(imm) };
        self.inst.push(imb [0]);
        self.inst.push(imb [1]);
        self.inst.push(imb [2]);
        self.inst.push(imb [3]);
    }
    
    fn imm_emit32_at(&mut self, pos: usize, imm: i32) {
        let imb = unsafe { transmute::<_, [u8; 4]>(imm) };
        self.inst.set_at(imb [0], pos);
        self.inst.set_at(imb [1], pos + 1);
        self.inst.set_at(imb [2], pos + 2);
        self.inst.set_at(imb [3], pos + 3);
    }
    
    // TODO: inst is the offset into the stream!
    fn imm_emit16(&mut self, imm: i32) {
        let imb = unsafe { transmute::<_, [u8; 2]>(imm as i16) };
        self.inst.push(imb [0]);
        self.inst.push(imb [1]);
    }
    
    fn imm_emit8(&mut self, imm: i32) {
        self.inst.push(imm as u8);
    }
    
    fn imm_emit8_at(&mut self, pos: usize, imm: i32) {
        self.inst.set_at(imm as u8, pos);
    }
    
    fn is_imm8(imm: i32) -> bool {
        ((imm) >= -128 && (imm) <= 127)
    }
    
    fn is_imm16(imm: i32) -> bool {
        ((imm) >= -(1<<16) && (imm) <= ((1<<16)-1))
    }
    
    fn reg_emit(&mut self, r: u8, regno: Reg) {
        self.address_byte (3, (r), (regno.value()));
    }
    
    fn reg8_emit(&mut self, r: u8, regno: Reg, is_rh: bool, is_rnoh: bool) {
        self.address_byte (3, if (is_rh) { (r)|4 } else { (r) }, if (is_rnoh) { ((regno.value())|4) } else { (regno.value()) });
    }
    
    fn regp_emit(&mut self, r: u8, regno: Reg) {
        self.address_byte (0, (r), (regno.value()));
    }
    
    fn mem_emit(&mut self, r: u8, disp: i32) {
        self.address_byte (0, (r), 5);
        self.imm_emit32((disp));
    }
    
    fn membase_emit(&mut self, r: u8, basereg: Reg, disp: i32) {
        if ((basereg) == Reg::ESP) {
            if ((disp) == 0) {
                self.address_byte (0, (r), X86_ESP);
                self.address_byte (0, X86_ESP, X86_ESP);
            } else if (Self::is_imm8((disp))) {
                self.address_byte (1, (r), X86_ESP);
                self.address_byte (0, X86_ESP, X86_ESP);
                self.imm_emit8 ((disp));
            } else {
                self.address_byte (2, (r), X86_ESP);
                self.address_byte (0, X86_ESP, X86_ESP);
                self.imm_emit32 ((disp));
            }
            return;
        }
        
        if ((disp) == 0 && (basereg) != Reg::EBP) {
            self.address_byte (0, (r), (basereg.value()));
            return;
        }
        
        if (Self::is_imm8((disp))) {
            self.address_byte (1, (r), (basereg.value()));
            self.imm_emit8 ((disp));
        } else {
            self.address_byte (2, (r), (basereg.value()));
            self.imm_emit32 ((disp));
        }
    }
    
    fn memindex_emit(&mut self, r: u8, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        if ((basereg) == Reg::NOBASEREG) {
            self.address_byte (0, (r), 4);
            self.address_byte ((shift), (indexreg.value()), 5);
            self.imm_emit32 ((disp));
        } else if ((disp) == 0 && (basereg) != Reg::EBP) {
            self.address_byte (0, (r), 4);
            self.address_byte ((shift), (indexreg.value()), (basereg.value()));
        } else if (Self::is_imm8((disp))) {
            self.address_byte (1, (r), 4);
            self.address_byte ((shift), (indexreg.value()), (basereg.value()));
            self.imm_emit8 ((disp));
        } else {
            self.address_byte (2, (r), 4);
            self.address_byte ((shift), (indexreg.value()), (basereg.value()));
            self.imm_emit32 ((disp));
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
    pub fn patch(&mut self, offset: usize, target: usize) {
        let mut pos = offset + 1;
        let disp;
        let mut size = 0;
        
        match self.inst.get() {
            0xe8 | 0xe9 => {
                size += 1;
            }
            /* call, jump32 */
            0x0f => {
                if (!(self.inst.get_at(pos) >= 0x70 && self.inst.get_at(pos) <= 0x8f)) {
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
        
        disp = (target as isize - pos as isize) as i32;
        if (size != 0) {
            self.imm_emit32_at (pos, disp - 4);
        } else if (Self::is_imm8 (disp - 1)) {
            self.imm_emit8_at (pos, disp - 1);
        } else {
            jit_assert! ();
        }
    }
    
    pub fn breakpoint(&mut self) {
        self.inst.push(0xcc);
    }
    
    pub fn cld(&mut self) {
        self.inst.push(0xfc);
    }
    
    pub fn stosb(&mut self) {
        self.inst.push(0xaa);
    }
    
    pub fn stosl(&mut self) {
        self.inst.push(0xab);
    }
    
    pub fn stosd(&mut self) {
        self.stosl();
    }
    
    pub fn movsb(&mut self) {
        self.inst.push(0xa4);
    }
    
    pub fn movsl(&mut self) {
        self.inst.push(0xa5);
    }
    
    pub fn movsd(&mut self) {
        self.movsl();
    }
    
    pub fn prefix(&mut self, p: Prefix) {
        self.inst.push( (x86_prefix_map[p.value()]));
    }
    
    pub fn rdtsc(&mut self) {
        self.inst.push(0x0f);
        self.inst.push(0x31);
    }
    
    pub fn cmpxchg_reg_reg(&mut self, dreg: Reg, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xb1);
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn cmpxchg_mem_reg(&mut self, mem: i32, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xb1);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn cmpxchg_membase_reg(&mut self, basereg: Reg, disp: i32, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xb1);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn xchg_reg_reg(&mut self, dreg: Reg, reg: Reg, size: i32) {
        if ((size) == 1){
            self.inst.push(0x86);
        } else {
            self.inst.push(0x87);
        }
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn xchg_mem_reg(&mut self, mem: i32, reg: Reg, size: i32) {
        if ((size) == 1) {
            self.inst.push(0x86);
        } else {
            self.inst.push(0x87);
        }
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn xchg_membase_reg(&mut self, basereg: Reg, disp: i32, reg: Reg, size: i32) {
        if ((size) == 1) {
            self.inst.push(0x86);
        } else {
            self.inst.push(0x87);
        }
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn xadd_reg_reg(&mut self, dreg: Reg, reg: Reg, size: i32) {
        self.inst.push(0x0F);
        if ((size) == 1) {
            self.inst.push(0xC0);
        } else {
            self.inst.push(0xC1);
        }
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn xadd_mem_reg(&mut self, mem: i32, reg: Reg, size: i32) {
        self.inst.push(0x0F);
        if ((size) == 1) {
            self.inst.push(0xC0);
        } else {
            self.inst.push(0xC1);
        }
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn xadd_membase_reg(&mut self, basereg: Reg, disp: i32, reg: Reg, size: i32) {
        self.inst.push(0x0F);
        if ((size) == 1) {
            self.inst.push(0xC0);
        } else {
            self.inst.push(0xC1);
        }
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn inc_mem(&mut self, mem: i32) {
        self.inst.push(0xff);
        self.mem_emit (0, (mem));
    }
    
    pub fn inc_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xff);
        self.membase_emit (0, (basereg), (disp));
    }
    
    pub fn inc_reg(&mut self, reg: Reg) {
        self.inst.push(0x40 + (reg.value()));
    }
    
    pub fn dec_mem(&mut self, mem: i32) {
        self.inst.push(0xff);
        self.mem_emit (1, (mem));
    }
    
    pub fn dec_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xff);
        self.membase_emit (1, (basereg), (disp));
    }
    
    pub fn dec_reg(&mut self, reg: Reg) {
        self.inst.push(0x48 + (reg.value()));
    }
    
    pub fn not_mem(&mut self, mem: i32) {
        self.inst.push(0xf7);
        self.mem_emit (2, (mem));
    }
    
    pub fn not_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xf7);
        self.membase_emit (2, (basereg), (disp));
    }
    
    pub fn not_reg(&mut self, reg: Reg) {
        self.inst.push(0xf7);
        self.reg_emit (2, (reg));
    }
    
    pub fn neg_mem(&mut self, mem: i32) {
        self.inst.push(0xf7);
        self.mem_emit (3, (mem));
    }
    
    pub fn neg_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xf7);
        self.membase_emit (3, (basereg), (disp));
    }
    
    pub fn neg_reg(&mut self, reg: Reg) {
        self.inst.push(0xf7);
        self.reg_emit (3, (reg));
    }
    
    pub fn nop(&mut self) {
        self.inst.push(0x90);
    }
    
    pub fn alu_reg_imm(&mut self, opc: AluOp, reg: Reg, imm: i32) {
        if ((reg) == Reg::EAX) {
            self.inst.push((((opc.value())) << 3) + 5);
            self.imm_emit32 ((imm));
            return;
        }
        if (Self::is_imm8((imm))) {
            self.inst.push(0x83);
            self.reg_emit ((opc.value()), (reg));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x81);
            self.reg_emit ((opc.value()), (reg));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn alu_reg16_imm(&mut self, opc: AluOp, reg: Reg, imm: i32) {
        self.inst.push(0x66);
        if ((reg) == Reg::EAX) {
            self.inst.push((((opc.value())) << 3) + 5);
            self.imm_emit16 ((imm));
            return;
        }
        if (Self::is_imm8((imm))) {
            self.inst.push(0x83);
            self.reg_emit ((opc.value()), (reg));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x81);
            self.reg_emit ((opc.value()), (reg));
            self.imm_emit16 ((imm));
        }
    }
    
    pub fn alu_mem_imm(&mut self, opc: AluOp, mem: i32, imm: i32) {
        if (Self::is_imm8((imm))) {
            self.inst.push(0x83);
            self.mem_emit ((opc.value()), (mem));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x81);
            self.mem_emit ((opc.value()), (mem));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn alu_membase_imm(&mut self, opc: AluOp, basereg: Reg, disp: i32, imm: i32) {
        if (Self::is_imm8((imm))) {
            self.inst.push(0x83);
            self.membase_emit ((opc.value()), (basereg), (disp));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x81);
            self.membase_emit ((opc.value()), (basereg), (disp));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn alu_membase8_imm(&mut self, opc: AluOp, basereg: Reg, disp: i32, imm: i32) {
        self.inst.push(0x80);
        self.membase_emit ((opc.value()), (basereg), (disp));
        self.imm_emit8 ((imm));
    }
    
    pub fn alu_mem_reg(&mut self, opc: AluOp, mem: i32, reg: Reg) {
        self.inst.push((((opc.value())) << 3) + 1);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn alu_membase_reg(&mut self, opc: AluOp, basereg: Reg, disp: i32, reg: Reg) {
        self.inst.push((((opc.value())) << 3) + 1);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn alu_reg_reg(&mut self, opc: AluOp, dreg: Reg, reg: Reg) {
        self.inst.push((((opc.value())) << 3) + 3);
        self.reg_emit ((dreg.value()), (reg));
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
    pub fn alu_reg8_reg8(&mut self, opc: AluOp, dreg: Reg, reg: Reg, is_dreg_h: bool, is_reg_h: bool) {
        self.inst.push((((opc.value())) << 3) + 2);
        self.reg8_emit ((dreg.value()), (reg), (is_dreg_h), (is_reg_h));
    }
    
    pub fn alu_reg_mem(&mut self, opc: AluOp, reg: Reg, mem: i32) {
        self.inst.push((((opc.value())) << 3) + 3);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn alu_reg_membase(&mut self, opc: AluOp, reg: Reg, basereg: Reg, disp: i32) {
        self.inst.push((((opc.value())) << 3) + 3);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn test_reg_imm(&mut self, reg: Reg, imm: i32) {
        if ((reg) == Reg::EAX) {
            self.inst.push(0xa9);
        } else {
            self.inst.push(0xf7);
            self.reg_emit (0, (reg));
        }
        self.imm_emit32 ((imm));
    }
    
    pub fn test_mem_imm(&mut self, mem: i32, imm: i32) {
        self.inst.push(0xf7);
        self.mem_emit (0, (mem));
        self.imm_emit32 ((imm));
    }
    
    pub fn test_membase_imm(&mut self, basereg: Reg, disp: i32, imm: i32) {
        self.inst.push(0xf7);
        self.membase_emit (0, (basereg), (disp));
        self.imm_emit32 ((imm));
    }
    
    pub fn test_reg_reg(&mut self, dreg: Reg, reg: Reg) {
        self.inst.push(0x85);
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn test_mem_reg(&mut self, mem: i32, reg: Reg) {
        self.inst.push(0x85);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn test_membase_reg(&mut self, basereg: Reg, disp: i32, reg: Reg) {
        self.inst.push(0x85);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }

    pub fn shift_reg_imm(&mut self, opc: ShiftOp, reg: Reg, imm: i32) {
        if ((imm) == 1) {
            self.inst.push(0xd1);
            self.reg_emit ((opc.value()), (reg));
        } else {
            self.inst.push(0xc1);
            self.reg_emit ((opc.value()), (reg));
            self.imm_emit8 ((imm));
        }
    }
    
    pub fn shift_mem_imm(&mut self, opc: ShiftOp, mem: i32, imm: i32) {
        if ((imm) == 1) {
            self.inst.push(0xd1);
            self.mem_emit ((opc.value()), (mem));
        } else {
            self.inst.push(0xc1);
            self.mem_emit ((opc.value()), (mem));
            self.imm_emit8 ((imm));
        }
    }
    
    pub fn shift_membase_imm(&mut self, opc: ShiftOp, basereg: Reg, disp: i32, imm: i32) {
        if ((imm) == 1) {
            self.inst.push(0xd1);
            self.membase_emit ((opc.value()), (basereg), (disp));
        } else {
            self.inst.push(0xc1);
            self.membase_emit ((opc.value()), (basereg), (disp));
            self.imm_emit8 ((imm));
        }
    }
    
    pub fn shift_reg(&mut self, opc: ShiftOp, reg: Reg) {
        self.inst.push(0xd3);
        self.reg_emit ((opc.value()), (reg));
    }
    
    pub fn shift_mem(&mut self, opc: ShiftOp, mem: i32) {
        self.inst.push(0xd3);
        self.mem_emit ((opc.value()), (mem));
    }
    
    pub fn shift_membase(&mut self, opc: ShiftOp, basereg: Reg, disp: i32) {
        self.inst.push(0xd3);
        self.membase_emit ((opc.value()), (basereg), (disp));
    }
    
    /*
     * Multi op shift missing.
     */
    
    pub fn shrd_reg(&mut self, dreg: Reg, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xad);
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn shrd_reg_imm(&mut self, dreg: Reg, reg: Reg, shamt: i32) {
        self.inst.push(0x0f);
        self.inst.push(0xac);
        self.reg_emit ((reg.value()), (dreg));
        self.imm_emit8 ((shamt));
    }
    
    pub fn shld_reg(&mut self, dreg: Reg, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xa5);
        self.reg_emit ((reg.value()), (dreg));
    }
    
    pub fn shld_reg_imm(&mut self, dreg: Reg, reg: Reg, shamt: i32) {
        self.inst.push(0x0f);
        self.inst.push(0xa4);
        self.reg_emit ((reg.value()), (dreg));
        self.imm_emit8 ((shamt));
    }
    
    /*
     * EDX:EAX = EAX * rm
     */
    pub fn mul_reg(&mut self, reg: Reg, is_signed: bool) {
        self.inst.push(0xf7);
        self.reg_emit (4 + if (is_signed) { 1 } else { 0 }, (reg));
    }
    
    pub fn mul_mem(&mut self, mem: i32, is_signed: bool) {
        self.inst.push(0xf7);
        self.mem_emit (4 + if (is_signed) { 1 } else { 0 }, (mem));
    }
    
    pub fn mul_membase(&mut self, basereg: Reg, disp: i32, is_signed: bool) {
        self.inst.push(0xf7);
        self.membase_emit (4 + if (is_signed) { 1 } else { 0 }, (basereg), (disp));
    }
    
    /*
     * r *= rm
     */
    pub fn imul_reg_reg(&mut self, dreg: Reg, reg: Reg) {
        self.inst.push(0x0f);
        self.inst.push(0xaf);
        self.reg_emit ((dreg.value()), (reg));
    }
    
    pub fn imul_reg_mem(&mut self, reg: Reg, mem: i32) {
        self.inst.push(0x0f);
        self.inst.push(0xaf);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn imul_reg_membase(&mut self, reg: Reg, basereg: Reg, disp: i32) {
        self.inst.push(0x0f);
        self.inst.push(0xaf);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    /*
     * dreg = rm * imm
     */
    pub fn imul_reg_reg_imm(&mut self, dreg: Reg, reg: Reg, imm: i32) {
        if (Self::is_imm8 ((imm))) {
            self.inst.push(0x6b);
            self.reg_emit ((dreg.value()), (reg));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x69);
            self.reg_emit ((dreg.value()), (reg));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn imul_reg_mem_imm(&mut self, reg: Reg, mem: i32, imm: i32) {
        if (Self::is_imm8 ((imm))) {
            self.inst.push(0x6b);
            self.mem_emit ((reg.value()), (mem));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x69);
            self.reg_emit ((reg.value()), Reg::from_u8((mem) as u8));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn imul_reg_membase_imm(&mut self, reg: Reg, basereg: Reg, disp: i32, imm: i32) {
        if (Self::is_imm8 ((imm))) {
            self.inst.push(0x6b);
            self.membase_emit ((reg.value()), (basereg), (disp));
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x69);
            self.membase_emit ((reg.value()), (basereg), (disp));
            self.imm_emit32 ((imm));
        }
    }
    
    /*
     * divide EDX:EAX by rm;
     * eax = quotient, edx = remainder
     */
    
    pub fn div_reg(&mut self, reg: Reg, is_signed: bool) {
        self.inst.push(0xf7);
        self.reg_emit (6 + if (is_signed) { 1 } else { 0 }, (reg));
    }
    
    pub fn div_mem(&mut self, mem: i32, is_signed: bool) {
        self.inst.push(0xf7);
        self.mem_emit (6 + if is_signed { 1 } else { 0 }, (mem));
    }
    
    pub fn div_membase(&mut self, basereg: Reg, disp: i32, is_signed: bool) {
        self.inst.push(0xf7);
        self.membase_emit (6 + if is_signed { 1 } else { 0 }, (basereg), (disp));
    }
    
    pub fn mov_mem_reg(&mut self, mem: i32, reg: Reg, size: i32) {
        match size {
            1 => self.inst.push(0x88),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x89);
            }
            _ => jit_assert! ()
        }
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn mov_regp_reg(&mut self, regp: Reg, reg: Reg, size: i32) {
        match size {
            1 => self.inst.push(0x88),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x89);
            }
            _ => jit_assert! ()
        }
        self.regp_emit ((reg.value()), (regp));
    }
    
    pub fn mov_membase_reg(&mut self, basereg: Reg, disp: i32, reg: Reg, size: i32) {
        match size {
            1 => self.inst.push(0x88),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x89);
            }
            _ => jit_assert! ()
        }
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn mov_memindex_reg(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, reg: Reg, size: i32) {
        match size {
            1 => self.inst.push(0x88),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x89);
            }
            _ => jit_assert! ()
        }
        self.memindex_emit ((reg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn mov_reg_reg(&mut self, dreg: Reg, reg: Reg, size: i32) {
        match size {
            1 => self.inst.push(0x8a),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x8b);
            }
            _ => jit_assert! ()
        }
        self.reg_emit ((dreg.value()), (reg));
    }
    
    pub fn mov_reg_mem(&mut self, reg: Reg, mem: i32, size: i32) {
        match size {
            1 => self.inst.push(0x8a),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x8b);
            }
            _ => jit_assert! ()
        }
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn mov_reg_membase(&mut self, reg: Reg, basereg: Reg, disp: i32, size: i32) {
        match size {
            1 => self.inst.push(0x8a),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x8b);
            }
            _ => jit_assert! ()
        }
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn mov_reg_memindex(&mut self, reg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, size: i32) {
        match size {
            1 => self.inst.push(0x8a),
            2 | 4 => {
                if size == 2 {
                    self.inst.push(0x66);
                }
                self.inst.push(0x8b);
            }
            _ => jit_assert! ()
        }
        self.memindex_emit ((reg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * Note: x86_clear_reg () chacnges the condition code!
     */
    pub fn clear_reg(&mut self, reg: Reg) {
        self.alu_reg_reg(AluOp::XOR, (reg), (reg));
    }
    
    pub fn mov_reg_imm(&mut self, reg: Reg, imm: i32) {
        self.inst.push(0xb8 + (reg.value()));
        self.imm_emit32 ((imm));
    }
    
    pub fn mov_mem_imm(&mut self, mem: i32, imm: i32, size: i32) {
        if ((size) == 1) {
            self.inst.push(0xc6);
            self.mem_emit (0, (mem));
            self.imm_emit8 ((imm));
        } else if ((size) == 2) {
            self.inst.push(0x66);
            self.inst.push(0xc7);
            self.mem_emit (0, (mem));
            self.imm_emit16 ((imm));
        } else {
            self.inst.push(0xc7);
            self.mem_emit (0, (mem));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn mov_membase_imm(&mut self, basereg: Reg, disp: i32, imm: i32, size: i32) {
        if ((size) == 1) {
            self.inst.push(0xc6);
            self.membase_emit (0, (basereg), (disp));
            self.imm_emit8 ((imm));
        } else if ((size) == 2) {
            self.inst.push(0x66);
            self.inst.push(0xc7);
            self.membase_emit (0, (basereg), (disp));
            self.imm_emit16 ((imm));
        } else {
            self.inst.push(0xc7);
            self.membase_emit (0, (basereg), (disp));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn mov_memindex_imm(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, imm: i32, size: i32) {
        if ((size) == 1) {
            self.inst.push(0xc6);
            self.memindex_emit (0, (basereg), (disp), (indexreg), (shift));
            self.imm_emit8 ((imm));
        } else if ((size) == 2) {
            self.inst.push(0x66);
            self.inst.push(0xc7);
            self.memindex_emit (0, (basereg), (disp), (indexreg), (shift));
            self.imm_emit16 ((imm));
        } else {
            self.inst.push(0xc7);
            self.memindex_emit (0, (basereg), (disp), (indexreg), (shift));
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn lea_mem(&mut self, reg: Reg, mem: i32) {
        self.inst.push(0x8d);
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn lea_membase(&mut self, reg: Reg, basereg: Reg, disp: i32) {
        self.inst.push(0x8d);
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn lea_memindex(&mut self, reg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.inst.push(0x8d);
        self.memindex_emit ((reg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn widen_reg(&mut self, dreg: Reg, reg: Reg, is_signed: bool, is_half: bool) {
        let mut op = 0xb6;
        jit_assert! (is_half ||  IS_BYTE_REG (reg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            op += 0x08;
        }
        if ((is_half)) {
            op += 0x01;
        }
        self.inst.push(op);
        self.reg_emit ((dreg.value()), (reg));
    }
    
    pub fn widen_mem(&mut self, dreg: Reg, mem: i32, is_signed: bool, is_half: bool) {
        let mut op = 0xb6;
        self.inst.push(0x0f);
        if ((is_signed)) {
            op += 0x08;
        }
        if ((is_half)) {
            op += 0x01;
        }
        self.inst.push(op);
        self.mem_emit ((dreg.value()), (mem));
    }
    
    pub fn widen_membase(&mut self, dreg: Reg, basereg: Reg, disp: i32, is_signed: bool, is_half: bool) {
        let mut op = 0xb6;
        self.inst.push(0x0f);
        if ((is_signed)) {
            op += 0x08;
        }
        if ((is_half)) {
            op += 0x01;
        }
        self.inst.push(op);
        self.membase_emit ((dreg.value()), (basereg), (disp));
    }
    
    pub fn widen_memindex(&mut self, dreg: Reg, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_signed: bool, is_half: bool) {
        let mut op = 0xb6;
        self.inst.push(0x0f);
        if ((is_signed)) {
            op += 0x08;
        }
        if ((is_half)) {
            op += 0x01;
        }
        self.inst.push(op);
        self.memindex_emit ((dreg.value()), (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn cdq(&mut self) {
        self.inst.push(0x99);
    }
    
    pub fn wait(&mut self) {
        self.inst.push(0x9b);
    }
    
    pub fn fp_op_mem(&mut self, opc: FpOp, mem: i32, is_double: bool) {
        self.inst.push(if (is_double) { 0xdc } else { 0xd8 });
        self.mem_emit ((opc.value()), (mem));
    }
    
    pub fn fp_op_membase(&mut self, opc: FpOp, basereg: Reg, disp: i32, is_double: bool) {
        self.inst.push(if (is_double) { 0xdc } else { 0xd8 });
        self.membase_emit ((opc.value()), (basereg), (disp));
    }
    
    pub fn fp_op(&mut self, opc: FpOp, index: u8) {
        self.inst.push(0xd8);
        self.inst.push(0xc0+((opc.value())<<3)+((index)&0x07));
    }
    
    pub fn fp_op_reg(&mut self, opc: FpOp, index: u8, pop_stack: bool) {
        self.inst.push(if (pop_stack) { 0xde } else { 0xdc });
        self.inst.push(0xc0+(x86_fp_op_reg_map[(opc) as usize]<<3)+((index)&0x07));
    }
    
    /**
     * @x86_fp_int_op_membase
     * Supports FPU operations between ST(0) and integer operand in memory.
     * Operation encoded using X86_FP_Opcode enum.
     * Operand is addressed by [basereg + disp].
     * is_int specifies whether operand is int32 (TRUE) or int16 (FALSE).
     */
    pub fn fp_int_op_membase(&mut self, opc: FpOp, basereg: Reg, disp: i32, is_int: bool) {
        self.inst.push(if (is_int) { 0xda } else { 0xde });
        self.membase_emit (opc.value(), (basereg), (disp));
    }
    
    pub fn fstp(&mut self, index: u8) {
        self.inst.push(0xdd);
        self.inst.push(0xd8+(index));
    }
    
    pub fn fcompp(&mut self) {
        self.inst.push(0xde);
        self.inst.push(0xd9);
    }
    
    pub fn fucompp(&mut self) {
        self.inst.push(0xda);
        self.inst.push(0xe9);
    }
    
    pub fn fnstsw(&mut self) {
        self.inst.push(0xdf);
        self.inst.push(0xe0);
    }
    
    pub fn fnstcw(&mut self, mem: i32) {
        self.inst.push(0xd9);
        self.mem_emit (7, (mem));
    }
    
    pub fn fnstcw_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xd9);
        self.membase_emit (7, (basereg), (disp));
    }
    
    pub fn fldcw(&mut self, mem: i32) {
        self.inst.push(0xd9);
        self.mem_emit (5, (mem));
    }
    
    pub fn fldcw_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xd9);
        self.membase_emit (5, (basereg), (disp));
    }
    
    pub fn fchs(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe0);
    }
    
    pub fn frem(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xf8);
    }
    
    pub fn fxch(&mut self, index: u8) {
        self.inst.push(0xd9);
        self.inst.push(0xc8 + ((index) & 0x07));
    }
    
    pub fn fcomi(&mut self, index: u8) {
        self.inst.push(0xdb);
        self.inst.push(0xf0 + ((index) & 0x07));
    }
    
    pub fn fcomip(&mut self, index: u8) {
        self.inst.push(0xdf);
        self.inst.push(0xf0 + ((index) & 0x07));
    }
    
    pub fn fucomi(&mut self, index: u8) {
        self.inst.push(0xdb);
        self.inst.push(0xe8 + ((index) & 0x07));
    }
    
    pub fn fucomip(&mut self, index: u8) {
        self.inst.push(0xdf);
        self.inst.push(0xe8 + ((index) & 0x07));
    }
    
    pub fn fld(&mut self, mem: i32, is_double: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.mem_emit (0, (mem));
    }
    
    pub fn fld_membase(&mut self, basereg: Reg, disp: i32, is_double: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.membase_emit (0, (basereg), (disp));
    }
    
    pub fn fld_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_double: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.memindex_emit (0, (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn fld80_mem(&mut self, mem: i32) {
        self.inst.push(0xdb);
        self.mem_emit (5, (mem));
    }
    
    pub fn fld80_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xdb);
        self.membase_emit (5, (basereg), (disp));
    }
    
    pub fn fld80_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.inst.push(0xdb);
        self.memindex_emit (5, (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn fild(&mut self, mem: i32, is_long: bool) {
        if ((is_long)) {
            self.inst.push(0xdf);
            self.mem_emit (5, (mem));
        } else {
            self.inst.push(0xdb);
            self.mem_emit (0, (mem));
        }
    }
    
    pub fn fild_membase(&mut self, basereg: Reg, disp: i32, is_long: bool) {
        if ((is_long)) {
            self.inst.push(0xdf);
            self.membase_emit (5, (basereg), (disp));
        } else {
            self.inst.push(0xdb);
            self.membase_emit (0, (basereg), (disp));
        }
    }
    
    pub fn fld_reg(&mut self, index: u8) {
        self.inst.push(0xd9);
        self.inst.push(0xc0 + ((index) & 0x07));
    }
    
    pub fn fldz(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xee);
    }
    
    pub fn fld1(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe8);
    }
    
    pub fn fldpi(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xeb);
    }
    
    pub fn fst(&mut self, mem: i32, is_double: bool, pop_stack: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.mem_emit (2 + if pop_stack { 1 } else { 0 }, (mem));
    }
    
    pub fn fst_membase(&mut self, basereg: Reg, disp: i32, is_double: bool, pop_stack: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.membase_emit (2 + if pop_stack { 1 } else { 0 }, (basereg), (disp));
    }
    
    pub fn fst_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8, is_double: bool, pop_stack: bool) {
        self.inst.push(if (is_double) { 0xdd } else { 0xd9 });
        self.memindex_emit (2 + if pop_stack { 1 } else { 0 }, (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn fst80_mem(&mut self, mem: i32) {
        self.inst.push(0xdb);
        self.mem_emit (7, (mem));
    }
    
    pub fn fst80_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xdb);
        self.membase_emit (7, (basereg), (disp));
    }
    
    pub fn fst80_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.inst.push(0xdb);
        self.memindex_emit (7, (basereg), (disp), (indexreg), (shift));
    }
    
    pub fn fist_pop(&mut self, mem: i32, is_long: bool) {
        if ((is_long)) {
            self.inst.push(0xdf);
            self.mem_emit (7, (mem));
        } else {
            self.inst.push(0xdb);
            self.mem_emit (3, (mem));
        }
    }
    
    pub fn fist_pop_membase(&mut self, basereg: Reg, disp: i32, is_long: bool) {
        if ((is_long)) {
            self.inst.push(0xdf);
            self.membase_emit (7, (basereg), (disp));
        } else {
            self.inst.push(0xdb);
            self.membase_emit (3, (basereg), (disp));
        }
    }
    
    pub fn fstsw(&mut self) {
        self.inst.push(0x9b);
        self.inst.push(0xdf);
        self.inst.push(0xe0);
    }
    
    /**
     * @x86_fist_membase
     * Converts content of ST(0) to integer and stores it at memory location
     * addressed by [basereg + disp].
     * is_int specifies whether destination is int32 (TRUE) or int16 (FALSE).
     */
    pub fn fist_membase(&mut self, basereg: Reg, disp: i32, is_int: bool) {
        if ((is_int)) {
            self.inst.push(0xdb);
            self.membase_emit (2, (basereg), (disp));
        } else {
            self.inst.push(0xdf);
            self.membase_emit (2, (basereg), (disp));
        }
    }
    
    pub fn push_reg(&mut self, reg: Reg) {
        self.inst.push(0x50 + (reg.value()));
    }
    
    pub fn push_regp(&mut self, reg: Reg) {
        self.inst.push(0xff);
        self.regp_emit (6, (reg));
    }
    
    pub fn push_mem(&mut self, mem: i32) {
        self.inst.push(0xff);
        self.mem_emit (6, (mem));
    }
    
    pub fn push_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xff);
        self.membase_emit (6, (basereg), (disp));
    }
    
    pub fn push_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.inst.push(0xff);
        self.memindex_emit (6, (basereg), (disp), (indexreg), (shift));
    }
    
    #[allow(overflowing_literals)]
    pub fn push_imm_template(&mut self) {
        self.push_imm (0xf0f0f0f0);
    }
    	
    pub fn push_imm(&mut self, imm: i32) {
        if (Self::is_imm8 (imm)) {
            self.inst.push(0x6A);
            self.imm_emit8 ((imm));
        } else {
            self.inst.push(0x68);
            self.imm_emit32 ((imm));
        }
    }
    
    pub fn pop_reg(&mut self, reg: Reg) {
        self.inst.push(0x58 + (reg.value()));
    }
    
    pub fn pop_mem(&mut self, mem: i32) {
        self.inst.push(0x8f);
        self.mem_emit (0, (mem));
    }
    
    pub fn pop_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0x8f);
        self.membase_emit (0, (basereg), (disp));
    }
    
    pub fn pushad(&mut self) {
        self.inst.push(0x60);
    }
    
    pub fn pushfd(&mut self) {
        self.inst.push(0x9c);
    }
    
    pub fn popad(&mut self) {
        self.inst.push(0x61);
    }
    
    pub fn popfd(&mut self) {
        self.inst.push(0x9d);
    }
    
    pub fn loop_(&mut self, imm: i32) {
        self.inst.push(0xe2);
        self.imm_emit8 ((imm));
    }
    
    pub fn loope(&mut self, imm: i32) {
        self.inst.push(0xe1);
        self.imm_emit8 ((imm));
    }
    
    pub fn loopne(&mut self, imm: i32) {
        self.inst.push(0xe0);
        self.imm_emit8 ((imm));
    }
    
    pub fn jump32(&mut self, imm: i32) {
        self.inst.push(0xe9);
        self.imm_emit32 ((imm));
    }
    
    pub fn jump8(&mut self, imm: i32) {
        self.inst.push(0xeb);
        self.imm_emit8 ((imm));
    }
    
    pub fn jump_reg(&mut self, reg: Reg) {
        self.inst.push(0xff);
        self.reg_emit (4, (reg));
    }
    
    pub fn jump_mem(&mut self, mem: i32) {
        self.inst.push(0xff);
        self.mem_emit (4, (mem));
    }
    
    pub fn jump_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xff);
        self.membase_emit (4, (basereg), (disp));
    }
    
    pub fn jump_memindex(&mut self, basereg: Reg, disp: i32, indexreg: Reg, shift: u8) {
        self.inst.push(0xff);
        self.memindex_emit (4, (basereg), (disp), (indexreg), (shift));
    }
    
    /*
     * target is a pointer in our buffer.
     */
    pub fn jump_code(&mut self, target: i32) {
        let mut t = (target) - 2;
        if (Self::is_imm8(t)) {
            self.jump8 (t);
        } else {
            t -= 3;
            self.jump32 (t);
        }
    }
    
    pub fn jump_disp(&mut self, disp: i32) {
        let mut t = (disp) - 2;
        if (Self::is_imm8(t)) {
            self.jump8 (t);
        } else {
            t -= 3;
            self.jump32 (t);
        }
    }
    
    pub fn branch8(&mut self, cond: Cond, imm: i32, is_signed: bool) {
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()]);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()]);
        }
        self.imm_emit8 ((imm));
    }
    
    pub fn branch32(&mut self, cond: Cond, imm: i32, is_signed: bool) {
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] + 0x10);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] + 0x10);
        }
        self.imm_emit32 ((imm));
    }
    
    pub fn branch(&mut self, cond: Cond, target: i32, is_signed: bool) {
        let mut offset = (target) - 2;
        if (Self::is_imm8 ((offset))) {
            self.branch8 ((cond), offset, (is_signed));
        } else {
            offset -= 4;
            self.branch32 ((cond), offset, (is_signed));
        }
    }
    
    pub fn branch_disp(&mut self, cond: Cond, disp: i32, is_signed: bool) {
        let mut offset = (disp) - 2;
        if (Self::is_imm8 ((offset))) {
            self.branch8 ((cond), offset, (is_signed));
        } else {
            offset -= 4;
            self.branch32 ((cond), offset, (is_signed));
        }
    }
    
    pub fn set_reg(&mut self, cond: Cond, reg: Reg, is_signed: bool) {
        jit_assert! (IS_BYTE_REG (reg));
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] + 0x20);
        }
        self.reg_emit (0, (reg));
    }
    
    pub fn set_mem(&mut self, cond: Cond, mem: i32, is_signed: bool) {
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] + 0x20);
        }
        self.mem_emit (0, (mem));
    }
    
    pub fn set_membase(&mut self, cond: Cond, basereg: Reg, disp: i32, is_signed: bool) {
        self.inst.push(0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] + 0x20);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] + 0x20);
        }
        self.membase_emit (0, (basereg), (disp));
    }
    
    pub fn call_imm(&mut self, disp: i32) {
        self.inst.push(0xe8);
        self.imm_emit32 ((disp));
    }
    
    pub fn call_reg(&mut self, reg: Reg) {
        self.inst.push(0xff);
        self.reg_emit (2, (reg));
    }
    
    pub fn call_mem(&mut self, mem: i32) {
        self.inst.push(0xff);
        self.mem_emit (2, (mem));
    }
    
    pub fn call_membase(&mut self, basereg: Reg, disp: i32) {
        self.inst.push(0xff);
        self.membase_emit (2, (basereg), (disp));
    }
    
    pub fn call_code(&mut self, target: i32) {
        let mut _x86_offset = (target);
        _x86_offset -= 5;
        self.call_imm (_x86_offset);
    }
    
    pub fn ret(&mut self) {
        self.inst.push(0xc3);
    }
    
    pub fn ret_imm(&mut self, imm: i32) {
        if ((imm) == 0) {
            self.ret ();
        } else {
            self.inst.push(0xc2);
            self.imm_emit16 ((imm));
        }
    }
    
    pub fn cmov_reg(&mut self, cond: Cond, is_signed: bool, dreg: Reg, reg: Reg) {
        self.inst.push( 0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] - 0x30);
        }
        self.reg_emit ((dreg.value()), (reg));
    }
    
    pub fn cmov_mem(&mut self, cond: Cond, is_signed: bool, reg: Reg, mem: i32) {
        self.inst.push( 0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] - 0x30);
        }  else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] - 0x30);
        }
        self.mem_emit ((reg.value()), (mem));
    }
    
    pub fn cmov_membase(&mut self, cond: Cond, is_signed: bool, reg: Reg, basereg: Reg, disp: i32) {
        self.inst.push( 0x0f);
        if ((is_signed)) {
            self.inst.push(x86_cc_signed_map [cond.value()] - 0x30);
        } else {
            self.inst.push(x86_cc_unsigned_map [cond.value()] - 0x30);
        }
        self.membase_emit ((reg.value()), (basereg), (disp));
    }
    
    pub fn enter(&mut self, framesize: i32) {
        self.inst.push(0xc8);
        self.imm_emit16 ((framesize));
        self.inst.push(0);
    }
    
    pub fn leave(&mut self) {
        self.inst.push(0xc9);
    }
    
    pub fn sahf(&mut self) {
        self.inst.push(0x9e);
    }
    
    pub fn fsin(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xfe);
    }
    
    pub fn fcos(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xff);
    }
    
    pub fn fabs(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe1);
    }
    
    pub fn ftst(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe4);
    }
    
    pub fn fxam(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xe5);
    }
    
    pub fn fpatan(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xf3);
    }
    
    pub fn fprem(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xf8);
    }
    
    pub fn fprem1(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xf5);
    }
    
    pub fn frndint(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xfc);
    }
    
    pub fn fsqrt(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xfa);
    }
    
    pub fn fptan(&mut self) {
        self.inst.push(0xd9);
        self.inst.push(0xf2);
    }
    
    pub fn padding(&mut self, size: i32) {
        match size {
            1 => {
                self.nop ();
            }
            2 => {
                self.inst.push(0x8b);
                self.inst.push(0xc0);
            }
            3 => {
                self.inst.push(0x8d);
                self.inst.push(0x6d);
                self.inst.push(0x00);
            }
            4 => {
                self.inst.push(0x8d);
                self.inst.push(0x64);
                self.inst.push(0x24);
                self.inst.push(0x00);
            }
            5 => {
                self.inst.push(0x8d);
                self.inst.push(0x64);
                self.inst.push(0x24);
                self.inst.push(0x00);
            }
            6 => {
                self.inst.push(0x8d);
                self.inst.push(0xad);
                self.inst.push(0x00);
                self.inst.push(0x00);
                self.inst.push(0x00);
                self.inst.push(0x00);
            }
            7 => {
                self.inst.push(0x8d);
                self.inst.push(0xa4);
                self.inst.push(0x24);
                self.inst.push(0x00);
                self.inst.push(0x00);
                self.inst.push(0x00);
                self.inst.push(0x00);
            }
            _ => jit_assert! ()
        }
    }
    
    pub fn prolog(&mut self, frame_size: i32, reg_mask: i32) {
        self.enter ((frame_size));
        let mut m = 1;
        for i in 0..X86_NREG {
            if ((reg_mask) & m) != 0 {
                self.push_reg (Reg::from_u8(i));
            }
            m <<= 1;
        }
    }
    
    pub fn epilog(&mut self, reg_mask: i32) {
        let mut m = 1 << X86_EDI;
        let mut i = X86_EDI;
        while m != 0 {
            if ((reg_mask) & m) != 0 {
                self.pop_reg (Reg::from_u8(i));
            }
            i -= 1;
            m=m>>1;
        }
        self.leave ();
        self.ret ();
    }
}
