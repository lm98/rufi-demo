use rf_distributed::discovery::Discovery;

pub struct MockDiscovery(i32);

impl MockDiscovery {
    pub fn mock_discovery(id: i32) -> Box<dyn Discovery> {
        Box::new(MockDiscovery(id))
    }
}

impl Discovery for MockDiscovery {
    fn discover_neighbors(&self) -> Vec<i32> {
        let self_id = self.0;
        vec![self_id - 1, self_id, self_id + 1]
            .into_iter()
            .filter(|n| (n > &0 && n < &6))
            .collect()
    }
}
