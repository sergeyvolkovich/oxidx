use std::ffi::CStr;

use windows::{core::PCSTR, Win32::Graphics::Direct3D12::{
    ID3D12ShaderReflection, ID3D12ShaderReflectionConstantBuffer, ID3D12ShaderReflectionVariable,
}};

use crate::{create_type, error::DxError, impl_trait, types::*, HasInterface};

/// A shader-reflection interface accesses shader information.
///
/// For more information: [`ID3D12ShaderReflection interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflection)
pub trait IShaderReflection: HasInterface {
    /// Gets the number of bitwise instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetBitwiseInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getbitwiseinstructioncount)
    fn get_bitwise_instruction_count(&self) -> u32;

    /// Gets a constant buffer by index.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByIndex function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyindex)
    fn get_constant_buffer_by_index(&self, index: usize) -> Option<ShaderReflectionConstantBuffer>;

    /// Gets a constant buffer by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConstantBufferByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconstantbufferbyname)
    fn get_constant_buffer_by_name(
        &self,
        name: impl AsRef<CStr>,
    ) -> Option<ShaderReflectionConstantBuffer>;

    /// Gets the number of conversion instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetConversionInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getconversioninstructioncount)
    fn get_conversion_instruction_count(&self) -> u32;

    /// Gets a shader description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getdesc)
    fn get_desc(&self) -> Result<ShaderDesc, DxError>;

    /// Gets the geometry-shader input-primitive description.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetGsInputPrimitive function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getgsinputprimitive)
    fn get_gs_input_primitive(&self) -> Primitive;

    /// Gets an input-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetInputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getinputparameterdesc)
    fn get_input_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError>;

    /// Gets the minimum feature level.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMinFeatureLevel function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getminfeaturelevel)
    fn get_min_feature_level(&self) -> Result<FeatureLevel, DxError>;

    /// Gets the number of Movc instructions.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovcInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovcinstructioncount)
    fn get_movc_instruction_count(&self) -> u32;

    /// Gets the number of Mov instructions
    ///
    /// For more information: [`ID3D12ShaderReflection::GetMovInstructionCount function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getmovinstructioncount)
    fn get_mov_instruction_count(&self) -> u32;

    /// Gets the number of interface slots in a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetNumInterfaceSlots function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getnuminterfaceslots)
    fn get_num_interface_slots(&self) -> u32;

    /// Gets an output-parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetOutputParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getoutputparameterdesc)
    fn get_output_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError>;

    /// Gets a patch-constant parameter description for a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetPatchConstantParameterDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getpatchconstantparameterdesc)
    fn get_patch_constant_parameter_desc(
        &self,
        index: usize,
    ) -> Result<SignatureParameterDesc, DxError>;

    /// Retrieves a group of flags that indicate the requirements of a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetRequiresFlags function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getrequiresflags)
    fn get_requires_flags(&self) -> ShaderRequirements;

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDesc function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdesc)
    fn get_resource_binding_desc(&self, index: usize) -> Result<ShaderInputBindDesc, DxError>;

    /// Gets a description of how a resource is bound to a shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetResourceBindingDescByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getresourcebindingdescbyname)
    fn get_resource_binding_desc_by_name(
        &self,
        name: impl AsRef<CStr>,
    ) -> Result<ShaderInputBindDesc, DxError>;

    /// Retrieves the sizes, in units of threads, of the X, Y, and Z dimensions of the shader's thread-group grid.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetThreadGroupSize function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getthreadgroupsize)
    fn get_thread_group_size(&self) -> (u32, u32, u32, u32);

    /// Gets a variable by name.
    ///
    /// For more information: [`ID3D12ShaderReflection::GetVariableByName function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-getvariablebyname)
    fn get_variable_by_name(
        &self,
        name: impl AsRef<CStr>,
    ) -> Option<ShaderReflectionVariable>;

    /// Indicates whether a shader is a sample frequency shader.
    ///
    /// For more information: [`ID3D12ShaderReflection::IsSampleFrequencyShader function`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nf-d3d12shader-id3d12shaderreflection-issamplefrequencyshader)
    fn is_sample_frequency_shader(&self) -> bool;
}

create_type! {
    /// A shader-reflection interface accesses shader information.
    ///
    /// For more information: [`ID3D12ShaderReflection interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflection)
    ShaderReflection wrap ID3D12ShaderReflection
}

