use super::{Label, MemLocation};
use super::{DEFAULT_OBJFIELDS, LG_WORD_SIZE, WORD_SIZE};

use std::{fmt, io};

// This module contains functions used to output MIPS code.

const FP_CALL_LOCATION: MemLocation = MemLocation {
    reg: Register::SP,
    offset: 3,
};

const SELF_CALL_LOCATION: MemLocation = MemLocation {
    reg: Register::SP,
    offset: 2,
};

const RA_CALL_LOCATION: MemLocation = MemLocation {
    reg: Register::SP,
    offset: 1,
};

const STACK_TOP_LOCATION: MemLocation = MemLocation {
    // TODO
    reg: Register::SP,
    offset: 0,
};

// Registers used by the assembly code
#[derive(Copy, Clone)]
pub enum Register {
    ZERO,
    ACC,
    A1,
    SELF,
    T1,
    T2,
    T3,
    SP,
    FP,
    RA,
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Register::ZERO => write!(f, "$zero"),
            Register::ACC => write!(f, "$a0"),
            Register::A1 => write!(f, "$a1"),
            Register::SELF => write!(f, "$s0"),
            Register::T1 => write!(f, "$t1"),
            Register::T2 => write!(f, "$t2"),
            Register::T3 => write!(f, "$t3"),
            Register::SP => write!(f, "$sp"),
            Register::FP => write!(f, "$fp"),
            Register::RA => write!(f, "$ra"),
        }
    }
}

pub fn emit_global(out_file: &mut Box<dyn io::Write>, sym: &str) -> Result<(), io::Error> {
    writeln!(out_file, "\t.globl {}", sym)
}

pub fn emit_align(out_file: &mut Box<dyn io::Write>, n: i16) -> Result<(), io::Error> {
    writeln!(out_file, "\t.align {}", n)
}

pub trait AsWord {
    fn as_word(self) -> String;
}

impl AsWord for u32 {
    fn as_word(self) -> String {
        format!("{:#010x}", self)
    }
}

impl AsWord for String {
    fn as_word(self) -> String {
        self
    }
}

impl AsWord for &str {
    fn as_word(self) -> String {
        self.to_string()
    }
}

pub fn emit_word(out_file: &mut Box<dyn io::Write>, w: impl AsWord) -> Result<(), io::Error> {
    writeln!(out_file, "\t.word {}", w.as_word())
}

// Needed before an object for the garbage collector
pub fn emit_gc_tag(out_file: &mut Box<dyn io::Write>) -> Result<(), io::Error> {
    writeln!(out_file, "\t.word -1")
}

pub fn emit_string_const(
    out_file: &mut Box<dyn io::Write>,
    base_name: &str,
    string_tag: u32,
    val: &str,
    val_id: u32,
) -> Result<(), io::Error> {
    emit_gc_tag(out_file)?;
    writeln!(out_file, "{}_{:08x}:", base_name, val_id)?;
    emit_word(out_file, string_tag)?;
    emit_word(out_file, (val.len() as u32) / (WORD_SIZE as u32) + 5)?;
    emit_word(out_file, "String_dispTab")?;
    emit_word(out_file, format!("int_const_{:08x}", val.len()))?;

    write!(out_file, "\t.byte")?;
    for b in val.as_bytes().iter() {
        write!(out_file, " {:#04x}", b)?;
    }
    writeln!(out_file, " 0x00")?; // Bytes are null terminated

    emit_align(out_file, LG_WORD_SIZE)?;

    Ok(())
}

pub fn emit_int_const(
    out_file: &mut Box<dyn io::Write>,
    int_tag: u32,
    val: u32,
) -> Result<(), io::Error> {
    emit_gc_tag(out_file)?;
    writeln!(out_file, "int_const_{:08x}:", val)?;
    emit_word(out_file, int_tag)?;
    emit_word(out_file, 4)?; // Length
    emit_word(out_file, "Int_dispTab")?;
    emit_word(out_file, val)?;

    Ok(())
}

