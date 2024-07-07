mod enums;
mod flags;
mod structs;

use std::mem::ManuallyDrop;

use compact_str::CompactString;
use windows::{
    core::PCSTR,
    Win32::{
        Foundation::*,
        Graphics::{
            Direct3D::*,
            Direct3D12::*,
            Dxgi::{Common::*, *},
        },
    },
};

use crate::{
    adapter::{AdapterDesc, AdapterFlags, Luid},
    error::DxError,
    resources::{
        BarrierType, RenderTargetViewDesc, ResourceBarrier, VertexBufferView, ViewDimension,
    },
    swapchain::{Rational, SwapchainDesc, SwapchainFullscreenDesc},
    types::*,
    HasInterface,
};

impl SwapchainDesc {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_CHAIN_DESC1 {
        DXGI_SWAP_CHAIN_DESC1 {
            Width: self.width,
            Height: self.height,
            Format: self.format.as_raw(),
            Stereo: self.stereo.into(),
            SampleDesc: self.sample_desc.as_raw(),
            BufferUsage: self.usage.as_raw(),
            BufferCount: self.buffer_count,
            Scaling: self.scaling.as_raw(),
            SwapEffect: self.swap_effect.as_raw(),
            AlphaMode: self.alpha_mode.as_raw(),
            Flags: self.flags.bits() as u32,
        }
    }
}

impl FrameBufferUsage {
    pub(crate) fn as_raw(&self) -> DXGI_USAGE {
        DXGI_USAGE(self.bits())
    }
}

impl Scaling {
    pub(crate) fn as_raw(&self) -> DXGI_SCALING {
        DXGI_SCALING(*self as i32)
    }
}

impl SwapEffect {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_EFFECT {
        DXGI_SWAP_EFFECT(*self as i32)
    }
}

impl AlphaMode {
    pub(crate) fn as_raw(&self) -> DXGI_ALPHA_MODE {
        DXGI_ALPHA_MODE(*self as i32)
    }
}

impl SwapchainFullscreenDesc {
    pub(crate) fn as_raw(&self) -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            RefreshRate: self.rational.as_raw(),
            ScanlineOrdering: self.scanline_ordering.as_raw(),
            Scaling: self.scaling.as_raw(),
            Windowed: self.windowed.into(),
        }
    }
}

impl Rational {
    pub(crate) fn as_raw(&self) -> DXGI_RATIONAL {
        DXGI_RATIONAL {
            Numerator: self.numerator,
            Denominator: self.denominator,
        }
    }
}

impl ScanlineOrdering {
    pub(crate) fn as_raw(&self) -> DXGI_MODE_SCANLINE_ORDER {
        DXGI_MODE_SCANLINE_ORDER(*self as i32)
    }
}

impl ScalingMode {
    pub(crate) fn as_raw(&self) -> DXGI_MODE_SCALING {
        DXGI_MODE_SCALING(*self as i32)
    }
}

impl From<DXGI_ADAPTER_DESC1> for AdapterDesc {
    fn from(value: DXGI_ADAPTER_DESC1) -> Self {
        AdapterDesc {
            description: CompactString::from_utf16_lossy(value.Description),
            vendor_id: value.VendorId,
            device_id: value.DeviceId,
            sub_sys_id: value.SubSysId,
            revision: value.Revision,
            dedicated_video_memory: value.DedicatedVideoMemory,
            dedicated_system_memory: value.SharedSystemMemory,
            shared_system_memory: value.SharedSystemMemory,
            adapter_luid: Luid {
                low_part: value.AdapterLuid.LowPart,
                high_part: value.AdapterLuid.HighPart,
            },
            flags: AdapterFlags::from_bits(value.Flags).unwrap_or(AdapterFlags::empty()),
        }
    }
}

impl Viewport {
    pub(crate) fn as_raw(&self) -> D3D12_VIEWPORT {
        D3D12_VIEWPORT {
            TopLeftX: self.x,
            TopLeftY: self.y,
            Width: self.width,
            Height: self.height,
            MinDepth: self.min_depth,
            MaxDepth: self.max_depth,
        }
    }
}

