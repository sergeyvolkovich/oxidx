use strum::FromRepr;
use windows::Win32::Graphics::Direct3D12::*;

/// Defines priority levels for a command queue.
///
/// For more information: [`D3D12_COMMAND_QUEUE_PRIORITY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_queue_priority)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum CommandQueuePriority {
    #[default]
    /// Normal priority.
    Normal = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL.0,

    /// High priority.
    High = D3D12_COMMAND_QUEUE_PRIORITY_HIGH.0,

    /// Global realtime priority.
    GlobalRealtime = D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME.0,
}

/// Specifies the type of a command list.
///
/// For more information: [`D3D12_COMMAND_LIST_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_command_list_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum CommandListType {
    #[default]
    /// Specifies a command buffer that the GPU can execute. A direct command list doesn't inherit any GPU state.
    Direct = D3D12_COMMAND_LIST_TYPE_DIRECT.0,

    /// Specifies a command buffer that can be executed only directly via a direct command list.
    /// A bundle command list inherits all GPU state (except for the currently set pipeline state object and primitive topology).
    Bundle = D3D12_COMMAND_LIST_TYPE_BUNDLE.0,

    /// Specifies a command buffer for computing.
    Compute = D3D12_COMMAND_LIST_TYPE_COMPUTE.0,

    /// Specifies a command buffer for copying.
    Copy = D3D12_COMMAND_LIST_TYPE_COPY.0,

    /// Specifies a command buffer for video decoding.
    VideoDecode = D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE.0,

    /// Specifies a command buffer for video processing.
    VideoProcess = D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS.0,

    /// Specifies a command buffer for video encoding.
    VideoEncode = D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE.0,
}

/// Specifies the CPU-page properties for the heap.
///
/// For more information: [`D3D12_CPU_PAGE_PROPERTY enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_cpu_page_property)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum CpuPageProperty {
    /// The CPU-page property is unknown.
    Unknown = D3D12_CPU_PAGE_PROPERTY_UNKNOWN.0,

    /// The CPU cannot access the heap, therefore no page properties are available.
    NotAvailable = D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE.0,

    /// The CPU-page property is write-combined.
    WriteCombine = D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE.0,

    /// The CPU-page property is write-back.
    WriteBack = D3D12_CPU_PAGE_PROPERTY_WRITE_BACK.0,
}

/// Specifies a type of descriptor heap.
///
/// For more information: [`D3D12_DESCRIPTOR_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_descriptor_heap_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum DescriptorHeapType {
    /// The descriptor heap for the render-target view.
    #[default]
    Rtv = D3D12_DESCRIPTOR_HEAP_TYPE_RTV.0,

    /// The descriptor heap for the depth-stencil view.
    Dsv = D3D12_DESCRIPTOR_HEAP_TYPE_DSV.0,

    /// The descriptor heap for the combination of constant-buffer, shader-resource, and unordered-access views.
    CbvSrvUav = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV.0,

    /// The descriptor heap for the sampler.
    Sampler = D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER.0,
}

