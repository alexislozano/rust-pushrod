// Widget Base Definition
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use piston_window::*;

use crate::core::callbacks::*;
use crate::core::point::*;
use crate::widget::config::*;

/// Implementable trait that is used by every `Widget`.  These are the public methods,
/// and a function _may_ override them.
///
/// You _must_ implement the following methods:
///
/// - config
/// - callbacks
///
/// You _should_ override `draw`, but you are not required to.  (If you don't, however, your
/// widget won't really do much.)
///
/// If you want a blank base widget, refer to the `CanvasWidget`, which will create a
/// base widget that paints the contents of its bounds with whatever color has been
/// specified with `set_color`.
pub trait Widget {
    /// Retrieves the configuration HashMap that stores the configuration list of settings
    /// for this widget.
    ///
    /// To implement this, the following code could be used in your object's structure:
    ///
    /// ```
    /// # use pushrod::widget::widget::*;
    /// # use pushrod::widget::config::*;
    /// struct MyWidget {
    ///   config: Configurable,
    /// }
    ///
    /// impl MyWidget {
    ///   fn new() -> Self {
    ///     Self {
    ///       config: Configurable::new(),
    ///     }
    ///   }
    /// }
    /// ```
    ///
    /// And in the overridden function for get_config in your implementation, use:
    ///
    /// ```
    /// # use pushrod::widget::widget::*;
    /// # use pushrod::widget::config::*;
    /// # use pushrod::core::callbacks::*;
    /// # use pushrod::core::point::Point;
    /// struct MyWidget {
    ///   config: Configurable,
    ///   callbacks: CallbackStore,
    /// }
    ///
    /// impl Widget for MyWidget {
    ///   fn config(&mut self) -> &mut Configurable {
    ///     &mut self.config
    ///   }
    ///
    ///   fn callbacks(&mut self) -> &mut CallbackStore {
    ///     &mut self.callbacks
    ///   }
    ///
    ///   // Not necessary below, but here for illustration if you want to override these calls.
    ///   fn mouse_entered(&mut self, widget_id: i32) {}
    ///   fn mouse_exited(&mut self, widget_id: i32) {}
    ///   fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {}
    /// }
    /// ```
    ///
    /// This uses a `RefCell`, since configurations require a mutable reference to the HashMap
    /// that stores the configs.
    fn config(&mut self) -> &mut Configurable;

    /// Returns the `CallbackStore` for this `Widget`.  This contains a set of callbacks that only
    /// apply to this `Widget`.
    fn callbacks(&mut self) -> &mut CallbackStore;

    /// Indicates that a widget needs to be redrawn/refreshed.
    fn invalidate(&mut self) {
        self.config().set(Invalidate);
    }

    /// Clears the invalidation flag.
    fn clear_invalidate(&mut self) {
        self.config().remove::<Invalidate>();
    }

    /// Checks to see whether or not the widget needs to be redrawn/refreshed.
    fn is_invalidated(&mut self) -> bool {
        self.config().contains_key::<Invalidate>()
    }

    /// Sets the `Point` of origin for this widget, given the X and Y origin points.  Invalidates the widget afterward.
    fn set_origin(&mut self, x: i32, y: i32) {
        self.config().set(Origin(Point { x, y }));
        self.invalidate();
    }

    /// Retrieves the `Point` of origin for this object.
    /// Defaults to origin (0, 0) if not set.
    fn get_origin(&mut self) -> Point {
        self.config()
            .get::<Origin>()
            .unwrap_or(&Origin(Point { x: 0, y: 0 }))
            .0
            .clone()
    }

    /// Sets the `Size` for this widget, given a width and height.  Invalidates the widget afterward.
    fn set_size(&mut self, w: i32, h: i32) {
        self.config()
            .set(BodySize(crate::core::point::Size { w, h }));
        self.invalidate();
    }

    /// Retrieves the `Size` bounds for this widget.
    /// Defaults to size (0, 0) if not set.
    fn get_size(&mut self) -> crate::core::point::Size {
        self.config()
            .get::<BodySize>()
            .unwrap_or(&BodySize(crate::core::point::Size { w: 0, h: 0 }))
            .0
            .clone()
    }

