/*
    Copyright (c) 2020 Sipeed Co.,Ltd.
    bl602-pac is licensed under Mulan PSL v2.
    You can use this software according to the terms and conditions of the Mulan PSL v2.
    You may obtain a copy of Mulan PSL v2 at:

        http://license.coscl.org.cn/MulanPSL2

    THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
    EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
    MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
    See the Mulan PSL v2 for more details.
*/
/* 
    memory-bl602.x: declares all memory regions in BL602 SoC.
    This .x file is not intended to use alone. You should use together with 
    the riscv-rust's `riscv-rt` universal RISC-V bare-metal runtime for Rust.

    To use this file, you may add it to `cargo/config.toml` like `-Tmemory-bl602.x`.
    Or include with `INCLUDE memory-bl602.x` then define your own memory configuration,
    e.g. run program in RAM or other possible values.
*/

/* Ref: bl602/bl602/evb/ld/flash.ld */

_max_hart_id = 0;

MEMORY
{
    ROM   (rxai!w) : ORIGIN = 0x21015000, LENGTH = 44K
    FLASH (rxai!w) : ORIGIN = 0x23000000, LENGTH = 4M
    /* put itcm, dtcm and OCRAM together */
    RAM_TCM  (wxa) : ORIGIN = 0x4200C000, LENGTH = (16K + 16K + 48K + 64K)
    /* leave 8K left for BLE */
    RAM_WIFI (wxa) : ORIGIN = 0x42030000, LENGTH = 104K
}

REGION_ALIAS("REGION_TEXT", FLASH);
REGION_ALIAS("REGION_RODATA", FLASH);
REGION_ALIAS("REGION_DATA", RAM_TCM);
REGION_ALIAS("REGION_BSS", RAM_TCM);
REGION_ALIAS("REGION_HEAP", RAM_TCM);
REGION_ALIAS("REGION_STACK", RAM_TCM);
