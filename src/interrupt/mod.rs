use bare_metal::Nr;
#[cfg(all(target_arch = "arm", feature = "rt"))]
global_asm ! ( "\n                    .thumb_func\n                    DH_TRAMPOLINE:\n                        b DEFAULT_HANDLER\n                    " ) ;
#[doc = r" Hack to compile on x86"]
#[cfg(all(target_arch = "x86_64", feature = "rt"))]
global_asm ! ( "\n                    DH_TRAMPOLINE:\n                        jmp DEFAULT_HANDLER\n                    " ) ;
#[cfg(feature = "rt")]
global_asm ! ( "\n.weak WWDG\nWWDG = DH_TRAMPOLINE\n.weak PVD\nPVD = DH_TRAMPOLINE\n.weak TAMP_STAMP\nTAMP_STAMP = DH_TRAMPOLINE\n.weak RTC_WKUP\nRTC_WKUP = DH_TRAMPOLINE\n.weak FLASH\nFLASH = DH_TRAMPOLINE\n.weak RCC\nRCC = DH_TRAMPOLINE\n.weak EXTI0\nEXTI0 = DH_TRAMPOLINE\n.weak EXTI1\nEXTI1 = DH_TRAMPOLINE\n.weak EXTI2_TSC\nEXTI2_TSC = DH_TRAMPOLINE\n.weak EXTI3\nEXTI3 = DH_TRAMPOLINE\n.weak EXTI4\nEXTI4 = DH_TRAMPOLINE\n.weak DMA1_CH1\nDMA1_CH1 = DH_TRAMPOLINE\n.weak DMA1_CH2\nDMA1_CH2 = DH_TRAMPOLINE\n.weak DMA1_CH3\nDMA1_CH3 = DH_TRAMPOLINE\n.weak DMA1_CH4\nDMA1_CH4 = DH_TRAMPOLINE\n.weak DMA1_CH5\nDMA1_CH5 = DH_TRAMPOLINE\n.weak DMA1_CH6\nDMA1_CH6 = DH_TRAMPOLINE\n.weak DMA1_CH7\nDMA1_CH7 = DH_TRAMPOLINE\n.weak ADC1_2\nADC1_2 = DH_TRAMPOLINE\n.weak USB_HP_CAN_TX\nUSB_HP_CAN_TX = DH_TRAMPOLINE\n.weak USB_LP_CAN_RX0\nUSB_LP_CAN_RX0 = DH_TRAMPOLINE\n.weak CAN_RX1\nCAN_RX1 = DH_TRAMPOLINE\n.weak CAN_SCE\nCAN_SCE = DH_TRAMPOLINE\n.weak EXTI9_5\nEXTI9_5 = DH_TRAMPOLINE\n.weak TIM1_BRK_TIM15\nTIM1_BRK_TIM15 = DH_TRAMPOLINE\n.weak TIM1_UP_TIM16\nTIM1_UP_TIM16 = DH_TRAMPOLINE\n.weak TIM1_TRG_COM_TIM17\nTIM1_TRG_COM_TIM17 = DH_TRAMPOLINE\n.weak TIM1_CC\nTIM1_CC = DH_TRAMPOLINE\n.weak TIM2\nTIM2 = DH_TRAMPOLINE\n.weak TIM3\nTIM3 = DH_TRAMPOLINE\n.weak TIM4\nTIM4 = DH_TRAMPOLINE\n.weak I2C1_EV_EXTI23\nI2C1_EV_EXTI23 = DH_TRAMPOLINE\n.weak I2C1_ER\nI2C1_ER = DH_TRAMPOLINE\n.weak I2C2_EV_EXTI24\nI2C2_EV_EXTI24 = DH_TRAMPOLINE\n.weak I2C2_ER\nI2C2_ER = DH_TRAMPOLINE\n.weak SPI1\nSPI1 = DH_TRAMPOLINE\n.weak SPI2\nSPI2 = DH_TRAMPOLINE\n.weak USART1_EXTI25\nUSART1_EXTI25 = DH_TRAMPOLINE\n.weak USART2_EXTI26\nUSART2_EXTI26 = DH_TRAMPOLINE\n.weak USART3_EXTI28\nUSART3_EXTI28 = DH_TRAMPOLINE\n.weak EXTI15_10\nEXTI15_10 = DH_TRAMPOLINE\n.weak RTCALARM\nRTCALARM = DH_TRAMPOLINE\n.weak USB_WKUP\nUSB_WKUP = DH_TRAMPOLINE\n.weak TIM8_BRK\nTIM8_BRK = DH_TRAMPOLINE\n.weak TIM8_UP\nTIM8_UP = DH_TRAMPOLINE\n.weak TIM8_TRG_COM\nTIM8_TRG_COM = DH_TRAMPOLINE\n.weak TIM8_CC\nTIM8_CC = DH_TRAMPOLINE\n.weak ADC3\nADC3 = DH_TRAMPOLINE\n.weak SPI3\nSPI3 = DH_TRAMPOLINE\n.weak UART4_EXTI34\nUART4_EXTI34 = DH_TRAMPOLINE\n.weak UART5_EXTI35\nUART5_EXTI35 = DH_TRAMPOLINE\n.weak TIM6_DACUNDER\nTIM6_DACUNDER = DH_TRAMPOLINE\n.weak TIM7\nTIM7 = DH_TRAMPOLINE\n.weak DMA2_CH1\nDMA2_CH1 = DH_TRAMPOLINE\n.weak DMA2_CH2\nDMA2_CH2 = DH_TRAMPOLINE\n.weak DMA2_CH3\nDMA2_CH3 = DH_TRAMPOLINE\n.weak DMA2_CH4\nDMA2_CH4 = DH_TRAMPOLINE\n.weak DMA2_CH5\nDMA2_CH5 = DH_TRAMPOLINE\n.weak ADC4\nADC4 = DH_TRAMPOLINE\n.weak COMP123\nCOMP123 = DH_TRAMPOLINE\n.weak COMP456\nCOMP456 = DH_TRAMPOLINE\n.weak COMP7\nCOMP7 = DH_TRAMPOLINE\n.weak USB_HP\nUSB_HP = DH_TRAMPOLINE\n.weak USB_LP\nUSB_LP = DH_TRAMPOLINE\n.weak USB_WKUP_EXTI\nUSB_WKUP_EXTI = DH_TRAMPOLINE\n.weak FPU\nFPU = DH_TRAMPOLINE" ) ;
#[cfg(feature = "rt")]
extern "C" {
    fn WWDG();
    fn PVD();
    fn TAMP_STAMP();
    fn RTC_WKUP();
    fn FLASH();
    fn RCC();
    fn EXTI0();
    fn EXTI1();
    fn EXTI2_TSC();
    fn EXTI3();
    fn EXTI4();
    fn DMA1_CH1();
    fn DMA1_CH2();
    fn DMA1_CH3();
    fn DMA1_CH4();
    fn DMA1_CH5();
    fn DMA1_CH6();
    fn DMA1_CH7();
    fn ADC1_2();
    fn USB_HP_CAN_TX();
    fn USB_LP_CAN_RX0();
    fn CAN_RX1();
    fn CAN_SCE();
    fn EXTI9_5();
    fn TIM1_BRK_TIM15();
    fn TIM1_UP_TIM16();
    fn TIM1_TRG_COM_TIM17();
    fn TIM1_CC();
    fn TIM2();
    fn TIM3();
    fn TIM4();
    fn I2C1_EV_EXTI23();
    fn I2C1_ER();
    fn I2C2_EV_EXTI24();
    fn I2C2_ER();
    fn SPI1();
    fn SPI2();
    fn USART1_EXTI25();
    fn USART2_EXTI26();
    fn USART3_EXTI28();
    fn EXTI15_10();
    fn RTCALARM();
    fn USB_WKUP();
    fn TIM8_BRK();
    fn TIM8_UP();
    fn TIM8_TRG_COM();
    fn TIM8_CC();
    fn ADC3();
    fn SPI3();
    fn UART4_EXTI34();
    fn UART5_EXTI35();
    fn TIM6_DACUNDER();
    fn TIM7();
    fn DMA2_CH1();
    fn DMA2_CH2();
    fn DMA2_CH3();
    fn DMA2_CH4();
    fn DMA2_CH5();
    fn ADC4();
    fn COMP123();
    fn COMP456();
    fn COMP7();
    fn USB_HP();
    fn USB_LP();
    fn USB_WKUP_EXTI();
    fn FPU();
}
#[allow(private_no_mangle_statics)]
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
#[used]
pub static INTERRUPTS: [Option<unsafe extern "C" fn()>; 82] = [
    Some(WWDG),
    Some(PVD),
    Some(TAMP_STAMP),
    Some(RTC_WKUP),
    Some(FLASH),
    Some(RCC),
    Some(EXTI0),
    Some(EXTI1),
    Some(EXTI2_TSC),
    Some(EXTI3),
    Some(EXTI4),
    Some(DMA1_CH1),
    Some(DMA1_CH2),
    Some(DMA1_CH3),
    Some(DMA1_CH4),
    Some(DMA1_CH5),
    Some(DMA1_CH6),
    Some(DMA1_CH7),
    Some(ADC1_2),
    Some(USB_HP_CAN_TX),
    Some(USB_LP_CAN_RX0),
    Some(CAN_RX1),
    Some(CAN_SCE),
    Some(EXTI9_5),
    Some(TIM1_BRK_TIM15),
    Some(TIM1_UP_TIM16),
    Some(TIM1_TRG_COM_TIM17),
    Some(TIM1_CC),
    Some(TIM2),
    Some(TIM3),
    Some(TIM4),
    Some(I2C1_EV_EXTI23),
    Some(I2C1_ER),
    Some(I2C2_EV_EXTI24),
    Some(I2C2_ER),
    Some(SPI1),
    Some(SPI2),
    Some(USART1_EXTI25),
    Some(USART2_EXTI26),
    Some(USART3_EXTI28),
    Some(EXTI15_10),
    Some(RTCALARM),
    Some(USB_WKUP),
    Some(TIM8_BRK),
    Some(TIM8_UP),
    Some(TIM8_TRG_COM),
    Some(TIM8_CC),
    Some(ADC3),
    None,
    None,
    None,
    Some(SPI3),
    Some(UART4_EXTI34),
    Some(UART5_EXTI35),
    Some(TIM6_DACUNDER),
    Some(TIM7),
    Some(DMA2_CH1),
    Some(DMA2_CH2),
    Some(DMA2_CH3),
    Some(DMA2_CH4),
    Some(DMA2_CH5),
    Some(ADC4),
    None,
    None,
    Some(COMP123),
    Some(COMP456),
    Some(COMP7),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(USB_HP),
    Some(USB_LP),
    Some(USB_WKUP_EXTI),
    None,
    None,
    None,
    None,
    Some(FPU),
];
#[doc = r" Enumeration of all the interrupts"]
pub enum Interrupt {
    #[doc = "0 - Window Watchdog interrupt"]
    WWDG,
    #[doc = "1 - PVD through EXTI line detection interrupt"]
    PVD,
    #[doc = "2 - Tamper and TimeStamp interrupts"]
    TAMP_STAMP,
    #[doc = "3 - RTC Wakeup interrupt through the EXTI line"]
    RTC_WKUP,
    #[doc = "4 - Flash global interrupt"]
    FLASH,
    #[doc = "5 - RCC global interrupt"]
    RCC,
    #[doc = "6 - EXTI Line0 interrupt"]
    EXTI0,
    #[doc = "7 - EXTI Line3 interrupt"]
    EXTI1,
    #[doc = "8 - EXTI Line2 and Touch sensing interrupts"]
    EXTI2_TSC,
    #[doc = "9 - EXTI Line3 interrupt"]
    EXTI3,
    #[doc = "10 - EXTI Line4 interrupt"]
    EXTI4,
    #[doc = "11 - DMA1 channel 1 interrupt"]
    DMA1_CH1,
    #[doc = "12 - DMA1 channel 2 interrupt"]
    DMA1_CH2,
    #[doc = "13 - DMA1 channel 3 interrupt"]
    DMA1_CH3,
    #[doc = "14 - DMA1 channel 4 interrupt"]
    DMA1_CH4,
    #[doc = "15 - DMA1 channel 5 interrupt"]
    DMA1_CH5,
    #[doc = "16 - DMA1 channel 6 interrupt"]
    DMA1_CH6,
    #[doc = "17 - DMA1 channel 7interrupt"]
    DMA1_CH7,
    #[doc = "18 - ADC1 and ADC2 global interrupt"]
    ADC1_2,
    #[doc = "19 - USB High Priority/CAN_TX interrupts"]
    USB_HP_CAN_TX,
    #[doc = "20 - USB Low Priority/CAN_RX0 interrupts"]
    USB_LP_CAN_RX0,
    #[doc = "21 - CAN_RX1 interrupt"]
    CAN_RX1,
    #[doc = "22 - CAN_SCE interrupt"]
    CAN_SCE,
    #[doc = "23 - EXTI Line5 to Line9 interrupts"]
    EXTI9_5,
    #[doc = "24 - TIM1 Break/TIM15 global interruts"]
    TIM1_BRK_TIM15,
    #[doc = "25 - TIM1 Update/TIM16 global interrupts"]
    TIM1_UP_TIM16,
    #[doc = "26 - TIM1 trigger and commutation/TIM17 interrupts"]
    TIM1_TRG_COM_TIM17,
    #[doc = "27 - TIM1 capture compare interrupt"]
    TIM1_CC,
    #[doc = "28 - TIM2 global interrupt"]
    TIM2,
    #[doc = "29 - TIM3 global interrupt"]
    TIM3,
    #[doc = "30 - TIM4 global interrupt"]
    TIM4,
    #[doc = "31 - I2C1 event interrupt and EXTI Line23 interrupt"]
    I2C1_EV_EXTI23,
    #[doc = "32 - I2C1 error interrupt"]
    I2C1_ER,
    #[doc = "33 - I2C2 event interrupt & EXTI Line24 interrupt"]
    I2C2_EV_EXTI24,
    #[doc = "34 - I2C2 error interrupt"]
    I2C2_ER,
    #[doc = "35 - SPI1 global interrupt"]
    SPI1,
    #[doc = "36 - SPI2 global interrupt"]
    SPI2,
    #[doc = "37 - USART1 global interrupt and EXTI Line 25 interrupt"]
    USART1_EXTI25,
    #[doc = "38 - USART2 global interrupt and EXTI Line 26 interrupt"]
    USART2_EXTI26,
    #[doc = "39 - USART3 global interrupt and EXTI Line 28 interrupt"]
    USART3_EXTI28,
    #[doc = "40 - EXTI Line15 to Line10 interrupts"]
    EXTI15_10,
    #[doc = "41 - RTC alarm interrupt"]
    RTCALARM,
    #[doc = "42 - USB wakeup from Suspend"]
    USB_WKUP,
    #[doc = "43 - TIM8 break interrupt"]
    TIM8_BRK,
    #[doc = "44 - TIM8 update interrupt"]
    TIM8_UP,
    #[doc = "45 - TIM8 Trigger and commutation interrupts"]
    TIM8_TRG_COM,
    #[doc = "46 - TIM8 capture compare interrupt"]
    TIM8_CC,
    #[doc = "47 - ADC3 global interrupt"]
    ADC3,
    #[doc = "51 - SPI3 global interrupt"]
    SPI3,
    #[doc = "52 - UART4 global and EXTI Line 34 interrupts"]
    UART4_EXTI34,
    #[doc = "53 - UART5 global and EXTI Line 35 interrupts"]
    UART5_EXTI35,
    #[doc = "54 - TIM6 global and DAC12 underrun interrupts"]
    TIM6_DACUNDER,
    #[doc = "55 - TIM7 global interrupt"]
    TIM7,
    #[doc = "56 - DMA2 channel1 global interrupt"]
    DMA2_CH1,
    #[doc = "57 - DMA2 channel2 global interrupt"]
    DMA2_CH2,
    #[doc = "58 - DMA2 channel3 global interrupt"]
    DMA2_CH3,
    #[doc = "59 - DMA2 channel4 global interrupt"]
    DMA2_CH4,
    #[doc = "60 - DMA2 channel5 global interrupt"]
    DMA2_CH5,
    #[doc = "61 - ADC4 global interrupt"]
    ADC4,
    #[doc = "64 - COMP1 & COMP2 & COMP3 interrupts combined with EXTI Lines 21, 22 and 29 interrupts"]
    COMP123,
    #[doc = "65 - COMP4 & COMP5 & COMP6 interrupts combined with EXTI Lines 30, 31 and 32 interrupts"]
    COMP456,
    #[doc = "66 - COMP7 interrupt combined with EXTI Line 33 interrupt"]
    COMP7,
    #[doc = "74 - USB High priority interrupt"]
    USB_HP,
    #[doc = "75 - USB Low priority interrupt"]
    USB_LP,
    #[doc = "76 - USB wakeup from Suspend and EXTI Line 18"]
    USB_WKUP_EXTI,
    #[doc = "81 - Floating point interrupt"]
    FPU,
}
unsafe impl Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        match *self {
            Interrupt::WWDG => 0,
            Interrupt::PVD => 1,
            Interrupt::TAMP_STAMP => 2,
            Interrupt::RTC_WKUP => 3,
            Interrupt::FLASH => 4,
            Interrupt::RCC => 5,
            Interrupt::EXTI0 => 6,
            Interrupt::EXTI1 => 7,
            Interrupt::EXTI2_TSC => 8,
            Interrupt::EXTI3 => 9,
            Interrupt::EXTI4 => 10,
            Interrupt::DMA1_CH1 => 11,
            Interrupt::DMA1_CH2 => 12,
            Interrupt::DMA1_CH3 => 13,
            Interrupt::DMA1_CH4 => 14,
            Interrupt::DMA1_CH5 => 15,
            Interrupt::DMA1_CH6 => 16,
            Interrupt::DMA1_CH7 => 17,
            Interrupt::ADC1_2 => 18,
            Interrupt::USB_HP_CAN_TX => 19,
            Interrupt::USB_LP_CAN_RX0 => 20,
            Interrupt::CAN_RX1 => 21,
            Interrupt::CAN_SCE => 22,
            Interrupt::EXTI9_5 => 23,
            Interrupt::TIM1_BRK_TIM15 => 24,
            Interrupt::TIM1_UP_TIM16 => 25,
            Interrupt::TIM1_TRG_COM_TIM17 => 26,
            Interrupt::TIM1_CC => 27,
            Interrupt::TIM2 => 28,
            Interrupt::TIM3 => 29,
            Interrupt::TIM4 => 30,
            Interrupt::I2C1_EV_EXTI23 => 31,
            Interrupt::I2C1_ER => 32,
            Interrupt::I2C2_EV_EXTI24 => 33,
            Interrupt::I2C2_ER => 34,
            Interrupt::SPI1 => 35,
            Interrupt::SPI2 => 36,
            Interrupt::USART1_EXTI25 => 37,
            Interrupt::USART2_EXTI26 => 38,
            Interrupt::USART3_EXTI28 => 39,
            Interrupt::EXTI15_10 => 40,
            Interrupt::RTCALARM => 41,
            Interrupt::USB_WKUP => 42,
            Interrupt::TIM8_BRK => 43,
            Interrupt::TIM8_UP => 44,
            Interrupt::TIM8_TRG_COM => 45,
            Interrupt::TIM8_CC => 46,
            Interrupt::ADC3 => 47,
            Interrupt::SPI3 => 51,
            Interrupt::UART4_EXTI34 => 52,
            Interrupt::UART5_EXTI35 => 53,
            Interrupt::TIM6_DACUNDER => 54,
            Interrupt::TIM7 => 55,
            Interrupt::DMA2_CH1 => 56,
            Interrupt::DMA2_CH2 => 57,
            Interrupt::DMA2_CH3 => 58,
            Interrupt::DMA2_CH4 => 59,
            Interrupt::DMA2_CH5 => 60,
            Interrupt::ADC4 => 61,
            Interrupt::COMP123 => 64,
            Interrupt::COMP456 => 65,
            Interrupt::COMP7 => 66,
            Interrupt::USB_HP => 74,
            Interrupt::USB_LP => 75,
            Interrupt::USB_WKUP_EXTI => 76,
            Interrupt::FPU => 81,
        }
    }
}
use core::convert::TryFrom;
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl TryFrom<u8> for Interrupt {
    type Error = TryFromInterruptError;
    #[inline]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Interrupt::WWDG),
            1 => Ok(Interrupt::PVD),
            2 => Ok(Interrupt::TAMP_STAMP),
            3 => Ok(Interrupt::RTC_WKUP),
            4 => Ok(Interrupt::FLASH),
            5 => Ok(Interrupt::RCC),
            6 => Ok(Interrupt::EXTI0),
            7 => Ok(Interrupt::EXTI1),
            8 => Ok(Interrupt::EXTI2_TSC),
            9 => Ok(Interrupt::EXTI3),
            10 => Ok(Interrupt::EXTI4),
            11 => Ok(Interrupt::DMA1_CH1),
            12 => Ok(Interrupt::DMA1_CH2),
            13 => Ok(Interrupt::DMA1_CH3),
            14 => Ok(Interrupt::DMA1_CH4),
            15 => Ok(Interrupt::DMA1_CH5),
            16 => Ok(Interrupt::DMA1_CH6),
            17 => Ok(Interrupt::DMA1_CH7),
            18 => Ok(Interrupt::ADC1_2),
            19 => Ok(Interrupt::USB_HP_CAN_TX),
            20 => Ok(Interrupt::USB_LP_CAN_RX0),
            21 => Ok(Interrupt::CAN_RX1),
            22 => Ok(Interrupt::CAN_SCE),
            23 => Ok(Interrupt::EXTI9_5),
            24 => Ok(Interrupt::TIM1_BRK_TIM15),
            25 => Ok(Interrupt::TIM1_UP_TIM16),
            26 => Ok(Interrupt::TIM1_TRG_COM_TIM17),
            27 => Ok(Interrupt::TIM1_CC),
            28 => Ok(Interrupt::TIM2),
            29 => Ok(Interrupt::TIM3),
            30 => Ok(Interrupt::TIM4),
            31 => Ok(Interrupt::I2C1_EV_EXTI23),
            32 => Ok(Interrupt::I2C1_ER),
            33 => Ok(Interrupt::I2C2_EV_EXTI24),
            34 => Ok(Interrupt::I2C2_ER),
            35 => Ok(Interrupt::SPI1),
            36 => Ok(Interrupt::SPI2),
            37 => Ok(Interrupt::USART1_EXTI25),
            38 => Ok(Interrupt::USART2_EXTI26),
            39 => Ok(Interrupt::USART3_EXTI28),
            40 => Ok(Interrupt::EXTI15_10),
            41 => Ok(Interrupt::RTCALARM),
            42 => Ok(Interrupt::USB_WKUP),
            43 => Ok(Interrupt::TIM8_BRK),
            44 => Ok(Interrupt::TIM8_UP),
            45 => Ok(Interrupt::TIM8_TRG_COM),
            46 => Ok(Interrupt::TIM8_CC),
            47 => Ok(Interrupt::ADC3),
            51 => Ok(Interrupt::SPI3),
            52 => Ok(Interrupt::UART4_EXTI34),
            53 => Ok(Interrupt::UART5_EXTI35),
            54 => Ok(Interrupt::TIM6_DACUNDER),
            55 => Ok(Interrupt::TIM7),
            56 => Ok(Interrupt::DMA2_CH1),
            57 => Ok(Interrupt::DMA2_CH2),
            58 => Ok(Interrupt::DMA2_CH3),
            59 => Ok(Interrupt::DMA2_CH4),
            60 => Ok(Interrupt::DMA2_CH5),
            61 => Ok(Interrupt::ADC4),
            64 => Ok(Interrupt::COMP123),
            65 => Ok(Interrupt::COMP456),
            66 => Ok(Interrupt::COMP7),
            74 => Ok(Interrupt::USB_HP),
            75 => Ok(Interrupt::USB_LP),
            76 => Ok(Interrupt::USB_WKUP_EXTI),
            81 => Ok(Interrupt::FPU),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
macro_rules ! interrupt { ( $ NAME : ident , $ path : path , locals : { $ ( $ lvar : ident : $ lty : ty = $ lval : expr ; ) * } ) => { # [ allow ( non_snake_case ) ] mod $ NAME { pub struct Locals { $ ( pub $ lvar : $ lty , ) * } } # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ( $ lvar : $ lval , ) * } ; let f : fn ( & mut self :: $ NAME :: Locals ) = $ path ; f ( unsafe { & mut LOCALS } ) ; } } ; ( $ NAME : ident , $ path : path ) => { # [ allow ( non_snake_case ) ] # [ no_mangle ] pub extern "C" fn $ NAME ( ) { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn ( ) = $ path ; f ( ) ; } } }
