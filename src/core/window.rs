// Window Container
// Contains a PistonWindow and a list of widgets
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

use crate::widget::widget::*;
use crate::core::point::*;

use piston_window::*;

pub struct PushrodWindow {
    pub window: PistonWindow,
    widgets: Vec<Box<dyn PushrodWidget>>,
}

impl PushrodWindow {
    pub fn new(window: PistonWindow) -> Self {
        Self {
            window,
            widgets: Vec::new(),
        }
    }

    pub fn add_widget(&mut self, widget: Box<dyn PushrodWidget>) {
        self.widgets.push(widget);
    }

    pub fn get_widget_id_for_point(&mut self, point: Point) -> u8 {
        for pos in 0..self.widgets.len() {
            // TODO Convert Box<dyn PushrodWidget> to PushrodWidget object that can be called
            // TODO Call PushrodWidget.get_point() and PushrodWidget.get_size() here
        }

        0
    }
}
