use super::{
  ScanCtx,
  Capturable,
};

#[derive(Debug, Clone)]
pub enum PinKind {
  Absolute,
  Relative,
  Partial,
  Situational,
}

#[derive(Debug, Clone)]
pub struct Pin {
  pub kind: PinKind,
  pub origin: usize,
  pub pinned: Capturable,
  pub shielded: Capturable,
}

impl Pin {
  pub fn new(ctx: &ScanCtx, pinned: Capturable, shielded: Capturable) -> Pin {
    Pin {
      pinned,
      shielded,
      kind: PinKind::Absolute,
      origin: ctx.origin,
    }
  }
}
