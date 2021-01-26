
use sse::sse_controller;
use crate::sse_controller::set_controller;
use tokio::time::{delay_for, Duration};
use rand::{self, Rng};

pub static MESSAGE_SENDING_TIMEOUT_SECONDS: u32 = 5;

#[tokio::main]
async fn main()
{
	let mut rng = rand::thread_rng();
    let controller = set_controller();          
    loop {
    	let ran_value = rng.gen_range(0..101);
    	let _charger_state = controller.send(ran_value.to_string());
    	println!{"RELEASE 2 - Battery percentage: {}", ran_value}
    	delay_for(Duration::from_secs(MESSAGE_SENDING_TIMEOUT_SECONDS.into())).await;
    }
}
