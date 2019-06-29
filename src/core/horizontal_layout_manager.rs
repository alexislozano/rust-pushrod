// Horizontal Layout Manager
// Lays out Widgets in a Horizontal Area
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

use crate::core::layout_manager::*;
use crate::core::point::{Point, Size};
use crate::core::widget_store::*;

pub struct HorizontalLayoutManager {
    container_widget_id: i32,
}

impl HorizontalLayoutManager {
    pub fn new(widget_id: i32) -> Self {
        Self {
            container_widget_id: widget_id,
        }
    }
}

impl LayoutManager for HorizontalLayoutManager {
    fn do_layout(
        &mut self,
        widget_ids: Vec<i32>,
        widget_positions: Vec<Point>,
        widget_store: &mut WidgetStore,
    ) {

    }

    fn resize(
        &mut self,
        size: Size,
        widget_ids: Vec<i32>,
        widget_positions: Vec<Point>,
        widget_store: &Vec<WidgetContainer>,
    ) {

    }

    fn get_widget_id(&self) -> i32 {
        return self.container_widget_id;
    }
}
