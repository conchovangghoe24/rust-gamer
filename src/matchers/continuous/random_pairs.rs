use rand;

pub struct RandomPairs<P: Clone + PartialEq> {
    players: Vec<P>
}

impl<P: Clone + PartialEq> RandomPairs<P> {
    pub fn new(players: Vec<P>) -> RandomPairs<P> {
        RandomPairs { players }
    }

    pub fn add_player(&mut self, player: P) {
        self.players.push(player);
    }

    pub fn remove_player(&mut self, player: P) {
        self.players.remove_item(&player);
    }
}
impl<P: Clone + PartialEq> Iterator for RandomPairs<P> {
    type Item = Vec<P>;

    fn next(&mut self) -> Option<Vec<P>> {
        assert!(self.players.len() >= 2);

        let mut rng = rand::thread_rng();
        let indices = rand::seq::sample_indices(&mut rng, self.players.len(), 2);

        Some(vec![
            self.players[indices[0]].clone(),
            self.players[indices[1]].clone()
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::RandomPairs;

    #[test]
    fn test_name() {
        let mut rp = RandomPairs::new(vec![1, 2]);

        let r = rp.next().unwrap();
        assert!(r.contains(&1) && r.contains(&2));
    }
}
