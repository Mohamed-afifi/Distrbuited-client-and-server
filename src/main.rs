use std::net::UdpSocket;
use base64;
use std::fs;
use serde::{Serialize, Deserialize};

const MAX_PACKET_SIZE: usize = 60000; // Maximum UDP payload size

// Define your custom struct
#[derive(Serialize, Deserialize)]
struct ImageWithIP {
    allowed_ips: Vec<String>,
    image_data: Option<Vec<u8>>,
}

fn main() {
    let socket = UdpSocket::bind("10.0.2.15:7878").expect("Failed to bind socket");

    // Load your image data from a file
    let image_path = "/home/afifi/Server/Untitled.jpeg";
    let image_data = fs::read(image_path).expect("Failed to read image file");

    // Encode the image data to base64
    let encoded_data = base64::encode(&image_data);

    // Create an instance of your struct
    let image_with_ip = ImageWithIP {
        allowed_ips: vec![
            "192.168.1.4".to_string(), // Replace with the actual allowed IPs
            "192.168.43.101".to_string(),
        ],
        image_data: Some(encoded_data.into_bytes()), // Store the encoded image data
    };

    // Serialize the struct to JSON
    let json_data = serde_json::to_string(&image_with_ip).expect("Failed to serialize to JSON");

    // Split the JSON data into chunks
    let chunks: Vec<&[u8]> = json_data.as_bytes().chunks(MAX_PACKET_SIZE).collect();
    let mut counter = 1;

    // Create a buffer for receiving client requests
    let mut request_buffer = [0; 1024];

    loop {
        // Receive a request from a client
        let (size, client_address) = socket.recv_from(&mut request_buffer).expect("Failed to receive request");
        let request = String::from_utf8_lossy(&request_buffer[..size]);
        println!("Received request from client: {}", request);

        // Check if the client's IP address is in the allowed_ips vector
        if image_with_ip.allowed_ips.contains(&client_address.ip().to_string()) {
            // Send the struct with the image data
            for chunk in &chunks {
                println!("{}", counter);
                counter = counter + 1;
                socket.send_to(chunk, client_address).expect("Failed to send data");
            }
        } else {
            // Send a default image or response (you need to adjust this part)
            let default_image_path = "afifi/Server/Sad-Face-Emoji.png";
            let default_image_data = fs::read(default_image_path).expect("Failed to read default image");
            let default_encoded_data = base64::encode(&default_image_data);
            let default_image_with_ip = ImageWithIP {
                allowed_ips: vec![client_address.ip().to_string()],
                image_data: Some(default_encoded_data.into_bytes()),
            };
            let default_json_data = serde_json::to_string(&default_image_with_ip).expect("Failed to serialize to JSON");
            let default_chunks: Vec<&[u8]> = default_json_data.as_bytes().chunks(MAX_PACKET_SIZE).collect();
            for chunk in &default_chunks {
                socket.send_to(chunk, client_address).expect("Failed to send data");
            }
        }
    }
}
