#[derive(Clone, Copy, PartialEq, Debug)]
pub enum BlockId {
    Root,
    SubscriptionsWindow,
    TabNav,
    SubscribeInput,
    SubscriptionsList,
    SubscriptionsListItem(usize),
}

pub struct Navigation(Vec<BlockId>);

impl Navigation {
    pub fn default() -> Navigation {
        let mut nav = Vec::new();
        nav.push(BlockId::Root);
        nav.push(BlockId::SubscriptionsWindow);
        nav.push(BlockId::TabNav);
        Navigation(nav)
    }

    pub fn push(&mut self, block_id: BlockId) {
        self.0.push(block_id);
    }

    pub fn peek(&self) -> BlockId {
        self.0.last().cloned().unwrap_or(BlockId::Root)
    }

    pub fn parent(&self) -> BlockId {
        if self.0.len() < 2 {
            BlockId::Root
        } else {
            self.0
                .get(self.0.len() - 2)
                .cloned()
                .unwrap_or(BlockId::Root)
        }
    }

    pub fn pop(&mut self) {
        self.0.pop();
    }

    pub fn modify_top(&mut self, new_value: BlockId) {
        *self.0.last_mut().unwrap() = new_value
    }
}
