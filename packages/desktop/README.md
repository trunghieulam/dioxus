# Dioxus-webview

Dioxus-webview bridges virtual and Webview DOMs together to make simple, portable, desktop applications.

Dioxus-webview is an attempt at making a simpler "Tauri" where creating desktop applications is as simple as:

```rust
// main.rs
fn main() {
    dioxus_desktop::new(App, |c| c)
    .launch()
    .await;
}
static App: FC<()> = |cx, props|{
    let (count, set_count) = use_state(cx, || 0);
    rsx!(cx, div {
        h1 { "Dioxus Desktop Demo" }
        p { "Count is {count}"}
        button { onclick: move |_| count += 1}
    })
};
```

and then to create a native .app:

```
dioxus bundle --platform macOS
```

## Goals

Because the host VirtualDOM is running in its own native process, native applications can unlock their full potential. Dioxus-webview is designed to be a 100% rust alternative to ElectronJS without the memory overhead or bloat of ElectronJS apps.

By bridging the native process, desktop apps can access full multithreading power, peripheral support, hardware access, and native filesystem controls without the hassle of web technologies. Our goal with Dioxus-webview is to make it easy to ship both a web and native application, and quickly see large performance boosts without having to re-write the whole stack. As the dioxus ecosystem grows, we hope to see 3rd parties providing wrappers for storage, offline mode, etc that supports both web and native technologies.

## Tech

Dioxus-desktop is a pure liveview application where all of the state and event handlers are proxied through the liveview and into the native process. For pure server-based liveview, this would normally be too slow (in both render performance and latency), but because the VDom is local, desktop apps are just as fast as Electron.

Dioxus-desktop leverages dioxus-liveview under the hood, but with convenience wrappers around setting up the VDom bridge, proxying events, and serving the initial WebSys-Renderer. The backend is served by Tide, so an async runtime _is_ needed - we recommend async-std in Tokio mode.


## Async Runtime

