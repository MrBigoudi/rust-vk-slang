static const int DESCRIPTOR_SET = 0;
static const int DESCRIPTOR_BINDING_OUTPUT_IMAGE = 0;
static const int DESCRIPTOR_BINDING_TRIANGLES = 1;

[[vk::binding(DESCRIPTOR_BINDING_OUTPUT_IMAGE, DESCRIPTOR_SET)]]
RWTexture2D<float4> gs_image;

struct Triangle {
    float4 p0;
    float4 p1;
    float4 p2;
    uint model;
};

[[vk::binding(DESCRIPTOR_BINDING_TRIANGLES, DESCRIPTOR_SET)]]
RWStructuredBuffer<Triangle> triangles;

static const int THREAD_GROUP_SIZE_X = 16;
static const int THREAD_GROUP_SIZE_Y = 16;
static const int THREAD_GROUP_SIZE_Z = 1;

[shader("compute")]
[numthreads(THREAD_GROUP_SIZE_X, THREAD_GROUP_SIZE_Y, THREAD_GROUP_SIZE_Z)]
void main(uint3 threadId : SV_DispatchThreadID)
{
    uint2 texelCoord = threadId.xy;
	uint2 size = uint2(0, 0); // initialize to arbitrary value
    gs_image.GetDimensions(size.x, size.y); // get size from bound image

    if(texelCoord.x < size.x && texelCoord.y < size.y)
    {
        float4 color = float4(0.0, 0.0, 0.0, 1.0);

        if(texelCoord.x % THREAD_GROUP_SIZE_X != 0 && texelCoord.y % THREAD_GROUP_SIZE_Y != 0){
            color.x = float(texelCoord.x) / float(size.x);
            color.y = float(texelCoord.y) / float(size.y);
        }
    
        color = triangles[0].p1;
        gs_image[texelCoord] = color;
    }
}