    /// Sets the color for this widget.  Invalidates the widget afterward.
    fn set_color(&mut self, color: types::Color) {
        self.config().set(MainColor(color));
        self.invalidate();
    }

    /// Retrieves the color of this widget.
    /// Defaults to white color `[1.0; 4]` if not set.
    fn get_color(&mut self) -> types::Color {
        self.config()
            .get::<MainColor>()
            .unwrap_or(&MainColor([1.0; 4]))
            .0
    }

    // Callbacks

    /// Performs a callback stored in the `CallbackStore` for this `Widget`, but only for the
    /// `CallbackTypes::SingleCallback` enum type.  If the callback does not exist, or is not
    /// defined properly, it will be silently dropped and ignored.
    fn perform_single_callback(&mut self, callback_id: u32, widget_id: i32) {
        match self.callbacks().get(callback_id) {
            CallbackTypes::SingleCallback { callback } => callback(widget_id),
            _ => (),
        }
    }

    /// Performs a callback stored in the `CallbackStore` for this `Widget`, but only for the
    /// `CallbackTypes::PointCallback` enum type.  If the callback does not exist, or is not
    /// defined properly, it will be silently dropped and ignored.
    fn perform_point_callback(&mut self, callback_id: u32, widget_id: i32, point: Point) {
        match self.callbacks().get(callback_id) {
            CallbackTypes::PointCallback { callback } => callback(widget_id, point.clone()),
            _ => (),
        }
    }

    fn perform_key_callback(
        &mut self,
        callback_id: u32,
        widget_id: i32,
        key: Key,
        state: ButtonState,
    ) {
        match self.callbacks().get(callback_id) {
            CallbackTypes::KeyCallback { callback } => {
                callback(widget_id, key.clone(), state.clone())
            }
            _ => (),
        }
    }

    // Callback Triggers

    /// Called when a keyboard event takes place within the bounds of a widget.  Includes the widget
    /// ID, the key code that was affected, and its state - pressed or released.
    fn key_pressed(&mut self, widget_id: i32, key: &Key, state: &ButtonState) {
        self.perform_key_callback(CALLBACK_KEY_PRESSED, widget_id, key.clone(), state.clone());
    }

    /// Called when a mouse enters the bounds of the widget.  Includes the widget ID.  Only override
    /// if you want to signal a mouse enter event.
    fn mouse_entered(&mut self, widget_id: i32) {
        self.perform_single_callback(CALLBACK_MOUSE_ENTERED, widget_id);
    }

    /// Called when a mouse exits the bounds of the widget.  Includes the widget ID.  Only override
    /// if you want to signal a mouse exit event.
    fn mouse_exited(&mut self, widget_id: i32) {
        self.perform_single_callback(CALLBACK_MOUSE_EXITED, widget_id);
    }

    /// Called when a scroll event is called within the bounds of the widget.  Includes the widget ID.
    /// Only override if you want to signal a mouse scroll event.
    fn mouse_scrolled(&mut self, widget_id: i32, point: Point) {
        self.perform_point_callback(CALLBACK_MOUSE_SCROLLED, widget_id, point.clone());
    }

    /// Called when the mouse pointer is moved inside a widget.  Includes the widget ID and point.
    /// Only override if you want to track mouse movement.
    fn mouse_moved(&mut self, widget_id: i32, point: Point) {
        self.perform_point_callback(CALLBACK_MOUSE_MOVED, widget_id, point.clone());
    }

    // Callback Setters
    fn on_key_pressed(&mut self, callback: KeyCallback) {
        self.callbacks().put(
            CALLBACK_KEY_PRESSED,
            CallbackTypes::KeyCallback { callback },
        );
    }

    /// Sets the closure action to be performed when a mouse enters a `Widget`.
    fn on_mouse_entered(&mut self, callback: SingleCallback) {
        self.callbacks().put(
            CALLBACK_MOUSE_ENTERED,
            CallbackTypes::SingleCallback { callback },
        );
    }

