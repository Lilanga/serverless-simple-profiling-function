use lambda_http::{run, service_fn, Body, Error, Request, RequestExt, Response};

async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let n = event.query_string_parameters()
    .first("n")
    .unwrap_or("0").to_string();

    let resp = Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(format!("Final count is {}", counter(n.parse::<u128>().unwrap())).into())
        .map_err(Box::new)?;
    Ok(resp)
}

fn counter(n: u128) ->u128{
    let mut count = 0;
    let mut i = 0;
    let mut range = n;

    if range>5000000000 { range = 5000000000; }

    while i <= range {
        count += i;
        i+=1;
    }

    return count;
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}
