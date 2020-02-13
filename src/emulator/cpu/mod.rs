mod opcodes;
mod cpu;

pub use cpu::CPU;

use opcodes::*;

type OpcodeFn = fn() -> ();

#[allow(dead_code)]
static OPCODES: [OpcodeFn; 256] = [
    opcode_00, opcode_01, opcode_02, opcode_03, opcode_04, opcode_05, opcode_06, opcode_07, opcode_08, opcode_09, opcode_0a, opcode_0b, opcode_0c, opcode_0d, opcode_0e, opcode_0f,
	opcode_10, opcode_11, opcode_12, opcode_13, opcode_14, opcode_15, opcode_16, opcode_17, opcode_18, opcode_19, opcode_1a, opcode_1b, opcode_1c, opcode_1d, opcode_1e, opcode_1f,
	opcode_20, opcode_21, opcode_22, opcode_23, opcode_24, opcode_25, opcode_26, opcode_27, opcode_28, opcode_29, opcode_2a, opcode_2b, opcode_2c, opcode_2d, opcode_2e, opcode_2f,
	opcode_30, opcode_31, opcode_32, opcode_33, opcode_34, opcode_35, opcode_36, opcode_37, opcode_38, opcode_39, opcode_3a, opcode_3b, opcode_3c, opcode_3d, opcode_3e, opcode_3f,
	opcode_40, opcode_41, opcode_42, opcode_43, opcode_44, opcode_45, opcode_46, opcode_47, opcode_48, opcode_49, opcode_4a, opcode_4b, opcode_4c, opcode_4d, opcode_4e, opcode_4f,
	opcode_50, opcode_51, opcode_52, opcode_53, opcode_54, opcode_55, opcode_56, opcode_57, opcode_58, opcode_59, opcode_5a, opcode_5b, opcode_5c, opcode_5d, opcode_5e, opcode_5f,
	opcode_60, opcode_61, opcode_62, opcode_63, opcode_64, opcode_65, opcode_66, opcode_67, opcode_68, opcode_69, opcode_6a, opcode_6b, opcode_6c, opcode_6d, opcode_6e, opcode_6f,
	opcode_70, opcode_71, opcode_72, opcode_73, opcode_74, opcode_75, opcode_76, opcode_77, opcode_78, opcode_79, opcode_7a, opcode_7b, opcode_7c, opcode_7d, opcode_7e, opcode_7f,
	opcode_80, opcode_81, opcode_82, opcode_83, opcode_84, opcode_85, opcode_86, opcode_87, opcode_88, opcode_89, opcode_8a, opcode_8b, opcode_8c, opcode_8d, opcode_8e, opcode_8f,
	opcode_90, opcode_91, opcode_92, opcode_93, opcode_94, opcode_95, opcode_96, opcode_97, opcode_98, opcode_99, opcode_9a, opcode_9b, opcode_9c, opcode_9d, opcode_9e, opcode_9f,
	opcode_a0, opcode_a1, opcode_a2, opcode_a3, opcode_a4, opcode_a5, opcode_a6, opcode_a7, opcode_a8, opcode_a9, opcode_aa, opcode_ab, opcode_ac, opcode_ad, opcode_ae, opcode_af,
	opcode_b0, opcode_b1, opcode_b2, opcode_b3, opcode_b4, opcode_b5, opcode_b6, opcode_b7, opcode_b8, opcode_b9, opcode_ba, opcode_bb, opcode_bc, opcode_bd, opcode_be, opcode_bf,
	opcode_c0, opcode_c1, opcode_c2, opcode_c3, opcode_c4, opcode_c5, opcode_c6, opcode_c7, opcode_c8, opcode_c9, opcode_ca, opcode_cb, opcode_cc, opcode_cd, opcode_ce, opcode_cf,
	opcode_d0, opcode_d1, opcode_d2, opcode_d3, opcode_d4, opcode_d5, opcode_d6, opcode_d7, opcode_d8, opcode_d9, opcode_da, opcode_db, opcode_dc, opcode_dd, opcode_de, opcode_df,
	opcode_e0, opcode_e1, opcode_e2, opcode_e3, opcode_e4, opcode_e5, opcode_e6, opcode_e7, opcode_e8, opcode_e9, opcode_ea, opcode_eb, opcode_ec, opcode_ed, opcode_ee, opcode_ef,
	opcode_f0, opcode_f1, opcode_f2, opcode_f3, opcode_f4, opcode_f5, opcode_f6, opcode_f7, opcode_f8, opcode_f9, opcode_fa, opcode_fb, opcode_fc, opcode_fd, opcode_fe, opcode_ff,
];

