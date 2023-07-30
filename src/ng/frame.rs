extern crate alloc;

use alloc::{collections::VecDeque, rc::Rc};
use core::cell::RefCell;
use leptos::{expect_context, provide_context, use_context, Scope};

/// Frame within which the widget will be rendered.
#[derive(Clone, Default, Debug)]
pub struct Frame {
    /// X-coordinate (in pixels) of the to left corner.
    pub x: i32,
    /// Y-coordinate (in pixels) of the to left corner.
    pub y: i32,
    /// Width (in pixels).
    pub width: i32,
    /// Height (in pixels).
    pub height: i32,
}

/// Frames stack.
#[derive(Clone, Default, Debug)]
pub struct Frames(VecDeque<Frame>);

/// Frames stack reference.
pub type FramesRef = Rc<RefCell<Frames>>;

impl Frames {
    /// Pushes a new child frame.
    pub fn push(&mut self, frame: Frame) {
        self.0.push_back(frame);
    }

    /// Pops the first child frame or initial frame if there are no children.
    pub fn pop(&mut self) -> Frame {
        let frames = &mut self.0;
        if frames.len() > 1 {
            frames.pop_back().unwrap()
        } else {
            frames.back().unwrap().clone()
        }
    }
}

/// Provides frame to the context.
pub fn provide_frame(cx: Scope, frame: Frame) {
    let frames: Option<FramesRef> = use_context(cx);
    if let Some(frames) = frames {
        frames.borrow_mut().0.push_back(frame);
    } else {
        let frames = Frames([frame].into());
        provide_context(cx, Rc::new(RefCell::new(frames)));
    }
}

/// Returns the current frame.
pub fn use_frame(cx: Scope) -> Frame {
    let frames: FramesRef = expect_context(cx);
    let mut frames = frames.borrow_mut();
    frames.pop()
}

/// Returns frames stack from context.
pub fn use_frames(cx: Scope) -> FramesRef {
    expect_context(cx)
}
