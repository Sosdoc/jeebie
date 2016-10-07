/// This file is autogenerated, check the generate_tables.py source file.
use jeebie::instr::alu16::*;
use jeebie::instr::alu8::*;
use jeebie::instr::bit::*;
use jeebie::instr::jumps::*;
use jeebie::instr::load::*;
use jeebie::instr::misc::*;
use jeebie::instr::rotates::*;
use jeebie::instr::stack::*;

use jeebie::core::cpu::CPU;
/// The type of functions that implement an opcode.
pub type OpcodeFunc = fn(&mut CPU) -> i32;

fn missing(cpu: &mut CPU) -> i32 { panic!("Opcode 0x{:02X} is not implemented!", cpu.mem.read_b(cpu.reg.pc-1)) }
fn missing_cb(cpu: &mut CPU) -> i32 { panic!("Opcode 0xCB{:02X} is not implemented!", cpu.mem.read_b(cpu.reg.pc-1)) }


pub static OPCODE_TABLE : [OpcodeFunc; 256] = [
    // 0x00
              nop,      LD_bc_nn,      LD_BCm_A,        INC_bc,
            INC_b,         DEC_b,        LD_B_n,          RLCA,
        LD_nnm_sp,     ADD_hl_bc,      LD_a_BCm,        DEC_bc,
            INC_c,         DEC_c,        LD_C_n,          RRCA,
    // 0x10
          missing,      LD_de_nn,      LD_DEm_A,        INC_de,
            INC_d,         DEC_d,        LD_D_n,           RLA,
             JR_n,     ADD_hl_de,      LD_a_DEm,        DEC_de,
            INC_e,         DEC_e,        LD_E_n,           RRA,
    // 0x20
          JR_NZ_n,      LD_hl_nn,     LDI_HLm_a,        INC_hl,
            INC_h,         DEC_h,        LD_H_n,       missing,
           JR_Z_n,     ADD_hl_hl,     LDI_a_HLm,        DEC_hl,
            INC_l,         DEC_l,        LD_L_n,       missing,
    // 0x30
          JR_NC_n,      LD_sp_nn,     LDD_HLm_a,        INC_sp,
          INC_hlm,       DEC_hlm,      LD_HLm_n,       missing,
           JR_C_n,     ADD_hl_sp,     LDD_a_HLm,        DEC_sp,
            INC_a,         DEC_a,        LD_a_n,       missing,
    // 0x40
           LD_b_b,        LD_b_c,        LD_b_d,        LD_b_e,
           LD_b_h,        LD_b_l,      LD_b_HLm,        LD_b_a,
           LD_c_b,        LD_c_c,        LD_c_d,        LD_c_e,
           LD_c_h,        LD_c_l,      LD_c_HLm,        LD_c_a,
    // 0x50
           LD_d_b,        LD_d_c,        LD_d_e,        LD_d_d,
           LD_d_h,        LD_d_l,      LD_d_HLm,        LD_d_a,
           LD_e_b,        LD_e_c,        LD_e_d,        LD_e_e,
           LD_e_h,        LD_e_l,      LD_e_HLm,        LD_e_a,
    // 0x60
           LD_h_b,        LD_h_c,        LD_h_d,        LD_h_e,
           LD_h_h,        LD_h_l,      LD_h_HLm,        LD_h_a,
           LD_l_b,        LD_l_c,        LD_l_d,        LD_l_e,
           LD_l_h,        LD_l_l,      LD_l_HLm,        LD_l_a,
    // 0x70
         LD_HLm_b,      LD_HLm_c,      LD_HLm_d,      LD_HLm_e,
         LD_HLm_h,      LD_HLm_l,       missing,      LD_HLm_a,
           LD_a_b,        LD_a_c,        LD_a_d,        LD_a_e,
           LD_a_h,        LD_a_l,      LD_a_HLm,        LD_a_a,
    // 0x80
          ADD_a_b,       ADD_a_c,       ADD_a_d,       ADD_a_e,
          ADD_a_h,       ADD_a_l,     ADD_a_hlm,       ADD_a_a,
          ADC_a_b,       ADC_a_c,       ADC_a_d,       ADC_a_e,
          ADC_a_h,       ADC_a_l,     ADC_a_hlm,       ADC_a_a,
    // 0x90
          SUB_a_B,       SUB_a_C,       SUB_a_D,       SUB_a_E,
          SUB_a_H,       SUB_a_L,     SUB_a_hlm,       SUB_a_A,
          SBC_a_b,       SBC_a_c,       SBC_a_d,       SBC_a_e,
          SBC_a_h,       SBC_a_l,     SBC_a_hlm,       SBC_a_a,
    // 0xa0
            AND_b,         AND_c,         AND_d,         AND_e,
            AND_h,         AND_l,       AND_hlm,         AND_a,
            XOR_b,         XOR_c,         XOR_d,         XOR_e,
            XOR_h,         XOR_l,       XOR_hlm,         XOR_a,
    // 0xb0
             OR_b,          OR_c,          OR_d,          OR_e,
             OR_h,          OR_l,        OR_hlm,          OR_a,
             CP_b,          CP_c,          CP_d,          CP_e,
             CP_h,          CP_l,        CP_hlm,          CP_a,
    // 0xc0
           RET_NZ,        POP_bc,      JP_NZ_nn,         JP_nn,
       CALL_NZ_nn,       PUSH_bc,       ADD_a_n,       RST_00h,
            RET_Z,           RET,       JP_Z_nn,       missing,
        CALL_Z_nn,       CALL_nn,       ADC_a_n,       RST_08h,
    // 0xd0
           RET_NC,        POP_de,      JP_NC_nn,       missing,
       CALL_NC_nn,       PUSH_de,       SUB_a_n,       RST_10h,
            RET_C,       missing,       JP_C_nn,       missing,
        CALL_C_nn,       missing,       missing,       RST_18h,
    // 0xe0
         LDH_nm_a,        POP_hl,    LD_c_mem_a,       missing,
          missing,       PUSH_hl,         AND_n,       RST_20h,
         ADD_sp_n,         JP_HL,      LD_nnm_A,       missing,
          missing,       missing,         XOR_n,       RST_28h,
    // 0xf0
         LDH_a_nm,        POP_af,    LD_a_c_mem,            DI,
          missing,       PUSH_af,          OR_n,       RST_30h,
        LDHL_sp_n,      LD_sp_hl,      LD_a_nnm,            EI,
          missing,       missing,          CP_n,       RST_38h,
];

