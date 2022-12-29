pub struct ConfigDescription {
    inner: ResTableConfig,
}

struct ResTableConfig {
    size: u32,
    imsi: Imsi,
}

#[repr(C)]
union Imsi {
    mcc: u16,
    mnc: u16,
    imsi: u32,
}

#[repr(C)]
union Locale {
    language: [u8; 2],
    country: [u8; 2],
    locale: u32,
}
