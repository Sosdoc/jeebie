use gbe::instr::alu16::*;
use gbe::instr::alu8::*;
use gbe::instr::load::*;
use gbe::instr::misc::*;
use gbe::instr::stack::*;

use gbe::cpu::CPU;
/// The type of functions that implement an opcode.
pub type OpcodeFunc = fn(&mut CPU) -> ();


pub static OPCODE_TABLE : [OpcodeFunc; 256] = [
    // 0x00
                           nop,                   LD_bc_nn,                   LD_BCm_A,                     INC_bc,
                         INC_b,                      DEC_b,                     LD_B_n, panic!("Not implemented."),
                     LD_nnm_sp,                  ADD_hl_bc,                   LD_a_BCm,                     DEC_bc,
                         INC_c,                      DEC_c,                     LD_C_n, panic!("Not implemented."),
    // 0x10
    panic!("Not implemented."),                   LD_de_nn,                   LD_DEm_A,                     INC_de,
                         INC_d,                      DEC_d,                     LD_D_n, panic!("Not implemented."),
    panic!("Not implemented."),                  ADD_hl_de,                   LD_a_DEm,                     DEC_de,
                         INC_e,                      DEC_e,                     LD_E_n, panic!("Not implemented."),
    // 0x20
    panic!("Not implemented."),                   LD_hl_nn,                  LDI_HLm_a,                     INC_hl,
                         INC_h,                      DEC_h,                     LD_H_n, panic!("Not implemented."),
    panic!("Not implemented."),                  ADD_hl_hl,                  LDI_a_HLm,                     DEC_hl,
                         INC_l,                      DEC_l,                     LD_L_n, panic!("Not implemented."),
    // 0x30
    panic!("Not implemented."),                   LD_sp_nn,                  LDD_HLm_a,                     INC_sp,
                       INC_hlm,                    DEC_hlm,                   LD_HLm_n, panic!("Not implemented."),
    panic!("Not implemented."),                  ADD_hl_sp,                  LDD_a_HLm,                     DEC_sp,
                         INC_a,                      DEC_a,                     LD_a_n, panic!("Not implemented."),
    // 0x40
                        LD_b_b,                     LD_b_c,                     LD_b_d,                     LD_b_e,
                        LD_b_h,                     LD_b_l,                   LD_b_HLm,                     LD_b_a,
                        LD_c_b,                     LD_c_c,                     LD_c_d,                     LD_c_e,
                        LD_c_h,                     LD_c_l,                   LD_c_HLm,                     LD_c_a,
    // 0x50
                        LD_d_b,                     LD_d_c,                     LD_d_e,                     LD_d_d,
                        LD_d_h,                     LD_d_l,                   LD_d_HLm,                     LD_d_a,
                        LD_e_b,                     LD_e_c,                     LD_e_d,                     LD_e_e,
                        LD_e_h,                     LD_e_l,                   LD_e_HLm,                     LD_e_a,
    // 0x60
                        LD_h_b,                     LD_h_c,                     LD_h_d,                     LD_h_e,
                        LD_h_h,                     LD_h_l,                   LD_h_HLm,                     LD_h_a,
                        LD_l_b,                     LD_l_c,                     LD_l_d,                     LD_l_e,
                        LD_l_h,                     LD_l_l,                   LD_l_HLm,                     LD_l_a,
    // 0x70
                      LD_HLm_b,                   LD_HLm_c,                   LD_HLm_d,                   LD_HLm_e,
                      LD_HLm_h,                   LD_HLm_l, panic!("Not implemented."),                   LD_HLm_a,
                        LD_a_b,                     LD_a_c,                     LD_a_d,                     LD_a_e,
                        LD_a_h,                     LD_a_l,                   LD_a_HLm,                     LD_a_a,
    // 0x80
                       ADD_a_b,                    ADD_a_c,                    ADD_a_d,                    ADD_a_e,
                       ADD_a_h,                    ADD_a_l,                  ADD_a_hlm,                    ADD_a_a,
                       ADC_a_b,                    ADC_a_c,                    ADC_a_d,                    ADC_a_e,
                       ADC_a_h,                    ADC_a_l,                  ADC_a_hlm,                    ADC_a_a,
    // 0x90
                       SUB_a_B,                    SUB_a_C,                    SUB_a_D,                    SUB_a_E,
                       SUB_a_H,                    SUB_a_L,                  SUB_a_hlm,                    SUB_a_A,
                       SBC_a_b,                    SBC_a_c,                    SBC_a_d,                    SBC_a_e,
                       SBC_a_h,                    SBC_a_l,                  SBC_a_hlm,                    SBC_a_a,
    // 0xa0
                         AND_b,                      AND_c,                      AND_d,                      AND_e,
                         AND_h,                      AND_l,                    AND_hlm,                      AND_a,
                         XOR_b,                      XOR_c,                      XOR_d,                      XOR_e,
                         XOR_h,                      XOR_l,                    XOR_hlm,                      XOR_a,
    // 0xb0
                          OR_b,                       OR_c,                       OR_d,                       OR_e,
                          OR_h,                       OR_l,                     OR_hlm,                       OR_a,
                          CP_b,                       CP_c,                       CP_d,                       CP_e,
                          CP_h,                       CP_l,                     CP_hlm,                       CP_a,
    // 0xc0
    panic!("Not implemented."),                     POP_bc, panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."),                    PUSH_bc,                    ADD_a_n, panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."),                    ADC_a_n, panic!("Not implemented."),
    // 0xd0
    panic!("Not implemented."),                     POP_de, panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."),                    PUSH_de,                    SUB_a_n, panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xe0
                      LDH_nm_a,                     POP_hl,                 LD_c_mem_a, panic!("Not implemented."),
    panic!("Not implemented."),                    PUSH_hl,                      AND_n, panic!("Not implemented."),
                      ADD_sp_n, panic!("Not implemented."),                   LD_nnm_A, panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."),                      XOR_n, panic!("Not implemented."),
    // 0xf0
                      LDH_a_nm,                     POP_af,                 LD_a_c_mem, panic!("Not implemented."),
    panic!("Not implemented."),                    PUSH_af,                       OR_n, panic!("Not implemented."),
                     LDHL_sp_n,                   LD_sp_hl,                   LD_a_nnm, panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."),                       CP_n, panic!("Not implemented."),
];

pub static CB_OPCODE_TABLE : [OpcodeFunc; 256] = [
    // 0x00
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x10
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x20
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x30
                        SWAP_b,                     SWAP_c,                     SWAP_d,                     SWAP_e,
                        SWAP_h,                     SWAP_l,                    SWAP_hl,                     SWAP_a,
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x40
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x50
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x60
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x70
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x80
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0x90
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xa0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xb0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xc0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xd0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xe0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    // 0xf0
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
    panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."), panic!("Not implemented."),
];
