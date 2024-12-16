use crate::Graph;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

const SERIAL_TO_PARALLEL_THRESHOLD: usize = 25;

/**
Returns the constraint on nodes in a graph.

Arguments:
* `graph`: a [Graph](../../../struct.Graph.html) instance
* `nodes`: an optional vector of node names to compute the constraint for
* `weighted`: set to `true` to use edge weights when computing the betweenness centrality

# Examples

```
use graphrs::{algorithms::{structural_holes::{constraint}}, generators};
let graph = generators::social::karate_club_graph();
let constraints = constraint::constraint(&graph, None, true);
```

# References

1. Burt, Ronald S.: Structural holes and good ideas. American Journal of Sociology (110): 349–399.

*/
pub fn constraint<T, A>(
    graph: &Graph<T, A>,
    nodes: Option<Vec<T>>,
    weighted: bool,
) -> HashMap<T, f64>
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone + Send + Sync,
{
    let parallel =
        graph.number_of_nodes() > SERIAL_TO_PARALLEL_THRESHOLD && rayon::current_num_threads() > 1;
    let node_indexes: Vec<usize> = match nodes {
        Some(nodes) => nodes
            .into_iter()
            .map(|n| graph.get_node_index(&n).unwrap())
            .collect(),
        None => (0..graph.number_of_nodes()).collect(),
    };
    match parallel {
        true => node_indexes
            .into_par_iter()
            .map(|v| match is_isolated(graph, v) {
                true => (graph.get_node_by_index(&v).unwrap().name.clone(), f64::NAN),
                false => constraint_single_node(graph, v, weighted),
            })
            .collect(),
        false => node_indexes
            .into_iter()
            .map(|v| match is_isolated(graph, v) {
                true => (graph.get_node_by_index(&v).unwrap().name.clone(), f64::NAN),
                false => constraint_single_node(graph, v, weighted),
            })
            .collect(),
    }
}

#[inline]
fn constraint_single_node<T, A>(graph: &Graph<T, A>, v: usize, weighted: bool) -> (T, f64)
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let lc = graph
        .get_neighbors_nodes_by_index(&v)
        .into_iter()
        .map(|n| local_constraint(graph, v, n, weighted))
        .sum::<f64>();
    (graph.get_node_by_index(&v).unwrap().name.clone(), lc)
}

fn is_isolated<T, A>(graph: &Graph<T, A>, u: usize) -> bool
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    match graph.specs.directed {
        true => graph.get_successor_nodes_by_index(&u).is_empty(),
        false => {
            graph.get_successor_nodes_by_index(&u).is_empty()
                && graph.get_predecessor_nodes_by_index(&u).is_empty()
        }
    }
}

fn local_constraint<T, A>(graph: &Graph<T, A>, u: usize, v: usize, weighted: bool) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let direct = normalized_mutual_weight(graph, u, v, weighted);
    let indirect = graph
        .get_neighbors_nodes_by_index(&u)
        .into_iter()
        .map(|w| {
            normalized_mutual_weight(graph, u, w, weighted)
                * normalized_mutual_weight(graph, w, v, weighted)
        })
        .sum::<f64>();
    (direct + indirect).powf(2.0)
}

/**
Returns the sum of the weights of the edge from `u` to `v` and
the edge from `v` to `u`.
*/
fn mutual_weight<T, A>(graph: &Graph<T, A>, u: usize, v: usize, weighted: bool) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let a_uv = match graph.get_edge_by_indexes(u, v) {
        Ok(e) => match weighted {
            true => e.weight,
            false => 1.0,
        },
        Err(_) => 0.0,
    };
    let a_vu = match graph.get_edge_by_indexes(v, u) {
        Ok(e) => match weighted {
            true => e.weight,
            false => 1.0,
        },
        Err(_) => 0.0,
    };
    a_uv + a_vu
}

