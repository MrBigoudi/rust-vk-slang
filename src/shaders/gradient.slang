static const int DESCRIPTOR_SET = 0;
static const int DESCRIPTOR_BINDING = 0;

[[vk::binding(DESCRIPTOR_SET, DESCRIPTOR_BINDING)]]
RWTexture2D<float4> gs_image;

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
    
        gs_image[texelCoord] = color;
    }
}