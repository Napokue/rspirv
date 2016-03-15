// This rust module is automatically generated from  SPIRV-Tools libspirv.h:
//   https://github.com/KhronosGroup/SPIRV-Tools/blob/master/include/spirv-tools/libspirv.h

#![allow(dead_code)]

#[derive(Clone, Copy, Debug)]
pub enum OperandType {
    None,
    Id,
    FirstConcreteType,
    TypeId,
    ResultId,
    MemorySemanticsId,
    ScopeId,
    LiteralInteger,
    ExtensionInstructionNumber,
    SpecConstantOpNumber,
    TypedLiteralNumber,
    LiteralString,
    SourceLanguage,
    ExecutionModel,
    AddressingModel,
    MemoryModel,
    ExecutionMode,
    StorageClass,
    Dimensionality,
    SamplerAddressingMode,
    SamplerFilterMode,
    SamplerImageFormat,
    ImageChannelOrder,
    ImageChannelDataType,
    FpRoundingMode,
    LinkageType,
    AccessQualifier,
    FunctionParameterAttribute,
    Decoration,
    BuiltIn,
    GroupOperation,
    KernelEnqFlags,
    KernelProfilingInfo,
    Capability,
    Image,
    FirstConcreteMaskType,
    FpFastMathMode,
    SelectionControl,
    LoopControl,
    FunctionControl,
    MemoryAccess,
    LastConcreteType,
    LastConcreteMaskType,
    OptionalId,
    FirstOptionalType,
    OptionalImage,
    OptionalMemoryAccess,
    OptionalLiteralInteger,
    OptionalLiteralNumber,
    OptionalTypedLiteralInteger,
    OptionalLiteralString,
    OptionalExecutionMode,
    OptionalAccessQualifier,
    OptionalCiv,
    VariableId,
    FirstVariableType,
    VariableLiteralInteger,
    VariableLiteralIntegerId,
    VariableIdLiteralInteger,
    VariableExecutionMode,
    LastVariableType,
    LastOptionalType,
    NumOperandTypes,
}