fn normalized_mutual_weight<T, A>(graph: &Graph<T, A>, u: usize, v: usize, weighted: bool) -> f64
where
    T: Hash + Eq + Clone + Ord + Display + Send + Sync,
    A: Clone,
{
    let succ = graph.get_successor_nodes_by_index(&u);
    let pred = graph.get_predecessor_nodes_by_index(&u);
    let sum_weights = succ
        .into_iter()
        .chain(pred.into_iter())
        .map(|x| match weighted {
            true => match graph.specs.directed {
                true => x.weight,
                false => x.weight * 2.0,
            },
            false => 2.0,
        })
        .sum::<f64>();
    match sum_weights == 0.0 {
        true => 0.0,
        false => mutual_weight(&graph, u, v, weighted) / sum_weights,
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::generators;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn test_mutual_weight() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.25, true, Some(1)).unwrap();
        assert_eq!(mutual_weight(&graph, 0, 1, true), 0.0);
        assert_eq!(mutual_weight(&graph, 0, 3, true), 0.1530042583196669);
        assert_eq!(mutual_weight(&graph, 3, 0, true), 0.1530042583196669);
        assert_eq!(mutual_weight(&graph, 5, 7, true), 0.1191637281251331);
        assert_eq!(mutual_weight(&graph, 0, 1, false), 0.0);
        assert_eq!(mutual_weight(&graph, 0, 3, false), 1.0);
        assert_eq!(mutual_weight(&graph, 3, 0, false), 1.0);
        assert_eq!(mutual_weight(&graph, 5, 7, false), 2.0);
    }

    #[test]
    fn test_normalized_mutual_weight_1() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.25, true, Some(1)).unwrap();
        assert_eq!(
            normalized_mutual_weight(&graph, 0 as usize, 1 as usize, true),
            0.0
        );
        assert_eq!(
            normalized_mutual_weight(&graph, 2 as usize, 3 as usize, true),
            0.18254328226130984
        );
        assert_eq!(
            normalized_mutual_weight(&graph, 5 as usize, 7 as usize, true),
            0.10686716480011678
        );
    }

    #[test]
    fn test_normalized_mutual_weight_2() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.05, true, Some(1)).unwrap();
        assert_eq!(
            normalized_mutual_weight(&graph, 0 as usize, 1 as usize, true),
            0.0
        );
        assert_eq!(
            normalized_mutual_weight(&graph, 2 as usize, 3 as usize, true),
            0.0
        );
        assert_eq!(
            normalized_mutual_weight(&graph, 5 as usize, 7 as usize, true),
            0.0
        );
    }

    #[test]
    fn test_local_constraint() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.25, true, Some(1)).unwrap();
        assert_eq!(
            local_constraint(&graph, 0 as usize, 1 as usize, true),
            0.003816247369111868
        );
        assert_eq!(
            local_constraint(&graph, 2 as usize, 3 as usize, true),
            0.1009124650499737
        );
        assert_eq!(
            local_constraint(&graph, 5 as usize, 7 as usize, true),
            0.011420590912415318
        );
    }

    #[test]
    fn test_constraint_single_node() {
        let graph = generators::social::karate_club_graph();
        let result = constraint_single_node(&graph, 0, true);
        assert_approx_eq!(result.1, 0.19032543361154225);
    }

    #[test]
    fn test_constraint_1() {
        let graph = generators::random::fast_gnp_random_graph(10, 0.25, true, Some(1)).unwrap();
        let result = constraint(&graph, None, true);
        assert_approx_eq!(result[&0], 0.5225162121022704);
        assert_approx_eq!(result[&1], 0.40854394753778966);
        assert_approx_eq!(result[&2], 0.35853243308197136);
        assert_approx_eq!(result[&3], 0.3835859618816542);
        assert_approx_eq!(result[&4], 0.8374885109240358);
        assert_approx_eq!(result[&5], 0.43439223368313096);
        assert_approx_eq!(result[&6], 0.549748359896555);
        assert_approx_eq!(result[&7], 0.7476383279585774);
        assert_approx_eq!(result[&8], 0.7547822068364783);
        assert_approx_eq!(result[&9], 0.4225977961291851);
    }

    #[test]
    fn test_constraint_2() {
        let graph = generators::social::karate_club_graph();
        let result = constraint(&graph, None, true);
        assert_approx_eq!(result[&0], 0.19032543361154225);
        assert_approx_eq!(result[&1], 0.3409019831285561);
        assert_approx_eq!(result[&2], 0.25690017307874163);
        assert_approx_eq!(result[&3], 0.4122226156115594);
        assert_approx_eq!(result[&4], 0.508088569654933);
        assert_approx_eq!(result[&5], 0.49112922777491624);
        assert_approx_eq!(result[&6], 0.5314047216519745);
        assert_approx_eq!(result[&7], 0.527081950416345);
        assert_approx_eq!(result[&8], 0.3363511339204627);
        assert_approx_eq!(result[&9], 0.5555555555555556);
        assert_approx_eq!(result[&10], 0.5304976482780612);
        assert_approx_eq!(result[&11], 1.0);
        assert_approx_eq!(result[&12], 0.7302295918367347);
        assert_approx_eq!(result[&13], 0.3827486243156607);
        assert_approx_eq!(result[&14], 0.6391917324128862);
        assert_approx_eq!(result[&15], 0.6305808812676944);
        assert_approx_eq!(result[&16], 0.9397491245018718);
        assert_approx_eq!(result[&17], 0.6653314075883578);
        assert_approx_eq!(result[&18], 0.6654807235736722);
        assert_approx_eq!(result[&19], 0.4391093639199636);
        assert_approx_eq!(result[&20], 0.7220631536098783);
        assert_approx_eq!(result[&21], 0.6236083811249431);
        assert_approx_eq!(result[&22], 0.6380745603221565);
        assert_approx_eq!(result[&23], 0.3120631827133658);
        assert_approx_eq!(result[&24], 0.5124716553287981);
        assert_approx_eq!(result[&25], 0.4563144985885511);
        assert_approx_eq!(result[&26], 0.65316243881949);
        assert_approx_eq!(result[&27], 0.3222101463859706);
        assert_approx_eq!(result[&28], 0.39898274124464594);
        assert_approx_eq!(result[&29], 0.4376298799954841);
        assert_approx_eq!(result[&30], 0.4041638692261506);
        assert_approx_eq!(result[&31], 0.27518142164082954);
        assert_approx_eq!(result[&32], 0.2568971955830605);
        assert_approx_eq!(result[&33], 0.18865024812268144);
    }
}
