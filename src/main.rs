extern crate hyper;
extern crate safe_core;
#[macro_use]
extern crate unwrap;
#[macro_use]
extern crate log;

use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{Write};
use std::os::unix::io::FromRawFd;

use safe_core::core::client::Client;
use safe_core::core::utility::{ generate_random_string };
use safe_core::ffi::{helper};
use safe_core::ffi::errors::{FfiError};

use hyper::Server;
use hyper::header;
use hyper::server::Request;
use hyper::server::Response;
use hyper::uri::RequestUri;
use hyper::uri::RequestUri::AbsolutePath;

use safe_core::dns::errors::DnsError;
use safe_core::dns::dns_operations::DnsOperations;

use safe_core::nfs::helper::file_helper::FileHelper;
use safe_core::nfs::helper::directory_helper::DirectoryHelper;

fn main() {

    let server_admin_id = generate_random_string(32).unwrap_or("test".to_owned());
    println!("managed ID: {}", server_admin_id);
     let client = Arc::new(Mutex::new(unwrap!(Client::create_unregistered_client())));

    unsafe {
        let mut ipc = File::from_raw_fd(3);
        match ipc.write_fmt(format_args!("{{\"manageId\": \"{}\"}}\n", server_admin_id)){
            Ok(_) => println!("wrote to channel"),
            Err(msg) => println!("couldn't write to channel: {:?}", msg)
        }
    }

    fn get_file (client: Arc<Mutex<Client>>,
                long_name: &str, service: &str, uri: RequestUri) -> Result<Vec<u8>, FfiError> {

        let dns_operations = DnsOperations::new_unregistered(client.clone());
        let directory_key = try!(dns_operations.get_service_home_directory_key(long_name,
                                                           service, None));

        let mut tokens = match uri {
            AbsolutePath(ref path) => helper::tokenise_path(path, false),
            _ => vec![]
        };

        let file_name = try!(tokens.pop().ok_or("index.html"));
        let file_dir = try!(helper::get_final_subdirectory(client.clone(),
                                                           &tokens,
                                                           Some(&directory_key)));
        let file = try!(file_dir.find_file(&file_name).ok_or("DnsError"));
        let mut file_helper = FileHelper::new(client.clone());
        let mut reader = try!(file_helper.read(file));
        let size = reader.size();
        Ok(try!(reader.read(0, size)))
    }

    println!("Server starting at 127.0.0.1:3000");

    Server::http("127.0.0.1:3000").unwrap()
        .handle(move | req: Request, mut res: Response| {
          match req.headers.get::<header::Host>() {
            Some(host) => match host.hostname.as_ref() {
              "api.safenet" => res.send(b"on API").unwrap(),
              _ => {
                let v: Vec<&str> = host.hostname.rsplitn(3, ".").collect();
                match v.get(0) {
                  Some(&"safenet") => {
                    let (long_name, service_name) = match v.len() {
                      3 => (v[0], v[1]),
                      2 => (v[0], "www"),
                      _ => ("n", "n") // this isn't really useful
                    };

                    match get_file(client.clone(),
                                   long_name, service_name, req.uri) {
                        Ok(content) => res.send(&content).unwrap(),
                        Err(msg) => res.send(b"error").unwrap() // do something with the actual message
                    }
                  },
                  _ => res.send(b"not a safenet address").unwrap()
                }
              }
            },
            _ => res.send(b"nope").unwrap()
          }
        }).unwrap();
}
