use egui::{Id, Pos2, Rect, Vec2};

/// The state of a [`Surface::Window`](crate::Surface::Window).
///
/// Doubles as a handle for the surface, allowing the user to set its size and position.
#[derive(Debug, Clone)]
pub struct WindowState {
    /// The [`Rect`] that this window was last taking up.
    screen_rect: Rect,

    /// Was this window dragged in the last frame?
    dragged: bool,

    /// The next position this window should be set to next frame.
    next_position: Option<Pos2>,

    /// The next size this window should be set to next frame.
    next_size: Option<Vec2>,

    /// true the first frame this window is drawn.
    /// handles opening collapsing header, etc.
    new: bool,
}

impl Default for WindowState {
    fn default() -> Self {
        Self {
            screen_rect: Rect::NOTHING,
            dragged: false,
            next_position: None,
            next_size: None,
            new: true,
        }
    }
}

impl WindowState {
    /// Create a default window state.
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Set the position for this window in screen coordinates.
    pub fn set_position(&mut self, position: Pos2) -> &mut Self {
        self.next_position = Some(position);
        self
    }

    /// Set the size of this window in egui points.
    pub fn set_size(&mut self, size: Vec2) -> &mut Self {
        self.next_size = Some(size);
        self
    }

    /// Get the [`Rect`] which this window occupies.
    /// If this window hasn't been shown before, this will be [`Rect::NOTHING`].
    pub fn rect(&self) -> Rect {
        self.screen_rect
    }

    /// Returns if this window is currently being dragged or not.
    pub fn dragged(&self) -> bool {
        self.dragged
    }

    #[inline(always)]
    pub(crate) fn next_position(&mut self) -> Option<Pos2> {
        self.next_position.take()
    }

    #[inline(always)]
    pub(crate) fn next_size(&mut self) -> Option<Vec2> {
        self.next_size.take()
    }

    //the 'static in this case means that the `open` field is always `None`
    pub(crate) fn create_window(&mut self, id: Id, bounds: Rect) -> (egui::Window<'static>, bool) {
        let new = self.new;
        let mut window_constructor = egui::Window::new("")
            .id(id)
            .drag_bounds(bounds)
            .title_bar(false);

        if let Some(position) = self.next_position() {
            window_constructor = window_constructor.current_pos(position);
        }
        if let Some(size) = self.next_size() {
            window_constructor = window_constructor.fixed_size(size);
        }
        self.new = false;
        (window_constructor, new)
    }
}
