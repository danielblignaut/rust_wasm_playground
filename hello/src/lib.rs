use pb_jelly::Message;
use ::proto_hello_world::proto_hello_world::{HelloOutput, HelloInput, Status};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub extern fn hello(input_arr: &[u8])-> Vec<u8> {

	let res: HelloOutput = match HelloInput::deserialize_from_slice(input_arr) {
		Ok(input)=> HelloOutput {
				response: format!("hello {}",input.name),
				status: Status::OK
		},
		Err(_)=> HelloOutput {
			response: String::from(""),
			status: Status::NOT_OK
		}
	};

	return res.serialize_to_vec()
	
}