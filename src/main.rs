use ffsvm::{DenseProblem, DenseSVM};
use ndarray::arr1;
use prophet::topology::{Topology, TopologyBuilder};
use prophet::NeuralNet;
use prophet::Predict;
use std::convert::TryFrom;

fn main() {
    let model = DenseSVM::try_from(include_str!("../dummy.model")).unwrap();
    let mut problem = DenseProblem::from(&model);
    let features = problem.features();
    let slice = features.as_slice_mut();

    let model = include_bytes!("../net");
    let mut net: NeuralNet = serde_json::from_slice(model).unwrap();

    let x = arr1(features.as_slice_mut());

    // Same code causes SIGILL in other project ... why doesn't it here?
    let base = net.predict(&x);
}
