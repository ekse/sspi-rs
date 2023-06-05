use crate::sspi_data_types::{SecChar, SecWChar};

#[derive(Debug)]
pub struct KdcProxySettings {
    pub proxy_server: String,
    #[allow(dead_code)]
    pub client_tls_cred: Option<String>,
}

#[derive(Default, Debug)]
pub struct CredentialsAttributes {
    pub package_list: Option<String>,
    pub kdc_url: Option<String>,
    pub kdc_proxy_settings: Option<KdcProxySettings>,
    pub workstation: Option<String>,
}

impl CredentialsAttributes {
    pub fn new() -> Self {
        CredentialsAttributes::default()
    }

    pub fn new_with_package_list(package_list: Option<String>) -> Self {
        CredentialsAttributes {
            package_list,
            ..Default::default()
        }
    }

    pub fn kdc_url(&self) -> Option<String> {
        if let Some(kdc_url) = &self.kdc_url {
            Some(kdc_url.to_string())
        } else {
            self.kdc_proxy_settings
                .as_ref()
                .map(|kdc_proxy_settings| kdc_proxy_settings.proxy_server.to_string())
        }
    }
}

#[repr(C)]
pub struct SecPkgCredentialsKdcProxySettingsA {
    pub version: u32,
    pub flags: u32,
    pub proxy_server_offset: u16,
    pub proxy_server_length: u16,
    pub client_tls_cred_offset: u16,
    pub client_tls_cred_length: u16,
}

#[repr(C)]
pub struct SecPkgCredentialsKdcProxySettingsW {
    pub version: u32,
    pub flags: u32,
    pub proxy_server_offset: u16,
    pub proxy_server_length: u16,
    pub client_tls_cred_offset: u16,
    pub client_tls_cred_length: u16,
}

#[repr(C)]
pub struct SecPkgCredentialsKdcUrlA {
    pub kdc_url: *mut SecChar,
}

#[repr(C)]
pub struct SecPkgCredentialsKdcUrlW {
    pub kdc_url: *mut SecWChar,
}