#[allow(dead_code)]
static OPCODES_CB: [OpcodeFn; 256] = [
	opcode_cb00, opcode_cb01, opcode_cb02, opcode_cb03, opcode_cb04, opcode_cb05, opcode_cb06, opcode_cb07, opcode_cb08, opcode_cb09, opcode_cb0a, opcode_cb0b, opcode_cb0c, opcode_cb0d, opcode_cb0e, opcode_cb0f,
	opcode_cb10, opcode_cb11, opcode_cb12, opcode_cb13, opcode_cb14, opcode_cb15, opcode_cb16, opcode_cb17, opcode_cb18, opcode_cb19, opcode_cb1a, opcode_cb1b, opcode_cb1c, opcode_cb1d, opcode_cb1e, opcode_cb1f,
	opcode_cb20, opcode_cb21, opcode_cb22, opcode_cb23, opcode_cb24, opcode_cb25, opcode_cb26, opcode_cb27, opcode_cb28, opcode_cb29, opcode_cb2a, opcode_cb2b, opcode_cb2c, opcode_cb2d, opcode_cb2e, opcode_cb2f,
	opcode_cb30, opcode_cb31, opcode_cb32, opcode_cb33, opcode_cb34, opcode_cb35, opcode_cb36, opcode_cb37, opcode_cb38, opcode_cb39, opcode_cb3a, opcode_cb3b, opcode_cb3c, opcode_cb3d, opcode_cb3e, opcode_cb3f,
	opcode_cb40, opcode_cb41, opcode_cb42, opcode_cb43, opcode_cb44, opcode_cb45, opcode_cb46, opcode_cb47, opcode_cb48, opcode_cb49, opcode_cb4a, opcode_cb4b, opcode_cb4c, opcode_cb4d, opcode_cb4e, opcode_cb4f,
	opcode_cb50, opcode_cb51, opcode_cb52, opcode_cb53, opcode_cb54, opcode_cb55, opcode_cb56, opcode_cb57, opcode_cb58, opcode_cb59, opcode_cb5a, opcode_cb5b, opcode_cb5c, opcode_cb5d, opcode_cb5e, opcode_cb5f,
	opcode_cb60, opcode_cb61, opcode_cb62, opcode_cb63, opcode_cb64, opcode_cb65, opcode_cb66, opcode_cb67, opcode_cb68, opcode_cb69, opcode_cb6a, opcode_cb6b, opcode_cb6c, opcode_cb6d, opcode_cb6e, opcode_cb6f,
	opcode_cb70, opcode_cb71, opcode_cb72, opcode_cb73, opcode_cb74, opcode_cb75, opcode_cb76, opcode_cb77, opcode_cb78, opcode_cb79, opcode_cb7a, opcode_cb7b, opcode_cb7c, opcode_cb7d, opcode_cb7e, opcode_cb7f,
	opcode_cb80, opcode_cb81, opcode_cb82, opcode_cb83, opcode_cb84, opcode_cb85, opcode_cb86, opcode_cb87, opcode_cb88, opcode_cb89, opcode_cb8a, opcode_cb8b, opcode_cb8c, opcode_cb8d, opcode_cb8e, opcode_cb8f,
	opcode_cb90, opcode_cb91, opcode_cb92, opcode_cb93, opcode_cb94, opcode_cb95, opcode_cb96, opcode_cb97, opcode_cb98, opcode_cb99, opcode_cb9a, opcode_cb9b, opcode_cb9c, opcode_cb9d, opcode_cb9e, opcode_cb9f,
	opcode_cba0, opcode_cba1, opcode_cba2, opcode_cba3, opcode_cba4, opcode_cba5, opcode_cba6, opcode_cba7, opcode_cba8, opcode_cba9, opcode_cbaa, opcode_cbab, opcode_cbac, opcode_cbad, opcode_cbae, opcode_cbaf,
	opcode_cbb0, opcode_cbb1, opcode_cbb2, opcode_cbb3, opcode_cbb4, opcode_cbb5, opcode_cbb6, opcode_cbb7, opcode_cbb8, opcode_cbb9, opcode_cbba, opcode_cbbb, opcode_cbbc, opcode_cbbd, opcode_cbbe, opcode_cbbf,
	opcode_cbc0, opcode_cbc1, opcode_cbc2, opcode_cbc3, opcode_cbc4, opcode_cbc5, opcode_cbc6, opcode_cbc7, opcode_cbc8, opcode_cbc9, opcode_cbca, opcode_cbcb, opcode_cbcc, opcode_cbcd, opcode_cbce, opcode_cbcf,
	opcode_cbd0, opcode_cbd1, opcode_cbd2, opcode_cbd3, opcode_cbd4, opcode_cbd5, opcode_cbd6, opcode_cbd7, opcode_cbd8, opcode_cbd9, opcode_cbda, opcode_cbdb, opcode_cbdc, opcode_cbdd, opcode_cbde, opcode_cbdf,
	opcode_cbe0, opcode_cbe1, opcode_cbe2, opcode_cbe3, opcode_cbe4, opcode_cbe5, opcode_cbe6, opcode_cbe7, opcode_cbe8, opcode_cbe9, opcode_cbea, opcode_cbeb, opcode_cbec, opcode_cbed, opcode_cbee, opcode_cbef,
	opcode_cbf0, opcode_cbf1, opcode_cbf2, opcode_cbf3, opcode_cbf4, opcode_cbf5, opcode_cbf6, opcode_cbf7, opcode_cbf8, opcode_cbf9, opcode_cbfa, opcode_cbfb, opcode_cbfc, opcode_cbfd, opcode_cbfe, opcode_cbff,
];


