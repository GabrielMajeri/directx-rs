extern crate winit;

extern crate winapi;
use winapi::shared::windef::HWND;

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

fn get_hwnd(window: &winit::Window) -> HWND {
	use winit::os::windows::WindowExt;
	unsafe {
		std::mem::transmute(window.get_hwnd())
	}
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
	let hwnd = get_hwnd(&window);

	let factory = Factory::new();

	let adapter = choose_adapter(&factory);

	let device = direct3d::device::Device::new(Some(adapter));

	let _swap_chain = dxgi::swap_chain::SwapChain::new(&factory, device.as_unknown(), hwnd);

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

		render();
	}
}

fn render() {

}
