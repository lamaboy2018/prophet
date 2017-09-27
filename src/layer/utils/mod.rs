mod error_signal_buffer;
mod matrix_base;
mod signal_buffer;
mod buffer_base;

pub(crate) use self::signal_buffer::SignalBuffer;
pub(crate) use self::error_signal_buffer::ErrorSignalBuffer;

pub(crate) use self::matrix_base::{WeightsMatrix, DeltaWeightsMatrix};
pub(crate) use self::buffer_base::{
	BiasedSignalView,
	UnbiasedSignalView,
	BiasedErrorSignalView,
	UnbiasedErrorSignalView,

	BiasedSignalViewMut,
	UnbiasedSignalViewMut,
	BiasedErrorSignalViewMut,
	UnbiasedErrorSignalViewMut,

	BiasedSignalBuffer,
	UnbiasedSignalBuffer,
	BiasedErrorSignalBuffer,
	UnbiasedErrorSignalBuffer,

	Iter,
	IterMut
};
