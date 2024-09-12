#[derive(Debug)]
pub struct HttpRequest{
    method : Method,
    route: Route
    version : Version,
    headers: HttpHeader,
    request_body: String
}

#[derive(Debug)]
struct HttpHeader{
    headers: HashMap<String, String>,
}

impl HttpHeader{
    
}

#[derive(Debug)]
enum Version{
    V1_1,
    V2_0
}

#[derive(Debug)]
enum Method{
    Get,
    Post,
    Unnistialised,
}

#[derive(Debug)]
struct Route{
    path: String;
}