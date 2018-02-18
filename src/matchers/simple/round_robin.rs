pub struct RoundRobin<P: Clone> {
    players: Vec<P>,
    round: usize,
}
impl<P: Clone> RoundRobin<P> {
    pub fn new(players: Vec<P>) -> RoundRobin<P> {
        assert!(players.len() > 0);

        RoundRobin {
            players,
            round: 0
        }
    }
}
impl<P: Clone> Iterator for RoundRobin<P> {
    type Item = Vec<P>;

    fn next(&mut self) -> Option<Vec<P>> {
        let i1 = self.round / self.players.len();
        let i2 = self.round % self.players.len();

        self.round += 1;

        if i1 == self.players.len() {
            return None;
        }

        if i1 >= i2 {
            return self.next();
        }

        Some(vec![
            self.players[i1].clone(),
            self.players[i2].clone()
        ])
    }
}


#[cfg(test)]
mod tests {
    use super::RoundRobin;

    #[test]
    fn it_works() {
        for n in 1..20 {
            let r: Vec<Vec<usize>> = RoundRobin::new(vec![1; n]).collect();
            assert_eq!(r.len(), (0..n).into_iter().sum::<usize>());
        }

        for n in 1..20 {
            let r: Vec<Vec<usize>> = RoundRobin::new((0..n).into_iter().collect()).collect();
            assert_eq!(r.len(), (0..n).into_iter().sum::<usize>());
        }
    }
}