impl Rect {
    pub(crate) fn as_raw(&self) -> RECT {
        RECT {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}

impl RenderTargetViewDesc {
    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_VIEW_DESC {
        D3D12_RENDER_TARGET_VIEW_DESC {
            Format: self.format.as_raw(),
            ViewDimension: self.dimension.as_type_raw(),
            Anonymous: self.dimension.as_raw(),
        }
    }
}

impl ViewDimension {
    pub(crate) fn as_type_raw(&self) -> D3D12_RTV_DIMENSION {
        match self {
            ViewDimension::Buffer { .. } => D3D12_RTV_DIMENSION_BUFFER,
            ViewDimension::Tex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1D,
            ViewDimension::Tex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2D,
            ViewDimension::Tex3D { .. } => D3D12_RTV_DIMENSION_TEXTURE3D,
            ViewDimension::ArrayTex1D { .. } => D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
            ViewDimension::ArrayTex2D { .. } => D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
            ViewDimension::Tex2DMs => D3D12_RTV_DIMENSION_TEXTURE2DMS,
            ViewDimension::Array2DMs { .. } => D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
        }
    }

    pub(crate) fn as_raw(&self) -> D3D12_RENDER_TARGET_VIEW_DESC_0 {
        match self {
            ViewDimension::Buffer {
                first_element,
                num_elements,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Buffer: D3D12_BUFFER_RTV {
                    FirstElement: *first_element,
                    NumElements: *num_elements,
                },
            },
            ViewDimension::Tex1D { mip_slice } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1D: D3D12_TEX1D_RTV {
                    MipSlice: *mip_slice,
                },
            },
            ViewDimension::Tex2D {
                mip_slice,
                plane_slice,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2D: D3D12_TEX2D_RTV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                },
            },
            ViewDimension::Tex3D {
                mip_slice,
                first_w_slice,
                w_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture3D: D3D12_TEX3D_RTV {
                    MipSlice: *mip_slice,
                    FirstWSlice: *first_w_slice,
                    WSize: *w_size,
                },
            },
            ViewDimension::ArrayTex1D {
                mip_slice,
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture1DArray: D3D12_TEX1D_ARRAY_RTV {
                    MipSlice: *mip_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            ViewDimension::ArrayTex2D {
                mip_slice,
                plane_slice,
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DArray: D3D12_TEX2D_ARRAY_RTV {
                    MipSlice: *mip_slice,
                    PlaneSlice: *plane_slice,
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
            ViewDimension::Tex2DMs => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMS: D3D12_TEX2DMS_RTV {
                    UnusedField_NothingToDefine: 0,
                },
            },
            ViewDimension::Array2DMs {
                first_array_slice,
                array_size,
            } => D3D12_RENDER_TARGET_VIEW_DESC_0 {
                Texture2DMSArray: D3D12_TEX2DMS_ARRAY_RTV {
                    FirstArraySlice: *first_array_slice,
                    ArraySize: *array_size,
                },
            },
        }
    }
}

impl RootSignatureFlags {
    pub(crate) fn as_raw(&self) -> D3D12_ROOT_SIGNATURE_FLAGS {
        D3D12_ROOT_SIGNATURE_FLAGS(self.bits())
    }
}

impl StaticSamplerDesc {
    pub(crate) fn as_raw(&self) -> D3D12_STATIC_SAMPLER_DESC {
        todo!()
    }
}

impl<'a> RootParameter<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_ROOT_PARAMETER {
        D3D12_ROOT_PARAMETER {
            ParameterType: self.r#type.as_type_raw(),
            Anonymous: self.r#type.as_raw(),
            ShaderVisibility: self.visibility.as_raw(),
        }
    }
}

impl ShaderVisibility {
    pub(crate) fn as_raw(&self) -> D3D12_SHADER_VISIBILITY {
        D3D12_SHADER_VISIBILITY(*self as i32)
    }
}

impl<'a> RootParameterType<'a> {
    pub(crate) fn as_type_raw(&self) -> D3D12_ROOT_PARAMETER_TYPE {
        match self {
            RootParameterType::DescriptorTable { .. } => D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
            RootParameterType::Constants { .. } => D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
            RootParameterType::Cbv { .. } => D3D12_ROOT_PARAMETER_TYPE_CBV,
            RootParameterType::Srv { .. } => D3D12_ROOT_PARAMETER_TYPE_SRV,
            RootParameterType::Uav { .. } => D3D12_ROOT_PARAMETER_TYPE_UAV,
        }
    }

