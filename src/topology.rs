//! Provides operations, data structures and error definitions for Disciple objects
//! which form the basis for topologies of neural networks.

use std::slice::Iter;
use activation::Activation;

/// Represents the topology element for a fully connected layer
/// with input neurons, output neurons and an activation function.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Layer{
	/// Number of input neurons to this layer.
	pub inputs: usize,

	/// Numer of output neurons from this layer.
	pub outputs: usize,

	/// Activation function for this layer.
	pub activation: Activation
}

impl Layer {
	/// Create a new layer.
	fn new(inputs: usize, outputs: usize, activation: Activation) -> Self {
		Layer{
			inputs: inputs,
			outputs: outputs,
			activation: activation
		}
	}
}

/// Used to build topologies and do some minor compile-time and 
/// runtime checks to enforce validity of the topology as a shape for neural nets.
/// 
/// Can be used by `Mentor` types to train it and become a trained neural network
/// with which the user can predict data.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyBuilder {
	last  : usize,
	layers: Vec<Layer>
}

/// Represents the neural network topology.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Topology {
	layers: Vec<Layer>
}

impl Topology {
	/// Creates a new topology.
	/// 
	/// # Panics
	/// 
	/// If size is zero.
	pub fn input(size: usize) -> TopologyBuilder {
		assert!(size >= 1, "cannot define a zero-sized input layer");

		TopologyBuilder{
			last  : size,
			layers: vec![]
		}
	}

	/// Returns the number of input neurons.
	///
	/// Used by mentors to validate their sample sizes.
	pub fn len_input(&self) -> usize {
		self.layers
			.first()
			.expect("a finished disciple must have a valid first layer!")
			.inputs
	}

	/// Returns the number of output neurons.
	///
	/// Used by mentors to validate their sample sizes.
	pub fn len_output(&self) -> usize {
		self.layers
			.last()
			.expect("a finished disciple must have a valid last layer!")
			.outputs
	}

	/// Iterates over the layer sizes of this topology.
	pub fn iter_layers(&self) -> Iter<Layer> {
		self.layers.iter()
	}
}

impl TopologyBuilder {
	fn push_layer(&mut self, layer_size: usize, act: Activation) {
		assert!(layer_size >= 1, "cannot define a zero-sized hidden layer");

		self.layers.push(Layer::new(self.last, layer_size, act));
		self.last = layer_size;
	}

	/// Adds a hidden layer to this topology with the given amount of neurons.
	///
	/// Bias-Neurons are implicitely added!
	/// 
	/// # Panics
	/// 
	/// If `layer_size` is zero.
	pub fn layer(mut self, layer_size: usize, act: Activation) -> TopologyBuilder {
		self.push_layer(layer_size, act);
		self
	}

	/// Adds some hidden layers to this topology with the given amount of neurons.
	///
	/// Bias-Neurons are implicitely added!
	/// 
	/// # Panics
	/// 
	/// If any of the specified layer sizes is zero.
	pub fn layers(mut self, layers: &[(usize, Activation)]) -> TopologyBuilder {
		for &layer in layers {
			self.push_layer(layer.0, layer.1);
		}
		self
	}

	/// Finishes constructing a topology by defining its output layer neurons.
	///
	/// Bias-Neurons are implicitely added!
	/// 
	/// # Panics
	/// 
	/// If `layer_size` is zero.
	pub fn output(mut self, layer_size: usize, act: Activation) -> Topology {
		assert!(layer_size >= 1, "cannot define a zero-sized output layer");

		self.push_layer(layer_size, act);
		Topology {
			layers: self.layers,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn construction() {
		use self::Activation::{Logistic, Identity, ReLU, Tanh};
		let dis = Topology::input(2)
			.layer(5, Logistic)
			.layers(&[
				(10, Identity),
				(10, ReLU)
			])
			.output(5, Tanh);
		let mut it = dis.iter_layers()
			.map(|&size| size);
		assert_eq!(it.next(), Some(Layer::new(2, 5, Logistic)));
		assert_eq!(it.next(), Some(Layer::new(5, 10, Identity)));
		assert_eq!(it.next(), Some(Layer::new(10, 10, ReLU)));
		assert_eq!(it.next(), Some(Layer::new(10, 5, Tanh)));
	}
}
