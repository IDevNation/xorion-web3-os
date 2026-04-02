// X-OS Kernel Interface — Cross-platform wallet client

#[cfg(target_family = "unix")]
use std::os::unix::net::UnixStream;

#[cfg(target_family = "windows")]
use std::net::TcpStream as UnixStream;

use serde::{Serialize, Deserialize};
use std::io::{Read, Write};
use std::path::PathBuf;
use zeroize::Zeroize;

#[derive(Debug, Serialize, Deserialize)]
pub enum WalletRequest {
    Init { mnemonic: String },
    GetEthAddress,
    GetSolanaAddress,
    SignTransaction { chain: u32, tx_data: Vec<u8> },
    GetBalance { chain: u32 },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum WalletResponse {
    Success { data: Vec<u8> },
    Error { message: String },
}

pub struct WalletClient {
    stream: UnixStream,
}

impl WalletClient {
    pub fn new() -> Result<Self, String> {
        let socket_path = Self::get_socket_path();
        
        #[cfg(target_family = "unix")]
        let stream = UnixStream::connect(&socket_path)
            .map_err(|e| format!("Failed to connect: {}", e))?;
            
        #[cfg(target_family = "windows")]
        let stream = UnixStream::connect("127.0.0.1:3030")
            .map_err(|e| format!("Failed to connect: {}", e))?;
            
        Ok(Self { stream })
    }
    
    #[cfg(target_family = "unix")]
    fn get_socket_path() -> PathBuf {
        PathBuf::from("/tmp/xorion-wallet.sock")
    }
    
    #[cfg(target_family = "windows")]
    fn get_socket_path() -> PathBuf {
        PathBuf::from("127.0.0.1:3030")
    }
    
    pub fn wallet_init(&mut self, mnemonic: &str) -> Result<(), String> {
        let req = WalletRequest::Init { mnemonic: mnemonic.to_string() };
        self.send_request(req)
    }
    
    pub fn wallet_eth_address(&mut self, buf: &mut [u8]) -> Result<usize, String> {
        let req = WalletRequest::GetEthAddress;
        let resp = self.send_request_with_response(req)?;
        let len = resp.len().min(buf.len());
        buf[..len].copy_from_slice(&resp[..len]);
        Ok(len)
    }
    
    fn send_request(&mut self, req: WalletRequest) -> Result<(), String> {
        let data = serde_json::to_vec(&req).map_err(|e| e.to_string())?;
        self.stream.write_all(&data).map_err(|e| e.to_string())?;
        Ok(())
    }
    
    fn send_request_with_response(&mut self, req: WalletRequest) -> Result<Vec<u8>, String> {
        self.send_request(req)?;
        let mut buf = vec![0u8; 4096];
        let n = self.stream.read(&mut buf).map_err(|e| e.to_string())?;
        let resp: WalletResponse = serde_json::from_slice(&buf[..n]).map_err(|e| e.to_string())?;
        match resp {
            WalletResponse::Success { data } => Ok(data),
            WalletResponse::Error { message } => Err(message),
        }
    }
}

impl Drop for WalletClient {
    fn drop(&mut self) {
        // Nothing to zeroize here, socket will close automatically
    }
}
