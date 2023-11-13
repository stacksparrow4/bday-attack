#[derive(Clone)]
pub(crate) struct Sha256 {
    inner: ring::digest::Context,
}

impl Default for Sha256 {
    fn default() -> Self {
        Self {
            inner: ring::digest::Context::new(&ring::digest::SHA256),
        }
    }
}

impl Sha256 {
    pub(crate) fn update(&mut self, data: &[u8]) {
        self.inner.update(data);
    }

    pub(crate) fn finish(self) -> [u8; 32] {
        let digest = self.inner.finish();
        let mut out = [0; 32];
        out.copy_from_slice(digest.as_ref());
        out
    }
}