pub fn emit_bool_const(
    out_file: &mut Box<dyn io::Write>,
    bool_tag: u32,
    val: bool,
) -> Result<(), io::Error> {
    emit_gc_tag(out_file)?;

    if val {
        writeln!(out_file, "bool_const1:")?;
    } else {
        writeln!(out_file, "bool_const0:")?;
    }

    emit_word(out_file, bool_tag)?;
    emit_word(out_file, 4)?; // Length
    emit_word(out_file, "Bool_dispTab")?;

    if val {
        emit_word(out_file, 1)?;
    } else {
        emit_word(out_file, 0)?;
    }

    Ok(())
}

pub fn emit_jalr(out_file: &mut Box<dyn io::Write>, dest: Register) -> Result<(), io::Error> {
    writeln!(out_file, "\tjalr {}", dest)
}

pub fn emit_jal(out_file: &mut Box<dyn io::Write>, target: &str) -> Result<(), io::Error> {
    writeln!(out_file, "\tjal {}", target)
}

pub fn emit_return(out_file: &mut Box<dyn io::Write>) -> Result<(), io::Error> {
    writeln!(out_file, "\tjr {}", Register::RA)
}

pub fn emit_store_word(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    MemLocation { reg: dest, offset }: MemLocation,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tsw {} {}({})", src, WORD_SIZE * offset, dest)
}

pub fn emit_load_word(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    MemLocation { reg: src, offset }: MemLocation,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tlw {} {}({})", dest, WORD_SIZE * offset, src)
}

