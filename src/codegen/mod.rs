#![allow(unused_variables)]
#![allow(dead_code)]

macro_rules! jit_assert {
    () => {
        panic!("jit assert");
    };
    ( $expr:expr ) => {
        assert!($expr);
    }
}

mod x86;
mod x86_64;

pub struct Emit {
    stream: Vec<u8>
}

impl Emit {
    pub fn push(&mut self, b: u8) {
        self.stream.push(b);
    }
    
    pub fn set_at(&mut self, b: u8, pos: i32) {
        let offset = (self.stream.len() as isize + pos as isize) as usize;
        self.stream[offset] = b;
    }
    
    pub fn get(&self) -> u8 {
        self.get_at(0)
    }
    
    pub fn get_at(&self, pos: i32) -> u8 {
        self.stream[(self.stream.len() as isize + pos as isize) as usize]
    }
}

#[derive(Copy, Clone)]
pub struct Ins(pub Op, pub Arg, pub Arg, pub Arg);

#[derive(Copy, Clone)]
pub enum Op {
    AddRelative,
    AddressOf,
    AddressOfLabel,
    Alloca,
    Br,
    BrDEq,
    BrDGe,
    BrDGeInv,
    BrDGt,
    BrDGtInv,
    BrDLe,
    BrDLeInv,
    BrDLt,
    BrDLtInv,
    BrDNe,
    BrFEq,
    BrFGe,
    BrFGeInv,
    BrFGt,
    BrFGtInv,
    BrFLe,
    BrFLeInv,
    BrFLt,
    BrFLtInv,
    BrFNe,
    BrIEq,
    BrIFalse,
    BrIGe,
    BrIGeUn,
    BrIGt,
    BrIGtUn,
    BrILe,
    BrILeUn,
    BrILt,
    BrILtUn,
    BrINe,
    BrITrue,
    BrLEq,
    BrLFalse,
    BrLGe,
    BrLGeUn,
    BrLGt,
    BrLGtUn,
    BrLLe,
    BrLLeUn,
    BrLLt,
    BrLLtUn,
    BrLNe,
    BrLTrue,
    Call,
    CallExternal,
    CallExternalTail,
    CallFilter,
    CallFilterReturn,
    CallFinally,
    CallIndirect,
    CallIndirectTail,
    CallTail,
    CallVTablePtr,
    CallVTablePtrTail,
    CallVTablePtrTrail,
    CheckInt,
    CheckNull,
    CheckSByte,
    CheckShort,
    CheckUByte,
    CheckUInt,
    CheckUShort,
    CopyFloat32,
    CopyFloat64,
    CopyIng,
    CopyInt,
    CopyLoadByte,
    CopyLoadSByte,
    CopyLoadShort,
    CopyLoadSShort,
    CopyLoadUByte,
    CopyLoadUShort,
    CopyLong,
    CopyNFloat,
    CopyStoreByte,
    CopyStoreShort,
    CopyStruct,
    DAbs,
    DAdd,
    DAtan,
    DCeil,
    DCos,
    DDiv,
    DEq,
    DFAdd,
    DFloor,
    DGe,
    DGeInv,
    DGt,
    DGtInv,
    DLe,
    DLeInv,
    DLt,
    DLtInv,
    DMax,
    DMin,
    DMul,
    DNe,
    DNeg,
    DRem,
    DSin,
    DSqrt,
    DSub,
    EnterFilter,
    EnterFinally,
    ExpandInt,
    ExpandUInt,
    FAbs,
    FAdd,
    FAtan,
    FCeil,
    FCos,
    FDiv,
    FEq,
    FFloor,
    FGe,
    FGeInv,
    FGt,
    FGtInv,
    FLe,
    FLeInv,
    Float32ToFloat64,
    Float32ToInt,
    Float32ToLong,
    Float32TONFloat,
    Float32ToUInt,
    Float64ToFloat32,
    Float64ToInt,
    Float64ToLong,
    Float64ToNFloat,
    Float64ToUInt,
    FLt,
    FLtInv,
    FlushSmallStruct,
    FMax,
    FMin,
    FMul,
    FNe,
    FNeg,
    FRem,
    FRInt,
    FSin,
    FSqrt,
    FSub,
    IAbs,
    IAdd,
    IAnd,
    IDiv,
    IDivUn,
    IEq,
    IGe,
    IGeUn,
    IGt,
    IGtUn,
    ILe,
    ILeUn,
    ILt,
    ILtUn,
    IMax,
    IMaxUn,
    IMin,
    IMinUn,
    Import,
    IMul,
    IncomingReg,
    INe,
    INeg,
    INot,
    IntToFloat32,
    IntToFloat64,
    IntToNFloat,
    IOr,
    IRem,
    IRemUn,
    IShl,
    IShr,
    IShrUn,
    ISign,
    ISub,
    IXOr,
    JumpTable,
    LAdd,
    LAnd,
    LDiv,
    LDivUn,
    LeaveFilter,
    LeaveFinally,
    LEq,
    LGe,
    LGeUn,
    LGt,
    LGtUn,
    LLe,
    LLeUn,
    LLt,
    LLtUn,
    LMax,
    LMaxUn,
    LMin,
    LMinUn,
    LMul,
    LNe,
    LNeg,
    LNot,
    LoadElementFloat32,
    LoadElementFloat64,
    LoadElementInt,
    LoadElementLong,
    LoadElementNFloat,
    LoadElementSByte,
    LoadElementShort,
    LoadElementUByte,
    LoadElementUShort,
    LoadExceptionPc,
    LoadPc,
    LoadRelativeFloat32,
    LoadRelativeFloat64,
    LoadRelativeInt,
    LoadRelativeLong,
    LoadRelativeNFloat,
    LoadRelativeSByte,
    LoadRelativeShort,
    LoadRelativeStruct,
    LoadRelativeUByte,
    LoadRelativeUShort,
    LongToFloat32,
    LongToFloat64,
    LongToNFloat,
    LOr,
    LowWord,
    LRem,
    LRemUn,
    LShl,
    LShr,
    LShrUn,
    LSign,
    LSub,
    LXOr,
    Memcpy,
    MemMove,
    Memset,
    NFAbs,
    NFAdd,
    NFAtan,
    NFCeil,
    NFCos,
    NFDiv,
    NFFloor,
    NFloatToFloat32,
    NFloatToFloat64,
    NFloatToInt,
    NFloatToLong,
    NFMul,
    NFNeg,
    NFRem,
    NFSin,
    NFSqrt,
    NFSub,
    PopStack,
    PushFloat32,
    PushFloat64,
    PushInt,
    PushLong,
    PushNFloat,
    PushStruct,
    Rethrow,
    Return,
    ReturnFloat32,
    ReturnFloat64,
    ReturnInt,
    ReturnLong,
    ReturnNFloat,
    ReturnReg,
    ReturnSmallStruct,
    SetParamFloat32,
    SetParamFloat64,
    SetParamInt,
    SetParamLong,
    SetParamNFloat,
    SetParamStruct,
    SetupForNested,
    SetupForSibling,
    StoreElementByte,
    StoreElementFloat32,
    StoreElementFloat64,
    StoreElementInt,
    StoreElementLong,
    StoreElementNFloat,
    StoreElementShort,
    StoreRelativeByte,
    StoreRelativeFloat32,
    StoreRelativeFloat64,
    StoreRelativeInt,
    StoreRelativeLong,
    StoreRelativeNFloat,
    StoreRelativeShort,
    StoreRelativeStruct,
    Throw,
    TruncInt,
    TruncSByte,
    TruncShort,
    TruncUByte,
    TruncUInt,
    TruncUShort,
    UIntToFloat32,
    UIntToFloat64,
    UIntToNFloat,
    ULongToFloat32,
    ULongToFloat64,
    ULongToNFloat
}

