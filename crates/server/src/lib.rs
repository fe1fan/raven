// 客户端 WASM 入口文件

// 导出共享的数据结构（供前后端使用）
pub mod server_functions;

// 服务端模块（仅非 WASM）
#[cfg(not(target_arch = "wasm32"))]
pub mod api;
#[cfg(not(target_arch = "wasm32"))]
pub mod websocket;

// 客户端模块
pub mod app;
pub mod pages;
pub mod components;
pub mod example_page;

#[cfg(feature = "hydrate")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "hydrate")]
#[wasm_bindgen(start)]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
