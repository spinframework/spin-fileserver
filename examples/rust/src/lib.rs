use {
    bindings::wasi::http::incoming_handler,
    futures::SinkExt,
    spin_sdk::{
        http::{Fields, IncomingRequest, Method, OutgoingResponse, ResponseOutparam},
        http_component,
    },
};

mod bindings {
    wit_bindgen::generate!({
        path: "../wit",
        world: "delegate",
        with: {
            "wasi:http/types@0.2.0": ::spin_sdk::wit::wasi::http::types,
        }
    });
}

#[http_component]
async fn handle_request(request: IncomingRequest, response_out: ResponseOutparam) {
    match (request.method(), request.path_with_query().as_deref()) {
        (Method::Get, Some("/hello")) => {
            let fields =
                Fields::from_list(&[("content-type".to_owned(), b"text/plain".to_vec())]).unwrap();
            let response = OutgoingResponse::new(fields);

            let mut body = response.take_body();

            response_out.set(response);

            if let Err(e) = body.send(b"Hello, world!".to_vec()).await {
                eprintln!("Error sending payload: {e}");
            }
        }

        (Method::Get, _) => {
            // Delegate to spin-fileserver component
            incoming_handler::handle(request, response_out.into_inner())
        }

        _ => {
            let response = OutgoingResponse::new(Fields::new());
            response.set_status_code(405).unwrap();
            response_out.set(response);
        }
    }
}
