use std::{
    fmt::Debug,
    fs::read_dir,
    mem::ManuallyDrop,
    path::PathBuf,
    pin::Pin,
    task::{Context, Poll, ready},
};

use futures::{Stream, StreamExt};
use nill::Nill;
use router_libs::log::{Level, instrument};
use tokio::task::{JoinHandle, spawn_blocking};

use crate::error::Result;

#[derive(Debug)]
struct Walk {
    base: PathBuf,
    rdirs: Vec<PathBuf>,
    items: Vec<PathBuf>,
}

impl Walk {
    const BATCH_SIZE: usize = 256;

    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let base = path.into();
        Self { base: base.clone(), rdirs: vec![base], items: Default::default() }
    }
}

impl Walk {
    #[instrument(level=Level::TRACE, skip_all, err)]
    fn call(mut self) -> Result<Self> {
        while let Some(rdir) = self.rdirs.pop() {
            let mut rd = read_dir(rdir)?;
            while let Some(entry) = rd.next() {
                let path = entry?.path();
                if path.is_dir() {
                    self.rdirs.push(path);
                } else {
                    let file = path.strip_prefix(&self.base)?;
                    self.items.push(file.into())
                }
            }
            if self.items.len() >= Self::BATCH_SIZE {
                break;
            }
        }
        Ok(self)
    }
}

#[derive(Debug)]
enum State {
    Idle(ManuallyDrop<Walk>),
    Read(JoinHandle<Result<Walk>>),
}

#[derive(Debug)]
pub struct WalkDir {
    state: State,
}

impl WalkDir {
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let walk = Walk::new(path);
        let state = State::Idle(ManuallyDrop::new(walk));
        Self { state }
    }

    pub async fn walk(&mut self) -> Result<Vec<PathBuf>> {
        let mut paths = Vec::new();
        while let Some(path) = self.next().await {
            paths.push(path?);
        }
        Ok(paths)
    }
}

impl Drop for WalkDir {
    fn drop(&mut self) {
        if let State::Idle(walk) = &mut self.state {
            unsafe { ManuallyDrop::drop(walk) }
        }
    }
}

impl Unpin for WalkDir {}

impl Stream for WalkDir {
    type Item = Result<PathBuf>;

    #[instrument(level=Level::TRACE, skip_all, ret)]
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            let state = match &mut self.state {
                State::Idle(walk) => {
                    if let Some(item) = walk.items.pop() {
                        return Poll::Ready(Some(Ok(item)))
                    }
                    if walk.rdirs.nil() {
                        return Poll::Ready(None)
                    }
                    let task = unsafe { ManuallyDrop::take(walk) };
                    State::Read(spawn_blocking(|| task.call()))
                },
                State::Read(rx) => {
                    let walk = ready!(Pin::new(rx).poll(cx))??;
                    State::Idle(ManuallyDrop::new(walk))
                },
            };
            self.state = state;
        }
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;
    use nill::{Nil, nil};

    use super::*;

    #[tokio::test]
    async fn test_walkdir() -> Result<Nil> {
        let dir = concat!(env!("CARGO_MANIFEST_DIR"), "/src/fs");
        let paths = WalkDir::new(dir).walk().await?;
        assert_debug_snapshot!(paths, @r#"
        [
            "mod.rs",
            "walkdir.rs",
        ]
        "#);
        Ok(nil)
    }
}
