// xfail-license
use dom::clientrect::ClientRect;
use dom::bindings::utils::WrapperCache;

pub struct ClientRectList {
    wrapper: WrapperCache,
    rects: ~[@mut ClientRect]
}

pub impl ClientRectList {
    fn new(rects: ~[@mut ClientRect]) -> @mut ClientRectList {
        let list = @mut ClientRectList {
            wrapper: WrapperCache::new(),
            rects: rects
        };
        list.init_wrapper();
        list
    }

    fn Length(&self) -> u32 {
        self.rects.len() as u32
    }

    fn Item(&self, index: u32) -> Option<@mut ClientRect> {
        if index < self.rects.len() as u32 {
            Some(self.rects[index])
        } else {
            None
        }
    }

    fn IndexedGetter(&self, index: u32, found: &mut bool) -> Option<@mut ClientRect> {
        *found = index < self.rects.len() as u32;
        self.Item(index)
    }
}
