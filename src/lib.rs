pub mod request;
pub mod response;
pub mod route;
pub mod server;

mod thread_pool;

// NOTE just made this macro for a bit of fun lol
#[macro_export]
macro_rules! mk_server {
    ($($route:literal => $handler:expr),+ $(,)?) => {
        $crate::server::Server::new(vec![
            $($crate::route::RouteHandler::new($crate::route::Route::new($route).unwrap(), $handler),)+
        ])
    };
}
