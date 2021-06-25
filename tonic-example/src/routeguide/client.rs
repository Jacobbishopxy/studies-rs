use std::error::Error;
use std::time::Duration;

use futures::stream;
use rand::rngs::ThreadRng;
use rand::Rng;
use tokio::time;
use tonic::transport::Channel;
use tonic::Request;

use routeguide::route_guide_client::RouteGuideClient;
use routeguide::{Point, Rectangle, RouteNote};

fn random_point(rng: &mut ThreadRng) -> Point {
    let latitude = (rng.gen_range(0..180) - 90) * 10_000_000;
    let longitude = (rng.gen_range(0..360) - 180) * 10_000_000;
    Point {
        latitude,
        longitude,
    }
}

pub mod routeguide {
    tonic::include_proto!("routeguide");
}

/// 1. 简易 RPC
async fn print_feature(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let response = client
        .get_feature(Request::new(Point {
            latitude: 409_146_138,
            longitude: -746_188_906,
        }))
        .await?;

    println!("RESPONSE = {:?}", response);

    Ok(())
}

/// 2. 服务端流式 RPC
async fn print_features(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let rectangle = Rectangle {
        lo: Some(Point {
            latitude: 400_000_000,
            longitude: -750_000_000,
        }),
        hi: Some(Point {
            latitude: 420_000_000,
            longitude: -730_000_000,
        }),
    };

    let mut stream = client
        .list_features(Request::new(rectangle))
        .await?
        .into_inner();

    while let Some(feature) = stream.message().await? {
        println!("NOTE = {:?}", feature);
    }

    Ok(())
}

/// 3. 客户端流式 RPC
async fn run_record_route(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();
    let point_count: i32 = rng.gen_range(2..100);

    let mut points = vec![];
    for _ in 0..=point_count {
        points.push(random_point(&mut rng))
    }

    println!("Traversing {} points", points.len());
    let request = Request::new(stream::iter(points));

    match client.record_route(request).await {
        Ok(res) => println!("SUMMARY: {:?}", res.into_inner()),
        Err(e) => println!("something went wrong: {:?}", e),
    }

    Ok(())
}

/// 4. 双向流式 RPC
async fn run_route_chat(client: &mut RouteGuideClient<Channel>) -> Result<(), Box<dyn Error>> {
    let start = time::Instant::now();

    let outbound = async_stream::stream! {
        let mut interval = time::interval(Duration::from_secs(1));

        while let time = interval.tick().await {
            let elapsed = time.duration_since(start);
            let note = RouteNote {
                location: Some(Point {
                    latitude: 409_146_138 + elapsed.as_secs() as i32,
                    longitude: -746_188_906,
                }),
                message: format!("at {:?}", elapsed),
            };
            yield note;
        }
    };

    let res = client.route_chat(Request::new(outbound)).await?;
    let mut inbound = res.into_inner();

    while let Some(note) = inbound.message().await? {
        println!("NOTE = {:?}", note);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RouteGuideClient::connect("http://[::1]:10000").await?;

    println!("*** 1. SIMPLE RPC ***");
    print_feature(&mut client).await?;

    println!("*** 2. SERVER STREAMING ***");
    print_features(&mut client).await?;

    println!("*** 3. CLIENT STREAMING ***");
    run_record_route(&mut client).await?;

    println!("*** 4. BIDIRECTIONAL STREAMING ***");
    run_route_chat(&mut client).await?;

    Ok(())
}