/// Defines constants that specify a Direct3D 12 feature or feature set to query about.
///
/// For more information: [`D3D12_FEATURE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_feature)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum FeatureType {
    /// Indicates a query for the level of support for basic Direct3D 12 feature options.
    Options = D3D12_FEATURE_D3D12_OPTIONS.0,

    /// Indicates a query for the adapter's architectural details, so that your application can better optimize for certain adapter properties.
    Architecture = D3D12_FEATURE_ARCHITECTURE.0,

    /// Indicates a query for info about the feature levels supported.
    FeatureLevels = D3D12_FEATURE_FEATURE_LEVELS.0,

    /// Indicates a query for the resources supported by the current graphics driver for a given format.
    FormatSupport = D3D12_FEATURE_FORMAT_SUPPORT.0,

    /// Indicates a query for the image quality levels for a given format and sample count.
    MultisampleQualityLevels = D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS.0,

    /// Indicates a query for the DXGI data format.
    FormatInfo = D3D12_FEATURE_FORMAT_INFO.0,

    /// Indicates a query for the GPU's virtual address space limitations.
    GpuVirtualAddressSupport = D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT.0,

    /// Indicates a query for the supported shader model.
    FeatureShaderModel = D3D12_FEATURE_SHADER_MODEL.0,

    /// Indicates a query for the level of support for HLSL 6.0 wave operations.
    Options1 = D3D12_FEATURE_D3D12_OPTIONS1.0,

    /// Indicates a query for the level of support for protected resource sessions.
    ProtectedResourceSessionSupport = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT.0,

    /// Indicates a query for root signature version support.
    RootSignature = D3D12_FEATURE_ROOT_SIGNATURE.0,

    /// Indicates a query for each adapter's architectural details, so that your application can better optimize for certain adapter properties.
    Architecture1 = D3D12_FEATURE_ARCHITECTURE1.0,

    /// Indicates a query for the level of support for depth-bounds tests and programmable sample positions.
    Options2 = D3D12_FEATURE_D3D12_OPTIONS2.0,

    /// Indicates a query for the level of support for shader caching.
    ShaderCache = D3D12_FEATURE_SHADER_CACHE.0,

    /// Indicates a query for the adapter's support for prioritization of different command queue types.
    CommandQueuePriority = D3D12_FEATURE_COMMAND_QUEUE_PRIORITY.0,

    /// Indicates a query for the level of support for timestamp queries, format-casting, immediate write, view instancing, and barycentrics.
    Options3 = D3D12_FEATURE_D3D12_OPTIONS3.0,

    /// Indicates a query for whether or not the adapter supports creating heaps from existing system memory.
    ExistingHeaps = D3D12_FEATURE_EXISTING_HEAPS.0,

    /// Indicates a query for the level of support for 64KB-aligned MSAA textures, cross-API sharing, and native 16-bit shader operations.
    Options4 = D3D12_FEATURE_D3D12_OPTIONS4.0,

    /// Indicates a query for the level of support for heap serialization.
    Serialization = D3D12_FEATURE_SERIALIZATION.0,

    /// Indicates a query for the level of support for the sharing of resources between different adapters—for example, multiple GPUs.
    CrossNode = D3D12_FEATURE_CROSS_NODE.0,

    /// Starting with Windows 10, version 1809 (10.0; Build 17763), indicates a query for the level of support for render passes, ray tracing, and shader-resource view tier 3 tiled resources.
    Options5 = D3D12_FEATURE_D3D12_OPTIONS5.0,

    /// Starting with Windows 11 (Build 10.0.22000.194).
    Displayable = D3D12_FEATURE_DISPLAYABLE.0,

    /// Starting with Windows 10, version 1903 (10.0; Build 18362), indicates a query for the level of support for variable-rate shading (VRS), and indicates whether or not background processing is supported.
    Options6 = D3D12_FEATURE_D3D12_OPTIONS6.0,

    /// Indicates a query for the level of support for metacommands.
    QueryMetaCommand = D3D12_FEATURE_QUERY_META_COMMAND.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query for the level of support for mesh and amplification shaders, and for sampler feedback.
    Options7 = D3D12_FEATURE_D3D12_OPTIONS7.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query to retrieve the count of protected resource session types.
    ProtectedResourceSessionTypeCount = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPE_COUNT.0,

    /// Starting with Windows 10, version 2004 (10.0; Build 19041), indicates a query to retrieve the list of protected resource session types.
    ProtectedResourceSessionTypes = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_TYPES.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not unaligned block-compressed textures are supported.
    Options8 = D3D12_FEATURE_D3D12_OPTIONS8.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not support exists for mesh shaders, values of SV_RenderTargetArrayIndex
    /// that are 8 or greater, typed resource 64-bit integer atomics, derivative and derivative-dependent texture sample operations, and the level of
    /// support for WaveMMA (wave_matrix) operations.
    Options9 = D3D12_FEATURE_D3D12_OPTIONS9.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not the SUM combiner can be used, and whether or not SV_ShadingRate can be set from a mesh shader.
    Options10 = D3D12_FEATURE_D3D12_OPTIONS10.0,

    /// Starting with Windows 11 (Build 10.0.22000.194), indicates whether or not 64-bit integer atomics on resources in descriptor heaps are supported.
    Options11 = D3D12_FEATURE_D3D12_OPTIONS11.0,

    /// TBD
    Options12 = D3D12_FEATURE_D3D12_OPTIONS12.0,

    /// TBD
    Options13 = D3D12_FEATURE_D3D12_OPTIONS13.0,

    /// TBD
    Options14 = D3D12_FEATURE_D3D12_OPTIONS14.0,

    /// TBD
    Options15 = D3D12_FEATURE_D3D12_OPTIONS15.0,

    /// TBD
    Options16 = D3D12_FEATURE_D3D12_OPTIONS16.0,

    /// TBD
    Options17 = D3D12_FEATURE_D3D12_OPTIONS17.0,

    /// TBD
    Options18 = D3D12_FEATURE_D3D12_OPTIONS18.0,

    /// TBD
    Options19 = D3D12_FEATURE_D3D12_OPTIONS19.0,

    /// TBD
    Options20 = D3D12_FEATURE_D3D12_OPTIONS20.0,

    /// TBD
    Predication = D3D12_FEATURE_PREDICATION.0,

    /// TBD
    PlacedResourceSupportInfo = D3D12_FEATURE_PLACED_RESOURCE_SUPPORT_INFO.0,

    /// TBD
    HardwareCopy = D3D12_FEATURE_HARDWARE_COPY.0,
}

