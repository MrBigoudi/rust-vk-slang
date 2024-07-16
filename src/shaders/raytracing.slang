import scene.scene; 

//////////// FRAMEBUFFER
static const int DESCRIPTOR_SET_FRAMEBUFFER = 0;
static const int DESCRIPTOR_BINDING_FRAMEBUFFER = 0;

[[vk::binding(DESCRIPTOR_BINDING_FRAMEBUFFER, DESCRIPTOR_SET_FRAMEBUFFER)]]
RWTexture2D<float4> _Framebuffer;


//////////// TRIANGLES
static const int DESCRIPTOR_SET_TRIANGLES = 0;
static const int DESCRIPTOR_BINDING_TRIANGLES = 1;

[[vk::binding(DESCRIPTOR_BINDING_TRIANGLES, DESCRIPTOR_SET_TRIANGLES)]]
RWStructuredBuffer<Triangle> _Triangles;


//////////// MODELS
static const int DESCRIPTOR_SET_MODELS = 0;
static const int DESCRIPTOR_BINDING_MODELS = 2;

[[vk::binding(DESCRIPTOR_BINDING_MODELS, DESCRIPTOR_SET_MODELS)]]
RWStructuredBuffer<Model> _Models;


//////////// MATERIALS
static const int DESCRIPTOR_SET_MATERIALS = 0;
static const int DESCRIPTOR_BINDING_MATERIALS = 3;

[[vk::binding(DESCRIPTOR_BINDING_MATERIALS, DESCRIPTOR_SET_MATERIALS)]]
RWStructuredBuffer<Material> _Materials;


//////////// DISPATCH COMPUTE
static const int THREAD_GROUP_SIZE_X = 16;
static const int THREAD_GROUP_SIZE_Y = 16;
static const int THREAD_GROUP_SIZE_Z = 1;

[shader("compute")]
[numthreads(THREAD_GROUP_SIZE_X, THREAD_GROUP_SIZE_Y, THREAD_GROUP_SIZE_Z)]
void main(uint3 threadId : SV_DispatchThreadID)
{
    uint2 texelCoord = threadId.xy;
	uint2 size = uint2(0, 0); // initialize to arbitrary value
    _Framebuffer.GetDimensions(size.x, size.y); // get size from bound image

    if(texelCoord.x < size.x && texelCoord.y < size.y)
    {
        float4 color = _Materials[_Models[_Triangles[0].model_index].material_index].albedo;
        _Framebuffer[texelCoord] = color;
    }
}