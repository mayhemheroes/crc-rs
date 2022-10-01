#![no_main]
use crc::*;
use libfuzzer_sys::fuzz_target;

const U8_ALGORITHMS: &[Algorithm<u8>] = &[
    CRC_3_GSM,
    CRC_3_ROHC,
    CRC_4_G_704,
    CRC_4_INTERLAKEN,
    CRC_5_EPC_C1G2,
    CRC_5_G_704,
    CRC_5_USB,
    CRC_6_CDMA2000_A,
    CRC_6_CDMA2000_B,
    CRC_6_DARC,
    CRC_6_G_704,
    CRC_6_GSM,
    CRC_7_MMC,
    CRC_7_ROHC,
    CRC_7_UMTS,
    CRC_8_AUTOSAR,
    CRC_8_BLUETOOTH,
    CRC_8_CDMA2000,
    CRC_8_DARC,
    CRC_8_DVB_S2,
    CRC_8_GSM_A,
    CRC_8_GSM_B,
    CRC_8_I_432_1,
    CRC_8_I_CODE,
    CRC_8_LTE,
    CRC_8_MAXIM_DOW,
    CRC_8_MIFARE_MAD,
    CRC_8_NRSC_5,
    CRC_8_OPENSAFETY,
    CRC_8_ROHC,
    CRC_8_SAE_J1850,
    CRC_8_SMBUS,
    CRC_8_TECH_3250,
    CRC_8_WCDMA,
];

const U16_ALGORITHMS: &[Algorithm<u16>] = &[
    CRC_10_ATM,
    CRC_10_CDMA2000,
    CRC_10_GSM,
    CRC_11_FLEXRAY,
    CRC_11_UMTS,
    CRC_12_CDMA2000,
    CRC_12_DECT,
    CRC_12_GSM,
    CRC_12_UMTS,
    CRC_13_BBC,
    CRC_14_DARC,
    CRC_14_GSM,
    CRC_15_CAN,
    CRC_15_MPT1327,
    CRC_16_ARC,
    CRC_16_CDMA2000,
    CRC_16_CMS,
    CRC_16_DDS_110,
    CRC_16_DECT_R,
    CRC_16_DECT_X,
    CRC_16_DNP,
    CRC_16_EN_13757,
    CRC_16_GENIBUS,
    CRC_16_GSM,
    CRC_16_IBM_3740,
    CRC_16_IBM_SDLC,
    CRC_16_ISO_IEC_14443_3_A,
    CRC_16_KERMIT,
    CRC_16_LJ1200,
    CRC_16_MAXIM_DOW,
    CRC_16_MCRF4XX,
    CRC_16_MODBUS,
    CRC_16_NRSC_5,
    CRC_16_OPENSAFETY_A,
    CRC_16_OPENSAFETY_B,
    CRC_16_PROFIBUS,
    CRC_16_RIELLO,
    CRC_16_SPI_FUJITSU,
    CRC_16_T10_DIF,
    CRC_16_TELEDISK,
    CRC_16_TMS37157,
    CRC_16_UMTS,
    CRC_16_USB,
    CRC_16_XMODEM,
];

const U32_ALGORITHMS: &[Algorithm<u32>] = &[
    CRC_17_CAN_FD,
    CRC_21_CAN_FD,
    CRC_24_BLE,
    CRC_24_FLEXRAY_A,
    CRC_24_FLEXRAY_B,
    CRC_24_INTERLAKEN,
    CRC_24_LTE_A,
    CRC_24_LTE_B,
    CRC_24_OPENPGP,
    CRC_24_OS_9,
    CRC_30_CDMA,
    CRC_31_PHILIPS,
    CRC_32_AIXM,
    CRC_32_AUTOSAR,
    CRC_32_BASE91_D,
    CRC_32_BZIP2,
    CRC_32_CD_ROM_EDC,
    CRC_32_CKSUM,
    CRC_32_ISCSI,
    CRC_32_ISO_HDLC,
    CRC_32_JAMCRC,
    CRC_32_MPEG_2,
    CRC_32_XFER,
];

const U64_ALGORITHMS: &[Algorithm<u64>] = &[
    CRC_40_GSM,
    CRC_64_ECMA_182,
    CRC_64_GO_ISO,
    CRC_64_WE,
    CRC_64_XZ,
];

const U128_ALGORITHMS: &[Algorithm<u128>] = &[CRC_82_DARC];

fuzz_target!(|data: &[u8]| {
    if data.len() > 2 {
        let opt = data[0];
        let alg_opt: usize = data[1].into();
        let new_data = &data[2..];
        match opt {
            0 => {
                let crc = Crc::<u128>::new(&U128_ALGORITHMS[alg_opt % U128_ALGORITHMS.len()]);
                let mut digest = crc.digest();
                digest.update(&new_data);
                digest.finalize();
            }
            1 => {
                let crc = Crc::<u64>::new(&U64_ALGORITHMS[alg_opt % U64_ALGORITHMS.len()]);
                let mut digest = crc.digest();
                digest.update(&new_data);
                digest.finalize();
            }
            2 => {
                let crc = Crc::<u32>::new(&U32_ALGORITHMS[alg_opt % U32_ALGORITHMS.len()]);
                let mut digest = crc.digest();
                digest.update(&new_data);
                digest.finalize();
            },
            3 => {
                let crc = Crc::<u16>::new(&U16_ALGORITHMS[alg_opt % U16_ALGORITHMS.len()]);
                let mut digest = crc.digest();
                digest.update(&new_data);
                digest.finalize();
            },
            4 => {
                let crc = Crc::<u8>::new(&U8_ALGORITHMS[alg_opt % U8_ALGORITHMS.len()]);
                let mut digest = crc.digest();
                digest.update(&new_data);
                digest.finalize();
            },
            5 => {
                let crc = Crc::<u128>::new(&U128_ALGORITHMS[alg_opt % U128_ALGORITHMS.len()]);
                crc.checksum(&new_data);
            }
            6 => {
                let crc = Crc::<u64>::new(&U64_ALGORITHMS[alg_opt % U64_ALGORITHMS.len()]);
                crc.checksum(&new_data);
            }
            7 => {
                let crc = Crc::<u32>::new(&U32_ALGORITHMS[alg_opt % U32_ALGORITHMS.len()]);
                crc.checksum(&new_data);
            },
            8 => {
                let crc = Crc::<u16>::new(&U16_ALGORITHMS[alg_opt % U16_ALGORITHMS.len()]);
                crc.checksum(&new_data);
            },
            9 => {
                let crc = Crc::<u8>::new(&U8_ALGORITHMS[alg_opt % U8_ALGORITHMS.len()]);
                crc.checksum(&new_data);
            }
            _ => (),
        }
    }
});