impl_trait! {
    impl IShaderReflection =>
    ShaderReflection;

    #[inline]
    fn get_bitwise_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetBitwiseInstructionCount()
        }
    }

    #[inline]
    fn get_constant_buffer_by_index(&self, index: usize) -> Option<ShaderReflectionConstantBuffer> {
        unsafe {
            self.0.GetConstantBufferByIndex(index as u32)
                .map(|v| ShaderReflectionConstantBuffer::new(v))
        }
    }

    #[inline]
    fn get_constant_buffer_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionConstantBuffer> {
        unsafe {
            let name = PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetConstantBufferByName(name)
                .map(|v| ShaderReflectionConstantBuffer::new(v))
        }
    }

    #[inline]
    fn get_conversion_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetConversionInstructionCount()
        }
    }

    #[inline]
    fn get_desc(&self) -> Result<ShaderDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetDesc(&mut raw).map_err(DxError::from)?;

            Ok(ShaderDesc(raw))
        }
    }

    #[inline]
    fn get_gs_input_primitive(&self) -> Primitive {
        unsafe {
            self.0.GetGSInputPrimitive().into()
        }
    }

    #[inline]
    fn get_input_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetInputParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    #[inline]
    fn get_min_feature_level(&self) -> Result<FeatureLevel, DxError> {
        unsafe {
            self.0.GetMinFeatureLevel()
                .map(|v| v.into())
                .map_err(DxError::from)
        }
    }

    #[inline]
    fn get_movc_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetMovcInstructionCount()
        }
    }

    #[inline]
    fn get_mov_instruction_count(&self) -> u32 {
        unsafe {
            self.0.GetMovInstructionCount()
        }
    }

    #[inline]
    fn get_num_interface_slots(&self) -> u32 {
        unsafe {
            self.0.GetNumInterfaceSlots()
        }
    }

    #[inline]
    fn get_output_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetOutputParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    #[inline]
    fn get_patch_constant_parameter_desc(&self, index: usize) -> Result<SignatureParameterDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetPatchConstantParameterDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(SignatureParameterDesc(raw))
        }
    }

    #[inline]
    fn get_requires_flags(&self) -> ShaderRequirements {
        unsafe {
            self.0.GetRequiresFlags().into()
        }
    }

    #[inline]
    fn get_resource_binding_desc(&self, index: usize) -> Result<ShaderInputBindDesc, DxError> {
        unsafe {
            let mut raw = Default::default();
            self.0.GetResourceBindingDesc(index as u32, &mut raw).map_err(DxError::from)?;

            Ok(ShaderInputBindDesc(raw))
        }
    }

    #[inline]
    fn get_resource_binding_desc_by_name(&self, name: impl AsRef<CStr>) -> Result<ShaderInputBindDesc, DxError> {
        unsafe {
            let name = PCSTR::from_raw(name.as_ref().as_ptr() as *const _);
            let mut raw = Default::default();

            self.0.GetResourceBindingDescByName(name, &mut raw).map_err(DxError::from)?;
            Ok(ShaderInputBindDesc(raw))
        }
    }

    #[inline]
    fn get_thread_group_size(&self) -> (u32, u32, u32, u32) {
        unsafe {
            let mut x = 0;
            let mut y = 0;
            let mut z = 0;
            let total = self.0.GetThreadGroupSize(Some(&mut x), Some(&mut y), Some(&mut z));

            (x, y, z, total)
        }
    }

    #[inline]
    fn get_variable_by_name(&self, name: impl AsRef<CStr>) -> Option<ShaderReflectionVariable> {
        unsafe {
            let name = PCSTR::from_raw(name.as_ref().as_ptr() as *const _);

            self.0.GetVariableByName(name)
                .map(|v| ShaderReflectionVariable::new(v))
        }
    }

    #[inline]
    fn is_sample_frequency_shader(&self) -> bool {
        unsafe {
            self.0.IsSampleFrequencyShader().into()
        }
    }
}

/// This shader-reflection interface provides access to a constant buffer.
///
/// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
pub trait IShaderReflectionConstantBuffer: HasInterface {}

create_type! {
    /// This shader-reflection interface provides access to a constant buffer.
    ///
    /// For more information: [`ID3D12ShaderReflectionConstantBuffer interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionconstantbuffer)
    ShaderReflectionConstantBuffer wrap ID3D12ShaderReflectionConstantBuffer
}

impl_trait! {
    impl IShaderReflectionConstantBuffer =>
    ShaderReflectionConstantBuffer;
}

/// This shader-reflection interface provides access to a variable.
///
/// For more information: [`ID3D12ShaderReflectionVariable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionvariable)
pub trait IShaderReflectionVariable: HasInterface {}

create_type! {
    /// This shader-reflection interface provides access to a variable.
    ///
    /// For more information: [`ID3D12ShaderReflectionVariable interface`](https://learn.microsoft.com/en-us/windows/win32/api/d3d12shader/nn-d3d12shader-id3d12shaderreflectionvariable)
    ShaderReflectionVariable wrap ID3D12ShaderReflectionVariable
}

impl_trait! {
    impl IShaderReflectionVariable =>
    ShaderReflectionVariable;
}
