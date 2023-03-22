use std::sync::Arc;
use std::thread;
use std::thread::JoinHandle;
use opcua::server::prelude::*;

pub struct OpcUaSimulator {
    name: Arc<String>,
    port: Arc<u16>,
}

impl OpcUaSimulator {
    pub fn new(name: String, port: u16) -> OpcUaSimulator {
        OpcUaSimulator {
            name: Arc::new(name),
            port: Arc::new(port),
        }
    }

    pub fn start(self) -> JoinHandle<()> {
        let name = self.name.clone();
        let port = self.port.clone();
        thread::spawn(move || {
            start_server(name, port);
        })
    }

    pub fn stop() {}
}

fn start_server(name: Arc<String>, port: Arc<u16>) {
    // let server = ServerBuilder::new()
    //     .application_name("RustOpcServer" + name)
    //     .application_uri("urn:simulator:opc:ua")
    //     .discovery_urls(vec![endpoint_url(port_offset)])
    //     .create_sample_keypair(true)
    //     .pki_dir("./pki-server")
    //     .discovery_server_url(None)
    //     .host_and_port(hostname(), *port)
    //     .user_token("test", ServerUserToken::new_user_pass("test", "test"))
    //     .endpoints(
    //         [
    //             ("none", endpoint_path, SecurityPolicy::None, MessageSecurityMode::None, &user_token_ids),
    //             ("basic128rsa15_sign", endpoint_path, SecurityPolicy::Basic128Rsa15, MessageSecurityMode::Sign, &user_token_ids),
    //             ("basic128rsa15_sign_encrypt", endpoint_path, SecurityPolicy::Basic128Rsa15, MessageSecurityMode::SignAndEncrypt, &user_token_ids),
    //             ("basic256_sign", endpoint_path, SecurityPolicy::Basic256, MessageSecurityMode::Sign, &user_token_ids),
    //             ("basic256_sign_encrypt", endpoint_path, SecurityPolicy::Basic256, MessageSecurityMode::SignAndEncrypt, &user_token_ids),
    //             ("basic256sha256_sign", endpoint_path, SecurityPolicy::Basic256Sha256, MessageSecurityMode::Sign, &user_token_ids),
    //             ("basic256sha256_sign_encrypt", endpoint_path, SecurityPolicy::Basic256Sha256, MessageSecurityMode::SignAndEncrypt, &user_token_ids),
    //         ].iter().map(|v| {
    //             (v.0.to_string(), ServerEndpoint::from((v.1, v.2, v.3, &v.4[..])))
    //         }).collect())
    //
    //     .server().unwrap();
    // server.
    //     server.run();
}