use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseMessage<V> {
    data: V,
    message: Option<String>,
    code: u32,
}

impl<V> ResponseMessage<V> {
    pub fn success(data: V) -> ResponseMessage<V> {
        Self {
            data,
            message: None,
            code: 200,
        }
    }
}

impl ResponseMessage<Option<String>> {
    pub fn fail(message: String) -> ResponseMessage<Option<String>> {
        Self {
            data: Option::<String>::None,
            message: Some(message),
            code: 400,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClientInfo {
    // 设备ID
    pub device_id: String,
    // 名称
    pub name: String,
    // 客户端间是否加密
    pub client_secret: bool,
    // 客户端和服务端是否加密
    pub server_secret: bool,
    // 链接服务器的来源地址
    pub address: SocketAddr,
    // 是否在线
    pub online: bool,
    // 分配的ip
    pub virtual_ip: Ipv4Addr,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkInfo {
    // 网段
    pub network_ip: Ipv4Addr,
    // 掩码
    pub mask_ip: Ipv4Addr,
    // 网关
    pub gateway_ip: Ipv4Addr,
    // 网段下的客户端列表
    pub clients: Vec<ClientInfo>,
}

impl NetworkInfo {
    pub fn new(network_ip: Ipv4Addr, mask_ip: Ipv4Addr, gateway_ip: Ipv4Addr) -> Self {
        Self {
            network_ip,
            mask_ip,
            gateway_ip,
            clients: Default::default(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupList {
    pub group_list: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GroupsInfo {
    pub data: HashMap<String, NetworkInfo>,
}

impl GroupsInfo {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}
