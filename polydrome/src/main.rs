use polydrome_subsonic::response::{GetLicenseResponse, Response, ResponseStatus};

fn main() {
    let response = Response {
        status: ResponseStatus::Ok,
        version: "1.16.1".to_string(),
        inner: ()
    };
    
    let response2 = Response {
        status: ResponseStatus::Ok,
        version: "1.16.1".to_string(),
        inner: GetLicenseResponse::default(),
    };
    
    println!("{}", quick_xml::se::to_string(&response).unwrap());
    println!("{}", quick_xml::se::to_string(&response2).unwrap());
}
