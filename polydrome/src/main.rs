use axum::Router;
use axum::routing::get;
use polydrome_axum_xml::Xml;
use polydrome_subsonic::response::{GetLicenseResponse, Response};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/rest/ping", get(ping))
        .route("/rest/getLicense", get(get_license));

    let response = Response::ok_from(());
    let response2 = Response::ok_from(GetLicenseResponse::default());
    
    println!("{}", quick_xml::se::to_string(&response).unwrap());
    println!("{}", quick_xml::se::to_string(&response2).unwrap());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn ping() -> Xml<Response<()>> {
    Xml(Response::ok_from(()))
}

async fn get_license() -> Xml<Response<GetLicenseResponse<'static>>> {
    Xml(Response::ok_from(GetLicenseResponse::default()))
}
