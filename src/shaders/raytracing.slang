import scene.scene; 
import raytracer.raytracer; 

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
void main(uint3 thread_id : SV_DispatchThreadID) {
    uint2 texel_coord = thread_id.xy;

    uint2 size = uint2(0, 0); 
    _Framebuffer.GetDimensions(size.x, size.y);

    float2 pixel_position = float2(0.f);
    pixel_position.x = float(texel_coord.x) / float(size.x);
    pixel_position.y = float(texel_coord.y) / float(size.y);

    Ray ray = get_ray(pixel_position, Camera());

    // no bvh
    Hit closest_hit;
    closest_hit.did_hit = 0;
    uint nb_triangles = 1;
    get_closest_hit(ray, nb_triangles, closest_hit, _Triangles, _Models);

    float4 color = float4(0.f, 0.f, 0.f, 1.f);
    get_color(closest_hit, color, _Triangles, _Models, _Materials);

    _Framebuffer[texel_coord] = color;
}