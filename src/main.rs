use gstreamer::glib;
use gstreamer_rtsp_server::prelude::*;

fn main() {
    gstreamer::init().unwrap();

    let server = gstreamer_rtsp_server::RTSPServer::default();
    let mounts = server.mount_points().unwrap();

    let factory = gstreamer_rtsp_server::RTSPMediaFactory::default();
    factory.set_launch(
        "filesrc location=./test.mp4 ! decodebin ! videoconvert ! x264enc ! rtph264pay name=pay0 pt=96"
    );

    factory.connect_media_configure(|_, media| {
        let pipeline = media.element();
        let bus = pipeline.bus().unwrap();
        bus.connect_message(Some("eos"), move |_, _| {
            pipeline.set_state(gstreamer::State::Null).unwrap();
            pipeline.set_state(gstreamer::State::Playing).unwrap();
        });
    });

    mounts.add_factory("/test", factory);

    let _ = server.attach(None);
    println!("Stream ready at rtsp://127.0.0.1:8554/test");

    let main_loop = glib::MainLoop::new(None, false);
    main_loop.run();
}
