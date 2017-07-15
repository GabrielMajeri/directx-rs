extern crate winit;

extern crate dxgi;

extern crate direct3d;

fn create_window() -> (winit::EventsLoop, winit::Window) {
	let events_loop = winit::EventsLoop::new();

	let (width, height) = (640, 480);

	let window = winit::WindowBuilder::new()
		.with_title("Hello DirectX")
		.with_dimensions(width, height)
		.build(&events_loop)
		.unwrap();

	let monitor = winit::get_primary_monitor();
	let (mwidth, mheight) = monitor.get_dimensions();

	window.set_position((mwidth - width) as i32 / 2, (mheight - height) as i32 / 2);

	(events_loop, window)
}

use dxgi::factory::Factory;
use dxgi::adapter::Adapter;

fn choose_adapter(factory: &Factory) -> Adapter {
	factory.adapters()
		// Rate the adapters and pick the best one.
		.max_by_key(|adapter| {
			let desc = adapter.description();

			let vid_mem_mib = desc.dedicated_video_memory() / 1024 / 1024;

			// Simply pick the adapter with the largest dedicated memory.
			vid_mem_mib as u64
		})
		// Panic if no graphics cards exist.
		.unwrap()
}

fn main() {
	let (mut events_loop, window) = create_window();
	let hwnd = {
		use winit::os::windows::WindowExt;
		unsafe {
			std::mem::transmute(window.get_hwnd())
		}
	};

	let factory = Factory::new();

	let adapter = choose_adapter(&factory);

	let (device, context) = direct3d::device::create_device(Some(&adapter));

	let swap_chain = dxgi::swap_chain::SwapChain::new(&factory, device.as_unknown(), hwnd);

	let rt_view = direct3d::render_target_view::RenderTargetView::from_swap_chain_back_buffer(&device, &swap_chain);

	context.output_merger().set_render_targets(Some(&[&rt_view]));

	let mut is_running = true;

	while is_running {
		events_loop.poll_events(|event| {
			match event {
				winit::Event::WindowEvent { event, .. } => {
					use winit::WindowEvent;
					match event {
						WindowEvent::Closed => {
							is_running = false
						},
						_ => ()
					}
				},
				_ => ()
			}
		});

		context.clear_render_target_view(&rt_view);

		render();

		swap_chain.present();
	}
}

fn render() {

}
