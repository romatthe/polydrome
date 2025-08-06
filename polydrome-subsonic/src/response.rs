use serde::Serialize;

#[derive(Serialize)]
#[serde(rename = "subsonic-response")]
pub struct Response<R: Serialize> {
    #[serde(rename = "@status")]
    status: ResponseStatus,
    #[serde(rename = "@version")]
    version: String,
    #[serde(rename = "$value")]
    inner: R,
}

impl<R: Serialize> Response<R> {
    pub fn ok_from(response: R) -> Response<R> {
        Response {
            status: ResponseStatus::Ok,
            version: "1.16.1".to_string(),
            inner: response,
        }
    }
}

#[derive(Serialize)]
pub enum ResponseStatus {
    #[serde(rename = "ok")]
    Ok,
}

#[derive(Serialize)]
#[serde(rename = "license")]
pub struct GetLicenseResponse<'a> {
    #[serde(rename = "@valid")]
    valid: bool,
    #[serde(rename = "@email")]
    email: &'a str,
    #[serde(rename = "@licenseExpires")]
    license_expires: &'a str,
}

impl Default for GetLicenseResponse<'_> {
    fn default() -> Self {
        Self {
            valid: true,
            email: "polydrome@example.com",
            license_expires: "2999-12-31T23:59:59",
        }
    }
}
