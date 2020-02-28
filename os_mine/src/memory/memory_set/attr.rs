use crate::memory::paging::PageEntry;

#[derive(Clone, Debug)]
pub struct MemoryAttr {
    user: bool,  // 用户态是否可访问
    readonly: bool, // 是否只读
    execute: bool // 是否可执行
}

impl MemoryAttr {
    // 默认 用户态不可访问，可写，不可执行
    pub fn new() -> Self {
        MemoryAttr {
            user: false,
            readonly: false,
            execute: false
        }
    }
    // 根据需求修改权限
    pub fn set_user(mut self) -> Self {
        self.user = true; self
    }
    pub fn set_readonly(mut self) -> Self {
        self.readonly = true; self
    }
    pub fn set_execute(mut self) -> Self {
        self.execute = true; self
    }
    // 根据设置的权限要求修改页表项
    pub fn apply(&self, entry: &mut PageEntry) {
        entry.set_present(true);
        entry.set_user(self.user);
        entry.set_writable(!self.readonly);
        entry.set_execute(self.execute);
    }
}