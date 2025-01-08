use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;


static NUM_UPTIME_REQUESTS: AtomicUsize = AtomicUsize::new(0);
///
/// Handy uptime struct for use in axum state
/// Like all substates ensure that `FromRef` is implemented:
/// ```rust ignore
/// impl FromRef<AppState> for Uptime {
///	  fn from_ref(app_state: &AppState) -> Uptime { app_state.uptime.clone() }
///	}
/// ```
///
#[derive(Debug, Clone)]
pub struct Uptime {
	pub start: std::time::Instant,
}
impl Default for Uptime {
	fn default() -> Self { Self::new() }
}



impl Uptime {
	pub fn new() -> Self {
		Self {
			start: std::time::Instant::now(),
		}
	}
	pub fn incr_requests(&self) -> usize {
		NUM_UPTIME_REQUESTS.fetch_add(1, Ordering::SeqCst) + 1
	}

	pub fn stats(&self) -> String {
		let uptime = self.start.elapsed().as_secs();
		let requests = self.incr_requests();
		format!("Uptime: {} seconds, Requests: {}", uptime, requests)
	}
}
