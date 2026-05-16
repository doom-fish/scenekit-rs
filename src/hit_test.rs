use crate::ffi;
use crate::math::Vector3;
use crate::node::Node;
use crate::private::handle_type;
use crate::view::View;

handle_type!(HitTestResult);
handle_type!(HitTestResults);

impl HitTestResults {
    #[must_use]
    pub fn count(&self) -> usize {
        unsafe { ffi::scn_hit_test_results_count(self.ptr) }
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.count() == 0
    }

    #[must_use]
    pub fn get(&self, index: usize) -> Option<HitTestResult> {
        unsafe { HitTestResult::from_raw(ffi::scn_hit_test_results_get(self.ptr, index)) }
    }

    #[must_use]
    pub fn first(&self) -> Option<HitTestResult> {
        self.get(0)
    }
}

impl HitTestResult {
    #[must_use]
    pub fn node(&self) -> Option<Node> {
        unsafe { Node::from_raw(ffi::scn_hit_test_result_node(self.ptr)) }
    }

    #[must_use]
    pub fn world_coordinates(&self) -> Option<Vector3> {
        let mut value = Vector3::default();
        let ok = unsafe {
            ffi::scn_hit_test_result_world_coordinates(self.ptr, value.as_mut_ptr().cast())
        };
        ok.then_some(value)
    }
}

impl View {
    #[must_use]
    pub fn hit_test(&self, point: crate::CGPoint) -> Option<HitTestResults> {
        unsafe { HitTestResults::from_raw(ffi::scn_view_hit_test(self.ptr, point.x, point.y)) }
    }
}
