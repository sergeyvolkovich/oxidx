use smallvec::SmallVec;
use windows::{
    core::{IUnknown, Interface, Param},
    Win32::Graphics::Direct3D12::*,
};

use crate::{
    command_list::CommandListInterface,
    create_type,
    error::DxError,
    impl_trait,
    resources::ResourceInterface,
    sync::Fence,
    types::{CommandListType, TileRegionSize, TiledResourceCoordinate},
    HasInterface,
};

/// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
///
/// For more information: [`ID3D12CommandQueue interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nn-d3d12-id3d12commandqueue)
pub trait CommandQueueInterface:
    for<'a> HasInterface<Raw: Interface, RawRef<'a>: Param<IUnknown>>
{
    // TODO: PIX FUNCTIONS
    // fn begin_event<'a>(&self, color: impl Into<u64>, label: &'a str);
    // fn end_event(&self);
    // fn set_marker<'a>(&self, color: impl Into<u64>, label: &'a str)

    /// Copies mappings from a source reserved resource to a destination reserved resource.
    ///
    /// # Arguments
    /// * `dst_resource` - A reference to the destination reserved resource.
    /// * `dst_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the destination reserved resource.
    /// * `src_resource` - A reference to the source reserved resource.
    /// * `src_region_start_coordinate` - A reference to a [`TiledResourceCoordinate`] structure that describes the starting coordinates of the source reserved resource.
    /// * `region_size` - A reference to a [`TileRegionSize`] structure that describes the size of the reserved region.
    ///
    /// For more information: [`ID3D12CommandQueue::CopyTileMappings method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-copytilemappings)
    fn copy_tile_mappings(
        &self,
        dst_resource: &impl ResourceInterface,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl ResourceInterface,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    );

    /// Submits an iterator of command lists for execution.
    ///
    /// # Arguments
    /// * `command_lists` - The iterator of [`CommandListInterface`] command lists to be executed.
    ///
    /// For more information: [`ID3D12CommandQueue::ExecuteCommandLists method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-executecommandlists)
    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: IntoIterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl;

    /// This method samples the CPU and GPU timestamp counters at the same moment in time.
    ///
    /// # Returns
    /// * `(u64, u64)` - (The value of the GPU timestamp counter, the value of the CPU timestamp counter)
    ///
    /// For more information: [`ID3D12CommandQueue::GetClockCalibration method`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/nf-d3d12-id3d12commandqueue-getclockcalibration)
    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError>;

    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError>;
}

create_type! {
    /// Provides methods for submitting command lists, synchronizing command list execution, instrumenting the command queue, and updating resource tile mappings.
    ///
    /// # Remarks
    /// Use [DeviceInterface::create_command_queue](`crate::device::DeviceInterface::create_command_queue`) to create a command queue object.
    CommandQueue wrap ID3D12CommandQueue
}

impl_trait! {
    impl CommandQueueInterface =>
    CommandQueue;

    fn copy_tile_mappings(
        &self,
        dst_resource: &impl ResourceInterface,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &impl ResourceInterface,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: &TileRegionSize,
    ) {
        let dst_region_start_coordinate = dst_region_start_coordinate.to_raw();
        let src_region_start_coordinate = src_region_start_coordinate.to_raw();
        let region_size = region_size.to_raw();

        unsafe {
            self.0.CopyTileMappings(
                dst_resource.as_raw_ref(),
                &dst_region_start_coordinate,
                src_resource.as_raw_ref(),
                &src_region_start_coordinate,
                &region_size,
                D3D12_TILE_MAPPING_FLAG_NONE
            );
        }
    }

    fn execute_command_lists<'cl, I, CL>(&self, command_lists: I)
    where
        I: IntoIterator<Item = &'cl CL>,
        CL: CommandListInterface + 'cl,
    {
        let command_lists = command_lists
            .into_iter()
            .map(|l| {
                Some(
                    l.as_raw()
                        .cast::<ID3D12CommandList>()
                        .expect("Can not cast command list raw into ID3D12CommandList"),
                )
            })
            .collect::<SmallVec<[_; 16]>>();
        unsafe { self.0.ExecuteCommandLists(command_lists.as_slice()) }
    }

    fn get_clock_calibration(&self) -> Result<(u64, u64), DxError> {
        let mut gpu = 0;
        let mut cpu = 0;

        unsafe {
            self.0.GetClockCalibration(&mut gpu, &mut cpu).map_err(DxError::from)?;
        }

        Ok((gpu, cpu))
    }

    fn signal(&self, fence: &Fence, value: u64) -> Result<(), DxError> {
        unsafe { self.0.Signal(fence.as_raw_ref(), value).map_err(|_| DxError::Dummy) }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CommandQueueDesc {
    pub r#type: CommandListType,
    pub priority: i32,
    pub flags: CommandQueueFlags,
    pub node_mask: u32,
}

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
    pub struct CommandQueueFlags: i32 {
        const DisableGpuTimeout = D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT.0;
    }
}