    /// Sets the closure action to be performed when a mouse exits a `Widget`.
    fn on_mouse_exited(&mut self, callback: SingleCallback) {
        self.callbacks().put(
            CALLBACK_MOUSE_EXITED,
            CallbackTypes::SingleCallback { callback },
        );
    }

    /// Sets the closure action to be performed when a mouse scrolls inside a `Widget`.
    fn on_mouse_scrolled(&mut self, callback: PointCallback) {
        self.callbacks().put(
            CALLBACK_MOUSE_SCROLLED,
            CallbackTypes::PointCallback { callback },
        );
    }

    /// Sets the closure action to be performed when a mouse moves within a `Widget`.
    fn on_mouse_moved(&mut self, callback: PointCallback) {
        self.callbacks().put(
            CALLBACK_MOUSE_MOVED,
            CallbackTypes::PointCallback { callback },
        );
    }

    // Draw routines

    /// Draws the contents of the widget, provided a `piston2d` `Context` and `G2d` object.
    ///
    /// It is **highly recommended** that you call `clear_invalidate()` after the draw completes,
    /// otherwise, this will continue to be redrawn continuously (unless this is the desired
    /// behavior.)
    fn draw(&mut self, c: Context, g: &mut G2d, clip: &DrawState) {
        let size: crate::core::point::Size = self.get_size();

        Rectangle::new(self.get_color())
            .draw([0.0 as f64, 0.0 as f64, size.w as f64, size.h as f64],
            clip,
            c.transform,
            g);

        self.clear_invalidate();
    }
}

/// This is the `CanvasWidget`, which contains a top-level widget for display.  It does
/// not contain any special logic other than being a base for a display layer.
pub struct CanvasWidget {
    config: Configurable,
    callbacks: CallbackStore,
}

/// Implementation of the constructor for the `CanvasWidget`.  Creates a new base widget
/// that can be positioned anywhere on the screen.
impl CanvasWidget {
    pub fn new() -> Self {
        Self {
            config: Configurable::new(),
            callbacks: CallbackStore::new(),
        }
    }
}

/// Implementation of the `CanvasWidget` object with the `Widget` traits implemented.
/// This function only implements `config` and `callbacks`, which are used as a base for
/// all `Widget`s.
///
/// Example usage:
/// ```no_run
/// # use piston_window::*;
/// # use pushrod::core::point::*;
/// # use pushrod::core::main::*;
/// # use pushrod::widget::widget::*;
/// # fn main() {
/// #   let mut prod: Pushrod = Pushrod::new(
/// #       WindowSettings::new("Pushrod Window", [640, 480])
/// #           .opengl(OpenGL::V3_2)
/// #           .build()
/// #           .unwrap_or_else(|error| panic!("Failed to build PistonWindow: {}", error)));
/// #
///    let mut base_widget = CanvasWidget::new();
///
///    base_widget.set_origin(100, 100);
///    base_widget.set_size(200, 200);
///    base_widget.set_color([0.5, 0.5, 0.5, 1.0]);
///
///    // Widgets must be boxed, as they are trait objects.
///    let widget_id = prod.widget_store.add_widget(Box::new(base_widget));
///
///    eprintln!("Added widget: ID={}", widget_id);
///
///    let mut base_widget_2 = CanvasWidget::new();
///
///    base_widget_2.set_origin(125, 125);
///    base_widget_2.set_size(100, 100);
///    base_widget_2.set_color([0.75, 0.75, 0.75, 1.0]);
///
///    // Add the second widget to the top level base widget.
///    let widget_id_2 = prod.widget_store.add_widget_to_parent(Box::new(base_widget_2), widget_id);
/// # }
/// ```
impl Widget for CanvasWidget {
    fn config(&mut self) -> &mut Configurable {
        &mut self.config
    }

    fn callbacks(&mut self) -> &mut CallbackStore {
        &mut self.callbacks
    }
}
