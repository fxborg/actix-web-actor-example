extern crate actix;
extern crate actix_web;
extern crate futures;

use actix_web::{middleware, web, App, HttpResponse, HttpRequest, HttpServer};
use actix::prelude::*;
use futures::future::Future;


/// メッセージの定義
struct Ping(usize);

impl Message for Ping {
    type Result = usize;
}

/// アクターの定義
struct MyActor {
    count: usize,
}

impl Actor for MyActor {
    type Context = Context<Self>;
}

/// Pingメッセージを受けるハンドラーを定義
impl Handler<Ping> for MyActor {
    type Result = usize;

    fn handle(&mut self, msg: Ping, _: &mut Context<Self>) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}
/// デフォルトページ
fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

/// アクター呼び出し用
fn hello(addr: web::Data<actix::Addr<MyActor>>) 
    -> impl Future<Item = HttpResponse, Error=actix_web::Error>
{
    // アドレスにメッセージを送信
    addr.send(Ping(1))
        .from_err()
        .and_then(|n| {
            // 受信した n を表示する
            HttpResponse::Ok().body(format!("Hello, Count:{} !!", n))
    })
}

/// メインルーチン
fn main() -> std::io::Result<()> {
    // actor スレッドを用意
    let sys = System::new("actor-example");
    // log の設定
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    //　アクターのアドレスを得る
    let address = MyActor { count: 0 }.start();
    HttpServer::new( move || {
        App::new()
            // 共有情報としてアドレスを格納
            .data(address.clone())
            // ログを有効化
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(index))
            .service(web::resource("/hello").to_async(hello))
    })
    .bind("127.0.0.1:8080")?
    .start();
    // スレッドを開始
    sys.run()
    
}   