pub fn emit_load_imm(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    imm: i16,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tli {} {:#06x}", dest, imm)
}

pub fn emit_load_address(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    addr: String,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tla {} {}", dest, addr)
}

pub fn emit_move(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tmove {} {}", dest, src)
}

pub fn emit_neg(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tneg {} {}", dest, src)
}

pub fn emit_add(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src1: Register,
    src2: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tadd {} {} {}", dest, src1, src2)
}

pub fn emit_addu(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src1: Register,
    src2: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\taddu {} {} {}", dest, src1, src2)
}

pub fn emit_addiu(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src: Register,
    imm: i16,
) -> Result<(), io::Error> {
    writeln!(out_file, "\taddiu {} {} {}", dest, src, imm)
}

pub fn emit_div(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src1: Register,
    src2: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tdiv {} {} {}", dest, src1, src2)
}

pub fn emit_mul(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src1: Register,
    src2: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tmul {} {} {}", dest, src1, src2)
}

pub fn emit_sub(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src1: Register,
    src2: Register,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tsub {} {} {}", dest, src1, src2)
}

pub fn emit_sll(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src: Register,
    shift: u8,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tsll {} {} {}", dest, src, shift)
}

pub fn emit_branch(out_file: &mut Box<dyn io::Write>, label: Label) -> Result<(), io::Error> {
    writeln!(out_file, "\tb {}", label)
}

pub fn emit_beqz(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tbeqz {} {}", src, label)
}

pub fn emit_bnez(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tbnez {} {}", src, label)
}

pub fn emit_beq(
    out_file: &mut Box<dyn io::Write>,
    src1: Register,
    src2: Register,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tbeq {} {} {}", src1, src2, label)
}

pub fn emit_bleq(
    out_file: &mut Box<dyn io::Write>,
    src1: Register,
    src2: Register,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tble {} {} {}", src1, src2, label)
}

pub fn emit_blt(
    out_file: &mut Box<dyn io::Write>,
    src1: Register,
    src2: Register,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tblt {} {} {}", src1, src2, label)
}

pub fn emit_blti(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    imm: u32,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tblt {} {} {}", src, imm, label)
}

pub fn emit_bgei(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    imm: u32,
    label: Label,
) -> Result<(), io::Error> {
    writeln!(out_file, "\tbge {} {} {}", src, imm, label)
}

pub fn emit_label_def(out_file: &mut Box<dyn io::Write>, label: Label) -> Result<(), io::Error> {
    writeln!(out_file, "{}:", label)
}

// Instructions to load an Int type's value pointed to by src into dest
pub fn emit_fetch_int(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    src: Register,
) -> Result<(), io::Error> {
    emit_load_word(
        out_file,
        dest,
        MemLocation {
            reg: src,
            offset: DEFAULT_OBJFIELDS,
        },
    )
}

// Instructions to load src's value in to the Int object pointed to by dest
pub fn emit_store_int(
    out_file: &mut Box<dyn io::Write>,
    src: Register,
    dest: Register,
) -> Result<(), io::Error> {
    emit_store_word(
        out_file,
        src,
        MemLocation {
            reg: dest,
            offset: DEFAULT_OBJFIELDS,
        },
    )
}

// Add the value in src to the stack
pub fn emit_push(out_file: &mut Box<dyn io::Write>, src: Register) -> Result<(), io::Error> {
    emit_store_word(out_file, src, STACK_TOP_LOCATION)?;
    emit_addiu(out_file, Register::SP, Register::SP, -WORD_SIZE)?;

    Ok(())
}

// Remove the top value on the stack and set dest to that value
pub fn emit_pop(out_file: &mut Box<dyn io::Write>, dest: Register) -> Result<(), io::Error> {
    emit_addiu(out_file, Register::SP, Register::SP, WORD_SIZE)?;
    emit_load_word(out_file, dest, STACK_TOP_LOCATION)?;

    Ok(())
}

// Set dest to point to the specified Bool constant
pub fn emit_load_bool(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    val: bool,
) -> Result<(), io::Error> {
    if val {
        emit_load_address(out_file, dest, "bool_const1".to_string())?;
    } else {
        emit_load_address(out_file, dest, "bool_const0".to_string())?;
    }

    Ok(())
}

// Set dest to point to the specified Int constant
pub fn emit_load_int(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    val: u32,
) -> Result<(), io::Error> {
    emit_load_address(out_file, dest, format!("int_const_{:08x}", val))
}

// Set dest to point to the specified String constant
pub fn emit_load_string(
    out_file: &mut Box<dyn io::Write>,
    dest: Register,
    base_name: &str,
    val_id: u32,
) -> Result<(), io::Error> {
    emit_load_address(out_file, dest, format!("{}_{:08x}", base_name, val_id))
}

pub fn emit_method_start(out_file: &mut Box<dyn io::Write>) -> Result<(), io::Error> {
    // Push values onto stack
    emit_addiu(out_file, Register::SP, Register::SP, -3 * WORD_SIZE)?;
    emit_store_word(out_file, Register::FP, FP_CALL_LOCATION)?;
    emit_store_word(out_file, Register::SELF, SELF_CALL_LOCATION)?;
    emit_store_word(out_file, Register::RA, RA_CALL_LOCATION)?;

    // Set FP to start of new frame
    emit_addiu(out_file, Register::FP, Register::SP, WORD_SIZE)?;

    // value passed in ACC is new SELF
    emit_move(out_file, Register::SELF, Register::ACC)?;

    Ok(())
}

// Undo emit_method_start and remove all arguments that were pushed onto stack
pub fn emit_method_end(
    out_file: &mut Box<dyn io::Write>,
    formal_count: i16,
) -> Result<(), io::Error> {
    emit_load_word(out_file, Register::FP, FP_CALL_LOCATION)?;
    emit_load_word(out_file, Register::SELF, SELF_CALL_LOCATION)?;
    emit_load_word(out_file, Register::RA, RA_CALL_LOCATION)?;

    emit_addiu(
        out_file,
        Register::SP,
        Register::SP,
        (3 + formal_count) * WORD_SIZE,
    )?;
    emit_return(out_file)?;

    Ok(())
}