pub static CB_OPCODE_TABLE : [OpcodeFunc; 256] = [
    // 0x00
            RLC_B,         RLC_C,         RLC_D,         RLC_E,
            RLC_H,         RLC_L,       RLC_HLm,         RLC_A,
            RRC_B,         RRC_C,         RRC_D,         RRC_E,
            RRC_H,         RRC_L,       RRC_HLm,         RRC_A,
    // 0x10
             RL_B,          RL_C,          RL_D,          RL_E,
             RL_H,          RL_L,        RL_HLm,          RL_A,
             RR_B,          RR_C,          RR_D,          RR_E,
             RR_H,          RR_L,        RR_HLm,          RR_A,
    // 0x20
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0x30
           SWAP_b,        SWAP_c,        SWAP_d,        SWAP_e,
           SWAP_h,        SWAP_l,       SWAP_hl,        SWAP_a,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0x40
          BIT_0_B,       BIT_0_C,       BIT_0_D,       BIT_0_E,
          BIT_0_H,       BIT_0_L,     BIT_0_HLm,       BIT_0_A,
          BIT_1_B,       BIT_1_C,       BIT_1_D,       BIT_1_E,
          BIT_1_H,       BIT_1_L,     BIT_1_HLm,       BIT_1_A,
    // 0x50
          BIT_2_B,       BIT_2_C,       BIT_2_D,       BIT_2_E,
          BIT_2_H,       BIT_2_L,     BIT_2_HLm,       BIT_2_A,
          BIT_3_B,       BIT_3_C,       BIT_3_D,       BIT_3_E,
          BIT_3_H,       BIT_3_L,     BIT_3_HLm,       BIT_3_A,
    // 0x60
          BIT_4_B,       BIT_4_C,       BIT_4_D,       BIT_4_E,
          BIT_4_H,       BIT_4_L,     BIT_4_HLm,       BIT_4_A,
          BIT_5_B,       BIT_5_C,       BIT_5_D,       BIT_5_E,
          BIT_5_H,       BIT_5_L,     BIT_5_HLm,       BIT_5_A,
    // 0x70
          BIT_6_B,       BIT_6_C,       BIT_6_D,       BIT_6_E,
          BIT_6_H,       BIT_6_L,     BIT_6_HLm,       BIT_6_A,
          BIT_7_B,       BIT_7_C,       BIT_7_D,       BIT_7_E,
          BIT_7_H,       BIT_7_L,     BIT_7_HLm,       BIT_7_A,
    // 0x80
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0x90
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xa0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xb0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xc0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xd0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xe0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
    // 0xf0
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
       missing_cb,    missing_cb,    missing_cb,    missing_cb,
];