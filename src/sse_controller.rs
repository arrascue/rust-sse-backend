use tokio::sync::broadcast::Sender;
use tokio::sync::broadcast::Receiver;
use futures::stream::iter;
use futures::Stream;
use std::convert::Infallible;

use warp::Filter;
use warp::sse::ServerSentEvent;
use warp::http::Method;

use tokio::sync::broadcast;

pub fn set_controller() -> Sender<String>{  
    let (sender, _) = broadcast::channel(100);  
    create_sse_endpoint(sender.clone());
    sender 
}

fn create_sse_endpoint(sender: Sender<String>) {
    let cors = warp::cors().allow_methods(&[Method::GET, Method::POST, Method::DELETE]);    
    let routes = warp::path("get-value")
        .and(warp::get())
        .and_then(move || {
            let input = sender.subscribe();
            async move {
                get_input(input).await
            }
        })
        .map(|message| {
            warp::sse::reply(convert_to_sse_stream(message))
        })
        .with(cors);

    tokio::spawn(async move {  
        warp::serve(routes).run(([0,0,0,0], 8000)).await;
    });        
}

async fn get_input(mut r: Receiver<String>) -> Result<String, Infallible> {
    Ok(r.recv().await.unwrap())
}

fn convert_to_sse_stream(message: String) -> impl Stream<Item = Result<impl ServerSentEvent, Infallible>> {
    iter(vec![
        Ok((
            warp::sse::event("value"),
            warp::sse::data(message)
        ))
    ])
}
