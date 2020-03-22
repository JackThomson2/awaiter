#[derive(Clone, Debug)]
pub enum Type {
    GET, POST, PUT, DELETE, PATCH, NULL
}

#[derive(Clone, Debug)]
pub struct Header {
    pub method: Type,
    pub location: String
}

impl Default for Header {
    fn default() -> Self {
        Header {
            method: Type::NULL,
            location: "".to_string()
        }
    }
}

pub fn try_parse_headers(headers: &str) -> Header {
    let mut returning = Header::default();
    let headers = headers.split('\n');

    for header in headers {
        if header.is_empty() {continue;}

        if header.starts_with("GET ") {
            let components:Vec<_> = header.split(' ').collect();
            if components.len() < 2 {
                println!("Bad request");
                continue;
            }

            let location = components[1];
            returning.method = Type::GET;
            returning.location = location.to_string();
        }
    }

    returning
}