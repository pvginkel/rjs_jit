/*
 * jit-rules-x86.ins - Instruction selector for x86.
 *
 * Copyright (C) 2004  Southern Storm Software, Pty Ltd.
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

use codegen::Op::*;
use codegen::Arg::*;
use std::mem::{transmute, size_of};
 
/*
 * Conversion opcodes.
 */

pub fn x86_write(inst: &Emit, ins: Ins) {
    match ins {
        Ins(TruncSByte, Reg(arg1) /* = */, Reg(arg2) /* breg */, None) => {
    		x86_widen_reg(inst, arg1, arg2, 1, 0);
    	}
    
        Ins(TruncUByte, Reg(arg1) /* = */, Reg(arg2) /* breg */, None) => {
    		x86_widen_reg(inst, arg1, arg2, 0, 0);
    	}
    
        Ins(TruncShort, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_widen_reg(inst, arg1, arg2, 1, 1);
    	}
    
        Ins(TruncUShort, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_widen_reg(inst, arg1, arg2, 0, 1);
    	}
    
        Ins(CheckSByte, Reg(arg1), None, None) => {
    		let mut patch1;
    		let mut patch2;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, -128);
    		patch1 = inst;
    		x86_branch8(inst, X86_CC_LE, 0, 1);
    		x86_alu_reg_imm(inst, X86_CMP, arg1, 127);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_LE, 0, 1);
    		x86_patch(patch1, inst);
    		inst = throw_builtin(inst, func, JIT_RESULT_OVERFLOW);
    		x86_patch(patch2, inst);
    	}
    
        Ins(CheckUByte, Reg(arg1), None, None) => {
    		let mut patch1;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, 256);
    		patch1 = inst;
    		x86_branch8(inst, X86_CC_LT, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_OVERFLOW);
    		x86_patch(patch1, inst);
    	}
    
        Ins(CheckShort, Reg(arg1), None, None) => {
    		let mut patch1;
    		let mut patch2;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, -32768);
    		patch1 = inst;
    		x86_branch8(inst, X86_CC_LE, 0, 1);
    		x86_alu_reg_imm(inst, X86_CMP, arg1, 32767);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_LE, 0, 1);
    		x86_patch(patch1, inst);
    		inst = throw_builtin(inst, func, JIT_RESULT_OVERFLOW);
    		x86_patch(patch2, inst);
    	}
    
        Ins(CheckUShort, Reg(arg1), None, None) => {
    		let mut patch1;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, 65536);
    		patch1 = inst;
    		x86_branch8(inst, X86_CC_LT, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_OVERFLOW);
    		x86_patch(patch1, inst);
    	}
    
        Ins(CheckInt, Reg(arg1), None, None) |
        Ins(CheckUInt, Reg(arg1), None, None) => {
    		let mut patch1;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, 0);
    		patch1 = inst;
    		x86_branch8(inst, X86_CC_GE, 0, 1);
    		inst = throw_builtin(inst, func, JIT_RESULT_OVERFLOW);
    		x86_patch(patch1, inst);
    	}
    
        Ins(LowWord, Reg(arg1) /* = */, Imm(arg2), None) => {
            let value = unsafe { transmute(arg2) };
    		x86_mov_reg_imm(inst, arg1, value);
    	}
        Ins(LowWord, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_mov_reg_membase(inst, arg1, X86_EBP, arg2, 4);
    	}
        Ins(LowWord, Reg(arg1) /* = */, Reg(arg2) /* lreg */, None) => {
    		if(arg1 != arg2)
    		{
    			x86_mov_reg_reg(inst, arg1, arg2, 4);
    		}
    	}
    	
        Ins(ExpandInt, Reg(arg1) /* =, lreg */, Reg(arg2), None) => {
    		if(arg1 != arg2)
    		{
    			x86_mov_reg_reg(inst, arg1, arg2, 4);
    		}
    		x86_mov_reg_reg(inst, x86_get_other_reg(arg1), arg1, 4);
    		x86_shift_reg_imm(inst, X86_SAR, x86_get_other_reg(arg1), 31);
    	}
    
        Ins(ExpandUInt, Reg(arg1) /* =, lreg */, Reg(arg2), None) => {
    		if(arg1 != arg2)
    		{
    			x86_mov_reg_reg(inst, arg1, arg2, 4);
    		}
    		x86_clear_reg(inst, x86_get_other_reg(arg1));
    	}
    
        Ins(Float32ToInt, Reg(arg1) /* = */, Reg(arg2) /* freg */, None) |
        Ins(Float64ToInt, Reg(arg1) /* = */, Reg(arg2) /* freg */, None) |
        Ins(NFloatToInt, Reg(arg1) /* = */, Reg(arg2) /* freg */, None) => {
    		/* allocate space on the stack for 2 shorts and 1 int */
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, 8);
    		/* store FPU control word */
    		x86_fnstcw_membase(inst, X86_ESP, 0);
    		/* set "round toward zero" mode */
    		x86_mov_reg_membase(inst, arg1, X86_ESP, 0, 2);
    		x86_alu_reg16_imm(inst, X86_OR, arg1, 0xc00);
    		x86_mov_membase_reg(inst, X86_ESP, 2, arg1, 2);
    		x86_fldcw_membase(inst, X86_ESP, 2);
    		/* convert float to int */
    		x86_fist_pop_membase(inst, X86_ESP, 4, 0);
    		/* restore FPU control word */
    		x86_fldcw_membase(inst, X86_ESP, 0);
    		/* move result to the destination */
    		x86_mov_reg_membase(inst, arg1, X86_ESP, 4, 4);
    		/* restore the stack */
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, 8);
    	}
    
        Ins(Float32ToLong, Reg(arg1) /* =, lreg */, Reg(arg2) /* freg */, None) |
        Ins(Float64ToLong, Reg(arg1) /* =, lreg */, Reg(arg2) /* freg */, None) |
        Ins(NFloatToLong, Reg(arg1) /* =, lreg */, Reg(arg2) /* freg */, None) => {
    		/* allocate space on the stack for 2 shorts and 1 long */
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, 12);
    		/* store FPU control word */
    		x86_fnstcw_membase(inst, X86_ESP, 0);
    		/* set "round toward zero" mode */
    		x86_mov_reg_membase(inst, arg1, X86_ESP, 0, 2);
    		x86_alu_reg16_imm(inst, X86_OR, arg1, 0xc00);
    		x86_mov_membase_reg(inst, X86_ESP, 2, arg1, 2);
    		x86_fldcw_membase(inst, X86_ESP, 2);
    		/* convert float to long */
    		x86_fist_pop_membase(inst, X86_ESP, 4, 1);
    		/* restore FPU control word */
    		x86_fldcw_membase(inst, X86_ESP, 0);
    		/* move result to the destination */
    		x86_mov_reg_membase(inst, arg1, X86_ESP, 4, 4);
    		x86_mov_reg_membase(inst, x86_get_other_reg(arg1), X86_ESP, 8, 4);
    		/* restore the stack */
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, 12);
    	}
    
        Ins(IntToFloat32, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<usize>());
    		x86_fild_membase(inst, X86_EBP, arg2, 0);
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<usize>());
    	}
        Ins(IntToFloat32, Reg(arg1) /* =, freg */, Reg(arg2), None) => {
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 0);
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<usize>());
    	}
    
        Ins(IntToFloat64, Reg(arg1) /* =, freg */, Local(arg2), None) |
        Ins(IntToNFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_fild_membase(inst, X86_EBP, arg2, 0);
    	}
        Ins(IntToFloat64, Reg(arg1) /* =, freg */, Reg(arg2), None) |
        Ins(IntToNFloat, Reg(arg1) /* =, freg */, Reg(arg2), None) => {
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<usize>());
    	}
    
        Ins(UIntToFloat32, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3) /* scratch */) => {
    		x86_clear_reg(inst, arg3);
    		x86_push_reg(inst, arg3);
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(UIntToFloat64, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3) /* scratch */) |
        Ins(UIntToNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3) /* scratch */) => {
    		x86_clear_reg(inst, arg3);
    		x86_push_reg(inst, arg3);
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(LongToFloat32, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f32>());
    		x86_fild_membase(inst, X86_EBP, arg2, 1);
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<f32>());
    	}
        Ins(LongToFloat32, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) => {
    		x86_push_reg(inst, x86_get_other_reg(arg2));
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(LongToFloat64, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f64>());
    		x86_fild_membase(inst, X86_EBP, arg2, 1);
    		x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 1);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<f64>());
    	}
        Ins(LongToFloat64, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) => {
    		x86_push_reg(inst, x86_get_other_reg(arg2));
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 1);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(LongToNFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_fild_membase(inst, X86_EBP, arg2, 1);
    	}
        Ins(LongToNFloat, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) => {
    		x86_push_reg(inst, x86_get_other_reg(arg2));
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(ULongToFloat32, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) |
        Ins(ULongToFloat64, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) |
        Ins(ULongToNFloat, Reg(arg1) /* =, freg */, Reg(arg2) /* lreg */, None) => {
    		/* TODO: review wrt relocation for elf pre-compilation */
    		static float f2pow64;
    		static int inited;
    		let mut patch;
    		if(!inited)
    		{
    			f2pow64 = jit_float32_pow(2.0, 64);
    			inited = 1;
    		}
    		x86_push_reg(inst, x86_get_other_reg(arg2));
    		x86_push_reg(inst, arg2);
    		x86_fild_membase(inst, X86_ESP, 0, 1);
    		x86_test_reg_reg(inst, x86_get_other_reg(arg2), x86_get_other_reg(arg2));
    		patch = inst;
    		x86_branch8(inst, X86_CC_NS, 0, 1);
    		x86_fp_op_mem(inst, X86_FADD, &f2pow64, 0);
    		x86_patch(patch, inst);
    		if(insn->opcode == JIT_OP_ULONG_TO_FLOAT32)
    		{
    			x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    			x86_fld_membase(inst, X86_ESP, 0, 0);
    		}
    		else if(insn->opcode == JIT_OP_ULONG_TO_FLOAT64)
    		{
    			x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    			x86_fld_membase(inst, X86_ESP, 0, 1);
    		}
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<i64>());
    	}
    
        Ins(Float64ToFloat32, Reg(arg1) /* freg */, None, None) |
        Ins(NFloatToFloat32, Reg(arg1) /* freg */, None, None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<usize>());
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 0);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<usize>());
    	}
    
        Ins(NFloatToFloat64, Reg(arg1) /* freg */, None, None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f64>());
    		x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    		x86_fld_membase(inst, X86_ESP, 0, 1);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, size_of::<f64>());
    	}
    
        Ins(Float32TONFloat, Reg(arg1) /* freg */, None, None) |
        Ins(Float32ToFloat64, Reg(arg1) /* freg */, None, None) |
        Ins(Float64ToNFloat, Reg(arg1) /* freg */, None, None) => {
    		/* Nothing to do: loading the value onto the FP stack is sufficient */
    	}
    
    /*
     * Arithmetic opcodes.
     */
    
        Ins(IAdd, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_ADD, arg1, arg2);
    	}
        Ins(IAdd, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_ADD, arg1, X86_EBP, arg2);
    	}
        Ins(IAdd, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_ADD, arg1, arg2);
    	}
    
        Ins(ISub, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_SUB, arg1, arg2);
    	}
        Ins(ISub, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_SUB, arg1, X86_EBP, arg2);
    	}
        Ins(ISub, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_SUB, arg1, arg2);
    	}
    
        Ins(IMul, Reg(arg1), Imm(arg2), None) => {
    		/* Handle special cases of immediate multiplies */
    		switch(arg2)
    		{
    			case 0:
    			{
    				x86_clear_reg(inst, arg1);
    			}
    			break;
    
    			case 1: break;
    
    			case -1:
    			{
    				x86_neg_reg(inst, arg1);
    			}
    			break;
    
    			case 2:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 1);
    			}
    			break;
    
    			case 3:
    			{
    				/* lea reg, [reg + reg * 2] */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 1);
    			}
    			break;
    
    			case 4:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 2);
    			}
    			break;
    
    			case 5:
    			{
    				/* lea reg, [reg + reg * 4] */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    			}
    			break;
    
    			case 6:
    			{
    				/* lea reg, [reg + reg * 2]; add reg, reg */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 1);
    				x86_alu_reg_reg(inst, X86_ADD, arg1, arg1);
    			}
    			break;
    
    			case 8:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 3);
    			}
    			break;
    
    			case 9:
    			{
    				/* lea reg, [reg + reg * 8] */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 3);
    			}
    			break;
    
    			case 10:
    			{
    				/* lea reg, [reg + reg * 4]; add reg, reg */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    				x86_alu_reg_reg(inst, X86_ADD, arg1, arg1);
    			}
    			break;
    
    			case 12:
    			{
    				/* lea reg, [reg + reg * 2]; shl reg, 2 */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 1);
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 2);
    			}
    			break;
    
    			case 16:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 4);
    			}
    			break;
    
    			case 25:
    			{
    				/* lea reg, [reg + reg * 4]; lea reg, [reg + reg * 4] */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    			}
    			break;
    
    			case 32:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 5);
    			}
    			break;
    
    			case 64:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 6);
    			}
    			break;
    
    			case 100:
    			{
    				/* lea reg, [reg + reg * 4]; shl reg, 2;
    				   lea reg, [reg + reg * 4] */
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 2);
    				x86_lea_memindex(inst, arg1, arg1, 0, arg1, 2);
    			}
    			break;
    
    			case 128:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 7);
    			}
    			break;
    
    			case 256:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 8);
    			}
    			break;
    
    			case 512:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 9);
    			}
    			break;
    
    			case 1024:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 10);
    			}
    			break;
    
    			case 2048:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 11);
    			}
    			break;
    
    			case 4096:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 12);
    			}
    			break;
    
    			case 8192:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 13);
    			}
    			break;
    
    			case 16384:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 14);
    			}
    			break;
    
    			case 32768:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 15);
    			}
    			break;
    
    			case 65536:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 16);
    			}
    			break;
    
    			case 0x00020000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 17);
    			}
    			break;
    
    			case 0x00040000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 18);
    			}
    			break;
    
    			case 0x00080000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 19);
    			}
    			break;
    
    			case 0x00100000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 20);
    			}
    			break;
    
    			case 0x00200000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 21);
    			}
    			break;
    
    			case 0x00400000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 22);
    			}
    			break;
    
    			case 0x00800000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 23);
    			}
    			break;
    
    			case 0x01000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 24);
    			}
    			break;
    
    			case 0x02000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 25);
    			}
    			break;
    
    			case 0x04000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 26);
    			}
    			break;
    
    			case 0x08000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 27);
    			}
    			break;
    
    			case 0x10000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 28);
    			}
    			break;
    
    			case 0x20000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 29);
    			}
    			break;
    
    			case 0x40000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 30);
    			}
    			break;
    
    			case (jit_nint)0x80000000:
    			{
    				x86_shift_reg_imm(inst, X86_SHL, arg1, 31);
    			}
    			break;
    
    			default:
    			{
    				x86_imul_reg_reg_imm(inst, arg1, arg1, arg2);
    			}
    			break;
    		}
    	}
        Ins(IMul, Reg(arg1), Local(arg2), None) => {
    		x86_imul_reg_membase(inst, arg1, X86_EBP, arg2);
    	}
        Ins(IMul, Reg(arg1), Reg(arg2), None) => {
    		x86_imul_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(IDiv, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply negates for other values */
    		let mut patch;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, jit_min_int);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_neg_reg(inst, arg1);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if arg2 == 2) => {
    		x86_mov_reg_reg(inst, arg3, arg1, 4);
    		x86_shift_reg_imm(inst, X86_SHR, arg3, 0x1f);
    		x86_alu_reg_reg(inst, X86_ADD, arg1, arg3);
    		x86_shift_reg_imm(inst, X86_SAR, arg1, 1);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if (arg2 > 0) && (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		/* This code is generated by gcc for pentium. */
    		/* We use this code because cmov is not available on all i386 cpus */
    		jit_nuint shift, temp, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		temp = 32 - shift;
    		x86_mov_reg_reg(inst, arg3, arg1, 4);
    		x86_shift_reg_imm(inst, X86_SAR, arg3, 0x1f);
    		x86_shift_reg_imm(inst, X86_SHR, arg3, temp);
    		x86_alu_reg_reg(inst, X86_ADD, arg1, arg3);
    		x86_shift_reg_imm(inst, X86_SAR, arg1, shift);
    	}
        Ins(IDiv, Reg(X86_EAX), Imm(arg1), Reg(arg2) /* scratch */, Reg(X86_EDX) /* scratch */, None) => {
    		x86_mov_reg_imm(inst, arg3, arg2);
    		x86_cdq(inst);
    		x86_div_reg(inst, arg3, 1);
    	}
        Ins(IDiv, Reg(X86_EAX), Reg(arg1), Reg(X86_EDX) /* scratch */, None, None) => {
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg2);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_alu_reg_imm(inst, X86_CMP, arg2, -1);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_alu_reg_imm(inst, X86_CMP, arg1, jit_min_int);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_cdq(inst);
    		x86_div_reg(inst, arg2, 1);
    	}
    
        Ins(IDivUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IDivUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(IDivUn, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		x86_shift_reg_imm(inst, X86_SHR, arg1, shift);
    	}
        Ins(IDivUn, Reg(X86_EAX), Imm(arg1), Reg(arg2) /* scratch */, Reg(X86_EDX) /* scratch */, None) => {
    		x86_mov_reg_imm(inst, arg3, arg2);
    		x86_clear_reg(inst, X86_EDX);
    		x86_div_reg(inst, arg3, 0);
    	}
        Ins(IDivUn, Reg(X86_EAX), Reg(arg1), Reg(X86_EDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg2);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_clear_reg(inst, X86_EDX);
    		x86_div_reg(inst, arg2, 0);
    	}
    
        Ins(IRem, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IRem, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_clear_reg(inst, arg1);
    	}
        Ins(IRem, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply gives a remainder of zero */
    		let mut patch;
    		x86_alu_reg_imm(inst, X86_CMP, arg1, jit_min_int);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_clear_reg(inst, arg1);
    	}
        Ins(IRem, Reg(X86_EDX) /* = */, Reg(X86_EAX) /* * */, Imm(arg1), Reg(arg2) /* scratch */, Reg(X86_EDX) /* scratch */, None) => {
    		x86_mov_reg_imm(inst, arg4, arg3);
    		x86_cdq(inst);
    		x86_div_reg(inst, arg4, 1);
    	}
        Ins(IRem, Reg(X86_EDX) /* = */, Reg(X86_EAX) /* * */, Reg(arg1), Reg(X86_EDX) /* scratch */, None, None) => {
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_alu_reg_reg(inst, X86_OR, arg3, arg3);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_alu_reg_imm(inst, X86_CMP, arg3, -1);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_alu_reg_imm(inst, X86_CMP, arg2, jit_min_int);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_cdq(inst);
    		x86_div_reg(inst, arg3, 1);
    	}
    
        Ins(IRemUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IRemUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_clear_reg(inst, arg1);
    	}
        Ins(IRemUn, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		x86_alu_reg_imm(inst, X86_AND, arg1, arg2 - 1);
    	}
        Ins(IRemUn, Reg(X86_EDX) /* = */, Reg(X86_EAX) /* * */, Imm(arg1), Reg(arg2) /* scratch */, Reg(X86_EDX) /* scratch */, None) => {
    		x86_mov_reg_imm(inst, arg4, arg3);
    		x86_clear_reg(inst, X86_EDX);
    		x86_div_reg(inst, arg4, 0);
    	}
        Ins(IRemUn, Reg(X86_EDX) /* = */, Reg(X86_EAX) /* * */, Reg(arg1), Reg(X86_EDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_alu_reg_reg(inst, X86_OR, arg3, arg3);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_clear_reg(inst, X86_EDX);
    		x86_div_reg(inst, arg3, 0);
    	}
    
        Ins(INeg, Reg(arg1), None, None) => {
    		x86_neg_reg(inst, arg1);
    	}
    
        Ins(LAdd, Reg(arg1) /* lreg */, Imm(arg2), None) => {
    		jit_int value1 = ((jit_int *)(arg2))[0];
    		jit_int value2 = ((jit_int *)(arg2))[1];
    		if(value1 != 0)
    		{
    			x86_alu_reg_imm(inst, X86_ADD, arg1, value1);
    			x86_alu_reg_imm(inst, X86_ADC, x86_get_other_reg(arg1), value2);
    		}
    		else
    		{
    			x86_alu_reg_imm(inst, X86_ADD, x86_get_other_reg(arg1), value2);
    		}
    	}
        Ins(LAdd, Reg(arg1) /* lreg */, Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_ADD, arg1, X86_EBP, arg2);
    		x86_alu_reg_membase(inst, X86_ADC, x86_get_other_reg(arg1), X86_EBP, arg2 + 4);
    	}
        Ins(LAdd, Reg(arg1) /* lreg */, Reg(arg2) /* lreg */, None) => {
    		x86_alu_reg_reg(inst, X86_ADD, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_ADC, x86_get_other_reg(arg1), x86_get_other_reg(arg2));
    	}
    
        Ins(LSub, Reg(arg1) /* lreg */, Imm(arg2), None) => {
    		jit_int value1 = ((jit_int *)(arg2))[0];
    		jit_int value2 = ((jit_int *)(arg2))[1];
    		if(value1 != 0)
    		{
    			x86_alu_reg_imm(inst, X86_SUB, arg1, value1);
    			x86_alu_reg_imm(inst, X86_SBB, x86_get_other_reg(arg1), value2);
    		}
    		else
    		{
    			x86_alu_reg_imm(inst, X86_SUB, x86_get_other_reg(arg1), value2);
    		}
    	}
        Ins(LSub, Reg(arg1) /* lreg */, Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_SUB, arg1, X86_EBP, arg2);
    		x86_alu_reg_membase(inst, X86_SBB, x86_get_other_reg(arg1), X86_EBP, arg2 + 4);
    	}
        Ins(LSub, Reg(arg1) /* lreg */, Reg(arg2) /* lreg */, None) => {
    		x86_alu_reg_reg(inst, X86_SUB, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_SBB, x86_get_other_reg(arg1), x86_get_other_reg(arg2));
    	}
    
        Ins(LNeg, Reg(arg1) /* lreg */, None, None) => {
    		/* TODO: gcc generates the first variant while
    		   AoA suggests the second. Figure out if one
    		   is better than other. */
    #if 1
    		x86_neg_reg(inst, arg1);
    		x86_alu_reg_imm(inst, X86_ADC, x86_get_other_reg(arg1), 0);
    		x86_neg_reg(inst, x86_get_other_reg(arg1));
    #else
    		x86_neg_reg(inst, x86_get_other_reg(arg1));
    		x86_neg_reg(inst, arg1);
    		x86_alu_reg_imm(inst, X86_SBB, x86_get_other_reg(arg1), 0);
    #endif
    	}
    
        Ins(FAdd, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(DAdd, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(NFAdd, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) => {
    		int flags;
    
    		flags = _jit_regs_select(&regs);
    
    		if((flags & _JIT_REGS_NO_POP) == 0)
    		{
    			x86_fp_op_reg(inst, X86_FADD,
    				fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 1);
    		}
    		else if((flags & _JIT_REGS_FLIP_ARGS) != 0)
    		{
    			x86_fp_op_reg(inst, X86_FADD,
    				fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 0);
    		}
    		else
    		{
    			x86_fp_op(inst, X86_FADD,
    				fp_stack_index(gen, arg2 + JIT_REG_STACK_START));
    		}
    	}
    
        Ins(FSub, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(DSub, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(NFSub, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) => {
    		int flags;
    
    		flags = _jit_regs_select(&regs);
    
    		if((flags & _JIT_REGS_NO_POP) == 0)
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op_reg(inst, X86_FSUB,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 1);
    			}
    			else
    			{
    				x86_fp_op_reg(inst, X86_FSUBR,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START), 1);
    			}
    		}
    		else if((flags & _JIT_REGS_FLIP_ARGS) != 0)
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op_reg(inst, X86_FSUB,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 0);
    			}
    			else
    			{
    				x86_fp_op(inst, X86_FSUBR,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START));
    			}
    		}
    		else
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op(inst, X86_FSUB,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START));
    			}
    			else
    			{
    				x86_fp_op_reg(inst, X86_FSUBR,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START), 0);
    			}
    		}
    	}
    
        Ins(FMul, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(DMul, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(NFMul, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) => {
    		int flags;
    
    		flags = _jit_regs_select(&regs);
    
    		if((flags & _JIT_REGS_NO_POP) == 0)
    		{
    			x86_fp_op_reg(inst, X86_FMUL, fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 1);
    		}
    		else if((flags & _JIT_REGS_FLIP_ARGS) != 0)
    		{
    			x86_fp_op_reg(inst, X86_FMUL, fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 0);
    		}
    		else
    		{
    			x86_fp_op(inst, X86_FMUL, fp_stack_index(gen, arg2 + JIT_REG_STACK_START));
    		}
    	}
    
        Ins(FDiv, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(DDiv, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) |
        Ins(NFDiv, Reg(arg1) /* freg */, Reg(arg2) /* freg */, None) => {
    		int flags;
    
    		flags = _jit_regs_select(&regs);
    
    		if((flags & _JIT_REGS_NO_POP) == 0)
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op_reg(inst, X86_FDIV,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 1);
    			}
    			else
    			{
    				x86_fp_op_reg(inst, X86_FDIVR,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START), 1);
    			}
    		}
    		else if((flags & _JIT_REGS_FLIP_ARGS) != 0)
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op_reg(inst, X86_FDIV,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START), 0);
    			}
    			else
    			{
    				x86_fp_op(inst, X86_FDIVR,
    					fp_stack_index(gen, arg1 + JIT_REG_STACK_START));
    			}
    		}
    		else
    		{
    			if((flags & _JIT_REGS_REVERSE) == 0)
    			{
    				x86_fp_op(inst, X86_FDIV,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START));
    			}
    			else
    			{
    				x86_fp_op_reg(inst, X86_FDIVR,
    					fp_stack_index(gen, arg2 + JIT_REG_STACK_START), 0);
    			}
    		}
    	}
    
        Ins(FRem, Reg(arg1) /* freg */, Reg(arg2) /* freg */, Reg(X86_EAX) /* scratch */, None) |
        Ins(DRem, Reg(arg1) /* freg */, Reg(arg2) /* freg */, Reg(X86_EAX) /* scratch */, None) |
        Ins(NFRem, Reg(arg1) /* freg */, Reg(arg2) /* freg */, Reg(X86_EAX) /* scratch */, None) => {
    		unsigned char *label;
    		label = inst;
    		x86_fprem(inst);
    		x86_fnstsw(inst);
    		x86_alu_reg_imm(inst, X86_AND, X86_EAX, 0x0400);
    		x86_branch(inst, X86_CC_NZ, label, 0);
    		x86_fstp(inst, 1);
    	}
    
        Ins(FNeg, Reg(arg1) /* freg */, None, None) |
        Ins(DNeg, Reg(arg1) /* freg */, None, None) |
        Ins(NFNeg, Reg(arg1) /* freg */, None, None) => {
    		x86_fchs(inst);
    	}
    
    /*
     * Bitwise opcodes.
     */
    
        Ins(IAnd, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_AND, arg1, arg2);
    	}
        Ins(IAnd, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_AND, arg1, X86_EBP, arg2);
    	}
        Ins(IAnd, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_AND, arg1, arg2);
    	}
    
        Ins(IOr, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_OR, arg1, arg2);
    	}
        Ins(IOr, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_OR, arg1, X86_EBP, arg2);
    	}
        Ins(IOr, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg2);
    	}
    
        Ins(IXOr, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_XOR, arg1, arg2);
    	}
        Ins(IXOr, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_XOR, arg1, X86_EBP, arg2);
    	}
        Ins(IXOr, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_XOR, arg1, arg2);
    	}
    
        Ins(INot, Reg(arg1), None, None) => {
    		x86_not_reg(inst, arg1);
    	}
    
        Ins(IShl, Reg(arg1), Imm(arg2), None) => {
    		x86_shift_reg_imm(inst, X86_SHL, arg1, (arg2 & 0x1F));
    	}
        Ins(IShl, Reg(arg1), Reg(X86_ECX), None, None) => {
    		x86_shift_reg(inst, X86_SHL, arg1);
    	}
    
        Ins(IShr, Reg(arg1), Imm(arg2), None) => {
    		x86_shift_reg_imm(inst, X86_SAR, arg1, (arg2 & 0x1F));
    	}
        Ins(IShr, Reg(arg1), Reg(X86_ECX), None, None) => {
    		x86_shift_reg(inst, X86_SAR, arg1);
    	}
    
        Ins(IShrUn, Reg(arg1), Imm(arg2), None) => {
    		x86_shift_reg_imm(inst, X86_SHR, arg1, (arg2 & 0x1F));
    	}
        Ins(IShrUn, Reg(arg1), Reg(X86_ECX), None, None) => {
    		x86_shift_reg(inst, X86_SHR, arg1);
    	}
    
        Ins(LAnd, Reg(arg1) /* lreg */, Imm(arg2), None) => {
    		jit_int value1 = ((jit_int *)(arg2))[0];
    		jit_int value2 = ((jit_int *)(arg2))[1];
    		x86_alu_reg_imm(inst, X86_AND, arg1, value1);
    		x86_alu_reg_imm(inst, X86_AND, x86_get_other_reg(arg1), value2);
    	}
        Ins(LAnd, Reg(arg1) /* lreg */, Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_AND, arg1, X86_EBP, arg2);
    		x86_alu_reg_membase(inst, X86_AND, x86_get_other_reg(arg1), X86_EBP, arg2 + 4);
    	}
        Ins(LAnd, Reg(arg1) /* lreg */, Reg(arg2) /* lreg */, None) => {
    		x86_alu_reg_reg(inst, X86_AND, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_AND, x86_get_other_reg(arg1), x86_get_other_reg(arg2));
    	}
    
        Ins(LOr, Reg(arg1) /* lreg */, Imm(arg2), None) => {
    		jit_int value1 = ((jit_int *)(arg2))[0];
    		jit_int value2 = ((jit_int *)(arg2))[1];
    		x86_alu_reg_imm(inst, X86_OR, arg1, value1);
    		x86_alu_reg_imm(inst, X86_OR, x86_get_other_reg(arg1), value2);
    	}
        Ins(LOr, Reg(arg1) /* lreg */, Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_OR, arg1, X86_EBP, arg2);
    		x86_alu_reg_membase(inst, X86_OR, x86_get_other_reg(arg1), X86_EBP, arg2 + 4);
    	}
        Ins(LOr, Reg(arg1) /* lreg */, Reg(arg2) /* lreg */, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_OR, x86_get_other_reg(arg1), x86_get_other_reg(arg2));
    	}
    
        Ins(LXOr, Reg(arg1) /* lreg */, Imm(arg2), None) => {
    		jit_int value1 = ((jit_int *)(arg2))[0];
    		jit_int value2 = ((jit_int *)(arg2))[1];
    		x86_alu_reg_imm(inst, X86_XOR, arg1, value1);
    		x86_alu_reg_imm(inst, X86_XOR, x86_get_other_reg(arg1), value2);
    	}
        Ins(LXOr, Reg(arg1) /* lreg */, Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_XOR, arg1, X86_EBP, arg2);
    		x86_alu_reg_membase(inst, X86_XOR, x86_get_other_reg(arg1), X86_EBP, arg2 + 4);
    	}
        Ins(LXOr, Reg(arg1) /* lreg */, Reg(arg2) /* lreg */, None) => {
    		x86_alu_reg_reg(inst, X86_XOR, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_XOR, x86_get_other_reg(arg1), x86_get_other_reg(arg2));
    	}
    
        Ins(LNot, Reg(arg1) /* lreg */, None, None) => {
    		x86_not_reg(inst, arg1);
    		x86_not_reg(inst, x86_get_other_reg(arg1));
    	}
    
    /*
     * Branch opcodes.
     */
    
        Ins(Br, None, None, None) => {
    		inst = output_branch(func, inst, 0xEB /* jmp */, insn);
    	}
    
        Ins(BrIFalse, Reg(arg1), None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg1);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrITrue, Reg(arg1), None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg1);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrIEq, Reg(arg1), Imm(0), None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg1);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrINe, Reg(arg1), Imm(0), None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg1);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrILt, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrILt, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrILt, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
    
        Ins(BrILtUn, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrILtUn, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrILtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
    
        Ins(BrILe, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrILe, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrILe, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
    
        Ins(BrILeUn, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrILeUn, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrILeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
    
        Ins(BrIGt, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrIGt, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrIGt, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
    
        Ins(BrIGtUn, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrIGtUn, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrIGtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
    
        Ins(BrIGe, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrIGe, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrIGe, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
    
        Ins(BrIGeUn, Reg(arg1), Imm(arg2), None) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrIGeUn, Reg(arg1), Local(arg2), None) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg1, X86_EBP, arg2);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrIGeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg1, arg2);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
    
        Ins(BrLFalse, Reg(arg1) /* lreg */, None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, x86_get_other_reg(arg1));
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrLTrue, Reg(arg1) /* lreg */, None, None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg1, x86_get_other_reg(arg1));
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
    /*
     * Comparison opcodes.
     */
    
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg2);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
    
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg2);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
    
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
    
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
    
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
    
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
    
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
    
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
    
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
    
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_alu_reg_imm(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_alu_reg_membase(inst, X86_CMP, arg2, X86_EBP, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_CMP, arg2, arg3);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
    
    /*
     * Mathematical opcodes.
     */
    
        Ins(FAtan, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(DAtan, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(NFAtan, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) => {
    		x86_fld1(inst);
    		x86_fpatan(inst);
    		x86_fldz(inst);
    		x86_fp_op_reg(inst, X86_FADD, 1, 1);
    	}
    
        Ins(FCos, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(DCos, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(NFCos, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) => {
    		x86_fcos(inst);
    		x86_fldz(inst);
    		x86_fp_op_reg(inst, X86_FADD, 1, 1);
    	}
    
        Ins(FSin, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(DSin, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) |
        Ins(NFSin, Reg(arg1) /* freg */, Reg(arg2) /* scratch, freg */, None) => {
    		x86_fsin(inst);
    		x86_fldz(inst);
    		x86_fp_op_reg(inst, X86_FADD, 1, 1);
    	}
    
        Ins(FSqrt, Reg(arg1) /* freg */, None, None) |
        Ins(DSqrt, Reg(arg1) /* freg */, None, None) |
        Ins(NFSqrt, Reg(arg1) /* freg */, None, None) => {
    		x86_fsqrt(inst);
    	}
    
    /*
     * Absolute, minimum, maximum, and sign.
     */
    
        Ins(IAbs, Reg(X86_EAX), Reg(X86_EDX) /* scratch */, None, None, None) => {
    		x86_cdq(inst);
    		x86_alu_reg_reg(inst, X86_XOR, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_SUB, arg1, arg2);
    	}
    
        Ins(labs, Reg(arg1) /* lreg */, Reg(arg2) /* scratch */, None) => {
    		x86_mov_reg_reg(inst, arg2, x86_get_other_reg(arg1), 4);
    		x86_shift_reg_imm(inst, X86_SAR, arg2, 31);
    		x86_alu_reg_reg(inst, X86_XOR, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_XOR, x86_get_other_reg(arg1), arg2);
    		x86_alu_reg_reg(inst, X86_SUB, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_SBB, x86_get_other_reg(arg1), arg2);
    	}
    
        Ins(FAbs, Reg(arg1) /* freg */, None, None) |
        Ins(DAbs, Reg(arg1) /* freg */, None, None) |
        Ins(NFAbs, Reg(arg1) /* freg */, None, None) => {
    		x86_fabs(inst);
    	}
    
        Ins(IMinUn, Reg(arg1) /* =, + */, Reg(arg2) /* + */, Reg(arg3)) => {
    		x86_alu_reg_reg(inst, X86_SUB, arg2, arg3);
    		x86_alu_reg_reg(inst, X86_SBB, arg1, arg1);
    		x86_alu_reg_reg(inst, X86_AND, arg1, arg2);
    		x86_alu_reg_reg(inst, X86_ADD, arg1, arg3);
    	}
    
        Ins(ISign, Reg(arg1) /* = */, Imm(arg2), None) => {
    		if(arg2 < 0)
    		{
    			x86_mov_reg_imm(inst, arg1, -1);
    		}
    		else if(arg2 > 0)
    		{
    			x86_mov_reg_imm(inst, arg1, 1);
    		}
    		else
    		{
    			x86_clear_reg(inst, arg1);
    		}
    	}
        Ins(ISign, Reg(arg1) /* =, + */, Reg(arg2) /* + */, None) => {
    		x86_clear_reg(inst, arg1);
    		x86_test_reg_reg(inst, arg2, arg2);
    		x86_set_reg(inst, X86_CC_NZ, arg1, 0);
    		x86_shift_reg_imm(inst, X86_SAR, arg2, 31);
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg2);
    	}
    
        Ins(LSign, Reg(arg1) /* = */, Imm(arg2), None) => {
    		jit_long value = *((jit_long *)(arg2));
    		if(value < 0)
    		{
    			x86_mov_reg_imm(inst, arg1, -1);
    		}
    		else if(value > 0)
    		{
    			x86_mov_reg_imm(inst, arg1, 1);
    		}
    		else
    		{
    			x86_clear_reg(inst, arg1);
    		}
    	}
        Ins(LSign, Reg(arg1) /* =, + */, Reg(arg2) /* +, lreg */, None) => {
    		x86_clear_reg(inst, arg1);
    		x86_alu_reg_reg(inst, X86_OR, arg2, x86_get_other_reg(arg2));
    		x86_set_reg(inst, X86_CC_NZ, arg1, 0);
    		x86_shift_reg_imm(inst, X86_SAR, x86_get_other_reg(arg2), 31);
    		x86_alu_reg_reg(inst, X86_OR, arg1, x86_get_other_reg(arg2));
    	}
    
    /*
     * Pointer check opcodes.
     */
    
        Ins(CheckNull, Reg(arg1), None, None) => {
    #if 0 && defined(JIT_USE_SIGNALS)
    		/* if arg1 contains NULL this generates SEGV and the signal
    		   handler will throw the exception  */
    		x86_alu_reg_membase(inst, X86_CMP, arg1, arg1, 0);
    #else
    		let mut patch;
    		x86_alu_reg_reg(inst, X86_OR, arg1, arg1);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_NULL_REFERENCE);
    		x86_patch(patch, inst);
    #endif
    	}
    
    /*
     * Function calls.
     */
    
        Ins(Call, None, None, None) => {
    		jit_function_t func = (jit_function_t)(insn->dest);
    		x86_call_code(inst, jit_function_to_closure(func));
    	}
    
        Ins(CallTail, None, None, None) => {
    		jit_function_t func = (jit_function_t)(insn->dest);
    		x86_mov_reg_reg(inst, X86_ESP, X86_EBP, size_of::<usize>());
    		x86_pop_reg(inst, X86_EBP);
    		x86_jump_code(inst, jit_function_to_closure(func));
    	}
    
        Ins(CallIndirect, None, None, None) => {
    		x86_call_reg(inst, X86_EAX);
    	}
    
        Ins(CallIndirectTail, None, None, None) => {
    		x86_mov_reg_reg(inst, X86_ESP, X86_EBP, size_of::<usize>());
    		x86_pop_reg(inst, X86_EBP);
    		x86_jump_reg(inst, X86_EAX);
    	}
    
        Ins(CallVTablePtr, None, None, None) => {
    		x86_call_reg(inst, X86_EAX);
    	}
    
        Ins(CallVTablePtrTail, None, None, None) => {
    		x86_mov_reg_reg(inst, X86_ESP, X86_EBP, size_of::<usize>());
    		x86_pop_reg(inst, X86_EBP);
    		x86_jump_reg(inst, X86_EAX);
    	}
    
        Ins(CallExternal, None, None, None) => {
    		x86_call_code(inst, (void *)(insn->dest));
    	}
    
        Ins(CallExternalTail, None, None, None) => {
    		x86_mov_reg_reg(inst, X86_ESP, X86_EBP, size_of::<usize>());
    		x86_pop_reg(inst, X86_EBP);
    		x86_jump_code(inst, (void *)(insn->dest));
    	}
    
        Ins(Return, None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnInt, Reg(X86_EAX), None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnLong, Reg(X86_EAX":"EDX) /* lreg */, None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnFloat32, Reg(arg1) /* freg */,  /* clobber(re) */, None, None) => {
    		/* clobber(freg) frees all registers on the fp stack */
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnFloat64, Reg(arg1) /* freg */,  /* clobber(re) */, None, None) => {
    		/* clobber(freg) frees all registers on the fp stack */
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnNFloat, Reg(arg1) /* freg */,  /* clobber(re) */, None, None) => {
    		/* clobber(freg) frees all registers on the fp stack */
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnSmallStruct, Reg(arg1), Imm(arg2), None) => {
    		switch(arg2)
    		{
    		case 1:
    			x86_widen_membase(inst, X86_EAX, arg1, 0, 0, 0);
    			break;
    
    		case 2:
    			x86_widen_membase(inst, X86_EAX, arg1, 0, 0, 1);
    			break;
    
    		case 3:
    			if(X86_EAX == arg1)
    			{
    				x86_widen_membase(inst, X86_EDX, arg1, 0, 0, 1);
    				x86_widen_membase(inst, X86_EAX, arg1, 2, 0, 0);
    			}
    			else
    			{
    				x86_widen_membase(inst, X86_EAX, arg1, 2, 0, 0);
    				x86_widen_membase(inst, X86_EDX, arg1, 0, 0, 1);
    			}
    			x86_shift_reg_imm(inst, X86_SHL, X86_EAX, 16);
    			x86_alu_reg_reg(inst, X86_OR, X86_EAX, X86_EDX);
    			break;
    
    		case 4:
    			x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    			break;
    
    		case 5:
    			if(X86_EAX == arg1)
    			{
    				x86_widen_membase(inst, X86_EDX, arg1, 4, 0, 0);
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    			}
    			else
    			{
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    				x86_widen_membase(inst, X86_EDX, arg1, 4, 0, 0);
    			}
    			break;
    
    		case 6:
    			if(X86_EAX == arg1)
    			{
    				x86_widen_membase(inst, X86_EDX, arg1, 4, 0, 1);
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    			}
    			else
    			{
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    				x86_widen_membase(inst, X86_EDX, arg1, 4, 0, 1);
    			}
    			break;
    
    		case 7:
    			if(X86_EAX == arg1)
    			{
    				x86_widen_membase(inst, X86_ECX, arg1, 4, 0, 1);
    				x86_widen_membase(inst, X86_EDX, arg1, 6, 0, 0);
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    			}
    			else if(X86_ECX == arg1)
    			{
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    				x86_widen_membase(inst, X86_EDX, arg1, 6, 0, 0);
    				x86_widen_membase(inst, X86_ECX, arg1, 4, 0, 1);
    			}
    			else
    			{
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    				x86_widen_membase(inst, X86_ECX, arg1, 4, 0, 1);
    				x86_widen_membase(inst, X86_EDX, arg1, 6, 0, 0);
    			}
    			x86_shift_reg_imm(inst, X86_SHL, X86_EDX, 16);
    			x86_alu_reg_reg(inst, X86_OR, X86_EDX, X86_ECX);
    			break;
    
    		case 8:
    			if(X86_EAX == arg1)
    			{
    				x86_mov_reg_membase(inst, X86_EDX, arg1, 4, 4);
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    			}
    			else
    			{
    				x86_mov_reg_membase(inst, X86_EAX, arg1, 0, 4);
    				x86_mov_reg_membase(inst, X86_EDX, arg1, 4, 4);
    			}
    			break;
    		}
    
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(SetupForNested, None, None, None) => {
    		jit_nint nest_reg = jit_value_get_nint_constant(insn->value1);
    		if(nest_reg == -1)
    		{
    			x86_push_reg(inst, X86_EBP);
    		}
    		else
    		{
    			x86_mov_reg_reg(inst, _jit_reg_info[nest_reg].cpu_reg,
    							X86_EBP, size_of::<usize>());
    		}
    	}
    
        Ins(SetupForSibling, None, None, None) => {
    		jit_value_t parent;
    		jit_nint level = jit_value_get_nint_constant(insn->value1);
    		jit_nint nest_reg = jit_value_get_nint_constant(insn->value2);
    		int cpu_reg;
    		if(nest_reg == -1)
    		{
    			cpu_reg = X86_EAX;
    		}
    		else
    		{
    			cpu_reg = _jit_reg_info[nest_reg].cpu_reg;
    		}
    		parent = func->builder->parent_frame;
    		if(parent->in_register)
    		{
    			x86_mov_reg_reg(inst, cpu_reg,
    							_jit_reg_info[parent->reg].cpu_reg,
    							size_of::<usize>());
    		}
    		else if(parent->in_global_register)
    		{
    			x86_mov_reg_reg(inst, cpu_reg,
    							_jit_reg_info[parent->global_reg].cpu_reg,
    							size_of::<usize>());
    		}
    		else
    		{
    			_jit_gen_fix_value(parent);
    			x86_mov_reg_membase(inst, cpu_reg, X86_EBP,
    							    parent->frame_offset, size_of::<usize>());
    		}
    		while(level > 0)
    		{
    			gen->ptr = inst;
    			_jit_gen_check_space(gen, 16);
    			x86_mov_reg_membase(inst, cpu_reg, cpu_reg, 0, size_of::<usize>());
    			--level;
    		}
    		if(nest_reg == -1)
    		{
    			x86_push_reg(inst, cpu_reg);
    		}
    	}
    
        Ins(Import, None, None, None) => {
    		unsigned char *inst;
    		int reg;
    		jit_nint level = jit_value_get_nint_constant(insn->value2);
    		_jit_gen_fix_value(insn->value1);
    		reg = _jit_regs_load_value
    			(gen, func->builder->parent_frame, 1, 0);
    		inst = gen->ptr;
    		_jit_gen_check_space(gen, 32 + level * 8);
    		reg = _jit_reg_info[reg].cpu_reg;
    		while(level > 0)
    		{
    			x86_mov_reg_membase(inst, reg, reg, 0, size_of::<usize>());
    			--level;
    		}
    		if(insn->value1->frame_offset != 0)
    		{
    			x86_alu_reg_imm(inst, X86_ADD, reg, insn->value1->frame_offset);
    		}
    		gen->ptr = inst;
    	}
    
    /*
     * Exception handling.
     */
    
        Ins(Throw, Reg(arg1), None, None) => {
    		x86_push_reg(inst, arg1);
    		if(func->builder->setjmp_value != 0)
    		{
    			/* We have a "setjmp" block in the current function,
    			   so we must record the location of the throw first */
    			_jit_gen_fix_value(func->builder->setjmp_value);
    			if(func->builder->position_independent)
    			{
    				x86_call_imm(inst, 0);
    				x86_pop_membase(inst, X86_EBP,
    						func->builder->setjmp_value->frame_offset
    						+ jit_jmp_catch_pc_offset);
    			}
    			else
    			{
    				int pc = (int) inst;
    				x86_mov_membase_imm(inst, X86_EBP,
    						    func->builder->setjmp_value->frame_offset
    						    + jit_jmp_catch_pc_offset, pc, 4);
    			}
    		}
    		x86_call_code(inst, (void *)jit_exception_throw);
    	}
    
        Ins(Rethrow, None, None, None) => { /* Not used in native code back ends */ }
    
        Ins(LoadPc, Reg(arg1) /* = */, None, None) => {
    		if(func->builder->position_independent)
    		{
    			x86_call_imm(inst, 0);
    			x86_pop_reg(inst, arg1);
    		}
    		else
    		{
    			int pc = (int) inst;
    			x86_mov_reg_imm(inst, arg1, pc);
    		}
    	}
    
        Ins(LoadExceptionPc, None, None, None) => { /* Not used in native code back ends */ }
    
        Ins(EnterFinally, None, None, None) => { /* Nothing to do here: return address on the stack */ }
    
        Ins(LeaveFinally, None, None, None) => {
    		/* The "finally" return address is on the stack */
    		x86_ret(inst);
    	}
    
        Ins(CallFinally, None, None, None) => {
    		jit_block_t block;
    
    		block = jit_block_from_label(func, (jit_label_t)(insn->dest));
    		if(!block)
    		{
    			return;
    		}
    
    		if(block->address)
    		{
    			x86_call_code(inst, block->address);
    		}
    		else
    		{
    			x86_call_imm(inst, block->fixup_list);
    			block->fixup_list = (void *)(inst - 4);
    		}
    	}
    
        Ins(EnterFilter, None, None, None) => {
    		/* TODO */
    		TODO();
    	}
    
        Ins(LeaveFilter, None, None, None) => {
    		/* TODO */
    		TODO();
    	}
    
        Ins(CallFilter, None, None, None) => {
    		/* TODO */
    		TODO();
    	}
    
        Ins(CallFilterReturn, None, None, None) => {
    		/* TODO */
    		TODO();
    	}
    
        Ins(AddressOfLabel, Reg(arg1) /* = */, None, None) => {
    		block = jit_block_from_label(func, (jit_label_t)(insn->value1));
    		if(func->builder->position_independent)
    		{
    			/* TODO */
    			TODO();
    		}
    		else
    		{
    			if(block->address)
    			{
    				x86_mov_reg_imm(inst, arg1, block->address);
    			}
    			else
    			{
    				/* Output a placeholder and record on the block's fixup list */
    				x86_mov_reg_imm(inst, arg1, (int)(block->fixup_absolute_list));
    				block->fixup_absolute_list = (void *)(inst - 4);
    			}
    		}
    	}
    
    /*
     * Data manipulation.
     */
    
        Ins(CopyLoadSByte, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyLoadUByte, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyStoreByte, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_mov_membase_imm(inst, X86_EBP, arg1, arg2, 1);
    	}
        Ins(CopyLoadSByte, Local(arg1) /* = */, Reg(arg2) /* breg */, None) |
        Ins(CopyLoadUByte, Local(arg1) /* = */, Reg(arg2) /* breg */, None) |
        Ins(CopyStoreByte, Local(arg1) /* = */, Reg(arg2) /* breg */, None) => {
    		x86_mov_membase_reg(inst, X86_EBP, arg1, arg2, 1);
    	}
        Ins(CopyLoadSByte, Reg(arg1), None, None) |
        Ins(CopyLoadUByte, Reg(arg1), None, None) |
        Ins(CopyStoreByte, Reg(arg1), None, None) => {}
    
        Ins(CopyLoadShort, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyLoadUShort, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyStoreShort, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_mov_membase_imm(inst, X86_EBP, arg1, arg2, 2);
    	}
        Ins(CopyLoadShort, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyLoadUShort, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyStoreShort, Local(arg1) /* = */, Reg(arg2), None) => {
    		x86_mov_membase_reg(inst, X86_EBP, arg1, arg2, 2);
    	}
        Ins(CopyLoadShort, Reg(arg1), None, None) |
        Ins(CopyLoadUShort, Reg(arg1), None, None) |
        Ins(CopyStoreShort, Reg(arg1), None, None) => {}
    
        Ins(CopyInt, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_mov_membase_imm(inst, X86_EBP, arg1, arg2, 4);
    	}
        Ins(CopyInt, Reg(arg1), None, None) => {}
    
        Ins(CopyLong, Reg(arg1) /* lreg */, None, None) => {}
    
        Ins(CopyFloat32, Reg(arg1) /* freg */, None, None) => {}
    
        Ins(CopyFloat64, Reg(arg1) /* freg */, None, None) => {}
    
        Ins(CopyNFloat, Reg(arg1) /* freg */, None, None) => {}
    
        Ins(CopyStruct, Frame /* = */, Frame,  /* clobber(e) */, None, None, None) => {
    		inst = memory_copy(gen, inst, X86_EBP, arg1, X86_EBP, arg2,
    				   jit_type_get_size(jit_value_get_type(insn->dest)));
    	}
    
        Ins(AddressOf, Reg(arg1) /* = */, Frame, None, None) => {
    		x86_lea_membase(inst, arg1, X86_EBP, arg2);
    	}
    
    /*
     * Stack pushes and pops.
     */
    
        Ins(IncomingReg, Reg(arg1), None, None) |
        Ins(ReturnReg, Reg(arg1), None, None) => {
    		/*
    		 * This rule does nothing itself. Also at this point
    		 * the value is supposed to be already in the register
    		 * so the "reg" pattern does not load it either. But
    		 * it allows the allocator to check the liveness flags
    		 * and free the register if the value is dead.
    		 */
    	}
    
        Ins(PushInt, Imm(arg1), None, None) => {
    		x86_push_imm(inst, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushInt, Local(arg1), None, None) => {
    		x86_push_membase(inst, X86_EBP, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushInt, Reg(arg1), None, None) => {
    		x86_push_reg(inst, arg1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushLong, Imm(arg1), None, None) => {
    		x86_push_imm(inst, ((jit_int *)(arg1))[1]);
    		x86_push_imm(inst, ((jit_int *)(arg1))[0]);
    		gen->stack_changed = 1;
    	}
        Ins(PushLong, Local(arg1), None, None) => {
    		x86_push_membase(inst, X86_EBP, arg1 + 4);
    		x86_push_membase(inst, X86_EBP, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushLong, Reg(arg1) /* lreg */, None, None) => {
    		x86_push_reg(inst, x86_get_other_reg(arg1));
    		x86_push_reg(inst, arg1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushFloat32, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_push_imm(inst, ptr[0]);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat32, Local(arg1), None, None) => {
    		x86_push_membase(inst, X86_EBP, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat32, Reg(arg1) /* freg */, None, None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f32>());
    		x86_fst_membase(inst, X86_ESP, 0, 0, 1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushFloat64, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_push_imm(inst, ptr[1]);
    		x86_push_imm(inst, ptr[0]);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat64, Local(arg1), None, None) => {
    		x86_push_membase(inst, X86_EBP, arg1 + 4);
    		x86_push_membase(inst, X86_EBP, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat64, Reg(arg1) /* freg */, None, None) => {
    		x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f64>());
    		x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushNFloat, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_push_imm(inst, ptr[2]);
    		}
    		x86_push_imm(inst, ptr[1]);
    		x86_push_imm(inst, ptr[0]);
    		gen->stack_changed = 1;
    	}
        Ins(PushNFloat, Local(arg1), None, None) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_push_membase(inst, X86_EBP, arg1 + 8);
    		}
    		x86_push_membase(inst, X86_EBP, arg1 + 4);
    		x86_push_membase(inst, X86_EBP, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushNFloat, Reg(arg1) /* freg */, None, None) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_alu_reg_imm(inst, X86_SUB, X86_ESP, sizeof(jit_nfloat));
    			x86_fst80_membase(inst, X86_ESP, 0);
    		}
    		else
    		{
    			x86_alu_reg_imm(inst, X86_SUB, X86_ESP, size_of::<f64>());
    			x86_fst_membase(inst, X86_ESP, 0, 1, 1);
    		}
    		gen->stack_changed = 1;
    	}
    
        Ins(PushStruct, Reg(arg1),  /* clobber(e) */, None, None) => {
    		jit_nuint size;
    		size = (jit_nuint)jit_value_get_nint_constant(insn->value2);
    		if((size % size_of::<usize>()) == 0 && size <= 4 * size_of::<usize>())
    		{
    			/* Handle small structures that are a multiple of the word size */
    			while(size > 0)
    			{
    				size -= size_of::<usize>();
    				x86_push_membase(inst, arg1, size);
    			}
    		}
    		else
    		{
    			/* Handle arbitrary-sized structures */
    			x86_alu_reg_imm(inst, X86_SUB, X86_ESP, ROUND_STACK(size));
    			inst = memory_copy(gen, inst, X86_ESP, 0, arg1, 0, size);
    		}
    		gen->stack_changed = 1;
    	}
    
        Ins(PopStack, None, None, None) => {
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, insn->value1->address);
    		gen->stack_changed = 1;
    	}
    
        Ins(FlushSmallStruct, None, None, None) => {
    		jit_nuint size;
    		jit_nint offset;
    		_jit_gen_fix_value(insn->value1);
    		size = jit_type_get_size(jit_value_get_type(insn->value1));
    		offset = insn->value1->frame_offset;
    		inst = store_small_struct
    			(inst, X86_EAX, X86_EDX, X86_EBP, offset, (jit_nint)size, 0);
    	}
    
    /*
     * Pointer-relative loads and stores.
     */
    
        Ins(LoadRelativeSByte, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_widen_membase(inst, arg1, arg2, arg3, 1, 0);
    	}
    
        Ins(LoadRelativeUByte, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_widen_membase(inst, arg1, arg2, arg3, 0, 0);
    	}
    
        Ins(LoadRelativeShort, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_widen_membase(inst, arg1, arg2, arg3, 1, 1);
    	}
    
        Ins(LoadRelativeUShort, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_widen_membase(inst, arg1, arg2, arg3, 0, 1);
    	}
    
        Ins(LoadRelativeInt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_mov_reg_membase(inst, arg1, arg2, arg3, 4);
    	}
    
        Ins(LoadRelativeLong, Reg(arg1) /* =, lreg */, Reg(arg2), Imm(arg3)) => {
    		if(arg1 == arg2)
    		{
    			x86_mov_reg_membase(inst, x86_get_other_reg(arg1), arg2, arg3 + 4, 4);
    			x86_mov_reg_membase(inst, arg1, arg2, arg3, 4);
    		}
    		else
    		{
    			x86_mov_reg_membase(inst, arg1, arg2, arg3, 4);
    			x86_mov_reg_membase(inst, x86_get_other_reg(arg1), arg2, arg3 + 4, 4);
    		}
    	}
    
        Ins(LoadRelativeFloat32, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3)) => {
    		x86_fld_membase(inst, arg2, arg3, 0);
    	}
    
        Ins(LoadRelativeFloat64, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3)) => {
    		x86_fld_membase(inst, arg2, arg3, 1);
    	}
    	
        Ins(LoadRelativeNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3),  if sizeof(jit_nfloat) != size_of::<f64>()) => {
    		x86_fld80_membase(inst, arg2, arg3);
    	}
        Ins(LoadRelativeNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3),  if sizeof(jit_nfloat) == size_of::<f64>()) => {
    		x86_fld_membase(inst, arg2, arg3, 1);
    	}
    
        Ins(LoadRelativeStruct, Frame /* = */, Reg(arg1), Imm(arg2),  /* clobber(e) */, None) => {
    		inst = memory_copy(gen, inst, X86_EBP, arg1, arg2, arg3,
    				   jit_type_get_size(jit_value_get_type(insn->dest)));
    	}
    
        Ins(StoreRelativeByte, Imm(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_mem_imm(inst, arg1 + arg3, arg2, 1);
    	}
        Ins(StoreRelativeByte, Imm(arg1), Reg(arg2) /* breg */, Imm(arg3)) => {
    		x86_mov_mem_reg(inst, arg1 + arg3, arg2, 1);
    	}
        Ins(StoreRelativeByte, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, arg2, 1);
    	}
        Ins(StoreRelativeByte, Reg(arg1), Reg(arg2) /* breg */, Imm(arg3)) => {
    		x86_mov_membase_reg(inst, arg1, arg3, arg2, 1);
    	}
    
        Ins(StoreRelativeShort, Imm(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_mem_imm(inst, arg1 + arg3, arg2, 2);
    	}
        Ins(StoreRelativeShort, Imm(arg1), Reg(arg2), Imm(arg3)) => {
    		x86_mov_mem_reg(inst, arg1 + arg3, arg2, 2);
    	}
        Ins(StoreRelativeShort, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, arg2, 2);
    	}
        Ins(StoreRelativeShort, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		x86_mov_membase_reg(inst, arg1, arg3, arg2, 2);
    	}
    
        Ins(StoreRelativeInt, Imm(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_mem_imm(inst, arg1 + arg3, arg2, 4);
    	}
        Ins(StoreRelativeInt, Imm(arg1), Reg(arg2), Imm(arg3)) => {
    		x86_mov_mem_reg(inst, arg1 + arg3, arg2, 4);
    	}
        Ins(StoreRelativeInt, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, arg2, 4);
    	}
        Ins(StoreRelativeInt, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		x86_mov_membase_reg(inst, arg1, arg3, arg2, 4);
    	}
    
        Ins(StoreRelativeLong, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, *(int *)(arg2), 4);
    		x86_mov_membase_imm(inst, arg1, arg3 + 4, *(int *)(arg2 + 4), 4);
    	}
        Ins(StoreRelativeLong, Reg(arg1), Local(arg2), Imm(arg3), Reg(arg4) /* scratch */) => {
    		x86_mov_reg_membase(inst, arg4, X86_EBP, arg2, 4);
    		x86_mov_membase_reg(inst, arg1, arg3, arg4, 4);
    		x86_mov_reg_membase(inst, arg4, X86_EBP, arg2 + 4, 4);
    		x86_mov_membase_reg(inst, arg1, arg3 + 4, arg4, 4);
    	}
        Ins(StoreRelativeLong, Reg(arg1), Reg(arg2) /* lreg */, Imm(arg3)) => {
    		x86_mov_membase_reg(inst, arg1, arg3, arg2, 4);
    		x86_mov_membase_reg(inst, arg1, arg3 + 4, x86_get_other_reg(arg2), 4);
    	}
    
        Ins(StoreRelativeFloat32, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, ((int *)(arg2))[0], 4);
    	}
        Ins(StoreRelativeFloat32, Reg(arg1), Reg(arg2) /* freg */, Imm(arg3)) => {
    		x86_fst_membase(inst, arg1, arg3, 0, 1);
    	}
    
        Ins(StoreRelativeFloat64, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, ((int *)(arg2))[0], 4);
    		x86_mov_membase_imm(inst, arg1, arg3 + 4, ((int *)(arg2))[1], 4);
    	}
        Ins(StoreRelativeFloat64, Reg(arg1), Reg(arg2) /* freg */, Imm(arg3)) => {
    		x86_fst_membase(inst, arg1, arg3, 1, 1);
    	}
    
        Ins(StoreRelativeNFloat, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_mov_membase_imm(inst, arg1, arg3, ((int *)(arg2))[0], 4);
    		x86_mov_membase_imm(inst, arg1, arg3 + 4, ((int *)(arg2))[1], 4);
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_mov_membase_imm(inst, arg1, arg3 + 8, ((int *)(arg2))[2], 4);
    		}
    	}
        Ins(StoreRelativeNFloat, Reg(arg1), Reg(arg2) /* freg */, Imm(arg3)) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_fst80_membase(inst, arg1, arg3);
    		}
    		else
    		{
    			x86_fst_membase(inst, arg1, arg3, 1, 1);
    		}
    	}
    
        Ins(StoreRelativeStruct, None, None, None) => {
    		unsigned char *inst;
    		int reg = _jit_regs_load_value
    			(gen, insn->dest, 0,
    			 (insn->flags & (JIT_INSN_DEST_NEXT_USE |
    			 				 JIT_INSN_DEST_LIVE)));
    		_jit_regs_spill_all(gen);
    		_jit_gen_fix_value(insn->value1);
    		inst = gen->ptr;
    		_jit_gen_check_space(gen, 128);
    		reg = _jit_reg_info[reg].cpu_reg;
    		inst = memory_copy(gen, inst, reg, (int)(insn->value2->address),
    						   X86_EBP, insn->value1->frame_offset,
    						   jit_type_get_size(jit_value_get_type(insn->value1)));
    		gen->ptr = inst;
    	}
    
        Ins(AddRelative, Reg(arg1), Imm(arg2), None) => {
    		if(insn->value2->address != 0)
    		{
    			x86_alu_reg_imm(inst, X86_ADD, arg1, arg2);
    		}
    	}
    
    /*
     * Array element loads and stores.
     */
    
        Ins(LoadElementSByte, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_widen_memindex(inst, arg1, arg2, 0, arg3, 0, 1, 0);
    	}
    
        Ins(LoadElementUByte, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_widen_memindex(inst, arg1, arg2, 0, arg3, 0, 0, 0);
    	}
    
        Ins(LoadElementShort, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_widen_memindex(inst, arg1, arg2, 0, arg3, 1, 1, 1);
    	}
    
        Ins(LoadElementUShort, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_widen_memindex(inst, arg1, arg2, 0, arg3, 1, 0, 1);
    	}
    
        Ins(LoadElementInt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_mov_reg_memindex(inst, arg1, arg2, 0, arg3, 2, 4);
    	}
    
        Ins(LoadElementLong, Reg(arg1) /* =, lreg */, Reg(arg2), Reg(arg3)) => {
    		if(arg1 == arg2 || arg1 == arg3)
    		{
    			x86_mov_reg_memindex(inst, x86_get_other_reg(arg1), arg2, 4, arg3, 3, 4);
    			x86_mov_reg_memindex(inst, arg1, arg2, 0, arg3, 3, 4);
    		}
    		else
    		{
    			x86_mov_reg_memindex(inst, arg1, arg2, 0, arg3, 3, 4);
    			x86_mov_reg_memindex(inst, x86_get_other_reg(arg1), arg2, 4, arg3, 3, 4);
    		}
    	}
    
        Ins(LoadElementFloat32, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3)) => {
    		x86_fld_memindex(inst, arg2, 0, arg3, 2, 0);
    	}
    
        Ins(LoadElementFloat64, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3)) => {
    		x86_fld_memindex(inst, arg2, 0, arg3, 3, 1);
    	}
    
        Ins(LoadElementNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3) /* + */,  if sizeof(jit_nfloat) != size_of::<f64>()) => {
        		/* lea arg3, [arg3 + arg3 * 2]  */
    		x86_lea_memindex(inst, arg3, arg3, 0, arg3, 1);
    		/* fld [arg2 * 4] */
    		x86_fld80_memindex(inst, arg2, 0, arg3, 2);
    	}
        Ins(LoadElementNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Reg(arg3),  if sizeof(jit_nfloat) == size_of::<f64>()) => {
    		x86_fld_memindex(inst, arg2, 0, arg3, 3, 1);
    	}
    
        Ins(StoreElementByte, Reg(arg1), Reg(arg2), Reg(arg3) /* breg */) => {
    		x86_mov_memindex_reg(inst, arg1, 0, arg2, 0, arg3, 1);
    	}
    
        Ins(StoreElementShort, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_mov_memindex_reg(inst, arg1, 0, arg2, 1, arg3, 2);
    	}
    
        Ins(StoreElementInt, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_mov_memindex_reg(inst, arg1, 0, arg2, 2, arg3, 4);
    	}
    
        Ins(StoreElementLong, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		x86_mov_memindex_imm(inst, arg1, 0, arg2, 3, *(int *)(arg3), 4);
    		x86_mov_memindex_imm(inst, arg1, 4, arg2, 3, *(int *)(arg3 + 4), 4);
    	}
        Ins(StoreElementLong, Reg(arg1), Reg(arg2), Local(arg3), Reg(arg4) /* scratch */) => {
    		x86_mov_reg_membase(inst, arg4, X86_EBP, arg3, 4);
    		x86_mov_memindex_reg(inst, arg1, 0, arg2, 3, arg4, 4);
    		x86_mov_reg_membase(inst, arg4, X86_EBP, arg3 + 4, 4);
    		x86_mov_memindex_reg(inst, arg1, 4, arg2, 3, arg4, 4);
    	}
        Ins(StoreElementLong, Reg(arg1), Reg(arg2), Reg(arg3) /* lreg */) => {
    		x86_mov_memindex_reg(inst, arg1, 0, arg2, 3, arg3, 4);
    		x86_mov_memindex_reg(inst, arg1, 4, arg2, 3, x86_get_other_reg(arg3), 4);
    	}
    
        Ins(StoreElementFloat32, Reg(arg1), Reg(arg2), Reg(arg3) /* freg */) => {
    		x86_fst_memindex(inst, arg1, 0, arg2, 2, 0, 1);
    	}
    
        Ins(StoreElementFloat64, Reg(arg1), Reg(arg2), Reg(arg3) /* freg */) => {
    		x86_fst_memindex(inst, arg1, 0, arg2, 3, 1, 1);
    	}
    
        Ins(StoreElementNFloat, Reg(arg1), Reg(arg2) /* + */, Reg(arg3) /* freg */,  if sizeof(jit_nfloat) != size_of::<f64>()) => {
        		/* lea reg2, [reg2 + reg2 * 2]  */
    		x86_lea_memindex(inst, arg2, arg2, 0, arg2, 1);
    		/* fst [reg2 * 4] */
    		x86_fst80_memindex(inst, arg1, 0, arg2, 2);
    	}
        Ins(StoreElementNFloat, Reg(arg1), Reg(arg2), Reg(arg3) /* freg */,  if sizeof(jit_nfloat) == size_of::<f64>()) => {
    		x86_fst_memindex(inst, arg1, 0, arg2, 3, 1, 1);
    	}
    
    /*
     * Block operations.
     */
    
        Ins(Memcpy, _, _, Imm(arg1),  if arg3 <= 0, None, None) => { }
        Ins(Memcpy, Reg(arg1), Reg(arg2), Imm(arg3), Reg(arg4) /* scratch, breg */,  if arg3 <= 32,  /* space(32 + arg3 * 4) */) => {
    		int disp;
    		disp = 0;
    		while(arg3 >= (disp + 4))
    		{
    			x86_mov_reg_membase(inst, arg4, arg2, disp, 4);
    			x86_mov_membase_reg(inst, arg1, disp, arg4, 4);
    			disp += 4;
    		}
    		if(arg3 >= (disp + 2))
    		{
    			x86_mov_reg_membase(inst, arg4, arg2, disp, 2);
    			x86_mov_membase_reg(inst, arg1, disp, arg4, 2);
    			disp += 2;
    		}
    		if(arg3 > disp)
    		{
    			x86_mov_reg_membase(inst, arg4, arg2, disp, 1);
    			x86_mov_membase_reg(inst, arg1, disp, arg4, 1);
    		}
    	}
        Ins(Memcpy, Reg(arg1), Reg(arg2), Reg(arg3),  /* clobber(eax", "ecx", "edx", "ebx) */) => {
    		x86_push_reg(inst, arg3);
    		x86_push_reg(inst, arg2);
    		x86_push_reg(inst, arg1);
    		x86_call_code(inst, jit_memcpy);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, 3 * size_of::<usize>());
    	}
    
        Ins(MemMove, _, _, Imm(arg1),  if arg3 <= 0, None, None) => { }
        Ins(MemMove, Reg(arg1), Reg(arg2), Reg(arg3),  /* clobber(eax", "ecx", "edx", "ebx) */) => {
    		x86_push_reg(inst, arg3);
    		x86_push_reg(inst, arg2);
    		x86_push_reg(inst, arg1);
    		x86_call_code(inst, jit_memmove);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, 3 * size_of::<usize>());
    	}
    
        Ins(Memset, _, _, Imm(arg1),  if arg3 <= 0, None, None) => { }
        Ins(Memset, Reg(arg1), Imm(arg2), Imm(arg3),  if arg3 <= 32,  /* space(32 + arg3 * 4) */) => {
    		int disp;
    		disp = 0;
    		while(arg3 >= (disp + 4))
    		{
    			x86_mov_membase_imm(inst, arg1, disp, arg2 * 0x01010101, 4);
    			disp += 4;
    		}
    		if(arg3 >= (disp + 2))
    		{
    			x86_mov_membase_imm(inst, arg1, disp, arg2 * 0x0101, 2);
    			disp += 2;
    		}
    		if(insn->value2->address > disp)
    		{
    			x86_mov_membase_imm(inst, arg1, disp, arg2, 1);
    		}
    	}
        Ins(Memset, Reg(arg1), Reg(arg2) /* breg */, Imm(arg3),  if arg3 < 4) => {
    		x86_mov_membase_reg(inst, arg1, 0, arg2, 1);
    		if(arg3 > 1)
    		{
    			x86_mov_membase_reg(inst, arg1, 1, arg2, 1);
    			if(arg3 > 2)
    			{
    				x86_mov_membase_reg(inst, arg1, 2, arg2, 1);
    			}
    		}
    	}
        Ins(Memset, Reg(arg1), Reg(arg2) /* + */, Imm(arg3), Reg(arg4) /* scratch */,  if arg3 <= 32 && (arg3 % 2) == 0,  /* space(32 + arg3 * 4) */) => {
    		int disp;
    		x86_mov_reg_reg(inst, arg4, arg2, 4);
    		x86_shift_reg_imm(inst, X86_SHL, arg2, 8);
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg4);
    		x86_mov_reg_reg(inst, arg4, arg2, 4);
    		x86_shift_reg_imm(inst, X86_SHL, arg2, 16);
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg4);
    		disp = 0;
    		while(arg3 >= (disp + 4))
    		{
    			x86_mov_membase_reg(inst, arg1, disp, arg2, 4);
    			disp += 4;
    		}
    		if(arg3 > disp)
    		{
    			x86_mov_membase_reg(inst, arg1, disp, arg2, 2);
    		}
    	}
        Ins(Memset, Reg(arg1), Reg(arg2) /* +, breg */, Imm(arg3), Reg(arg4) /* scratch */,  if arg3 <= 32 && (arg3 % 2) != 0,  /* space(32 + arg3 * 4) */) => {
    		int disp;
    		x86_mov_reg_reg(inst, arg4, arg2, 4);
    		x86_shift_reg_imm(inst, X86_SHL, arg2, 8);
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg4);
    		x86_mov_reg_reg(inst, arg4, arg2, 4);
    		x86_shift_reg_imm(inst, X86_SHL, arg2, 16);
    		x86_alu_reg_reg(inst, X86_OR, arg2, arg4);
    		disp = 0;
    		while(arg3 >= (disp + 4))
    		{
    			x86_mov_membase_reg(inst, arg1, disp, arg2, 4);
    			disp += 4;
    		}
    		if(arg3 >= (disp + 2))
    		{
    			x86_mov_membase_reg(inst, arg1, disp, arg2, 2);
    			disp += 2;
    		}
    		if(arg3 > disp)
    		{
    			x86_mov_membase_reg(inst, arg1, disp, arg2, 1);
    		}
    	}
        Ins(Memset, Reg(arg1), Reg(arg2), Reg(arg3),  /* clobber(eax", "ecx", "edx", "ebx) */) => {
    		x86_push_reg(inst, arg3);
    		x86_push_reg(inst, arg2);
    		x86_push_reg(inst, arg1);
    		x86_call_code(inst, jit_memset);
    		x86_alu_reg_imm(inst, X86_ADD, X86_ESP, 3 * size_of::<usize>());
    	}
    
    /*
     * Allocate memory from the stack.
     */
    
        Ins(Alloca, Reg(arg1), None, None) => {
    		x86_alu_reg_imm(inst, X86_ADD, arg1, 15);
    		x86_alu_reg_imm(inst, X86_AND, arg1, ~15);
    		x86_alu_reg_reg(inst, X86_SUB, X86_ESP, arg1);
    		x86_mov_reg_reg(inst, arg1, X86_ESP, 4);
    		gen->stack_changed = 1;
    	}
    
        Ins(JumpTable, Reg(arg1), Imm(arg2), Imm(arg3),  /* space(32 + size_of::<usize>() * arg3) */) => {
    		let mut patch_jump_table;
    		let mut patch_fall_through;
    		int index;
    		jit_label_t *labels;
    		jit_nint num_labels;
    		jit_block_t block;
    
    		labels = (jit_label_t *) arg2;
    		num_labels = arg3;
    
    		x86_alu_reg_imm(inst, X86_CMP, arg1, num_labels);
    		patch_fall_through = inst;
    		x86_branch32(inst, X86_CC_AE, 0, 0);
    
    		if(func->builder->position_independent)
    		{
    			/* TODO */
    			TODO();
    		}
    		else
    		{
    			patch_jump_table = inst;
    			x86_jump_memindex(inst, X86_NOBASEREG, 0, arg1, 2);
    			while(((jit_nint) inst & (sizeof(void*) - 1)) != 0)
    			{
    				x86_nop(inst);
    			}
    
    			// displacement goes after opcode. ModR/M, and SIB bytes
    			*((void **)(patch_jump_table + 3)) = inst;
    		}
    
    		for(index = 0; index < num_labels; index++)
    		{
    			block = jit_block_from_label(func, labels[index]);
    			if(!block)
    			{
    				return;
    			}
    
    			if(func->builder->position_independent)
    			{
    				/* TODO */
    				TODO();
    			}
    			else
    			{
    				if(block->address)
    				{
    					x86_imm_emit32(inst, block->address);
    				}
    				else
    				{
    					/* Output a placeholder and record on the block's fixup list */
    					x86_imm_emit32(inst, (int)(block->fixup_absolute_list));
    					block->fixup_absolute_list = (void *)(inst - 4);
    				}
    			}
    		}
    
    		x86_patch(patch_fall_through, inst);
    	}
    }
}



