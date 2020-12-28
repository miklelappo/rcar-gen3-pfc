#![no_std]

mod pfc_reg;
mod reg_def;
pub mod salvator;

pub struct RCarPortConfig {
    pub pfc_mod_sel: [u32; 3],
    pub pfc_ipsr: [u32; 19],
    pub pfc_gpsr: [u32; 8],
    pub pfc_pocctrl: [u32; 1],
    pub pfc_drvctrl: [u32; 25],
    pub pfc_pud: [u32; 7],
    pub pfc_puen: [u32; 7],
}


impl RCarPortConfig {
    pub fn apply(&self) {
        unsafe {
            self.init_modesel();
            self.init_ipsr();
            self.init_gpsr();
            self.init_pocctrl();
            self.init_drvctrl();
            self.init_pud();
            self.init_puen();
        }
    }

    unsafe fn init_modesel(&self) {
        let modesel_regs = 0xE606_0500 as *const reg_def::Pfc_Mode_Sel_RegisterBlock;
        (*modesel_regs).PFC_MODE_SEL0.set(self.pfc_mod_sel[0]);
        (*modesel_regs).PFC_MODE_SEL1.set(self.pfc_mod_sel[1]);
        (*modesel_regs).PFC_MODE_SEL2.set(self.pfc_mod_sel[2]);
    }

    unsafe fn init_ipsr(&self) {
        let ipsr_regs = 0xE606_0200 as *const reg_def::Pfc_Ipsr_RegisterBlock;
        (*ipsr_regs).PFC_IPSR0.set(self.pfc_ipsr[0]);
        (*ipsr_regs).PFC_IPSR1.set(self.pfc_ipsr[1]);
        (*ipsr_regs).PFC_IPSR2.set(self.pfc_ipsr[2]);
        (*ipsr_regs).PFC_IPSR3.set(self.pfc_ipsr[3]);
        (*ipsr_regs).PFC_IPSR4.set(self.pfc_ipsr[4]);
        (*ipsr_regs).PFC_IPSR5.set(self.pfc_ipsr[5]);
        (*ipsr_regs).PFC_IPSR6.set(self.pfc_ipsr[6]);
        (*ipsr_regs).PFC_IPSR7.set(self.pfc_ipsr[7]);
        (*ipsr_regs).PFC_IPSR8.set(self.pfc_ipsr[8]);
        (*ipsr_regs).PFC_IPSR9.set(self.pfc_ipsr[9]);
        (*ipsr_regs).PFC_IPSR10.set(self.pfc_ipsr[10]);
        (*ipsr_regs).PFC_IPSR11.set(self.pfc_ipsr[11]);
        (*ipsr_regs).PFC_IPSR12.set(self.pfc_ipsr[12]);
        (*ipsr_regs).PFC_IPSR13.set(self.pfc_ipsr[13]);
        (*ipsr_regs).PFC_IPSR14.set(self.pfc_ipsr[14]);
        (*ipsr_regs).PFC_IPSR15.set(self.pfc_ipsr[15]);
        (*ipsr_regs).PFC_IPSR16.set(self.pfc_ipsr[16]);
        (*ipsr_regs).PFC_IPSR17.set(self.pfc_ipsr[17]);
        (*ipsr_regs).PFC_IPSR18.set(self.pfc_ipsr[18]);
    }

    unsafe fn init_gpsr(&self) {
        let gpsr_regs = 0xE6060100 as *const reg_def::Pfc_Gpsr_RegisterBlock;
        (*gpsr_regs).PFC_GPSR0.set(self.pfc_gpsr[0]);
        (*gpsr_regs).PFC_GPSR1.set(self.pfc_gpsr[1]);
        (*gpsr_regs).PFC_GPSR2.set(self.pfc_gpsr[2]);
        (*gpsr_regs).PFC_GPSR3.set(self.pfc_gpsr[3]);
        (*gpsr_regs).PFC_GPSR4.set(self.pfc_gpsr[4]);
        (*gpsr_regs).PFC_GPSR5.set(self.pfc_gpsr[5]);
        (*gpsr_regs).PFC_GPSR6.set(self.pfc_gpsr[6]);
        (*gpsr_regs).PFC_GPSR7.set(self.pfc_gpsr[7]);
    }

    unsafe fn init_pocctrl(&self) {
        let pocctrl_regs = 0xE6060100 as *const reg_def::Pfc_Poc_Ctrl_RegisterBlock;
        (*pocctrl_regs).PFC_POCCTRL0.set(self.pfc_pocctrl[0]);
    }