#[derive(Copy, Clone)]
pub enum Arg {
    None,
	Reg(u8),
	Imm(i64),
	Local(i32)
}

/*
#[derive(Copy, Clone)]
pub enum Value {
	U8(u8),
	I8(i8),
	U16(u16),
	I16(i16),
	U32(u32),
	I32(i32),
	U64(u64),
	I64(i64),
	F32(f32),
	F64(f64),
	ISize(isize),
	USize(usize)
}
*/

/*
impl Value {
	fn u8(&self) -> u8 {
		if let Value::U8(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn i8(&self) -> i8 {
		if let Value::I8(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn u16(&self) -> u16 {
		if let Value::U16(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn i16(&self) -> i16 {
		if let Value::I16(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn u32(&self) -> u32 {
		if let Value::U32(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn i32(&self) -> i32 {
		if let Value::I32(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn u64(&self) -> u64 {
		if let Value::U64(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn i64(&self) -> i64{
		if let Value::I64(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn f32(&self) -> f32 {
		if let Value::F32(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn f64(&self) -> f64 {
		if let Value::F64(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn isize(&self) -> isize {
		if let Value::ISize(value) = *self {
			value
		} else {
			panic!();
		}
	}

	fn usize(&self) -> usize {
		if let Value::USize(value) = *self {
			value
		} else {
			panic!();
		}
	}
}
*/