// XXXX64XXXX

/*
 * jit-rules-x86-64.ins - Instruction selector for x86_64.
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
 
use codegen::Op::*;
use codegen::Arg::*;
 
/*
 * Conversion opcodes.
 */

pub fn x86_64_write(inst: &Emit, ins: Ins) {
    match ins {
        Ins(TruncSByte, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_movsx8_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(TruncUByte, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_movzx8_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(TruncShort, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_movsx16_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(TruncUShort, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_movzx16_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(TruncInt, Reg(arg1) /* = */, Reg(arg2), None) => {
    		if(arg1 != arg2)
    		{
    			x86_64_mov_reg_reg_size(inst, arg1, arg2, 4);
    		}
    	}
    
        Ins(TruncUInt, Reg(arg1) /* = */, Reg(arg2), None) => {
    		if(arg1 != arg2)
    		{
    			x86_64_mov_reg_reg_size(inst, arg1, arg2, 4);
    		}
    	}
    
        Ins(LowWord, Reg(arg1) /* = */, Imm(arg2), None) => {
    		x86_64_mov_reg_imm_size(inst, arg1, arg2, 4);
    	}
        Ins(LowWord, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_mov_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(LowWord, Reg(arg1) /* = */, Reg(arg2), None) => {
    		if(arg1 != arg2)
    		{
    			x86_64_mov_reg_reg_size(inst, arg1, arg2, 4);
    		}
    	}
    	
        Ins(ExpandInt, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_movsx32_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(ExpandUInt, Reg(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_mov_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(IntToNFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_64_fild_membase_size(inst, X86_64_RBP, arg2, 4);
    	}
        Ins(IntToNFloat, Reg(arg1) /* =, freg */, Reg(arg2), None) => {
    #ifdef HAVE_RED_ZONE
    		x86_64_mov_membase_reg_size(inst, X86_64_RSP, -8, arg2, 4);
    		x86_64_fild_membase_size(inst, X86_64_RSP, -8, 4);
    #else
    		x86_64_push_reg_size(inst, arg2, 8);
    		x86_64_fild_membase_size(inst, X86_64_RSP, 0, 4);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, size_of::<usize>(), 8);
    #endif
    	}
    
        Ins(LongToNFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_64_fild_membase_size(inst, X86_64_RBP, arg2, 8);
    	}
        Ins(LongToNFloat, Reg(arg1) /* =, freg */, Reg(arg2), None) => {
    #ifdef HAVE_RED_ZONE
    		x86_64_mov_membase_reg_size(inst, X86_64_RSP, -8, arg2, 8);
    		x86_64_fild_membase_size(inst, X86_64_RSP, -8, 8);
    #else
    		x86_64_push_reg_size(inst, arg2, 8);
    		x86_64_fild_membase_size(inst, X86_64_RSP, 0, 8);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, size_of::<usize>(), 8);
    #endif
    	}
    
        Ins(Float32ToInt, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttss2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(Float32ToInt, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttss2si_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(Float32ToUInt, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttss2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(Float32ToUInt, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttss2si_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(Float32ToLong, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttss2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(Float32ToLong, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttss2si_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(IntToFloat32, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtsi2ss_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IntToFloat32, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_cvtsi2ss_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(UIntToFloat32, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_mov_reg_reg_size(inst, arg2, arg2, 4);
    		x86_64_cvtsi2ss_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LongToFloat32, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtsi2ss_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LongToFloat32, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_cvtsi2ss_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(Float64ToFloat32, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtsd2ss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(Float64ToFloat32, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvtsd2ss_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(Float64ToInt, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttsd2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(Float64ToInt, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttsd2si_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(Float64ToUInt, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttsd2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(Float64ToUInt, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttsd2si_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(Float64ToLong, Reg(arg1) /* = */, Local(arg2), None) => {
    		x86_64_cvttsd2si_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(Float64ToLong, Reg(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvttsd2si_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(IntToFloat64, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtsi2sd_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IntToFloat64, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_cvtsi2sd_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(UIntToFloat64, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_mov_reg_reg_size(inst, arg2, arg2, 4);
    		x86_64_cvtsi2sd_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LongToFloat64, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtsi2sd_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LongToFloat64, Reg(arg1) /* =, xreg */, Reg(arg2), None) => {
    		x86_64_cvtsi2sd_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(Float32ToFloat64, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_cvtss2sd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(Float32ToFloat64, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_cvtss2sd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(NFloatToInt, Reg(arg1) /* = */, Reg(arg2) /* freg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_nfloat_to_int(inst, arg1, arg3, 4);
    	}
    
        Ins(NFloatToLong, Reg(arg1) /* = */, Reg(arg2) /* freg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_nfloat_to_int(inst, arg1, arg3, 8);
    	}
    
        Ins(Float32TONFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_64_fld_membase_size(inst, X86_64_RBP, arg2, 4);
    	}
        Ins(Float32TONFloat, Reg(arg1) /* =, freg */, Reg(arg2) /* xreg */, None) => {
    #ifdef HAVE_RED_ZONE
    		x86_64_movss_membase_reg(inst, X86_64_RSP, -8, arg2);
    		x86_64_fld_membase_size(inst, X86_64_RSP, -8, 4);
    #else
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_movss_regp_reg(inst, X86_64_RSP, arg2);
    		x86_64_fld_regp_size(inst, X86_64_RSP, 4);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, 8, 8);
    #endif
    	}
    
        Ins(Float64ToNFloat, Reg(arg1) /* =, freg */, Local(arg2), None) => {
    		x86_64_fld_membase_size(inst, X86_64_RBP, arg2, 8);
    	}
        Ins(Float64ToNFloat, Reg(arg1) /* =, freg */, Reg(arg2) /* xreg */, None) => {
    #ifdef HAVE_RED_ZONE
    		x86_64_movsd_membase_reg(inst, X86_64_RSP, -8, arg2);
    		x86_64_fld_membase_size(inst, X86_64_RSP, -8, 8);
    #else
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_movsd_regp_reg(inst, X86_64_RSP, arg2);
    		x86_64_fld_regp_size(inst, X86_64_RSP, 8);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, 8, 8);
    #endif
    	}
    
        Ins(NFloatToFloat32, Local(arg1) /* = */, Reg(arg2) /* freg */, None) => {
    		x86_64_fstp_membase_size(inst, X86_64_RBP, arg1, 4);
    	}
        Ins(NFloatToFloat32, Reg(arg1) /* =, xreg */, Reg(arg2) /* freg */, None) => {
    #ifdef HAVE_RED_ZONE
    		/* Avoid modifying the stack pointer by simply using negative */
    		/* offsets here. */
    		x86_64_fstp_membase_size(inst, X86_64_RSP, -8, 4);
    		x86_64_movss_reg_membase(inst, arg1, X86_64_RSP, -8);
    #else
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_fstp_regp_size(inst, X86_64_RSP, 4);
    		x86_64_movss_reg_regp(inst, arg1, X86_64_RSP);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, 8, 8);
    #endif
    	}
    
        Ins(NFloatToFloat64, Local(arg1) /* = */, Reg(arg2) /* freg */, None) => {
    		x86_64_fstp_membase_size(inst, X86_64_RBP, arg1, 8);
    	}
        Ins(NFloatToFloat64, Reg(arg1) /* =, xreg */, Reg(arg2) /* freg */, None) => {
    #ifdef HAVE_RED_ZONE
    		/* Avoid modifying the stack pointer by simply using negative */
    		/* offsets here. */
    		x86_64_fstp_membase_size(inst, X86_64_RSP, -8, 8);
    		x86_64_movsd_reg_membase(inst, arg1, X86_64_RSP, -8);
    #else
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_fstp_regp_size(inst, X86_64_RSP, 8);
    		x86_64_movsd_reg_regp(inst, arg1, X86_64_RSP);
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, 8, 8);
    #endif
    	}
    
    /*
     * Data manipulation.
     */
    
        Ins(CopyLoadSByte, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyLoadUByte, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyStoreByte, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RBP, arg1, arg2, 1);
    	}
        Ins(CopyLoadSByte, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyLoadUByte, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyStoreByte, Local(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_mov_membase_reg_size(inst, X86_64_RBP, arg1, arg2, 1);
    	}
        Ins(CopyLoadSByte, Reg(arg1), None, None) |
        Ins(CopyLoadUByte, Reg(arg1), None, None) |
        Ins(CopyStoreByte, Reg(arg1), None, None) => {}
    
        Ins(CopyLoadShort, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyLoadUShort, Local(arg1) /* = */, Imm(arg2), None) |
        Ins(CopyStoreShort, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RBP, arg1, arg2, 2);
    	}
        Ins(CopyLoadShort, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyLoadUShort, Local(arg1) /* = */, Reg(arg2), None) |
        Ins(CopyStoreShort, Local(arg1) /* = */, Reg(arg2), None) => {
    		x86_64_mov_membase_reg_size(inst, X86_64_RBP, arg1, arg2, 2);
    	}
        Ins(CopyLoadShort, Reg(arg1), None, None) |
        Ins(CopyLoadUShort, Reg(arg1), None, None) |
        Ins(CopyStoreShort, Reg(arg1), None, None) => {}
    
        Ins(CopyInt, Local(arg1) /* = */, Imm(arg2), None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RBP, arg1, arg2, 4);
    	}
        Ins(CopyInt, Reg(arg1), None, None) => {}
    
        Ins(CopyLong, Local(arg1) /* = */, Imm(arg2) /* imms32 */, None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RBP, arg1, arg2, 8);
    	}
        Ins(CopyLong, Reg(arg1), None, None) => {}
    
        Ins(CopyFloat32, Local(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_movss_membase_reg(inst, X86_64_RBP, arg1, arg2);
    	}
        Ins(CopyFloat32, Reg(arg1) /* xreg */, None, None) => {}
    
        Ins(CopyFloat64, Local(arg1) /* = */, Reg(arg2) /* xreg */, None) => {
    		x86_64_movsd_membase_reg(inst, X86_64_RBP, arg1, arg2);
    	}
        Ins(CopyFloat64, Reg(arg1) /* xreg */, None, None) => {}
    
        Ins(CopyNFloat, Reg(arg1) /* freg */, None, None) => {}
    
        Ins(CopyStruct, Frame /* = */, Frame, Reg(arg1) /* scratch */, Reg(arg2) /* scratch, xreg */,  if jit_type_get_size(jit_value_get_type(insn->dest)) <= _JIT_MAX_MEMCPY_INLINE, None) => {
    		inst = small_struct_copy(gen, inst, X86_64_RBP, arg1, X86_64_RBP, arg2,
    								 jit_value_get_type(insn->dest), arg3, arg4);
    	}
        Ins(CopyStruct, Frame /* = */, Frame,  /* clobber(reg), clobber(xre) */, None, None, None) => {
    		inst = memory_copy(gen, inst, X86_64_RBP, arg1, X86_64_RBP, arg2,
    				   jit_type_get_size(jit_value_get_type(insn->dest)));
    	}
    
        Ins(AddressOf, Reg(arg1) /* = */, Frame, None, None) => {
    		x86_64_lea_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
    
    /*
     * Stack pushes and pops.
     */
    
        Ins(IncomingReg, Reg(arg1), None, None) |
        Ins(ReturnReg, Reg(arg1), None, None) => {
    		/*
    		 * This rule does nothing itself. Also at this point
    		 * the value is supposed to be already in the register
    		 * so the "reg" pattern does not load it either. But
    		 * it allows the allocator to check the liveness flags
    		 * and free the register if the value is dead.
    		 */
    	}
    
        Ins(PushInt, Imm(arg1), None, None) => {
    		x86_64_push_imm(inst, arg1);
    		gen->stack_changed = 1;
    	}
        Ins(PushInt, Local(arg1), None, None) => {
    		x86_64_push_membase_size(inst, X86_64_RBP, arg1, 4);
    		gen->stack_changed = 1;
    	}
        Ins(PushInt, Reg(arg1), None, None) => {
    		x86_64_push_reg_size(inst, arg1, 4);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushLong, Imm(arg1), None, None) => {
    		if((arg1 >= (jit_nint)jit_min_int) && (arg1 <= (jit_nint)jit_max_int))
    		{
    			x86_64_push_imm(inst, arg1);
    		}
    		else
    		{
    			jit_int *ptr = (jit_int *)&(arg1);
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    			x86_64_mov_membase_imm_size(inst, X86_64_RSP, 4, ptr[1], 4);
    			x86_64_mov_membase_imm_size(inst, X86_64_RSP, 0, ptr[0], 4);
    		}
    		gen->stack_changed = 1;
    	}
        Ins(PushLong, Local(arg1), None, None) => {
    		x86_64_push_membase_size(inst, X86_64_RBP, arg1, 8);
    		gen->stack_changed = 1;
    	}
        Ins(PushLong, Reg(arg1), None, None) => {
    		x86_64_push_reg_size(inst, arg1, 8);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushFloat32, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_64_push_imm_size(inst, ptr[0], 4);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat32, Local(arg1), None, None) => {
    		x86_64_push_membase_size(inst, X86_64_RBP, arg1, 4);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat32, Reg(arg1) /* xreg */, None, None) => {
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_movss_membase_reg(inst, X86_64_RSP, 0, arg1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushFloat64, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, 4, ptr[1], 4);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, 0, ptr[0], 4);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat64, Local(arg1), None, None) => {
    		x86_64_push_membase_size(inst, X86_64_RBP, arg1, 8);
    		gen->stack_changed = 1;
    	}
        Ins(PushFloat64, Reg(arg1) /* xreg */, None, None) => {
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_movsd_membase_reg(inst, X86_64_RSP, 0, arg1);
    		gen->stack_changed = 1;
    	}
    
        Ins(PushNFloat, Imm(arg1), None, None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, 16, 8);
    			x86_64_mov_membase_imm_size(inst, X86_64_RSP, 8, ptr[2], 4);
    		}
    		else
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, size_of::<f64>(), 8);
    		}
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, 4, ptr[1], 4);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, 0, ptr[0], 4);
    		gen->stack_changed = 1;
    	}
        Ins(PushNFloat, Local(arg1), Reg(arg2) /* scratch */, None) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, 16, 8);
    			x86_64_mov_reg_membase_size(inst, arg2, X86_64_RBP, arg1 + 8, 4);
    			x86_64_mov_membase_reg_size(inst, X86_64_RSP, 8, arg2, 4);
    		}
    		else
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		}
    		x86_64_mov_reg_membase_size(inst, arg2, X86_64_RBP, arg1, 8);
    		x86_64_mov_membase_reg_size(inst, X86_64_RSP, 0, arg2, 8);
    		gen->stack_changed = 1;
    	}
        Ins(PushNFloat, Reg(arg1) /* freg */, None, None) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, 16, 8);
    			x86_64_fstp_membase_size(inst, X86_64_RSP, 0, 10);
    		}
    		else
    		{
    			x86_64_sub_reg_imm_size(inst, X86_64_RSP, size_of::<f64>(), 8);
    			x86_64_fstp_membase_size(inst, X86_64_RSP, 0, 8);
    		}
    		gen->stack_changed = 1;
    	}
    
        Ins(PushStruct, Reg(arg1),  if ((jit_nuint)jit_value_get_nint_constant(insn->value2)) <= 32, None, None) => {
    		jit_nuint size;
    		jit_nuint last_part;
    		size = (jit_nuint)jit_value_get_nint_constant(insn->value2);
    		last_part = size & 0x7;
    		if(last_part)
    		{
    			/* Handle the possible last part smaller than 8 bytes */
    			size -= last_part;
    
    			/* We don't care about the last not needed bytes */
    			x86_64_push_membase_size(inst, arg1, size, 8);
    		}
    		/* Handle full multiple pointer sized parts */
    		while(size > 0)
    		{
    			size -= size_of::<usize>();
    			x86_64_push_membase_size(inst, arg1, size, 8);
    		}
    		gen->stack_changed = 1;
    	}
        Ins(PushStruct, Reg(arg1),  /* clobber(reg), clobber(xre) */, None, None) => {
    		/* Handle arbitrary-sized structures */
    		jit_nuint size;
    		size = (jit_nuint)jit_value_get_nint_constant(insn->value2);
    		/* TODO: Maybe we should check for sizes > 2GB? */
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, ROUND_STACK(size), 8);
    		inst = memory_copy(gen, inst, X86_64_RSP, 0, arg1, 0, size);
    		gen->stack_changed = 1;
    	}
    
        Ins(PopStack, None, None, None) => {
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, insn->value1->address, 8);
    		gen->stack_changed = 1;
    	}
    
    /*
     * Parameter passing via parameter area
     */
    
        Ins(SetParamInt, Imm(arg1), Imm(arg2), None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, arg1, 4);
    	}
        Ins(SetParamInt, Reg(arg1), Imm(arg2), None) => {
    		x86_64_mov_membase_reg_size(inst, X86_64_RSP, arg2, arg1, 4);
    	}
    
        Ins(SetParamLong, Imm(arg1) /* imms32 */, Imm(arg2), None) => {
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, arg1, 8);
    	}
        Ins(SetParamLong, Imm(arg1), Imm(arg2), None) => {
    		jit_int *ptr = (jit_int *)&(arg1);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2 + 4, ptr[1], 4);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, ptr[0], 4);
    	}
        Ins(SetParamLong, Reg(arg1), Imm(arg2), None) => {
    		x86_64_mov_membase_reg_size(inst, X86_64_RSP, arg2, arg1, 8);
    	}
    
        Ins(SetParamFloat32, Imm(arg1), Imm(arg2), None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, ptr[0], 4);
    	}
        Ins(SetParamFloat32, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		x86_64_movss_membase_reg(inst, X86_64_RSP, arg2, arg1);
    	}
    
        Ins(SetParamFloat64, Imm(arg1), Imm(arg2), None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2 + 4, ptr[1], 4);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, ptr[0], 4);
    	}
        Ins(SetParamFloat64, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		x86_64_movsd_membase_reg(inst, X86_64_RSP, arg2, arg1);
    	}
    
        Ins(SetParamNFloat, Imm(arg1), Imm(arg2), None) => {
    		jit_int *ptr = (jit_int *)(arg1);
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2 + 8, ptr[2], 4);
    		}
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2 + 4, ptr[1], 4);
    		x86_64_mov_membase_imm_size(inst, X86_64_RSP, arg2, ptr[0], 4);
    	}
        Ins(SetParamNFloat, Reg(arg1) /* freg */, Imm(arg2), None) => {
    		if(sizeof(jit_nfloat) != size_of::<f64>())
    		{
    			x86_64_fstp_membase_size(inst, X86_64_RSP, arg2, 10);
    		}
    		else
    		{
    			x86_64_fstp_membase_size(inst, X86_64_RSP, arg2, 8);
    		}
    	}
    
        Ins(SetParamStruct, Reg(arg1), Imm(arg2),  /* clobber(reg), clobber(xre) */, None) => {
    		/* Handle arbitrary-sized structures */
    		jit_nint offset = jit_value_get_nint_constant(insn->dest);
    		/* TODO: Maybe we should check for sizes > 2GB? */
    		inst = memory_copy(gen, inst, X86_64_RSP, offset, arg1, 0, arg2);
    	}
    
    
    /*
     * Opcodes to handle return values
     */
    
        Ins(FlushSmallStruct, None, None, None) => {
    		inst = flush_return_struct(inst, insn->value1);
    	}
    
        Ins(Return, None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnInt, Reg(X86_64_RAX), None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnLong, Reg(X86_64_RAX), None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnFloat32, Reg(X86_64_XMM0) /* xreg */, None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnFloat64, Reg(X86_64_XMM0) /* xreg */, None, None, None) => {
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnNFloat, Reg(arg1) /* freg */,  /* clobber(re) */, None, None) => {
    		/* clobber(freg) frees all registers on the fp stack */
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
        Ins(ReturnSmallStruct, Reg(arg1) /* rreg */, Imm(arg2), None) => {
    		inst = return_struct(inst, func, arg1);
    		inst = jump_to_epilog(gen, inst, block);
    	}
    
    /*
     * Pointer-relative loads and stores.
     */
    
        Ins(LoadRelativeSByte, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movsx8_reg_regp_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_movsx8_reg_membase_size(inst, arg1, arg2, arg3, 8);
    		}
    	}
    
        Ins(LoadRelativeUByte, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movzx8_reg_regp_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_movzx8_reg_membase_size(inst, arg1, arg2, arg3, 8);
    		}
    	}
    
        Ins(LoadRelativeShort, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movsx16_reg_regp_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_movsx16_reg_membase_size(inst, arg1, arg2, arg3, 8);
    		}
    	}
    
        Ins(LoadRelativeUShort, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movzx16_reg_regp_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_movzx16_reg_membase_size(inst, arg1, arg2, arg3, 8);
    		}
    	}
    
        Ins(LoadRelativeInt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_reg_regp_size(inst, arg1, arg2, 4);
    		}
    		else
    		{
    			x86_64_mov_reg_membase_size(inst, arg1, arg2, arg3, 4);
    		}
    	}
    
        Ins(LoadRelativeLong, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_reg_regp_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_mov_reg_membase_size(inst, arg1, arg2, arg3, 8);
    		}
    	}
    
        Ins(LoadRelativeFloat32, Reg(arg1) /* =, xreg */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movss_reg_regp(inst, arg1, arg2);
    		}
    		else
    		{
    			x86_64_movss_reg_membase(inst, arg1, arg2, arg3);
    		}
    	}
    
        Ins(LoadRelativeFloat64, Reg(arg1) /* =, xreg */, Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movsd_reg_regp(inst, arg1, arg2);
    		}
    		else
    		{		
    			x86_64_movsd_reg_membase(inst, arg1, arg2, arg3);
    		}
    	}
    	
        Ins(LoadRelativeNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3),  if sizeof(jit_nfloat) != size_of::<f64>()) => {
    		x86_64_fld_membase_size(inst, arg2, arg3, 10);
    	}
        Ins(LoadRelativeNFloat, Reg(arg1) /* =, freg */, Reg(arg2), Imm(arg3),  if sizeof(jit_nfloat) == size_of::<f64>()) => {
    		x86_64_fld_membase_size(inst, arg2, arg3, 8);
    	}
    
        Ins(LoadRelativeStruct, Frame /* = */, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */, Reg(arg4) /* scratch, xreg */,  if jit_type_get_size(jit_value_get_type(insn->dest)) <= _JIT_MAX_MEMCPY_INLINE) => {
    		inst = small_struct_copy(gen, inst, X86_64_RBP, arg1, arg2, arg3,
    								 jit_value_get_type(insn->dest), arg4, arg5);
    	}
        Ins(LoadRelativeStruct, Frame /* = */, Reg(arg1), Imm(arg2),  /* clobber(reg), clobber(xre) */, None) => {
    		inst = memory_copy(gen, inst, X86_64_RBP, arg1, arg2, arg3,
    				   jit_type_get_size(jit_value_get_type(insn->dest)));
    	}
    
        Ins(StoreRelativeByte, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_imm_size(inst, arg1, arg2, 1);
    		}
    		else
    		{
    			x86_64_mov_membase_imm_size(inst, arg1, arg3, arg2, 1);
    		}
    	}
        Ins(StoreRelativeByte, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_reg_size(inst, arg1, arg2, 1);
    		}
    		else
    		{
    			x86_64_mov_membase_reg_size(inst, arg1, arg3, arg2, 1);
    		}
    	}
    
        Ins(StoreRelativeShort, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_imm_size(inst, arg1, arg2, 2);
    		}
    		else
    		{
    			x86_64_mov_membase_imm_size(inst, arg1, arg3, arg2, 2);
    		}
    	}
        Ins(StoreRelativeShort, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_reg_size(inst, arg1, arg2, 2);
    		}
    		else
    		{
    			x86_64_mov_membase_reg_size(inst, arg1, arg3, arg2, 2);
    		}
    	}
    
        Ins(StoreRelativeInt, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_imm_size(inst, arg1, arg2, 4);
    		}
    		else
    		{
    			x86_64_mov_membase_imm_size(inst, arg1, arg3, arg2, 4);
    		}
    	}
        Ins(StoreRelativeInt, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_reg_size(inst, arg1, arg2, 4);
    		}
    		else
    		{
    			x86_64_mov_membase_reg_size(inst, arg1, arg3, arg2, 4);
    		}
    	}
    
        Ins(StoreRelativeLong, Reg(arg1), Imm(arg2) /* imms32 */, Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_imm_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_mov_membase_imm_size(inst, arg1, arg3, arg2, 8);
    		}
    	}
        Ins(StoreRelativeLong, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_reg_size(inst, arg1, arg2, 8);
    		}
    		else
    		{
    			x86_64_mov_membase_reg_size(inst, arg1, arg3, arg2, 8);
    		}
    	}
    
        Ins(StoreRelativeFloat32, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_mov_regp_imm_size(inst, arg1, ((jit_int *)(arg2))[0], 4);
    		}
    		else
    		{
    			x86_64_mov_membase_imm_size(inst, arg1, arg3, ((jit_int *)(arg2))[0], 4);
    		}
    	}
        Ins(StoreRelativeFloat32, Reg(arg1), Reg(arg2) /* xreg */, Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movss_regp_reg(inst, arg1, arg2);
    		}
    		else
    		{	
    			x86_64_movss_membase_reg(inst, arg1, arg3, arg2);
    		}
    	}
    
        Ins(StoreRelativeFloat64, Reg(arg1), Imm(arg2), Imm(arg3)) => {
    		x86_64_mov_membase_imm_size(inst, arg1, arg3, ((int *)(arg2))[0], 4);
    		x86_64_mov_membase_imm_size(inst, arg1, arg3 + 4, ((int *)(arg2))[1], 4);
    	}
        Ins(StoreRelativeFloat64, Reg(arg1), Reg(arg2) /* xreg */, Imm(arg3)) => {
    		if(arg3 == 0)
    		{
    			x86_64_movsd_regp_reg(inst, arg1, arg2);
    		}
    		else
    		{	
    			x86_64_movsd_membase_reg(inst, arg1, arg3, arg2);
    		}
    	}
    
        Ins(StoreRelativeStruct, Reg(arg1), Frame, Imm(arg2), Reg(arg3) /* scratch */, Reg(arg4) /* scratch, xreg */,  if jit_type_get_size(jit_value_get_type(insn->value1)) <= _JIT_MAX_MEMCPY_INLINE) => {
    		inst = small_struct_copy(gen, inst, arg1, arg3, X86_64_RBP, arg2,
    								 jit_value_get_type(insn->value1), arg4, arg5);
    	}
        Ins(StoreRelativeStruct, Reg(arg1), Frame, Imm(arg2),  /* clobber(reg), clobber(xre) */, None) => {
    		inst = memory_copy(gen, inst, arg1, arg3, X86_64_RBP, arg2,
    				   jit_type_get_size(jit_value_get_type(insn->value1)));
    	}
    
        Ins(AddRelative, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		if(arg2 != 0)
    		{
    			x86_64_add_reg_imm_size(inst, arg1, arg2, 8);
    		}
    	}
    
    /*
     * Array element loads and stores.
     */
    
        Ins(LoadElementSByte, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movsx8_reg_memindex_size(inst, arg1, arg2, 0, arg3, 0, 4);
    	}
    
        Ins(LoadElementUByte, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movzx8_reg_memindex_size(inst, arg1, arg2, 0, arg3, 0, 4);
    	}
    
        Ins(LoadElementShort, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movsx16_reg_memindex_size(inst, arg1, arg2, 0, arg3, 1, 4);
    	}
    
        Ins(LoadElementUShort, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movzx16_reg_memindex_size(inst, arg1, arg2, 0, arg3, 1, 4);
    	}
    
        Ins(LoadElementInt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_reg_memindex_size(inst, arg1, arg2, 0, arg3, 2, 4);
    	}
    
        Ins(LoadElementLong, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_reg_memindex_size(inst, arg1, arg2, 0, arg3, 3, 8);
    	}
    
        Ins(LoadElementFloat32, Reg(arg1) /* =, xreg */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movss_reg_memindex(inst, arg1, arg2, 0, arg3, 2);
    	}
    
        Ins(LoadElementFloat64, Reg(arg1) /* =, xreg */, Reg(arg2), Reg(arg3)) => {
    		x86_64_movsd_reg_memindex(inst, arg1, arg2, 0, arg3, 3);
    	}
    
        Ins(StoreElementByte, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_memindex_reg_size(inst, arg1, 0, arg2, 0, arg3, 1);
    	}
    
        Ins(StoreElementShort, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_memindex_reg_size(inst, arg1, 0, arg2, 1, arg3, 2);
    	}
    
        Ins(StoreElementInt, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_memindex_reg_size(inst, arg1, 0, arg2, 2, arg3, 4);
    	}
    
        Ins(StoreElementLong, Reg(arg1), Reg(arg2), Imm(arg3)) => {
    		if(arg3 >= (jit_nint)jit_min_int && arg3 <= (jit_nint)jit_max_int)
    		{
    			x86_64_mov_memindex_imm_size(inst, arg1, 0, arg2, 3, arg3, 8);
    		}
    		else
    		{
    			jit_int *long_ptr = (jit_int *)(&(arg3));
    
    			x86_64_mov_memindex_imm_size(inst, arg1, 0, arg2, 3, long_ptr[0], 4);
    			x86_64_mov_memindex_imm_size(inst, arg1, 4, arg2, 3, long_ptr[1], 4);
    		}
    	}
        Ins(StoreElementLong, Reg(arg1), Reg(arg2), Reg(arg3)) => {
    		x86_64_mov_memindex_reg_size(inst, arg1, 0, arg2, 3, arg3, 8);
    	}
    
        Ins(StoreElementFloat32, Reg(arg1), Reg(arg2), Reg(arg3) /* xreg */) => {
    		x86_64_movss_memindex_reg(inst, arg1, 0, arg2, 2, arg3);
    	}
    
        Ins(StoreElementFloat64, Reg(arg1), Reg(arg2), Reg(arg3) /* xreg */) => {
    		x86_64_movsd_memindex_reg(inst, arg1, 0, arg2, 3, arg3);
    	}
    
    /*
     * Arithmetic opcodes.
     */
    
    /*
     * 4 byte integer versions
     */
    
        Ins(IAdd, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 1)
    		{
    			x86_64_inc_reg_size(inst, arg1, 4);
    		}
    		else
    		{
    			x86_64_add_reg_imm_size(inst, arg1, arg2, 4);
    		}
    	}
        Ins(IAdd, Reg(arg1), Local(arg2), None) => {
    		x86_64_add_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IAdd, Reg(arg1), Reg(arg2), None) => {
    		x86_64_add_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(ISub, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 1)
    		{
    			x86_64_dec_reg_size(inst, arg1, 4);
    		}
    		else
    		{
    			x86_64_sub_reg_imm_size(inst, arg1, arg2, 4);
    		}
    	}
        Ins(ISub, Reg(arg1), Local(arg2), None) => {
    		x86_64_sub_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(ISub, Reg(arg1), Reg(arg2), None) => {
    		x86_64_sub_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(INeg, Reg(arg1), None, None) => {
    		x86_64_neg_reg_size(inst, arg1, 4);
    	}
    
        Ins(IMul, Reg(arg1), Imm(0), None, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(IMul, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		x86_64_neg_reg_size(inst, arg1, 4);
    	}
        Ins(IMul, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(IMul, Reg(arg1), Imm(arg2),  if arg2 == 2, None) => {
    		x86_64_add_reg_reg_size(inst, arg1, arg1, 4);
    	}
        Ins(IMul, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		x86_64_shl_reg_imm_size(inst, arg1, shift, 4);
    	}
        Ins(IMul, Reg(arg1), Imm(arg2), None) => {
    		x86_64_imul_reg_reg_imm_size(inst, arg1, arg1, arg2, 4);
    	}
        Ins(IMul, Reg(arg1), Local(arg2), None) => {
    		x86_64_imul_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IMul, Reg(arg1), Reg(arg2), None) => {
    		x86_64_imul_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(IDiv, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply negates for other values */
    		jit_int min_int = jit_min_int;
    		let mut patch;
    		x86_64_cmp_reg_imm_size(inst, arg1, min_int, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_64_neg_reg_size(inst, arg1, 4);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if arg2 == 2) => {
    		/* move the value to be divided to the temporary */
    		x86_64_mov_reg_reg_size(inst, arg3, arg1, 4);
    		/* shift the temporary to the 31 bits right */
    		/* The result is 1 for negative values and 0 for zero or */
    		/* positive values. (corrective value for negatives) */
    		x86_64_shr_reg_imm_size(inst, arg3, 0x1f, 4);
    		/* Add the corrective value to the divident */
    		x86_64_add_reg_reg_size(inst, arg1, arg3, 4);
    		/* and do the right shift */
    		x86_64_sar_reg_imm_size(inst, arg1, 1, 4);
    	}
        Ins(IDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if (arg2 > 0) && (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, corr, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		corr = arg2 - 1;
    		x86_64_lea_membase_size(inst, arg3, arg1, corr, 4);
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_S, arg1, arg3, 1, 4);
    		x86_64_sar_reg_imm_size(inst, arg1, shift, 4);
    	}
        Ins(IDiv, Reg(X86_64_RAX), Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg3, arg2, 4);
    		x86_64_cdq(inst);
    		x86_64_idiv_reg_size(inst, arg3, 4);
    	}
        Ins(IDiv, Reg(X86_64_RAX), Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    		jit_int min_int = jit_min_int;
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_cmp_reg_imm_size(inst, arg2, -1, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_64_cmp_reg_imm_size(inst, arg1, min_int, 4);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_64_cdq(inst);
    		x86_64_idiv_reg_size(inst, arg2, 4);
    	}
    
        Ins(IDivUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IDivUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(IDivUn, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		x86_64_shr_reg_imm_size(inst, arg1, shift, 4);
    	}
        Ins(IDivUn, Reg(X86_64_RAX), Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg3, arg2, 4);
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg3, 4);
    	}
        Ins(IDivUn, Reg(X86_64_RAX), Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg2, 4);
    	}
    
        Ins(IRem, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IRem, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(IRem, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply gives a remainder of zero */
    		jit_int min_int = jit_min_int;
    		let mut patch;
    		x86_64_cmp_reg_imm_size(inst, arg1, min_int, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(IRem, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg4, arg3, 4);
    		x86_64_cdq(inst);
    		x86_64_idiv_reg_size(inst, arg4, 4);
    	}
        Ins(IRem, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    		jit_int min_int = jit_min_int;
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_64_test_reg_reg_size(inst, arg3, arg3, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_cmp_reg_imm_size(inst, arg3, -1, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_64_cmp_reg_imm_size(inst, arg2, min_int, 4);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_64_cdq(inst);
    		x86_64_idiv_reg_size(inst, arg3, 4);
    	}
    
        Ins(IRemUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(IRemUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(IRemUn, Reg(arg1), Imm(arg2),  if (arg2 & (arg2 - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		x86_64_and_reg_imm_size(inst, arg1, arg2 - 1, 4);
    	}
        Ins(IRemUn, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg4, arg3, 4);
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg4, 4);
    	}
        Ins(IRemUn, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_64_test_reg_reg_size(inst, arg3, arg3, 4);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg3, 4);
    	}
    
    /*
     * 8 byte integer versions
     */
    
        Ins(LAdd, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		if(arg2 == 1)
    		{
    			x86_64_inc_reg_size(inst, arg1, 8);
    		}
    		else
    		{
    			x86_64_add_reg_imm_size(inst, arg1, arg2, 8);
    		}
    	}
        Ins(LAdd, Reg(arg1), Local(arg2), None) => {
    		x86_64_add_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LAdd, Reg(arg1), Reg(arg2), None) => {
    		x86_64_add_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LSub, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		if(arg2 == 1)
    		{
    			x86_64_dec_reg_size(inst, arg1, 8);
    		}
    		else
    		{
    			x86_64_sub_reg_imm_size(inst, arg1, arg2, 8);
    		}
    	}
        Ins(LSub, Reg(arg1), Local(arg2), None) => {
    		x86_64_sub_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LSub, Reg(arg1), Reg(arg2), None) => {
    		x86_64_sub_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LNeg, Reg(arg1), None, None) => {
    		x86_64_neg_reg_size(inst, arg1, 8);
    	}
    
        Ins(LMul, Reg(arg1), Imm(0), None, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(LMul, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		x86_64_neg_reg_size(inst, arg1, 8);
    	}
        Ins(LMul, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(LMul, Reg(arg1), Imm(arg2),  if arg2 == 2, None) => {
    		x86_64_add_reg_reg_size(inst, arg1, arg1, 8);
    	}
        Ins(LMul, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		x86_64_shl_reg_imm_size(inst, arg1, shift, 8);
    	}
        Ins(LMul, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_imul_reg_reg_imm_size(inst, arg1, arg1, arg2, 8);
    	}
        Ins(LMul, Reg(arg1), Local(arg2), None) => {
    		x86_64_imul_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LMul, Reg(arg1), Reg(arg2), None) => {
    		x86_64_imul_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LDiv, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(LDiv, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(LDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if arg2 == -1) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply negates for other values */
    		jit_long min_long = jit_min_long;
    		let mut patch;
    		x86_64_mov_reg_imm_size(inst, arg3, min_long, 8);
    		x86_64_cmp_reg_reg_size(inst, arg1, arg3, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_64_neg_reg_size(inst, arg1, 8);
    	}
        Ins(LDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if arg2 == 2) => {
    		/* move the value to be divided to the temporary */
    		x86_64_mov_reg_reg_size(inst, arg3, arg1, 8);
    		/* shift the temporary to the 63 bits right */
    		/* The result is 1 for negative values and 0 for zero or */
    		/* positive values. (corrective value for negatives) */
    		x86_64_shr_reg_imm_size(inst, arg3, 0x3f, 8);
    		/* Add the corrective value to the divident */
    		x86_64_add_reg_reg_size(inst, arg1, arg3, 8);
    		/* and do the right shift */
    		x86_64_sar_reg_imm_size(inst, arg1, 1, 8);
    	}
        Ins(LDiv, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if (arg2 > 0) && (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		if((jit_nuint)arg2 <= (jit_nuint)jit_max_uint)
    		{
    			jit_nuint corr = (arg2 - 1);
    
    			x86_64_lea_membase_size(inst, arg3, arg1, corr, 8);
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		}
    		else
    		{
    			jit_nuint corr = (arg2 - 1);
    
    			if(corr <= (jit_nuint)jit_max_uint)
    			{
    				x86_64_mov_reg_imm_size(inst, arg3, corr, 4);
    			}
    			else
    			{
    				x86_64_mov_reg_imm_size(inst, arg3, corr, 8);
    			}
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    			x86_64_lea_memindex_size(inst, arg3, arg1, 0, arg3, 0, 8);
    		}
    		x86_64_cmov_reg_reg_size(inst, X86_CC_S, arg1, arg3, 1, 8);
    		x86_64_sar_reg_imm_size(inst, arg1, shift, 8);
    	}
        Ins(LDiv, Reg(X86_64_RAX), Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg3, arg2, 8);
    		x86_64_cqo(inst);
    		x86_64_idiv_reg_size(inst, arg3, 8);
    	}
        Ins(LDiv, Reg(X86_64_RAX), Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    		jit_long min_long = jit_min_long;
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_64_or_reg_reg_size(inst, arg2, arg2, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_cmp_reg_imm_size(inst, arg2, -1, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_64_mov_reg_imm_size(inst, arg3, min_long, 8);
    		x86_64_cmp_reg_reg_size(inst, arg1, arg3, 8);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_64_cqo(inst);
    		x86_64_idiv_reg_size(inst, arg2, 8);
    	}
    
        Ins(LDivUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(LDivUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    	}
        Ins(LDivUn, Reg(arg1), Imm(arg2),  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0, None) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		jit_nuint shift, value = arg2 >> 1;
    		for(shift = 0; value; value >>= 1)
    		{
    		    ++shift;
    		}
    		x86_64_shr_reg_imm_size(inst, arg1, shift, 8);
    	}
        Ins(LDivUn, Reg(X86_64_RAX), Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg3, arg2, 8);
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg3, 8);
    	}
        Ins(LDivUn, Reg(X86_64_RAX), Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg2, 8);
    	}
    
        Ins(LRem, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(LRem, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(LRem, Reg(arg1), Imm(arg2),  if arg2 == -1, None) => {
    		/* Dividing by -1 gives an exception if the argument
    		   is minint, or simply gives a remainder of zero */
    		jit_long min_long = jit_min_long;
    		let mut patch;
    		x86_64_cmp_reg_imm_size(inst, arg1, min_long, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(LRem, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg4, arg3, 8);
    		x86_64_cqo(inst);
    		x86_64_idiv_reg_size(inst, arg4, 8);
    	}
        Ins(LRem, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    		jit_long min_long = jit_min_long;
    		let mut patch, *patch2;
    #ifndef JIT_USE_SIGNALS
    		x86_64_test_reg_reg_size(inst, arg3, arg3, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_mov_reg_imm_size(inst, arg1, min_long, 8);
    		x86_64_cmp_reg_imm_size(inst, arg3, -1, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		x86_64_cmp_reg_reg_size(inst, arg2, arg1, 8);
    		patch2 = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_ARITHMETIC);
    		x86_patch(patch, inst);
    		x86_patch(patch2, inst);
    		x86_64_cqo(inst);
    		x86_64_idiv_reg_size(inst, arg3, 8);
    	}
    
        Ins(LRemUn, _, Imm(0), None, None, None) => {
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    	}
        Ins(LRemUn, Reg(arg1), Imm(arg2),  if arg2 == 1, None) => {
    		x86_64_clear_reg(inst, arg1);
    	}
        Ins(LRemUn, Reg(arg1), Imm(arg2), Reg(arg3) /* scratch */,  if (((jit_nuint)arg2) & (((jit_nuint)arg2) - 1)) == 0) => {
    		/* x & (x - 1) is equal to zero if x is a power of 2  */
    		if((arg2 >= jit_min_int) && (arg2 <= jit_max_int))
    		{
    			x86_64_and_reg_imm_size(inst, arg1, arg2 - 1, 8);
    		}
    		else
    		{
    			jit_long temp = arg2 - 1;
    
    			x86_64_mov_reg_imm_size(inst, arg3, temp, 8);
    			x86_64_and_reg_reg_size(inst, arg1, arg3, 8);
    		}
    	}
        Ins(LRemUn, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Imm(arg1), Reg(arg2) /* scratch, dreg */, Reg(X86_64_RDX) /* scratch */, None) => {
    		x86_64_mov_reg_imm_size(inst, arg4, arg3, 8);
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg4, 8);
    	}
        Ins(LRemUn, Reg(X86_64_RDX) /* = */, Reg(X86_64_RAX) /* * */, Reg(arg1) /* dreg */, Reg(X86_64_RDX) /* scratch */, None, None) => {
    #ifndef JIT_USE_SIGNALS
    		let mut patch;
    		x86_64_test_reg_reg_size(inst, arg3, arg3, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_DIVISION_BY_ZERO);
    		x86_patch(patch, inst);
    #endif
    		x86_64_clear_reg(inst, X86_64_RDX);
    		x86_64_div_reg_size(inst, arg3, 8);
    	}
    
    /*
     * single precision float versions
     */
    
        Ins(FAdd, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float32(gen, &inst, XMM1_ADD, arg1, (jit_float32 *)arg2);
    	}
        Ins(FAdd, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_addss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(FAdd, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_addss_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(FSub, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float32(gen, &inst, XMM1_SUB, arg1, (jit_float32 *)arg2);
    	}
        Ins(FSub, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_subss_reg_reg(inst, arg1, arg2);
    	}
        Ins(FSub, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_subss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
    
        Ins(FMul, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float32(gen, &inst, XMM1_MUL, arg1, (jit_float32 *)arg2);
    	}
        Ins(FMul, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_mulss_reg_reg(inst, arg1, arg2);
    	}
        Ins(FMul, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_mulss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
    
        Ins(FDiv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float32(gen, &inst, XMM1_DIV, arg1, (jit_float32 *)arg2);
    	}
        Ins(FDiv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_divss_reg_reg(inst, arg1, arg2);
    	}
        Ins(FDiv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_divss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
    
        Ins(FAbs, Reg(arg1) /* xreg */, None, None) => {
    		/* Simply clear the sign */
    		jit_uint values[4] = {0x7fffffff, 0x7fffffff, 0x7fffffff, 0x7fffffff};
    
    		_jit_plops_reg_imm(gen, &inst, XMM_ANDP, arg1, &(values[0]));
    	}
    
        Ins(FNeg, Reg(arg1) /* xreg */, None, None) => {
    		/* Simply toggle the sign */
    		jit_uint values[4] = {0x80000000, 0x80000000, 0x80000000, 0x80000000};
    
    		_jit_plops_reg_imm(gen, &inst, XMM_XORP, arg1, &(values[0]));
    	}
    
    /*
     * double precision float versions
     */
    
        Ins(DAdd, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float64(gen, &inst, XMM1_ADD, arg1, (jit_float64 *)arg2);
    	}
        Ins(DAdd, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_addsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DAdd, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_addsd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DSub, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float64(gen, &inst, XMM1_SUB, arg1, (jit_float64 *)arg2);
    	}
        Ins(DSub, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_subsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DSub, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_subsd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DMul, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float64(gen, &inst, XMM1_MUL, arg1, (jit_float64 *)arg2);
    	}
        Ins(DMul, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_mulsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DMul, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_mulsd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DDiv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		_jit_xmm1_reg_imm_size_float64(gen, &inst, XMM1_DIV, arg1, (jit_float64 *)arg2);
    	}
        Ins(DDiv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_divsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DDiv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_divsd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DAbs, Reg(arg1) /* xreg */, None, None) => {
    		/* Simply clear the sign */
    		jit_ulong values[2] = {0x7fffffffffffffff, 0x7fffffffffffffff};
    
    		_jit_plopd_reg_imm(gen, &inst, XMM_ANDP, arg1, &(values[0]));
    	}
    
        Ins(DNeg, Reg(arg1) /* xreg */, None, None) => {
    		/* Simply toggle the sign */
    		jit_ulong values[2] = {0x8000000000000000, 0x8000000000000000};
    
    		_jit_plopd_reg_imm(gen, &inst, XMM_XORP, arg1, &(values[0]));
    	}
    
    /*
     * native float versions
     */
        Ins(NFAbs, Reg(arg1) /* freg */, None, None) => {
    		x86_64_fabs(inst);
    	}
    
        Ins(NFNeg, Reg(arg1) /* freg */, None, None) => {
    		x86_64_fchs(inst);
    	}
    
    /*
     * Bitwise opcodes.
     */
    
        Ins(IAnd, Reg(arg1), Imm(arg2), None) => {
    		x86_64_and_reg_imm_size(inst, arg1, arg2, 4);
    	}
        Ins(IAnd, Reg(arg1), Local(arg2), None) => {
    		x86_64_and_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IAnd, Reg(arg1), Reg(arg2), None) => {
    		x86_64_and_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(IOr, Reg(arg1), Imm(arg2), None) => {
    		x86_64_or_reg_imm_size(inst, arg1, arg2, 4);
    	}
        Ins(IOr, Reg(arg1), Local(arg2), None) => {
    		x86_64_or_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IOr, Reg(arg1), Reg(arg2), None) => {
    		x86_64_or_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(IXOr, Reg(arg1), Imm(arg2), None) => {
    		x86_64_xor_reg_imm_size(inst, arg1, arg2, 4);
    	}
        Ins(IXOr, Reg(arg1), Local(arg2), None) => {
    		x86_64_xor_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    	}
        Ins(IXOr, Reg(arg1), Reg(arg2), None) => {
    		x86_64_xor_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(INot, Reg(arg1), None, None) => {
    		x86_64_not_reg_size(inst, arg1, 4);
    	}
    
        Ins(IShl, Reg(arg1), Imm(arg2), None) => {
    		x86_64_shl_reg_imm_size(inst, arg1, (arg2 & 0x1F), 4);
    	}
        Ins(IShl, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_shl_reg_size(inst, arg1, 4);
    	}
    
        Ins(IShr, Reg(arg1), Imm(arg2), None) => {
    		x86_64_sar_reg_imm_size(inst, arg1, (arg2 & 0x1F), 4);
    	}
        Ins(IShr, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_sar_reg_size(inst, arg1, 4);
    	}
    
        Ins(IShrUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_shr_reg_imm_size(inst, arg1, (arg2 & 0x1F), 4);
    	}
        Ins(IShrUn, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_shr_reg_size(inst, arg1, 4);
    	}
    
        Ins(LAnd, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_and_reg_imm_size(inst, arg1, arg2, 8);
    	}
        Ins(LAnd, Reg(arg1), Local(arg2), None) => {
    		x86_64_and_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LAnd, Reg(arg1), Reg(arg2), None) => {
    		x86_64_and_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LOr, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_or_reg_imm_size(inst, arg1, arg2, 8);
    	}
        Ins(LOr, Reg(arg1), Local(arg2), None) => {
    		x86_64_or_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LOr, Reg(arg1), Reg(arg2), None) => {
    		x86_64_or_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LXOr, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_xor_reg_imm_size(inst, arg1, arg2, 8);
    	}
        Ins(LXOr, Reg(arg1), Local(arg2), None) => {
    		x86_64_xor_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    	}
        Ins(LXOr, Reg(arg1), Reg(arg2), None) => {
    		x86_64_xor_reg_reg_size(inst, arg1, arg2, 8);
    	}
    
        Ins(LNot, Reg(arg1), None, None) => {
    		x86_64_not_reg_size(inst, arg1, 8);
    	}
    
        Ins(LShl, Reg(arg1), Imm(arg2), None) => {
    		x86_64_shl_reg_imm_size(inst, arg1, (arg2 & 0x3F), 8);
    	}
        Ins(LShl, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_shl_reg_size(inst, arg1, 8);
    	}
    
        Ins(LShr, Reg(arg1), Imm(arg2), None) => {
    		x86_64_sar_reg_imm_size(inst, arg1, (arg2 & 0x3F), 8);
    	}
        Ins(LShr, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_sar_reg_size(inst, arg1, 8);
    	}
    
        Ins(LShrUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_shr_reg_imm_size(inst, arg1, (arg2 & 0x3F), 8);
    	}
        Ins(LShrUn, Reg(arg1) /* sreg */, Reg(X86_64_RCX), None, None) => {
    		x86_64_shr_reg_size(inst, arg1, 8);
    	}
    
    /*
     * Branch opcodes.
     */
    
        Ins(Br, None, None, None) => {
    		inst = output_branch(func, inst, 0xEB /* jmp */, insn);
    	}
    
        Ins(BrIFalse, Reg(arg1), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrITrue, Reg(arg1), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrIEq, Reg(arg1), Imm(0), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrIEq, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrINe, Reg(arg1), Imm(0), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrINe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrILt, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 0)
    		{
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		}
    		else
    		{
    			x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		}
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrILt, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrILt, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
    
        Ins(BrILtUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrILtUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrILtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
    
        Ins(BrILe, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 0)
    		{
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		}
    		else
    		{
    			x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		}
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrILe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrILe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
    
        Ins(BrILeUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrILeUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrILeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
    
        Ins(BrIGt, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 0)
    		{
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		}
    		else
    		{
    			x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		}
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrIGt, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrIGt, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
    
        Ins(BrIGtUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrIGtUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrIGtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
    
        Ins(BrIGe, Reg(arg1), Imm(arg2), None) => {
    		if(arg2 == 0)
    		{
    			x86_64_test_reg_reg_size(inst, arg1, arg1, 4);
    		}
    		else
    		{
    			x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		}
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrIGe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrIGe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
    
        Ins(BrIGeUn, Reg(arg1), Imm(arg2), None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrIGeUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 4);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrIGeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
    
        Ins(BrLFalse, Reg(arg1), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrLTrue, Reg(arg1), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrLEq, Reg(arg1), Imm(0), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrLEq, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrLEq, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
        Ins(BrLEq, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x74 /* eq */, insn);
    	}
    
        Ins(BrLNe, Reg(arg1), Imm(0), None, None) => {
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrLNe, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrLNe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
        Ins(BrLNe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x75 /* ne */, insn);
    	}
    
        Ins(BrLLt, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrLLt, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
        Ins(BrLLt, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7C /* lt */, insn);
    	}
    
        Ins(BrLLtUn, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrLLtUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
        Ins(BrLLtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x72 /* lt_un */, insn);
    	}
    
        Ins(BrLLe, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrLLe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
        Ins(BrLLe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7E /* le */, insn);
    	}
    
        Ins(BrLLeUn, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrLLeUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
        Ins(BrLLeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x76 /* le_un */, insn);
    	}
    
        Ins(BrLGt, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrLGt, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
        Ins(BrLGt, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7F /* gt */, insn);
    	}
    
        Ins(BrLGtUn, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrLGtUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
        Ins(BrLGtUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x77 /* gt_un */, insn);
    	}
    
        Ins(BrLGe, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrLGe, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
        Ins(BrLGe, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x7D /* ge */, insn);
    	}
    
        Ins(BrLGeUn, Reg(arg1), Imm(arg2) /* imms32 */, None) => {
    		x86_64_cmp_reg_imm_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrLGeUn, Reg(arg1), Local(arg2), None) => {
    		x86_64_cmp_reg_membase_size(inst, arg1, X86_64_RBP, arg2, 8);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
        Ins(BrLGeUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		inst = output_branch(func, inst, 0x73 /* ge_un */, insn);
    	}
    
        Ins(BrFEq, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_Z, arg1, (void *)arg2, 0, 0, insn);
    	}
        Ins(BrFEq, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_Z, arg1, X86_64_RBP, arg2, 0, 0, insn);
    	}
        Ins(BrFEq, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_Z, arg1, arg2, 0, 0, insn);
    	}
    
        Ins(BrFNe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NZ, arg1, (void *)arg2, 0, 1, insn);
    	}
        Ins(BrFNe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NZ, arg1, X86_64_RBP, arg2, 0, 1, insn);
    	}
        Ins(BrFNe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */,  /* space(20) */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NZ, arg1, arg2, 0, 1, insn);
    	}
    
        Ins(BrFLt, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_C, arg1, (void *)arg2, 0, 0, insn);
    	}
        Ins(BrFLt, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_C, arg1, X86_64_RBP, arg2, 0, 0, insn);
    	}
        Ins(BrFLt, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_C, arg1, arg2, 0, 0, insn);
    	}
    
        Ins(BrFLtInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_C, arg1, (void *)arg2, 0, 1, insn);
    	}
        Ins(BrFLtInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_C, arg1, X86_64_RBP, arg2, 0, 1, insn);
    	}
        Ins(BrFLtInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_C, arg1, arg2, 0, 1, insn);
    	}
    
        Ins(BrFLe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_BE, arg1, (void *)arg2, 0, 0, insn);
    	}
        Ins(BrFLe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_BE, arg1, X86_64_RBP, arg2, 0, 0, insn);
    	}
        Ins(BrFLe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_BE, arg1, arg2, 0, 0, insn);
    	}
    
        Ins(BrFLeInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_BE, arg1, (void *)arg2, 0, 1, insn);
    	}
        Ins(BrFLeInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_BE, arg1, X86_64_RBP, arg2, 0, 1, insn);
    	}
        Ins(BrFLeInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_BE, arg1, arg2, 0, 1, insn);
    	}
    
        Ins(BrFGt, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NBE, arg1, (void *)arg2, 0, 0, insn);
    	}
        Ins(BrFGt, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NBE, arg1, X86_64_RBP, arg2, 0, 0, insn);
    	}
        Ins(BrFGt, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NBE, arg1, arg2, 0, 0, insn);
    	}
    
        Ins(BrFGtInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NBE, arg1, (void *)arg2, 0, 1, insn);
    	}
        Ins(BrFGtInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NBE, arg1, X86_64_RBP, arg2, 0, 1, insn);
    	}
        Ins(BrFGtInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NBE, arg1, arg2, 0, 1, insn);
    	}
    
        Ins(BrFGe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NC, arg1, (void *)arg2, 0, 0, insn);
    	}
        Ins(BrFGe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NC, arg1, X86_64_RBP, arg2, 0, 0, insn);
    	}
        Ins(BrFGe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NC, arg1, arg2, 0, 0, insn);
    	}
    
        Ins(BrFGeInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NC, arg1, (void *)arg2, 0, 1, insn);
    	}
        Ins(BrFGeInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NC, arg1, X86_64_RBP, arg2, 0, 1, insn);
    	}
        Ins(BrFGeInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NC, arg1, arg2, 0, 1, insn);
    	}
    
        Ins(BrDEq, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_Z, arg1, (void *)arg2, 1, 0, insn);
    	}
        Ins(BrDEq, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_Z, arg1, X86_64_RBP, arg2, 1, 0, insn);
    	}
        Ins(BrDEq, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_Z, arg1, arg2, 1, 0, insn);
    	}
    
        Ins(BrDNe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NZ, arg1, (void *)arg2, 1, 1, insn);
    	}
        Ins(BrDNe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NZ, arg1, X86_64_RBP, arg2, 1, 1, insn);
    	}
        Ins(BrDNe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */,  /* space(20) */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NZ, arg1, arg2, 1, 1, insn);
    	}
    
        Ins(BrDLt, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_C, arg1, (void *)arg2, 1, 0, insn);
    	}
        Ins(BrDLt, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_C, arg1, X86_64_RBP, arg2, 1, 0, insn);
    	}
        Ins(BrDLt, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_C, arg1, arg2, 1, 0, insn);
    	}
    
        Ins(BrDLtInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_C, arg1, (void *)arg2, 1, 1, insn);
    	}
        Ins(BrDLtInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_C, arg1, X86_64_RBP, arg2, 1, 1, insn);
    	}
        Ins(BrDLtInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_C, arg1, arg2, 1, 1, insn);
    	}
    
        Ins(BrDLe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_BE, arg1, (void *)arg2, 1, 0, insn);
    	}
        Ins(BrDLe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_BE, arg1, X86_64_RBP, arg2, 1, 0, insn);
    	}
        Ins(BrDLe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_BE, arg1, arg2, 1, 0, insn);
    	}
    
        Ins(BrDLeInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_BE, arg1, (void *)arg2, 1, 1, insn);
    	}
        Ins(BrDLeInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_BE, arg1, X86_64_RBP, arg2, 1, 1, insn);
    	}
        Ins(BrDLeInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_BE, arg1, arg2, 1, 1, insn);
    	}
    
        Ins(BrDGt, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NBE, arg1, (void *)arg2, 1, 0, insn);
    	}
        Ins(BrDGt, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NBE, arg1, X86_64_RBP, arg2, 1, 0, insn);
    	}
        Ins(BrDGt, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NBE, arg1, arg2, 1, 0, insn);
    	}
    
        Ins(BrDGtInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NBE, arg1, (void *)arg2, 1, 1, insn);
    	}
        Ins(BrDGtInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NBE, arg1, X86_64_RBP, arg2, 1, 1, insn);
    	}
        Ins(BrDGtInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NBE, arg1, arg2, 1, 1, insn);
    	}
    
        Ins(BrDGe, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NC, arg1, (void *)arg2, 1, 0, insn);
    	}
        Ins(BrDGe, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NC, arg1, X86_64_RBP, arg2, 1, 0, insn);
    	}
        Ins(BrDGe, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NC, arg1, arg2, 1, 0, insn);
    	}
    
        Ins(BrDGeInv, Reg(arg1) /* xreg */, Imm(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_imm(gen, func, inst, X86_CC_NC, arg1, (void *)arg2, 1, 1, insn);
    	}
        Ins(BrDGeInv, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		inst = xmm_cmp_brcc_reg_membase(func, inst, X86_CC_NC, arg1, X86_64_RBP, arg2, 1, 1, insn);
    	}
        Ins(BrDGeInv, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		inst = xmm_cmp_brcc_reg_reg(func, inst, X86_CC_NC, arg1, arg2, 1, 1, insn);
    	}
    
    /*
     * Comparison opcodes.
     */
    
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(IEq, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
    
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(INe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
    
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(ILt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
    
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(ILtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
    
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(ILe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
    
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(ILeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
    
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(IGt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
    
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(IGtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
    
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(IGe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
    
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3)) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(IGeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 4);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
    
        Ins(LEq, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(LEq, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(LEq, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
        Ins(LEq, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_EQ, 0);
    	}
    
        Ins(LNe, Reg(arg1) /* = */, Reg(arg2), Imm(0), None) => {
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(LNe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(LNe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
        Ins(LNe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_NE, 0);
    	}
    
        Ins(LLt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(LLt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
        Ins(LLt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 1);
    	}
    
        Ins(LLtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(LLtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
        Ins(LLtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LT, 0);
    	}
    
        Ins(LLe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(LLe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
        Ins(LLe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 1);
    	}
    
        Ins(LLeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(LLeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
        Ins(LLeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_LE, 0);
    	}
    
        Ins(LGt, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(LGt, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
        Ins(LGt, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 1);
    	}
    
        Ins(LGtUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(LGtUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
        Ins(LGtUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GT, 0);
    	}
    
        Ins(LGe, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(LGe, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
        Ins(LGe, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 1);
    	}
    
        Ins(LGeUn, Reg(arg1) /* = */, Reg(arg2), Imm(arg3) /* imms32 */) => {
    		x86_64_cmp_reg_imm_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(LGeUn, Reg(arg1) /* = */, Reg(arg2), Local(arg3)) => {
    		x86_64_cmp_reg_membase_size(inst, arg2, X86_64_RBP, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
        Ins(LGeUn, Reg(arg1) /* = */, Reg(arg2), Reg(arg3)) => {
    		x86_64_cmp_reg_reg_size(inst, arg2, arg3, 8);
    		inst = setcc_reg(inst, arg1, X86_CC_GE, 0);
    	}
    
        Ins(FEq, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_Z, arg2, (void *)arg3, arg4, 0, 0);
    	}
        Ins(FEq, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_Z, arg2, arg3, arg4, 0, 0);
    	}
    
        Ins(FNe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NZ, arg2, (void *)arg3, arg4, 0, 1);
    	}
        Ins(FNe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NZ, arg2, arg3, arg4, 0, 1);
    	}
    
        Ins(FLt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_C, arg2, (void *)arg3, arg4, 0, 0);
    	}
        Ins(FLt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_C, arg2, arg3, arg4, 0, 0);
    	}
    
        Ins(FLtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_C, arg2, (void *)arg3, arg4, 0, 1);
    	}
        Ins(FLtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_C, arg2, arg3, arg4, 0, 1);
    	}
    
        Ins(FLe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_BE, arg2, (void *)arg3, arg4, 0, 0);
    	}
        Ins(FLe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_BE, arg2, arg3, arg4, 0, 0);
    	}
    
        Ins(FLeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_BE, arg2, (void *)arg3, arg4, 0, 1);
    	}
        Ins(FLeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_BE, arg2, arg3, arg4, 0, 1);
    	}
    
        Ins(FGt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NBE, arg2, (void *)arg3, arg4, 0, 0);
    	}
        Ins(FGt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NBE, arg2, arg3, arg4, 0, 0);
    	}
    
        Ins(FGtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NBE, arg2, (void *)arg3, arg4, 0, 1);
    	}
        Ins(FGtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NBE, arg2, arg3, arg4, 0, 1);
    	}
    
        Ins(FGe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NC, arg2, (void *)arg3, arg4, 0, 0);
    	}
        Ins(FGe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NC, arg2, arg3, arg4, 0, 0);
    	}
    
        Ins(FGeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(23) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NC, arg2, (void *)arg3, arg4, 0, 1);
    	}
        Ins(FGeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NC, arg2, arg3, arg4, 0, 1);
    	}
    
        Ins(DEq, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_Z, arg2, (void *)arg3, arg4, 1, 0);
    	}
        Ins(DEq, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_Z, arg2, arg3, arg4, 1, 0);
    	}
    
        Ins(DNe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NZ, arg2, (void *)arg3, arg4, 1, 1);
    	}
        Ins(DNe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NZ, arg2, arg3, arg4, 1, 1);
    	}
    
        Ins(DLt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_C, arg2, (void *)arg3, arg4, 1, 0);
    	}
        Ins(DLt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_C, arg2, arg3, arg4, 1, 0);
    	}
    
        Ins(DLtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_C, arg2, (void *)arg3, arg4, 1, 1);
    	}
        Ins(DLtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_C, arg2, arg3, arg4, 1, 1);
    	}
    
        Ins(DLe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_BE, arg2, (void *)arg3, arg4, 1, 0);
    	}
        Ins(DLe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_BE, arg2, arg3, arg4, 1, 0);
    	}
    
        Ins(DLeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_BE, arg2, (void *)arg3, arg4, 1, 1);
    	}
        Ins(DLeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_BE, arg2, arg3, arg4, 1, 1);
    	}
    
        Ins(DGt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NBE, arg2, (void *)arg3, arg4, 1, 0);
    	}
        Ins(DGt, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NBE, arg2, arg3, arg4, 1, 0);
    	}
    
        Ins(DGtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NBE, arg2, (void *)arg3, arg4, 1, 1);
    	}
        Ins(DGtInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NBE, arg2, arg3, arg4, 1, 1);
    	}
    
        Ins(DGe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NC, arg2, (void *)arg3, arg4, 1, 0);
    	}
        Ins(DGe, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NC, arg2, arg3, arg4, 1, 0);
    	}
    
        Ins(DGeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Imm(arg3), Reg(arg4) /* scratch */,  /* space(24) */) => {
    		inst = xmm_cmp_setcc_reg_imm(gen, inst, arg1, X86_CC_NC, arg2, (void *)arg3, arg4, 1, 1);
    	}
        Ins(DGeInv, Reg(arg1) /* =, + */, Reg(arg2) /* xreg */, Reg(arg3) /* xreg */, Reg(arg4) /* scratch */,  /* space(20) */) => {
    		inst = xmm_cmp_setcc_reg_reg(inst, arg1, X86_CC_NC, arg2, arg3, arg4, 1, 1);
    	}
    
        Ins(FSqrt, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_sqrtss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(FSqrt, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_sqrtss_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DSqrt, Reg(arg1) /* =, xreg */, Local(arg2), None) => {
    		x86_64_sqrtsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DSqrt, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_sqrtsd_reg_reg(inst, arg1, arg2);
    	}
    
    /*
     * Absolute, minimum, maximum, and sign.
     */
        Ins(IMax, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_LT, arg1, arg2, 1, 4);
    	}
    
        Ins(IMaxUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_LT, arg1, arg2, 0, 4);
    	}
    
        Ins(IMin, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_GT, arg1, arg2, 1, 4);
    	}
    
        Ins(IMinUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 4);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_GT, arg1, arg2, 0, 4);
    	}
    
        Ins(ISign, Reg(arg1) /* = */, Imm(arg2), None) => {
    		if(arg2 < 0)
    		{
    			x86_64_mov_reg_imm_size(inst, arg1, -1, 4);
    		}
    		else if(arg2 > 0)
    		{
    			x86_64_mov_reg_imm_size(inst, arg1, 1, 4);
    		}
    		else
    		{
    			x86_64_clear_reg(inst, arg1);
    		}
    	}
        Ins(ISign, Reg(arg1) /* =, + */, Reg(arg2) /* + */, None) => {
    		x86_64_clear_reg(inst, arg1);
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 4);
    		x86_64_set_reg(inst, X86_CC_NZ, arg1, 0);
    		x86_64_sar_reg_imm_size(inst, arg2, 31, 4);
    		x86_64_or_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(LMax, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_LT, arg1, arg2, 1, 8);
    	}
    
        Ins(LMaxUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_LT, arg1, arg2, 0, 8);
    	}
    
        Ins(LMin, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_GT, arg1, arg2, 1, 8);
    	}
    
        Ins(LMinUn, Reg(arg1), Reg(arg2), None) => {
    		x86_64_cmp_reg_reg_size(inst, arg1, arg2, 8);
    		x86_64_cmov_reg_reg_size(inst, X86_CC_GT, arg1, arg2, 0, 8);
    	}
    
        Ins(LSign, Reg(arg1) /* = */, Imm(arg2), None) => {
    		if(arg2 < 0)
    		{
    			x86_64_mov_reg_imm_size(inst, arg1, -1, 4);
    		}
    		else if(arg2 > 0)
    		{
    			x86_64_mov_reg_imm_size(inst, arg1, 1, 4);
    		}
    		else
    		{
    			x86_64_clear_reg(inst, arg1);
    		}
    	}
        Ins(LSign, Reg(arg1) /* =, + */, Reg(arg2) /* + */, None) => {
    		x86_64_clear_reg(inst, arg1);
    		x86_64_test_reg_reg_size(inst, arg2, arg2, 8);
    		x86_64_set_reg(inst, X86_CC_NZ, arg1, 0);
    		x86_64_sar_reg_imm_size(inst, arg2, 63, 8);
    		x86_64_or_reg_reg_size(inst, arg1, arg2, 4);
    	}
    
        Ins(FMax, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_maxss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(FMax, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_maxss_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(FMin, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_minss_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(FMin, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_minss_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DMax, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_maxsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DMax, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_maxsd_reg_reg(inst, arg1, arg2);
    	}
    
        Ins(DMin, Reg(arg1) /* xreg */, Local(arg2), None) => {
    		x86_64_minsd_reg_membase(inst, arg1, X86_64_RBP, arg2);
    	}
        Ins(DMin, Reg(arg1) /* xreg */, Reg(arg2) /* xreg */, None) => {
    		x86_64_minsd_reg_reg(inst, arg1, arg2);
    	}
    
    /*
     * Rounding
     */
        Ins(FFloor, Reg(arg1) /* =, xreg */, Local(arg2), Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_membase(inst, arg1, arg2, arg3, X86_ROUND_DOWN);
    	}
        Ins(FFloor, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_reg(inst, arg1, arg2, arg3, X86_ROUND_DOWN);
    	}
    
        Ins(DFloor, Reg(arg1) /* =, xreg */, Local(arg2), Reg(arg3) /* scratch */) => {
    		inst = x86_64_roundd_reg_membase(inst, arg1, arg2, arg3, X86_ROUND_DOWN);
    	}
        Ins(DFloor, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_roundd_reg_reg(inst, arg1, arg2, arg3, X86_ROUND_DOWN);
    	}
    
        Ins(NFFloor, Reg(arg1) /* freg */, Reg(arg2) /* scratch */, None) => {
    		inst = x86_64_roundnf(inst, arg2, X86_ROUND_DOWN);
    	}
    
        Ins(FCeil, Reg(arg1) /* =, xreg */, Local(arg2), Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_membase(inst, arg1, arg2, arg3, X86_ROUND_UP);
    	}
        Ins(FCeil, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_reg(inst, arg1, arg2, arg3, X86_ROUND_UP);
    	}
    
        Ins(DCeil, Reg(arg1) /* =, xreg */, Local(arg2), Reg(arg3) /* scratch */) => {
    		inst = x86_64_roundd_reg_membase(inst, arg1, arg2, arg3, X86_ROUND_UP);
    	}
        Ins(DCeil, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_roundd_reg_reg(inst, arg1, arg2, arg3, X86_ROUND_UP);
    	}
    
        Ins(NFCeil, Reg(arg1) /* freg */, Reg(arg2) /* scratch */, None) => {
    		inst = x86_64_roundnf(inst, arg2, X86_ROUND_UP);
    	}
    
    /*
        Ins(FRInt, Reg(arg1) /* =, xreg */, Local(arg2), Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_membase(inst, arg1, arg2, arg3, X86_ROUND_ZERO);
    	}
        Ins(FRInt, Reg(arg1) /* =, xreg */, Reg(arg2) /* xreg */, Reg(arg3) /* scratch */) => {
    		inst = x86_64_rounds_reg_reg(inst, arg1, arg2, arg3, X86_ROUND_ZERO);
    	}
    */
    
    /*
     * Pointer check opcodes.
     */
    
        Ins(CheckNull, Reg(arg1), None, None) => {
    #if 0 && defined(JIT_USE_SIGNALS)
    		/* if arg1 contains NULL this generates SEGV and the signal
    		   handler will throw the exception  */
    		x86_64_cmp_reg_membase_size(inst, arg1, arg1, 0, 8);
    #else
    		let mut patch;
    		x86_64_test_reg_reg_size(inst, arg1, arg1, 8);
    		patch = inst;
    		x86_branch8(inst, X86_CC_NE, 0, 0);
    		inst = throw_builtin(inst, func, JIT_RESULT_NULL_REFERENCE);
    		x86_patch(patch, inst);
    #endif
    	}
    
    /*
     * Function calls.
     */
    
        Ins(Call, None, None, None) => {
    		jit_function_t func = (jit_function_t)(insn->dest);
    		inst = x86_64_call_code(inst, (jit_nint)jit_function_to_closure(func));
    	}
    
        Ins(CallTail, None, None, None) => {
    		jit_function_t func = (jit_function_t)(insn->dest);
    		x86_64_mov_reg_reg_size(inst, X86_64_RSP, X86_64_RBP, 8);
    		x86_64_pop_reg_size(inst, X86_64_RBP, 8);
    		x86_64_jump_to_code(inst, (jit_nint)jit_function_to_closure(func));
    	}
    
        Ins(CallIndirect, None, None, None) => {
    		x86_64_mov_reg_imm_size(inst, X86_64_RAX, 8, 4);
    		x86_64_call_reg(inst, X86_64_SCRATCH);
    	}
    
        Ins(CallIndirectTail, None, None, None) => {
    		x86_64_mov_reg_reg_size(inst, X86_64_RSP, X86_64_RBP, 8);
    		x86_64_pop_reg_size(inst, X86_64_RBP, 8);
    		x86_64_jmp_reg(inst, X86_64_SCRATCH);
    	}
    
        Ins(CallVTablePtr, None, None, None) => {
    		x86_64_mov_reg_imm_size(inst, X86_64_RAX, 8, 4);
    		x86_64_call_reg(inst, X86_64_SCRATCH);
    	}
    
        Ins(CallVTablePtrTail, None, None, None) => {
    		x86_64_mov_reg_reg_size(inst, X86_64_RSP, X86_64_RBP, 8);
    		x86_64_pop_reg_size(inst, X86_64_RBP, 8);
    		x86_64_jmp_reg(inst, X86_64_SCRATCH);
    	}
    
        Ins(CallExternal, None, None, None) => {
    		inst = x86_64_call_code(inst, (jit_nint)(insn->dest));
    	}
    
        Ins(CallExternalTail, None, None, None) => {
    		x86_64_mov_reg_reg_size(inst, X86_64_RSP, X86_64_RBP, 8);
    		x86_64_pop_reg_size(inst, X86_64_RBP, 8);
    		x86_64_jump_to_code(inst, (jit_nint)(insn->dest));
    	}
    
    
    /*
     * Exception handling.
     */
    
        Ins(Throw, Reg(arg1), None, None) => {
    		x86_64_mov_reg_reg_size(inst, X86_64_RDI, arg1, 8);
    		if(func->builder->setjmp_value != 0)
    		{
    			jit_nint pc_offset;
    
    			/* We have a "setjmp" block in the current function,
    			   so we must record the location of the throw first */
    			_jit_gen_fix_value(func->builder->setjmp_value);
    			pc_offset = func->builder->setjmp_value->frame_offset +
    							jit_jmp_catch_pc_offset;
    
    			x86_64_lea_membase_size(inst, X86_64_SCRATCH, X86_64_RIP, 0, 8);
    			x86_64_mov_membase_reg_size(inst, X86_64_RBP, pc_offset,
    										X86_64_SCRATCH, 8);
    		}
    		inst = x86_64_call_code(inst, (jit_nint)jit_exception_throw);
    	}
    
        Ins(Rethrow, None, None, None) => { /* Not used in native code back ends */ }
    
        Ins(LoadPc, Reg(arg1) /* = */, None, None) => {
    		x86_64_lea_membase_size(inst, arg1, X86_64_RIP, 0, 8);
    	}
    
        Ins(LoadExceptionPc, None, None, None) => { /* Not used in native code back ends */ }
    
        Ins(EnterFinally, None, None, None) => {
    		/* The return address is on the stack */
    		x86_64_sub_reg_imm_size(inst, X86_64_RSP, 8, 8);
    	 }
    
        Ins(LeaveFinally, None, None, None) => {
    		/* The "finally" return address is on the stack */
    		x86_64_add_reg_imm_size(inst, X86_64_RSP, 8, 8);
    		x86_64_ret(inst);
    	}
    
        Ins(CallFinally, None, None, None) => {
    		jit_block_t block;
    
    		block = jit_block_from_label(func, (jit_label_t)(insn->dest));
    		if(!block)
    		{
    			return;
    		}
    
    		if(block->address)
    		{
    			inst = x86_64_call_code(inst, (jit_nint)block->address);
    		}
    		else
    		{
    			jit_int fixup;
    
    			if(block->fixup_list)
    			{
    				fixup = _JIT_CALC_FIXUP(block->fixup_list, inst + 1);
    			}
    			else
    			{
    				fixup = 0;
    			}
    			block->fixup_list = (void *)(inst + 1);
    			x86_64_call_imm(inst, fixup);
    		}
    	}
    
        Ins(AddressOfLabel, Reg(arg1) /* = */, None, None) => {
    		jit_int *fixup;
    
    		block = jit_block_from_label(func, (jit_label_t)(insn->value1));
    		if(block->address)
    		{
    			/* The label is in the current function so we assume that the */
    			/* displacement to the current instruction is in the +-2GB range */
    
    			x86_64_lea_membase_size(inst, arg1, X86_64_RIP, 0, 8);
    			fixup = (jit_int *)(inst - 4);
    			fixup[0] = (jit_int)((jit_nint)block->address - (jit_nint)inst);
    		}
    		else
    		{
    			/* Output a placeholder and record on the block's fixup list */
    			/* The label is in the current function so we assume that the */
    			/* displacement to the current instruction will be in the +-2GB range */
    			x86_64_lea_membase_size(inst, arg1, X86_64_RIP, 0, 8);
    			fixup = (jit_int *)(inst - 4);
    			if(block->fixup_list)
    			{
    				fixup[0] = _JIT_CALC_FIXUP(block->fixup_list, fixup);
    			}
    			block->fixup_list = (void *)fixup;
    		}
    	}
    
    /*
     * Block operations.
     */
    
        Ins(Memcpy, _, _, Imm(arg1),  if arg3 <= 0, None, None) => { }
        Ins(Memcpy, Reg(arg1), Reg(arg2), Imm(arg3), Reg(arg4) /* scratch */, Reg(arg5) /* scratch, xreg */,  if arg3 <= _JIT_MAX_MEMCPY_INLINE) => {
    		inst = small_block_copy(gen, inst, arg1, 0, arg2, 0, arg3, arg4, arg5, 0);
    	}
        Ins(Memcpy, Reg(arg1), Reg(arg2), Imm(arg3),  /* clobber(reg), clobber(xre) */) => {
    		inst = memory_copy(gen, inst, arg1, 0, arg2, 0, arg3);
    	}
        Ins(Memcpy, Reg(X86_64_RDI), Reg(X86_64_RSI), Reg(X86_64_RDX),  /* clobber(reg), clobber(xre) */, None, None, None) => {
    		inst = x86_64_call_code(inst, (jit_nint)jit_memcpy);
    	}
    
        Ins(Memset, Reg(X86_64_RDI), Reg(X86_64_RSI), Reg(X86_64_RDX),  /* clobber(reg), clobber(xre) */, None, None, None) => {
    		inst = x86_64_call_code(inst, (jit_nint)jit_memset);
    	}
    
        Ins(Alloca, Reg(arg1), None, None) => {
    		x86_64_add_reg_imm_size(inst, arg1, 15, 8);
    		x86_64_and_reg_imm_size(inst, arg1, ~15, 8);
    		x86_64_sub_reg_reg_size(inst, X86_64_RSP, arg1, 8);
    		x86_64_mov_reg_reg_size(inst, arg1, X86_64_RSP, 8);
    		inst = fixup_alloca(gen, inst, arg1);
    		gen->stack_changed = 1;
        }
    
        Ins(JumpTable, Reg(arg1), Imm(arg2), Imm(arg3), Reg(arg4) /* scratch */,  /* space(64) */) => {
    		let mut patch_jump_table;
    		let mut patch_fall_through;
    		int index;
    		jit_label_t *labels;
    		jit_nint num_labels;
    		jit_block_t block;
    
    		labels = (jit_label_t *) arg2;
    		num_labels = arg3;
    
    		patch_jump_table = (unsigned char *)_jit_gen_alloc(gen, size_of::<usize>() * arg3);
    		if(!patch_jump_table)
    		{
    			/* The cache is full */
    			return;
    		}
    
    		x86_64_mov_reg_imm_size(inst, arg4, (jit_nint)patch_jump_table, 8);
    		x86_64_cmp_reg_imm_size(inst, arg1, num_labels, 8);
    		patch_fall_through = inst;
    		x86_branch32(inst, X86_CC_AE, 0, 0);
    
    		if(func->builder->position_independent)
    		{
    			/* TODO */
    			TODO();
    		}
    		else
    		{
    			x86_64_jmp_memindex(inst, arg4, 0, arg1, 3);
    		}
    
    		for(index = 0; index < num_labels; index++)
    		{
    			block = jit_block_from_label(func, labels[index]);
    			if(!block)
    			{
    				return;
    			}
    
    			if(func->builder->position_independent)
    			{
    				/* TODO */
    				TODO();
    			}
    			else
    			{
    				if(block->address)
    				{
    					x86_64_imm_emit64(patch_jump_table, (jit_nint)(block->address));
    				}
    				else
    				{
    					/* Output a placeholder and record on the block's absolute fixup list */
    					x86_64_imm_emit64(patch_jump_table, (jit_nint)(block->fixup_absolute_list));
    					block->fixup_absolute_list = (void *)(patch_jump_table - 8);
    				}
    			}
    		}
    
    		x86_patch(patch_fall_through, inst);
    	}
    }
}
