use std::sync::Arc;

pub struct RenderTarget {
    pub info: RenderTargetInfo,
}

pub struct RenderTargetInfo {
}

impl RenderTarget {
    pub fn new(info: RenderTargetInfo) -> Arc<RenderTarget> {
        Arc::new(RenderTarget{
            info: info,
        })
    }
}