    pub(crate) fn as_raw(&self) -> D3D12_ROOT_PARAMETER_0 {
        match self {
            RootParameterType::Cbv {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::Srv {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::Uav {
                shader_register,
                register_space,
            } => D3D12_ROOT_PARAMETER_0 {
                Descriptor: D3D12_ROOT_DESCRIPTOR {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                },
            },
            RootParameterType::DescriptorTable { ranges } => D3D12_ROOT_PARAMETER_0 {
                DescriptorTable: D3D12_ROOT_DESCRIPTOR_TABLE {
                    NumDescriptorRanges: ranges.len() as u32,
                    pDescriptorRanges: ranges.as_ptr() as *const _,
                },
            },
            RootParameterType::Constants {
                shader_register,
                register_space,
                num_32bit_values,
            } => D3D12_ROOT_PARAMETER_0 {
                Constants: D3D12_ROOT_CONSTANTS {
                    ShaderRegister: *shader_register,
                    RegisterSpace: *register_space,
                    Num32BitValues: *num_32bit_values,
                },
            },
        }
    }
}

impl VertexBufferView {
    pub(crate) fn as_raw(&self) -> D3D12_VERTEX_BUFFER_VIEW {
        D3D12_VERTEX_BUFFER_VIEW {
            BufferLocation: self.buffer_location,
            SizeInBytes: self.size_in_bytes,
            StrideInBytes: self.stride_in_bytes,
        }
    }
}

impl<'a> ResourceBarrier<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_BARRIER {
        D3D12_RESOURCE_BARRIER {
            Type: self.r#type.as_type_raw(),
            Flags: D3D12_RESOURCE_BARRIER_FLAGS(self.flags.bits()),
            Anonymous: self.r#type.as_raw(),
        }
    }
}

impl<'a> BarrierType<'a> {
    pub(crate) fn as_raw(&self) -> D3D12_RESOURCE_BARRIER_0 {
        match self {
            BarrierType::Transition {
                resource,
                subresource,
                before,
                after,
            } => D3D12_RESOURCE_BARRIER_0 {
                Transition: ManuallyDrop::new(D3D12_RESOURCE_TRANSITION_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(resource.as_raw_ref()) },
                    Subresource: *subresource,
                    StateBefore: before.as_raw(),
                    StateAfter: after.as_raw(),
                }),
            },
            BarrierType::Aliasing { before, after } => D3D12_RESOURCE_BARRIER_0 {
                Aliasing: ManuallyDrop::new(D3D12_RESOURCE_ALIASING_BARRIER {
                    pResourceBefore: unsafe { std::mem::transmute_copy(before.as_raw_ref()) },
                    pResourceAfter: unsafe { std::mem::transmute_copy(after.as_raw_ref()) },
                }),
            },
            BarrierType::Uav { resource } => D3D12_RESOURCE_BARRIER_0 {
                UAV: ManuallyDrop::new(D3D12_RESOURCE_UAV_BARRIER {
                    pResource: unsafe { std::mem::transmute_copy(resource.as_raw_ref()) },
                }),
            },
        }
    }

    pub(crate) fn as_type_raw(&self) -> D3D12_RESOURCE_BARRIER_TYPE {
        match self {
            BarrierType::Transition { .. } => D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
            BarrierType::Aliasing { .. } => D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
            BarrierType::Uav { .. } => D3D12_RESOURCE_BARRIER_TYPE_UAV,
        }
    }
}

impl From<windows::core::Error> for DxError {
    fn from(value: windows::core::Error) -> Self {
        match value.code() {
            D3D12_ERROR_ADAPTER_NOT_FOUND => DxError::AdapterNotFound,
            D3D12_ERROR_DRIVER_VERSION_MISMATCH => DxError::DriverVersionMismatch,
            E_FAIL => DxError::Fail(value.message()),
            E_INVALIDARG => DxError::InvalidArgs,
            E_OUTOFMEMORY => DxError::Oom,
            E_NOTIMPL => DxError::NotImpl,
            _ => DxError::Dxgi(value.message()),
        }
    }
}
