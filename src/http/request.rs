use std::collections::HashMap;
use std::str::FromStr;
use std::fmt::Display;
use std::io;

#[derive(Debug)]
pub struct HttpRequest {
    method: Method,
    resource: Resource,
    version: Version,
    headers: HttpHeader,
    pub request_body: String,
}


impl HttpRequest{
    pub fn new(request: &str) -> io::Result<HttpRequest>{
        let method: Method = Method::new(request);
        let resource: Resource = if let Some(resource)  =  Resource::new(request){
            resource
        }else{
            Resource{path: "".to_string()}
        };
        let version: Version = Version::new(request).map_err(|err| {
            io::Error::new(io::ErrorKind::InvalidData, err.msg)
        })?;
        let headers: HttpHeader = if let Some(headers)= HttpHeader::new(request){
            headers
        }else{
            HttpHeader{
                headers: HashMap::new()
            }
        };
        let request_body : String = if let Some(_header, body) =  request.split_once("\r\n\r\n"){
            body.to_string()
        }else{
            String::new()
        };

        Ok(HttpRequest{
            method,
            resource,
            version,
            headers,
            request_body
        })

    }
}

#[derive(Debug)]
struct HttpHeader {
    headers: HashMap<String, String>,
}

impl HttpHeader {
    pub fn new(request: &str) -> Option<HttpHeader> {
        let mut httpheader = HttpHeader { headers: HashMap::new() };
        let (_, header_str) = request.split_once("\r\n")?;

        for line in header_str.split_terminator("\r\n") {
            if line.is_empty() {
                break;
            }
            let (header, value) = line.split_once(":")?;
            httpheader.headers.insert(header.trim().to_string(), value.trim().to_string());
        }

        Some(httpheader)
    }
}

#[derive(Debug)]
enum Version {
    V1_1,
    V2_0,
}

#[derive(Debug)]
struct VersionError {
    msg: String,
}

impl Display for VersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Version {
    pub fn new(request: &str) -> Result<Self, VersionError> {
        Version::from_str(request)
    }
}

impl FromStr for Version {
    type Err = VersionError;

    fn from_str(request: &str) -> Result<Self, Self::Err> {
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let splits = method_line.split_ascii_whitespace();
            for split in splits {
                if split == "HTTP/1.1" {
                    return Ok(Version::V1_1);
                } else if split == "HTTP/2" || split == "HTTP/2.0" {
                    return Ok(Version::V2_0);
                }
            }
        }
        let invalid = format!("Unknown protocol version in {}", request);
        let version_error = VersionError { msg: invalid };
        Err(version_error)
    }
}

#[derive(Debug)]
enum Method {
    Get,
    Post,
    Uninitialized,
}

impl Method {
    pub fn new(request: &str) -> Method {
        let request_split = request.split_once("\r\n");
        if let Some((method_line, _rest)) = request_split {
            let method_line = method_line.split_once(" ");
            if let Some((method, _rest)) = method_line {
                return Method::identify(method);
            }
        }
        Method::Uninitialized
    }

    pub fn identify(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug)]
struct Resource {
    path: String,
}

impl Resource {
    pub fn new(request: &str) -> Option<Resource> {
        if let Some((request_method, _)) = request.split_once("\r\n") {
            let (method, rest) = request_method.split_once(' ')?;
            return match Method::identify(method) {
                Method::Get | Method::Post => {
                    let (resource, _protocol_version) = rest.split_once(' ')?;
                    let resource = resource.trim();
                    let resource = resource.trim_start_matches('/');
                    Some(Resource { path: resource.to_string() })
                }
                Method::Uninitialized => None,
            };
        }
        None
    }
}