    unsafe fn init_drvctrl(&self) {
        let drvctrl_regs = 0xE606_0300 as *const reg_def::Pfc_Drv_Ctrl_RegisterBlock;
        (*drvctrl_regs).PFC_DRVCTRL0.set(self.pfc_drvctrl[0]);
        (*drvctrl_regs).PFC_DRVCTRL1.set(self.pfc_drvctrl[1]);
        (*drvctrl_regs).PFC_DRVCTRL2.set(self.pfc_drvctrl[2]);
        (*drvctrl_regs).PFC_DRVCTRL3.set(self.pfc_drvctrl[3]);
        (*drvctrl_regs).PFC_DRVCTRL4.set(self.pfc_drvctrl[4]);
        (*drvctrl_regs).PFC_DRVCTRL5.set(self.pfc_drvctrl[5]);
        (*drvctrl_regs).PFC_DRVCTRL6.set(self.pfc_drvctrl[6]);
        (*drvctrl_regs).PFC_DRVCTRL7.set(self.pfc_drvctrl[7]);
        (*drvctrl_regs).PFC_DRVCTRL8.set(self.pfc_drvctrl[8]);
        (*drvctrl_regs).PFC_DRVCTRL9.set(self.pfc_drvctrl[9]);
        (*drvctrl_regs).PFC_DRVCTRL10.set(self.pfc_drvctrl[10]);
        (*drvctrl_regs).PFC_DRVCTRL11.set(self.pfc_drvctrl[11]);
        (*drvctrl_regs).PFC_DRVCTRL12.set(self.pfc_drvctrl[12]);
        (*drvctrl_regs).PFC_DRVCTRL13.set(self.pfc_drvctrl[13]);
        (*drvctrl_regs).PFC_DRVCTRL14.set(self.pfc_drvctrl[14]);
        (*drvctrl_regs).PFC_DRVCTRL15.set(self.pfc_drvctrl[15]);
        (*drvctrl_regs).PFC_DRVCTRL16.set(self.pfc_drvctrl[16]);
        (*drvctrl_regs).PFC_DRVCTRL17.set(self.pfc_drvctrl[17]);
        (*drvctrl_regs).PFC_DRVCTRL18.set(self.pfc_drvctrl[18]);
        (*drvctrl_regs).PFC_DRVCTRL19.set(self.pfc_drvctrl[19]);
        (*drvctrl_regs).PFC_DRVCTRL20.set(self.pfc_drvctrl[20]);
        (*drvctrl_regs).PFC_DRVCTRL21.set(self.pfc_drvctrl[21]);
        (*drvctrl_regs).PFC_DRVCTRL22.set(self.pfc_drvctrl[22]);
        (*drvctrl_regs).PFC_DRVCTRL23.set(self.pfc_drvctrl[23]);
        (*drvctrl_regs).PFC_DRVCTRL24.set(self.pfc_drvctrl[24]);
    }

    unsafe fn init_pud(&self) {
        let pud_regs = 0xE606_0440 as *const reg_def::Pfc_Pud_RegisterBlock;
        (*pud_regs).PFC_PUD0.set(self.pfc_pud[0]);
        (*pud_regs).PFC_PUD1.set(self.pfc_pud[1]);
        (*pud_regs).PFC_PUD2.set(self.pfc_pud[2]);
        (*pud_regs).PFC_PUD3.set(self.pfc_pud[3]);
        (*pud_regs).PFC_PUD4.set(self.pfc_pud[4]);
        (*pud_regs).PFC_PUD5.set(self.pfc_pud[5]);
        (*pud_regs).PFC_PUD6.set(self.pfc_pud[6]);
    }

    unsafe fn init_puen(&self) {
        let puen_regs = 0xE606_0400 as *const reg_def::Pfc_Puen_RegisterBlock;
        (*puen_regs).PFC_PUEN0.set(self.pfc_puen[0]);
        (*puen_regs).PFC_PUEN1.set(self.pfc_puen[1]);
        (*puen_regs).PFC_PUEN2.set(self.pfc_puen[2]);
        (*puen_regs).PFC_PUEN3.set(self.pfc_puen[3]);
        (*puen_regs).PFC_PUEN4.set(self.pfc_puen[4]);
        (*puen_regs).PFC_PUEN5.set(self.pfc_puen[5]);
        (*puen_regs).PFC_PUEN6.set(self.pfc_puen[6]);
    }
}

