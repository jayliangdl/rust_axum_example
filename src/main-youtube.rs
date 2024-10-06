// use std::io::Read;

use std::env::args;

use tracing_subscriber::{self, fmt::{self, format::FmtSpan}};
use tokio;
use tracing::{info, span, Span,trace,instrument};

#[derive(Debug)]
struct Food{
    a:bool,
    b:u32,
}

#[tokio::main]
async fn main(){
    let args:Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <file>...", args[0]);
        std::process::exit(1);
    }else{
        let level = &args[1];
        if level=="trace"{
            fmt::SubscriberBuilder::default()
            .with_max_level(tracing::Level::TRACE)
            // .with_span_events(FmtSpan::FULL)
            .init();
        }else if level=="debug"{
            fmt::SubscriberBuilder::default()
            .with_max_level(tracing::Level::DEBUG)
            // .with_span_events(FmtSpan::FULL)
            .init();
        }else if level=="info"{
            fmt::SubscriberBuilder::default()
            .with_max_level(tracing::Level::INFO)
            // .with_span_events(FmtSpan::FULL)
            .init();
        }else if level=="warn"{
            fmt::SubscriberBuilder::default()
            .with_max_level(tracing::Level::WARN)
            // .with_span_events(FmtSpan::FULL)
            .init();
        }else if level=="error"{
            fmt::SubscriberBuilder::default()
            .with_max_level(tracing::Level::ERROR)
            // .with_span_events(FmtSpan::FULL)
            .init();
        }else{
            fmt::SubscriberBuilder::default()
            // .with_span_events(FmtSpan::FULL)
            .init();
        }
        // tracing_subscriber::fmt::init();
            // fmt::SubscriberBuilder::default()
            // .with_max_level(tracing::Level::INFO)
            // .init();
        let span: span::Span = span!(tracing::Level::INFO, "MAIN", a=123);
        let _guard = span.enter();
        info!("starting--- ");
        let mut jhs = Vec::new();
        for file in &args[2..]{
            let file = file.clone();
            jhs.push(tokio::spawn(
                async move {
                    food::on_thread(file).await;
                }
            ));
        }

        for jh in jhs{
            jh.await.unwrap();
        }
    }
}

// async fn on_thread(){
//     trace!(file, "OPEN FILE");
//     warn!(file,"READ CONTENT");
//     info!()
//     let span = span!(tracing::Level::INFO, "THREAD");
//     let _guard = span.enter();
//     println!("on_thread");
// }

/**
 * 一个耗时的计算(如果日志级别不是trace，这个函数不会被调用)
 */
fn expensive_cal() -> u32{
    println!("expensive_cal");
    1
}

mod food{
    use super::*;
    #[instrument]
    pub(super) async fn on_thread(file:String){
        trace!(value=expensive_cal(),"starting {}",file);
        info!("opening the file");
        // let mut file = std::fs::File::open(file).unwrap();
        info!("reading file content");
        // let mut bytes = Vec::new();
        // file.read(&mut bytes).unwrap();
        // info!("file read, content: {:?}", bytes);
        info!(bytes.number = 0, file=file, "parsing");
        let food = Food{a:true,b:1};
        info!(parsed=?food, file=file, "done processing file");
    }
}





// async fn on_thread(file:String){
//     use std::time::Duration;
//     let span = span!(tracing::Level::INFO, "FILE", fname=%file);
//     let _guard = span.enter();
    
//     trace!(value=expensive_cal(),"starting {}",file);
//     info!("opening the file");
//     // let mut file = std::fs::File::open(file).unwrap();
//     info!("reading file content");
//     for i in 1..3{
//         tokio::time::sleep(Duration::from_secs(1)).await;
//     }
//     // let mut bytes = Vec::new();
//     // file.read(&mut bytes).unwrap();
//     // info!("file read, content: {:?}", bytes);
//     info!(bytes.number = 0, file=file, "parsing");
//     let food = Food{a:true,b:1};
//     info!(parsed=?food, file=file, "done processing file");
// }


// #[tokio::main]
// async fn main(){
//     let args:Vec<String> = std::env::args().collect();
//     if args.len() < 3 {
//         eprintln!("Usage: {} <file>...", args[0]);
//         std::process::exit(1);
//     }else{
//         let level = &args[1];
//         if level=="trace"{
//             fmt::SubscriberBuilder::default()
//             .with_max_level(tracing::Level::TRACE)
//             .init();
//         }else if level=="debug"{
//             fmt::SubscriberBuilder::default()
//             .with_max_level(tracing::Level::DEBUG)
//             .init();
//         }else if level=="info"{
//             fmt::SubscriberBuilder::default()
//             .with_max_level(tracing::Level::INFO)
//             .init();
//         }else if level=="warn"{
//             fmt::SubscriberBuilder::default()
//             .with_max_level(tracing::Level::WARN)
//             .init();
//         }else if level=="error"{
//             fmt::SubscriberBuilder::default()
//             .with_max_level(tracing::Level::ERROR)
//             .init();
//         }else{
//             fmt::SubscriberBuilder::default()
//             .init();
//         }
//         let span: span::Span = span!(tracing::Level::INFO, "MAIN", a=123);
//         let _guard = span.enter();
//         info!("starting--- ");
//         let mut jhs = Vec::new();
//         for file in &args[2..]{
//             let file = file.clone();
//             jhs.push(tokio::spawn(
//                 async move {
//                     on_thread(file).await;
//                 }
//             ));
//         }

//         for jh in jhs{
//             jh.await.unwrap();
//         }
//     }
// }