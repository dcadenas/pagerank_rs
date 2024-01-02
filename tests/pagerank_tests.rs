#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;
    use pagerank_rs::Pagerank; // You might need the 'float-cmp' crate for floating-point comparisons

    fn round_to_places(num: f64, places: u32) -> f64 {
        let multiplier = 10f64.powi(places as i32);
        (num * multiplier).round() / multiplier
    }

    fn to_percentage(f: f64) -> f64 {
        round_to_places(100.000 * f, 1)
    }

    fn assert_rank(page_rank: &mut Pagerank, expected: &[(usize, f64)], tolerance: f64) {
        let mut expected_entries = &expected[..];
        let result = page_rank.rank(0.85, tolerance);

        for (node_id, node_rank) in result {
            let (expected_id, expected_rank) = expected_entries[0];
            expected_entries = &expected_entries[1..];

            assert_eq!(
                expected_id, node_id,
                "Node id should be {} but was {}",
                expected_id, node_id,
            );

            assert!(
                approx_eq!(
                    f64,
                    to_percentage(node_rank),
                    expected_rank,
                    epsilon = tolerance
                ),
                "Rank for {} should be {} but was {}",
                expected_id,
                expected_rank,
                node_rank,
            );
        }
    }

    #[test]
    fn test_round() {
        assert!(approx_eq!(
            f64,
            round_to_places(0.6666666, 1),
            0.7,
            epsilon = 0.1
        ));
    }

    #[test]
    fn test_rank_to_percentage() {
        assert!(approx_eq!(
            f64,
            to_percentage(0.6666666),
            66.7,
            epsilon = 0.1
        ));
    }

    #[test]
    fn test_should_enter_the_block() -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(2);
        page_rank.link(0, 1)?;

        let result = page_rank.rank(0.85, 0.0001);

        assert_ne!(0, result.len());
        Ok(())
    }

    #[test]
    fn test_should_be_possible_to_recalculate_the_ranks_after_a_new_link_is_added(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        page_rank.link(0, 1)?;
        let expected = vec![(1, 64.9), (0, 35.1)];
        assert_rank(&mut page_rank, &expected, 0.0001);

        page_rank.link(1, 2)?;
        let expected = vec![(2, 47.4), (1, 34.1), (0, 18.4)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_be_possible_to_clear_the_graph() -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        page_rank.link(0, 1)?;
        page_rank.link(1, 2)?;
        page_rank.clear();
        page_rank.link(0, 1)?;

        let expected = vec![(1, 64.9), (0, 35.1)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_not_fail_when_calculating_the_rank_of_an_empty_graph(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(1);

        let result = page_rank.rank(0.85, 0.0001);

        assert_eq!(
            0,
            result.len(),
            "Rank calculation should not have entered the block for an empty graph."
        );
        Ok(())
    }

    #[test]
    fn test_should_return_correct_results_when_having_a_dangling_node(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        // Node 2 is a dangling node because it has no outbound links.
        page_rank.link(0, 2)?;
        page_rank.link(1, 2)?;

        let expected = vec![(2, 57.4), (0, 21.3), (1, 21.3)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_not_change_the_graph_when_adding_the_same_link_many_times(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        page_rank.link(0, 2)?;
        page_rank.link(0, 2)?; // Duplicate link
        page_rank.link(0, 2)?; // Duplicate link
        page_rank.link(1, 2)?;
        page_rank.link(1, 2)?; // Duplicate link

        let expected = vec![(2, 57.4), (0, 21.3), (1, 21.3)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_return_correct_results_for_a_star_graph(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        page_rank.link(0, 2)?;
        page_rank.link(1, 2)?;
        page_rank.link(2, 2)?; // Node 2 links to itself, forming a star graph

        let expected = vec![(2, 90.0), (0, 5.0), (1, 5.0)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_be_uniform_for_a_circular_graph() -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(5);
        page_rank.link(0, 1)?;
        page_rank.link(1, 2)?;
        page_rank.link(2, 3)?;
        page_rank.link(3, 4)?;
        page_rank.link(4, 0)?; // Creates a circular graph

        let expected = vec![(0, 20.0), (1, 20.0), (2, 20.0), (3, 20.0), (4, 20.0)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_return_correct_results_for_a_converging_graph(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut page_rank = Pagerank::new(3);
        page_rank.link(0, 1)?;
        page_rank.link(0, 2)?;
        page_rank.link(1, 2)?;
        page_rank.link(2, 2)?; // Node 2 links to itself, forming a converging graph

        let expected = vec![(2, 87.9), (1, 7.1), (0, 5.0)];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }

    #[test]
    fn test_should_correctly_reproduce_the_wikipedia_example(
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Based on the example from: http://en.wikipedia.org/wiki/File:PageRanks-Example.svg
        let mut page_rank = Pagerank::new(11);
        page_rank.link(1, 2)?;
        page_rank.link(2, 1)?;
        page_rank.link(3, 0)?;
        page_rank.link(3, 1)?;
        page_rank.link(4, 3)?;
        page_rank.link(4, 1)?;
        page_rank.link(4, 5)?;
        page_rank.link(5, 4)?;
        page_rank.link(5, 1)?;
        page_rank.link(6, 1)?;
        page_rank.link(6, 4)?;
        page_rank.link(7, 1)?;
        page_rank.link(7, 4)?;
        page_rank.link(8, 1)?;
        page_rank.link(8, 4)?;
        page_rank.link(9, 4)?;
        page_rank.link(10, 4)?;

        let expected = vec![
            (1, 38.4), // Node 'b'
            (2, 34.3), // Node 'c'
            (4, 8.1),  // Node 'e'
            (3, 3.9),  // Node 'd'
            (5, 3.9),  // Node 'f'
            (0, 3.3),  // Node 'a'
            (6, 1.6),  // Node 'g'
            (7, 1.6),  // Node 'h'
            (8, 1.6),  // Node 'i'
            (9, 1.6),  // Node 'j'
            (10, 1.6), // Node 'k'
        ];
        assert_rank(&mut page_rank, &expected, 0.0001);
        Ok(())
    }
}
