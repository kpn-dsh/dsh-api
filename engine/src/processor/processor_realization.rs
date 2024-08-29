//! # Defines the behavior of a Trifonius `ProcessorRealization`

#![allow(clippy::module_inception)]

use crate::engine_target::EngineTarget;
use crate::pipeline::PipelineId;
use crate::processor::processor_descriptor::ProcessorDescriptor;
use crate::processor::processor_instance::ProcessorInstance;
use crate::processor::{ProcessorId, ProcessorIdentifier, ProcessorRealizationId, ProcessorType};

/// Defines the behavior of a Trifonius `ProcessorRealization`
pub trait ProcessorRealization<'a> {
  /// # Get this `ProcessorRealization`s descriptor
  ///
  /// ## Returns
  /// * This `ProcessorRealization`s descriptor.
  fn descriptor(&self) -> ProcessorDescriptor;

  /// # Get this `ProcessorRealization`s id (name)
  ///
  /// ## Returns
  /// * This `ProcessorRealization`s id.
  fn id(&self) -> &ProcessorRealizationId;

  /// # Get this `ProcessorRealization`s `ProcessorIdentifier`
  ///
  /// ## Returns
  /// * This `ProcessorRealization`s identifier.
  fn identifier(&self) -> &ProcessorIdentifier;

  /// # Get this `ProcessorRealization`s label
  ///
  /// A `Processor`s label should be used to present it to a user.
  ///
  /// ## Returns
  /// * This `ProcessorRealization`s label.
  fn label(&self) -> &str;

  /// # Create a `ProcessorInstance` from this `ProcessorRealization`
  ///
  /// ## Parameters
  /// * `pipeline_id` - Pipeline id wrapped in a `Some` when the created
  ///                   `ProcessorInstance` is part of a _Pipeline_,
  ///                   `None` when it is not.
  /// * `processor_id` - Processor id.
  /// * `target_client_factory` - Target client factory.
  ///
  /// ## Returns
  /// * The created `ProcessorInstance`.
  fn processor_instance(&'a self, pipeline_id: Option<&PipelineId>, processor_id: &ProcessorId, engine_target: &'a EngineTarget)
    -> Result<Box<dyn ProcessorInstance + 'a>, String>;

  /// # Get this `ProcessorRealization`s type
  ///
  /// ## Returns
  /// * This `ProcessorRealization`s type.
  fn processor_type(&self) -> ProcessorType;
}
