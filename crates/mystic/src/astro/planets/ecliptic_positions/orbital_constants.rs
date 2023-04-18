#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitalConstants {
	/// Longitude of asc. node
	pub n_offset: f64,
	pub n_scalar: f64,
	/// Inclination
	pub i_offset: f64,
	pub i_scalar: f64,
	/// Longitude of perihelion
	pub w_offset: f64,
	pub w_scalar: f64,
	/// mean distance, a.u.
	pub a_offset: f64,
	pub a_scalar: f64,
	/// eccentricity
	pub e_offset: f64,
	pub e_scalar: f64,
	/// mean anomaly
	pub m_offset: f64,
	pub m_scalar: f64,
}
