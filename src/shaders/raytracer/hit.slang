implementing raytracer;

import scene.scene; 


public struct Hit {
    public float4 coords; // (b0, b1, b2, t)
    public uint did_hit;
    public uint triangle_index;
};

public Hit ray_triangle_intersection(Ray ray, uint triangle_index, RWStructuredBuffer<Triangle> triangles, RWStructuredBuffer<Model> models){
    Hit hit;

    Triangle triangle = triangles[triangle_index];

    float3 p0 = mul(models[triangle.model_index].model_matrix, triangle.p0).xyz;
    float3 p1 = mul(models[triangle.model_index].model_matrix, triangle.p1).xyz;
    float3 p2 = mul(models[triangle.model_index].model_matrix, triangle.p2).xyz;

    float3 tri_edge_0 = p1 - p0;
    float3 tri_edge_1 = p2 - p0;
    float3 tri_normale = normalize(cross(tri_edge_0, tri_edge_1));

    float3 q = cross(ray.direction.xyz, tri_edge_1);
    float a = dot(tri_edge_0, q);
    float epsilon = 1e-4;

    if(dot(tri_normale, ray.direction.xyz) >= 0 || abs(a) < epsilon){
        hit.did_hit = 0;
        return hit;
    }

    float4 s = (ray.origin - float4(p0, 1.f)) / a;
    float3 r = cross(s.xyz, tri_edge_0);

    hit.coords.x = dot(s.xyz, q);
    hit.coords.y = dot(r, ray.direction.xyz);
    hit.coords.z = 1 - hit.coords.x - hit.coords.y;

    if(hit.coords.x < 0 || hit.coords.y < 0 || hit.coords.z < 0){
        hit.did_hit = 0;
        return hit;
    }
    
    float t = dot(tri_edge_1, r);
    if(t < 0){
        hit.did_hit = 0;
        return hit;
    }

    hit.did_hit = 1;
    hit.coords.w = t;
    hit.triangle_index = triangle_index;

    return hit;
}

public void get_closest_hit(Ray ray, uint nb_triangles, inout Hit closest_hit, RWStructuredBuffer<Triangle> triangles, RWStructuredBuffer<Model> models){
    for(uint i=0; i<nb_triangles; i++){
        Hit cur_hit = ray_triangle_intersection(ray, i, triangles, models);
        if(cur_hit.did_hit == 0) continue;
        if(closest_hit.did_hit == 0 || cur_hit.coords.w < closest_hit.coords.w){
            closest_hit = cur_hit;
        }
    }
}

public void get_color(Hit hit, inout float4 color, RWStructuredBuffer<Triangle> triangles, RWStructuredBuffer<Model> models, RWStructuredBuffer<Material> materials){
    if(hit.did_hit == 0) return;
    Triangle hit_triangle = triangles[hit.triangle_index];
    color += materials[models[hit_triangle.model_index].material_index].albedo;
}