use crate::pfc_reg::PfcReadWrite;
use tock_registers::{register_structs, registers::ReadWrite};

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_ModeSel_RegisterBlock {
        (0x000 => pub PFC_MODESEL0: PfcReadWrite<u32>),
        (0x004 => pub PFC_MODESEL1: PfcReadWrite<u32>),
        (0x008 => pub PFC_MODESEL2: PfcReadWrite<u32>),
        (0x00C => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_Ipsr_RegisterBlock {
        (0x000 => pub PFC_IPSR0: PfcReadWrite<u32>),
        (0x004 => pub PFC_IPSR1: PfcReadWrite<u32>),
        (0x008 => pub PFC_IPSR2: PfcReadWrite<u32>),
        (0x00C => pub PFC_IPSR3: PfcReadWrite<u32>),
        (0x010 => pub PFC_IPSR4: PfcReadWrite<u32>),
        (0x014 => pub PFC_IPSR5: PfcReadWrite<u32>),
        (0x018 => pub PFC_IPSR6: PfcReadWrite<u32>),
        (0x01C => pub PFC_IPSR7: PfcReadWrite<u32>),
        (0x020 => pub PFC_IPSR8: PfcReadWrite<u32>),
        (0x024 => pub PFC_IPSR9: PfcReadWrite<u32>),
        (0x028 => pub PFC_IPSR10: PfcReadWrite<u32>),
        (0x02C => pub PFC_IPSR11: PfcReadWrite<u32>),
        (0x030 => pub PFC_IPSR12: PfcReadWrite<u32>),
        (0x034 => pub PFC_IPSR13: PfcReadWrite<u32>),
        (0x038 => pub PFC_IPSR14: PfcReadWrite<u32>),
        (0x03C => pub PFC_IPSR15: PfcReadWrite<u32>),
        (0x040 => pub PFC_IPSR16: PfcReadWrite<u32>),
        (0x044 => pub PFC_IPSR17: PfcReadWrite<u32>),
        (0x048 => pub PFC_IPSR18: PfcReadWrite<u32>),
        (0x04C => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_Gpsr_RegisterBlock {
        (0x000 => pub PFC_GPSR0: PfcReadWrite<u32>),
        (0x004 => pub PFC_GPSR1: PfcReadWrite<u32>),
        (0x008 => pub PFC_GPSR2: PfcReadWrite<u32>),
        (0x00C => pub PFC_GPSR3: PfcReadWrite<u32>),
        (0x010 => pub PFC_GPSR4: PfcReadWrite<u32>),
        (0x014 => pub PFC_GPSR5: PfcReadWrite<u32>),
        (0x018 => pub PFC_GPSR6: PfcReadWrite<u32>),
        (0x01C => pub PFC_GPSR7: PfcReadWrite<u32>),
        (0x020 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_Poc_Ctrl_RegisterBlock {
        (0x000 => pub PFC_POCCTRL0: PfcReadWrite<u32>),
        (0x004 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_DrvCtrl_RegisterBlock {
        (0x000 => pub PFC_DRVCTRL0: PfcReadWrite<u32>),
        (0x004 => pub PFC_DRVCTRL1: PfcReadWrite<u32>),
        (0x008 => pub PFC_DRVCTRL2: PfcReadWrite<u32>),
        (0x00C => pub PFC_DRVCTRL3: PfcReadWrite<u32>),
        (0x010 => pub PFC_DRVCTRL4: PfcReadWrite<u32>),
        (0x014 => pub PFC_DRVCTRL5: PfcReadWrite<u32>),
        (0x018 => pub PFC_DRVCTRL6: PfcReadWrite<u32>),
        (0x01C => pub PFC_DRVCTRL7: PfcReadWrite<u32>),
        (0x020 => pub PFC_DRVCTRL8: PfcReadWrite<u32>),
        (0x024 => pub PFC_DRVCTRL9: PfcReadWrite<u32>),
        (0x028 => pub PFC_DRVCTRL10: PfcReadWrite<u32>),
        (0x02C => pub PFC_DRVCTRL11: PfcReadWrite<u32>),
        (0x030 => pub PFC_DRVCTRL12: PfcReadWrite<u32>),
        (0x034 => pub PFC_DRVCTRL13: PfcReadWrite<u32>),
        (0x038 => pub PFC_DRVCTRL14: PfcReadWrite<u32>),
        (0x03C => pub PFC_DRVCTRL15: PfcReadWrite<u32>),
        (0x040 => pub PFC_DRVCTRL16: PfcReadWrite<u32>),
        (0x044 => pub PFC_DRVCTRL17: PfcReadWrite<u32>),
        (0x048 => pub PFC_DRVCTRL18: PfcReadWrite<u32>),
        (0x04C => pub PFC_DRVCTRL19: PfcReadWrite<u32>),
        (0x050 => pub PFC_DRVCTRL20: PfcReadWrite<u32>),
        (0x054 => pub PFC_DRVCTRL21: PfcReadWrite<u32>),
        (0x058 => pub PFC_DRVCTRL22: PfcReadWrite<u32>),
        (0x05C => pub PFC_DRVCTRL23: PfcReadWrite<u32>),
        (0x060 => pub PFC_DRVCTRL24: PfcReadWrite<u32>),
        (0x064 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_Pud_RegisterBlock {
        (0x000 => pub PFC_PUD0: PfcReadWrite<u32>),
        (0x004 => pub PFC_PUD1: PfcReadWrite<u32>),
        (0x008 => pub PFC_PUD2: PfcReadWrite<u32>),
        (0x00C => pub PFC_PUD3: PfcReadWrite<u32>),
        (0x010 => pub PFC_PUD4: PfcReadWrite<u32>),
        (0x014 => pub PFC_PUD5: PfcReadWrite<u32>),
        (0x018 => pub PFC_PUD6: PfcReadWrite<u32>),
        (0x01C => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub Pfc_PuEn_RegisterBlock {
        (0x000 => pub PFC_PUEN0: PfcReadWrite<u32>),
        (0x004 => pub PFC_PUEN1: PfcReadWrite<u32>),
        (0x008 => pub PFC_PUEN2: PfcReadWrite<u32>),
        (0x00C => pub PFC_PUEN3: PfcReadWrite<u32>),
        (0x010 => pub PFC_PUEN4: PfcReadWrite<u32>),
        (0x014 => pub PFC_PUEN5: PfcReadWrite<u32>),
        (0x018 => pub PFC_PUEN6: PfcReadWrite<u32>),
        (0x01C => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub GPIO_RegisterBlock {
        (0x0000 => pub GPIO_IOINTSEL0: ReadWrite<u32>),
        (0x0004 => pub GPIO_INOUTSEL0: ReadWrite<u32>),
        (0x0008 => pub GPIO_OUTDT0: ReadWrite<u32>),
        (0x0020 => pub GPIO_POSNEG0: ReadWrite<u32>),
        (0x1000 => pub GPIO_IOINTSEL1: ReadWrite<u32>),
        (0x1004 => pub GPIO_INOUTSEL1: ReadWrite<u32>),
        (0x1008 => pub GPIO_OUTDT1: ReadWrite<u32>),
        (0x1020 => pub GPIO_POSNEG1: ReadWrite<u32>),
        (0x2000 => pub GPIO_IOINTSEL2: ReadWrite<u32>),
        (0x2004 => pub GPIO_INOUTSEL2: ReadWrite<u32>),
        (0x2008 => pub GPIO_OUTDT2: ReadWrite<u32>),
        (0x2020 => pub GPIO_POSNEG2: ReadWrite<u32>),
        (0x3000 => pub GPIO_IOINTSEL3: ReadWrite<u32>),
        (0x3004 => pub GPIO_INOUTSEL3: ReadWrite<u32>),
        (0x3008 => pub GPIO_OUTDT3: ReadWrite<u32>),
        (0x3020 => pub GPIO_POSNEG3: ReadWrite<u32>),
        (0x4000 => pub GPIO_IOINTSEL4: ReadWrite<u32>),
        (0x4004 => pub GPIO_INOUTSEL4: ReadWrite<u32>),
        (0x4008 => pub GPIO_OUTDT4: ReadWrite<u32>),
        (0x4020 => pub GPIO_POSNEG4: ReadWrite<u32>),
        (0x5000 => pub GPIO_IOINTSEL5: ReadWrite<u32>),
        (0x5004 => pub GPIO_INOUTSEL5: ReadWrite<u32>),
        (0x5008 => pub GPIO_OUTDT5: ReadWrite<u32>),
        (0x5020 => pub GPIO_POSNEG5: ReadWrite<u32>),
        (0x5400 => pub GPIO_IOINTSEL6: ReadWrite<u32>),
        (0x5404 => pub GPIO_INOUTSEL6: ReadWrite<u32>),
        (0x5408 => pub GPIO_OUTDT6: ReadWrite<u32>),
        (0x5420 => pub GPIO_POSNEG6: ReadWrite<u32>),
        (0x5804 => pub GPIO_INOUTSEL7: ReadWrite<u32>),
        (0x5808 => pub GPIO_OUTDT7: ReadWrite<u32>),
        (0x580C => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub GPIO_Outdt_RegisterBlock {
        (0x020 => @END),
    }
}

register_structs! {
    #[allow(non_snake_case)]
    pub GPIO_InOutSel_RegisterBlock {
        (0x020 => @END),
    }
}
