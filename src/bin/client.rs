use lazy_static::lazy_static;
use mini_redis_base::LogLayer;
use std::{io, net::SocketAddr, process};

lazy_static! {
    static ref CLIENT: volo_gen::redis::base::RedisServiceClient = {
        let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
        volo_gen::redis::base::RedisServiceClientBuilder::new("volo-example")
            .layer_outer(LogLayer)
            .address(addr)
            .build()
    };
}

#[volo::main]
async fn main() {
    tracing_subscriber::fmt::init();
    loop {
        let mut flag = false;
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.strip_suffix("\n").unwrap().to_string();
        let str_vec: Vec<String> = input.split(' ').map(|str| str.to_string()).collect();
        // println!("{:?}", &str_vec);
        let mut req = volo_gen::redis::base::RedisRequest {
            key: None,
            value: None,
            r#type: volo_gen::redis::base::RequestType::Illegal,
        };
        if str_vec[0] == "PING" {
            // println!("{}", input.strip_prefix("PING ").unwrap().to_string());
            req = volo_gen::redis::base::RedisRequest {
                key: None,
                value: Some(input.strip_prefix("PING ").unwrap().to_string().into()),
                r#type: volo_gen::redis::base::RequestType::Ping,
            }
        } else if str_vec[0] == "DEL" {
            let mut temp = vec![];
            for i in 1..str_vec.len() {
                temp.push(str_vec.get(i).unwrap().clone().into());
            }
            req = volo_gen::redis::base::RedisRequest {
                key: Some(temp),
                value: None,
                r#type: volo_gen::redis::base::RequestType::Del,
            }
        } else if str_vec.len() == 2 {
            if str_vec[0] == "GET" {
                req = volo_gen::redis::base::RedisRequest {
                    key: Some(vec![str_vec.get(1).unwrap().clone().into()]),
                    value: None,
                    r#type: volo_gen::redis::base::RequestType::Get,
                }
            }
        } else if str_vec.len() == 3 {
            if str_vec[0] == "SET" {
                req = volo_gen::redis::base::RedisRequest {
                    key: Some(vec![str_vec.get(1).unwrap().clone().into()]),
                    value: Some(str_vec.get(2).unwrap().clone().into()),
                    r#type: volo_gen::redis::base::RequestType::Set,
                }
            }
        } else if str_vec.len() == 1 && str_vec[0] == "exit" {
            flag = true;
            req = volo_gen::redis::base::RedisRequest {
                key: None,
                value: None,
                r#type: volo_gen::redis::base::RequestType::Exit,
            }
        }
        let resp = CLIENT.redis_command(req).await;
        if flag {
            process::exit(0);
        }
        match resp {
            Ok(info) => tracing::info!("{:?}", info.value.unwrap()),
            Err(e) => tracing::error!("{:?}", e),
        }
    }
}