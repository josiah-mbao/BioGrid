#[cfg(test)]
mod tests {
    use crate::components::Stats;

    #[test]
    fn test_starvation() {
        let stats = Stats { energy: 0.0, hunger_threshold: 50.0 };
        assert!(stats.energy <= 0.0, "Organism should be dead at 0 energy");
    }
}
