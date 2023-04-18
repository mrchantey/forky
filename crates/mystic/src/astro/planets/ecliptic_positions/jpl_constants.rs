use super::super::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JPLConstants {
	ja0: f64,
	jac: f64,
	je0: f64,
	jec: f64,
	ji0: f64,
	jic: f64,
	jl0: f64,
	jlc: f64,
	ju0: f64,
	juc: f64,
	jq0: f64,
	jqc: f64,
}

impl JPLConstants {
	pub const fn new(
		ja0: f64,
		jac: f64,
		je0: f64,
		jec: f64,
		ji0: f64,
		jic: f64,
		jl0: f64,
		jlc: f64,
		ju0: f64,
		juc: f64,
		jq0: f64,
		jqc: f64,
	) -> Self {
		Self {
			ja0,
			jac,
			je0,
			jec,
			ji0,
			jic,
			jl0,
			jlc,
			ju0,
			juc,
			jq0,
			jqc,
		}
	}
}

impl Into<OrbitalConstants> for JPLConstants {
	fn into(self) -> OrbitalConstants {
		let dpc = 36525.0;
		let cofs = 1.5 / dpc;

		let n0 = self.jq0 - cofs * self.jqc;
		let nc = self.jqc / dpc;

		let i0 = self.ji0 - cofs * self.jic;
		let ic = self.jic / dpc;

		// w = ju - jQ
		let w0 = wrap_deg(
			(self.ju0 - cofs * self.juc) - (self.jq0 - cofs * self.jqc),
		);
		let wc = (self.juc - self.jqc) / dpc;

		let a0 = self.ja0 - cofs * self.jac;
		let ac = self.jac / dpc;

		let e0 = self.je0 - cofs * self.jec;
		let ec = self.jec / dpc;

		// M = jL - ju
		let m0 = wrap_deg(
			(self.jl0 - cofs * self.jlc) - (self.ju0 - cofs * self.juc),
		);
		let mc = (self.jlc - self.juc) / dpc;

		OrbitalConstants {
			n_offset: n0,
			n_scalar: nc,
			i_offset: i0,
			i_scalar: ic,
			w_offset: w0,
			w_scalar: wc,
			a_offset: a0,
			a_scalar: ac,
			e_offset: e0,
			e_scalar: ec,
			m_offset: m0,
			m_scalar: mc,
		}
	}
}