/// Heap alignment variants.
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(u64)]
pub enum HeapAlignment {
    /// An alias for 64KB.
    #[default]
    Default = 0,

    /// Defined as 64KB.
    ResourcePlacement = D3D12_DEFAULT_RESOURCE_PLACEMENT_ALIGNMENT as u64,

    /// Defined as 4MB. An application must decide whether the heap will contain multi-sample anti-aliasing (MSAA), in which case, the application must choose this
    MsaaResourcePlacement = D3D12_DEFAULT_MSAA_RESOURCE_PLACEMENT_ALIGNMENT as u64,
}

/// Specifies the type of heap. When resident, heaps reside in a particular physical memory pool with certain CPU cache properties.
///
/// For more information: [`D3D12_HEAP_TYPE enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_heap_type)
#[derive(Clone, Copy, Debug, Default, FromRepr)]
#[repr(i32)]
pub enum HeapType {
    /// Specifies the default heap. This heap type experiences the most bandwidth for the GPU, but cannot provide CPU access.
    /// The GPU can read and write to the memory from this pool, and resource transition barriers may be changed.
    /// The majority of heaps and resources are expected to be located here, and are typically populated through resources in upload heaps.
    #[default]
    Default = D3D12_HEAP_TYPE_DEFAULT.0,

    /// Specifies a heap used for uploading. This heap type has CPU access optimized for uploading to the GPU,
    /// but does not experience the maximum amount of bandwidth for the GPU. This heap type is best for CPU-write-once, GPU-read-once data;
    /// but GPU-read-once is stricter than necessary. GPU-read-once-or-from-cache is an acceptable use-case for the data;
    /// but such usages are hard to judge due to differing GPU cache designs and sizes.
    /// If in doubt, stick to the GPU-read-once definition or profile the difference on many GPUs between copying the data to a _DEFAULT heap vs.
    /// reading the data from an _UPLOAD heap.
    Upload = D3D12_HEAP_TYPE_UPLOAD.0,

    /// Specifies a heap used for reading back. This heap type has CPU access optimized for reading data back from the GPU,
    /// but does not experience the maximum amount of bandwidth for the GPU. This heap type is best for GPU-write-once, CPU-readable data.
    /// The CPU cache behavior is write-back, which is conducive for multiple sub-cache-line CPU reads.
    Readback = D3D12_HEAP_TYPE_READBACK.0,

    /// Specifies a custom heap. The application may specify the memory pool and CPU cache properties directly, which can be useful for UMA optimizations,
    /// multi-engine, multi-adapter, or other special cases. To do so, the application is expected to understand the adapter architecture to make the right choice.
    Custom = D3D12_HEAP_TYPE_CUSTOM.0,

    /// TBD
    GpuUpload = D3D12_HEAP_TYPE_GPU_UPLOAD.0,
}

/// Specifies the memory pool for the heap.
///
/// For more information: [`D3D12_MEMORY_POOL enumeration`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12/ne-d3d12-d3d12_memory_pool)
#[derive(Clone, Copy, Debug, FromRepr)]
#[repr(i32)]
pub enum MemoryPool {
    /// The memory pool is unknown.
    Unknown = D3D12_MEMORY_POOL_UNKNOWN.0,

    /// The memory pool is L0.
    ///
    /// L0 is the physical system memory pool.
    ///
    /// When the adapter is discrete/NUMA, this pool has greater bandwidth for the CPU and less bandwidth for the GPU.
    ///
    /// When the adapter is UMA, this pool is the only one which is valid.
    L0 = D3D12_MEMORY_POOL_L0.0,

    /// The memory pool is L1.
    ///
    /// L1 is typically known as the physical video memory pool.
    ///
    /// L1 is only available when the adapter is discrete/NUMA, and has greater bandwidth for the GPU and cannot even be accessed by the CPU.
    ///
    /// When the adapter is UMA, this pool is not available.
    L1 = D3D12_MEMORY_POOL_L1.0,
}
